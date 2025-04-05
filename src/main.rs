use axum::{Router,
           routing::get};
use config::env_config::Config;
use database::maintenance::{connection_pool,
                            initialize_database};
use templates::{app::app,
                index::index};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{fmt::init,
                         layer::SubscriberExt,
                         util::SubscriberInitExt};

mod config;
mod database;
mod templates;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "expenditure-overview=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    initialize_database().await;
    let connection = connection_pool().await;
    let config = Config::default();

    info!("Initializing Router ..");

    let assets_path = std::env::current_dir().unwrap();
    let router = Router::new()
        .route("/", get(index))
        .route("/app", get(app))
        .nest_service("/assets", ServeDir::new(format!("{}/templates/assets", assets_path.to_str().unwrap())))
        .with_state(connection);

    let listener = tokio::net::TcpListener::bind(config.server_address()).await.unwrap();
    axum::serve(listener, router.into_make_service()).await.unwrap();
}
