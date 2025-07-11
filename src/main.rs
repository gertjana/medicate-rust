mod config;
mod handlers;
mod models;
mod repositories;

use axum::{
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::Config;
use handlers::{medicine_handlers, schedule_handlers, dosage_history_handlers};
use repositories::{MedicineRepository, MedicineScheduleRepository, DosageHistoryRepository};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    tracing::info!("Server running on port: {}", config.server_port);
    tracing::info!("Redis connection: {}:{}", config.redis_host, config.redis_port);

    // Initialize repositories
    let medicine_repo = Arc::new(
        MedicineRepository::new(&config.redis_url(), "prod:medicine:".to_string())?
    );
    let _schedule_repo = Arc::new(
        MedicineScheduleRepository::new(&config.redis_url(), "prod:schedule:".to_string())?
    );
    let dosage_history_repo = Arc::new(
        DosageHistoryRepository::new(&config.redis_url(), "prod:dosage:".to_string())?
    );

    // Create a tuple of repositories for schedule routes
    let schedule_repos = Arc::new((
        MedicineRepository::new(&config.redis_url(), "prod:medicine:".to_string())?,
        MedicineScheduleRepository::new(&config.redis_url(), "prod:schedule:".to_string())?
    ));

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(Any);

    // Build application with routes
    let app = Router::new()
        .merge(medicine_handlers::medicine_routes().with_state(medicine_repo))
        .merge(schedule_handlers::schedule_routes().with_state(schedule_repos))
        .merge(dosage_history_handlers::dosage_history_routes().with_state(dosage_history_repo))
        .route("/health", get(health_check))
        .layer(cors);

    // Run server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.server_port)).await?;
    tracing::info!("Listening on http://0.0.0.0:{}", config.server_port);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
