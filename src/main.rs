use crate::env::EnvConfig;
use axum::Router;

mod db;
mod env;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let envs = EnvConfig::get();
    tracing_subscriber::fmt().json().init();

    let pool = db::create_db_pool(&envs.database_url)?;
    tracing::info!("Database pool created successfully.");

    let app = Router::new().merge(routes::routes()).with_state(pool);

    let addr = format!("{}:{}", envs.server_host, envs.server_port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Server listening on {}.", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
