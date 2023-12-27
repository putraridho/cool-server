use axum::{middleware, Router};
use dotenv::dotenv;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tracing::info;

pub use error::Result;
use tracing_subscriber::EnvFilter;

use crate::{
    config::config,
    model::{DbConfig, ModelManager},
};

mod config;
mod error;
mod log;
mod migration;
mod model;
mod mw;
mod router;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let config = config();

    tracing_subscriber::fmt()
        .without_time() // For early local development.
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mm = ModelManager::new(DbConfig {
        url: &config.DB_URL,
        username: &config.DB_USERNAME,
        password: &config.DB_PASSWORD,
        ns: &config.DB_NS,
        db_name: &config.DB_NAME,
    })
    .await?;

    let routes_all = Router::new()
        .merge(router::routes(mm))
        .layer(middleware::map_response(mw::response_map))
        .layer(CookieManagerLayer::new());

    let listener = TcpListener::bind(format!("{}:{}", &config.SERVICE_HOST, &config.SERVICE_PORT))
        .await
        .unwrap();
    info!(
        "{:<12} -> {:?}\n",
        "LISTENING",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
