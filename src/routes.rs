use crate::db::DBPool;
use crate::routes::health::health_check;
use axum::Router;
use axum::routing::get;

pub mod health;

pub fn routes() -> Router<DBPool> {
    Router::new().route("/health", get(health_check))
}
