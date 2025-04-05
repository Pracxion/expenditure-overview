use axum::{Router,
           extract::DefaultBodyLimit,
           routing::{get,
                     post}};
use config::env_config::Config;
use database::maintenance::{connection_pool,
                            initialize_database};
use templates::{app::app,
                index::index};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt,
                         util::SubscriberInitExt};
use upload::upload_csv::upload_csv;

mod config;
mod database;
mod templates;
mod upload;

const BODY_SIZE_LIMIT: usize = 32 * 1024 * 1024;

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
        .route("/upload-csv", post(upload_csv))
        .nest_service("/assets", ServeDir::new(format!("{}/templates/assets", assets_path.to_str().unwrap())))
        .with_state(connection)
        .layer(DefaultBodyLimit::max(BODY_SIZE_LIMIT))
        .layer(tower_http::limit::RequestBodyLimitLayer::new(BODY_SIZE_LIMIT));

    let listener = tokio::net::TcpListener::bind(config.server_address()).await.unwrap();
    axum::serve(listener, router.into_make_service()).await.unwrap();
}
