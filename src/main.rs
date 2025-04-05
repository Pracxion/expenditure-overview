use axum::{Router,
           routing::get};
use templates::{app::app,
                index::index};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt,
                         util::SubscriberInitExt};

mod templates;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "expenditure-overview=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Initializing Router ..");

    let assets_path = std::env::current_dir().unwrap();
    let router = Router::new()
        .route("/", get(index))
        .route("/app", get(app))
        .nest_service("/assets", ServeDir::new(format!("{}/templates/assets", assets_path.to_str().unwrap())));
    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    info!("Router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router.into_make_service()).await.unwrap();
}
