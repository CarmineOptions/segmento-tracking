use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use database::{create_pool, referral_repo::ReferralRepo};
use services::referral_service::ReferralService;

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

    let app = Router::new()
        .route("/create-owner", post(handlers::create_owner))
        .route("/redemption/:code", get(handlers::create_redemption))
        .route("/health", get(handlers::health_check))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
