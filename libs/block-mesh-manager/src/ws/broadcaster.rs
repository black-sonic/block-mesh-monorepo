use crate::ws::ws_loop::ws_loop;
use anyhow::Context;
use block_mesh_common::constants::BLOCKMESH_SERVER_UUID_ENVAR;
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use dashmap::DashMap;
use futures::future::join_all;
use sqlx::PgPool;
use std::collections::VecDeque;
use std::env;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::broadcast::error::SendError;
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Broadcaster {
    pub global_transmitter: broadcast::Sender<WsServerMessage>,
    pub sockets: Arc<DashMap<(Uuid, String), mpsc::Sender<WsServerMessage>>>,
    pub queue: Arc<Mutex<VecDeque<(Uuid, String)>>>,
}

impl Default for Broadcaster {
    fn default() -> Self {
        Self::new()
    }
}

impl Broadcaster {
    #[tracing::instrument(name = "new", skip_all)]
    pub fn new() -> Self {
        let (global_transmitter, _) = broadcast::channel(10000);
        Self {
            global_transmitter,
            sockets: Arc::new(DashMap::new()),
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    #[tracing::instrument(name = "broadcast", skip_all)]
    pub fn broadcast(&self, message: WsServerMessage) -> Result<usize, SendError<WsServerMessage>> {
        let subscribers = self.global_transmitter.send(message.clone())?;
        tracing::info!("Sent {message:?} to {subscribers} subscribers");
        Ok(subscribers)
    }

    #[tracing::instrument(name = "batch", skip_all)]
    pub async fn batch(&self, message: WsServerMessage, targets: &[(Uuid, String)]) {
        join_all(targets.iter().filter_map(|target| {
            if let Some(entry) = self.sockets.get(target) {
                let sink_tx = entry.value().clone();
                let msg = message.clone();
                let future = async move {
                    if let Err(_error) = sink_tx.send(msg).await {
                        tracing::error!("Batch broadcast failed");
                    }
                };
                Some(future)
            } else {
                None
            }
        }))
        .await;
    }

    #[tracing::instrument(name = "move_queue", skip_all)]
    pub fn move_queue(&self, count: usize) -> Vec<(Uuid, String)> {
        let queue = &mut self.queue.lock().unwrap();
        let count = count.min(queue.len());
        let drained: Vec<(Uuid, String)> = queue.drain(0..count).collect();
        queue.extend(drained.clone());
        drained
    }

    #[tracing::instrument(name = "broadcast_to_user", skip_all)]
    pub async fn broadcast_to_user(
        &self,
        messages: impl IntoIterator<Item = WsServerMessage> + Clone,
        id: (Uuid, String),
    ) {
        let entry = self.sockets.get(&id);
        let msgs = messages.clone();
        if let Some(entry) = entry {
            let tx = entry.value().clone();
            for msg in msgs {
                if let Err(error) = tx.send(msg).await {
                    tracing::error!("Error while queuing WS message: {error}");
                }
            }
        }
    }

    /// returns a number of nodes to which [`WsServerMessage`]s were sent
    #[tracing::instrument(name = "queue_multiple", skip_all)]
    pub async fn queue_multiple(
        &self,
        messages: impl IntoIterator<Item = WsServerMessage> + Clone,
        count: usize,
    ) -> Vec<(Uuid, String)> {
        let drained = self.move_queue(count);
        join_all(drained.clone().into_iter().filter_map(|user_id| {
            if let Some(entry) = self.sockets.get(&user_id) {
                let tx = entry.value().clone();
                let msgs = messages.clone();
                Some(async move {
                    for msg in msgs {
                        if let Err(error) = tx.send(msg).await {
                            tracing::error!("Error while queuing WS message: {error}");
                        }
                    }
                })
            } else {
                None
            }
        }))
        .await;
        drained
    }

    #[tracing::instrument(name = "subscribe", skip_all)]
    pub fn subscribe(
        &self,
        user_id: Uuid,
        ip: String,
        sink_sender: mpsc::Sender<WsServerMessage>,
    ) -> broadcast::Receiver<WsServerMessage> {
        let _ = self
            .sockets
            .insert((user_id, ip.clone()), sink_sender.clone());
        let queue = &mut self.queue.lock().unwrap();
        queue.push_back((user_id, ip));
        self.global_transmitter.subscribe()
    }

    #[tracing::instrument(name = "unsubscribe", skip_all)]
    pub fn unsubscribe(&self, user_id: Uuid, ip: String) {
        self.sockets.remove(&(user_id, ip.clone()));
        let queue = &mut self.queue.lock().unwrap();
        if let Some(pos) = queue.iter().position(|(a, b)| a == &user_id && b == &ip) {
            queue.remove(pos);
        } else {
            tracing::error!("Failed to remove a socket from the queue");
        }
    }

    #[tracing::instrument(name = "cron_reports", skip_all)]
    pub async fn cron_reports(
        &mut self,
        period: Duration,
        messages: impl Into<Vec<WsServerMessage>> + Clone + Send + 'static,
        window_size: usize,
        pool: PgPool,
    ) -> anyhow::Result<()> {
        let broadcaster = self.clone();
        let pool = pool.clone();
        let user_id = Uuid::parse_str(
            env::var(BLOCKMESH_SERVER_UUID_ENVAR)
                .context("Could not find SERVER_UUID env var")?
                .as_str(),
        )
        .context("SERVER_UUID evn var contains invalid UUID value")?;

        let _cron_task = tokio::spawn(async move {
            let _ = ws_loop(&pool, &user_id, period, messages, window_size, broadcaster).await;
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use std::collections::VecDeque;
    use uuid::Uuid;

    pub fn create_queue(size: usize) -> VecDeque<Uuid> {
        let mut queue: VecDeque<Uuid> = VecDeque::new();
        for _ in 0..size {
            queue.push_back(Uuid::new_v4());
        }
        queue
    }
    #[test]
    pub fn test_deque_perf_drain() {
        let mut q = create_queue(50_000);
        use std::time::Instant;
        let now = Instant::now();
        let count = 1_000;
        let drained: Vec<Uuid> = q.drain(0..count).collect();
        q.extend(drained.iter());
        let elapsed = now.elapsed();
        assert!(elapsed.as_micros() < 30);
    }

    #[test]
    pub fn test_deque_perf_find() {
        let q = create_queue(50_000);
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            let random_number = rng.gen_range(0..=49_000);
            use std::time::Instant;
            let now = Instant::now();
            let user_id = q[random_number].clone();
            let _ = q.iter().position(|a| a == &user_id);
            let elapsed = now.elapsed();
            assert!(elapsed.as_micros() < 1_000);
        }
    }
}
