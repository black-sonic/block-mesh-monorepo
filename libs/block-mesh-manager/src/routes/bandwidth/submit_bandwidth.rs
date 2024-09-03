use crate::database::aggregate::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name;
use crate::database::api_token::find_token::find_token;
use crate::database::bandwidth::delete_bandwidth_reports_by_time::delete_bandwidth_reports_by_time;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::domain::aggregate::AggregateName;
use crate::errors::error::Error;
use crate::middlewares::rate_limit::filter_request;
use crate::startup::application::AppState;
use crate::worker::aggregate_agg::AggregateMessage;
use anyhow::Context;
use axum::extract::State;
use axum::{Extension, Json};
use block_mesh_common::feature_flag_client::FlagValue;
use block_mesh_common::interfaces::server_api::{ReportBandwidthRequest, ReportBandwidthResponse};
use http::{HeaderMap, StatusCode};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::task::JoinHandle;

#[tracing::instrument(name = "submit_bandwidth", skip(pool, body, state), level = "trace", fields(email = body.email), ret)]
pub async fn handler(
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<ReportBandwidthRequest>,
) -> Result<Json<ReportBandwidthResponse>, Error> {
    let ip = headers
        .get("cf-connecting-ip")
        .context("Missing CF-CONNECTING-IP")?
        .to_str()
        .context("Unable to STR CF-CONNECTING-IP")?;
    let mut redis = state.redis.clone();
    if !filter_request(&mut redis, &body.api_token, ip)
        .await
        .context("Rate limit")?
    {
        return Err(Error::NotAllowedRateLimit);
    }

    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let api_token = find_token(&mut transaction, &body.api_token)
        .await?
        .ok_or(Error::ApiTokenNotFound)?;
    let user = get_user_opt_by_id(&mut transaction, &api_token.user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    if user.email.to_ascii_lowercase() != body.email.to_ascii_lowercase() {
        return Err(Error::UserNotFound);
    }

    let download_speed = serde_json::Value::from(body.download_speed)
        .as_f64()
        .unwrap_or_default();
    let upload_speed = serde_json::Value::from(body.upload_speed)
        .as_f64()
        .unwrap_or_default();
    let latency_report = serde_json::Value::from(body.latency)
        .as_f64()
        .unwrap_or_default();

    let download = get_or_create_aggregate_by_user_and_name(
        &mut transaction,
        AggregateName::Download,
        user.id,
    )
    .await?;
    let upload =
        get_or_create_aggregate_by_user_and_name(&mut transaction, AggregateName::Upload, user.id)
            .await?;

    let latency =
        get_or_create_aggregate_by_user_and_name(&mut transaction, AggregateName::Latency, user.id)
            .await?;
    transaction.commit().await.map_err(Error::from)?;

    let _ = state.tx_aggregate_agg.try_send(AggregateMessage {
        id: download.id.unwrap_or_default(),
        value: serde_json::Value::from(
            (download.value.as_f64().unwrap_or_default() + download_speed) / 2.0,
        ),
    });
    let _ = state.tx_aggregate_agg.try_send(AggregateMessage {
        id: upload.id.unwrap_or_default(),
        value: serde_json::Value::from(
            (upload.value.as_f64().unwrap_or_default() + upload_speed) / 2.0,
        ),
    });
    let _ = state.tx_aggregate_agg.try_send(AggregateMessage {
        id: latency.id.unwrap_or_default(),
        value: serde_json::Value::from(
            (latency.value.as_f64().unwrap_or_default() + latency_report) / 2.0,
        ),
    });

    let flag = state
        .flags
        .get("submit_bandwidth_run_background")
        .unwrap_or(&FlagValue::Boolean(false));
    let flag: bool = <FlagValue as TryInto<bool>>::try_into(flag.to_owned()).unwrap_or_default();
    if flag {
        let handle: JoinHandle<()> = tokio::spawn(async move {
            let mut transaction = pool.begin().await.map_err(Error::from).unwrap();
            delete_bandwidth_reports_by_time(&mut transaction, user.id, 60 * 60)
                .await
                .map_err(Error::from)
                .unwrap();
            transaction.commit().await.map_err(Error::from).unwrap();
        });
        let _ = state.tx.try_send(handle);
    }

    Ok(Json(ReportBandwidthResponse {
        status_code: u16::from(StatusCode::OK),
    }))
}
