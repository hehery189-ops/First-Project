use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod middleware;
mod models;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub jwt: middleware::auth::JwtConfig,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret".to_string());

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let state = AppState {
        db,
        jwt: middleware::auth::JwtConfig::new(jwt_secret),
    };

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .nest("/auth", routes::auth_routes())
        .nest("/users", routes::user_routes())
        .nest("/items", routes::item_routes())
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on {addr}");
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}
