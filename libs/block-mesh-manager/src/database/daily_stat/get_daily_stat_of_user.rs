use crate::domain::daily_stat::DailyStat;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "get_daily_stat_of_user", skip(transaction), ret, err)]
pub(crate) async fn get_daily_stat_of_user(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
) -> anyhow::Result<DailyStat> {
    let now = Utc::now();
    let day = now.date_naive();
    let dail_stat = sqlx::query_as!(
        DailyStat,
        r#"
        SELECT
        id, created_at, user_id, tasks_count, status, day, uptime, updated_at
        FROM daily_stats
        WHERE user_id = $1 AND day = $2
        LIMIT 1"#,
        user_id,
        day,
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(dail_stat)
}
