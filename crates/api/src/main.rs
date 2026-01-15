use std::sync::Arc;

use axum::{
    Router,
    http::{HeaderValue, Method},
    routing::{get, post},
};
use database::{create_pool, referral_repo::ReferralRepo};
use services::referral_service::ReferralService;
use tower_http::cors::{Any, CorsLayer};

mod handlers;

pub struct AppState {
    pub referral_service: ReferralService,
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    // Logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // DB pool
    let pool = create_pool();

    let referral_service = ReferralService::new(ReferralRepo::new(pool));

    let app_state = Arc::new(AppState { referral_service });

    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("https://bitdca.segmento.tech"),
            HeaderValue::from_static("http://localhost:3000"),
            HeaderValue::from_static("http://localhost:5173"),
        ])
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    let app = Router::new()
        .route(
            "/create-owner",
            post(handlers::create_owner).options(handlers::options_ok),
        )
        .route(
            "/redemption/{code}",
            get(handlers::create_redemption).options(handlers::options_ok),
        )
        .route(
            "/export/project/{project}",
            get(handlers::export_project_csv).options(handlers::options_ok),
        )
        .route(
            "/export",
            get(handlers::export_bitdca_csv).options(handlers::options_ok),
        )
        .route("/health", get(handlers::health_check))
        .with_state(app_state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
