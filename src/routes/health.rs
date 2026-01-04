use crate::db::DBPool;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use diesel::RunQueryDsl;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    status: String,
}

pub async fn health_check(
    State(pool): State<DBPool>,
) -> Result<Json<HealthCheckResponse>, (StatusCode, Json<HealthCheckResponse>)> {
    let mut conn = pool.get().map_err(|e| {
        tracing::error!("Failed to get database connection: {}", e);
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(HealthCheckResponse {
                status: "unhealthy".to_string(),
            }),
        )
    })?;

    diesel::sql_query("SELECT 1")
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to retrieve database connection: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(HealthCheckResponse {
                    status: "unhealthy".to_string(),
                }),
            )
        })?;

    Ok(Json(HealthCheckResponse {
        status: "ok".to_string(),
    }))
}
