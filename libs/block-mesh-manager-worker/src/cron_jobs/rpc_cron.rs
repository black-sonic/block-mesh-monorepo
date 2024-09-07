use crate::db_calls::create_server_user::create_server_user;
use crate::db_calls::get_all_rpcs::get_all_rpcs;
use anyhow::Context;
use block_mesh_common::constants::BLOCKMESH_SERVER_UUID_ENVAR;
use sqlx::PgPool;
use std::env;
use std::time::Duration;
use uuid::Uuid;

#[tracing::instrument(name = "create_rpc_tasks", level = "trace", skip(pool))]
pub async fn create_rpc_tasks(pool: PgPool) -> anyhow::Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .context("Failed to create DB transaction")?;
    let uuid = Uuid::parse_str(env::var(BLOCKMESH_SERVER_UUID_ENVAR).unwrap().as_str()).unwrap();
    for rpc in get_all_rpcs(&mut transaction).await? {
        rpc.create_rpc_task(&mut transaction, &uuid).await?;
    }
    transaction
        .commit()
        .await
        .context("Failed to commit DB transaction")?;
    Ok(())
}

#[tracing::instrument(name = "rpc_worker_loop", skip(pool))]
pub async fn rpc_worker_loop(pool: PgPool) -> Result<(), anyhow::Error> {
    let interval = env::var("RPC_CRON_INTERVAL")
        .unwrap_or("30000".to_string())
        .parse()
        .unwrap_or(30_000);
    let mut transaction = pool
        .begin()
        .await
        .context("Failed to create DB transaction")?;
    create_server_user(&mut transaction).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit DB transaction")?;
    loop {
        match create_rpc_tasks(pool.clone()).await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("worker_loop: create_rpc_tasks: error: {}", e);
            }
        }
        tokio::time::sleep(Duration::from_millis(interval)).await;
    }
}