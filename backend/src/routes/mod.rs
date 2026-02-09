use axum::{routing::{get, post, put, delete}, Router};

use crate::{handlers, middleware::auth::AuthUser, AppState};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::auth::register))
        .route("/login", post(handlers::auth::login))
}

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/me", get(handlers::users::me))
        .route_layer(axum::middleware::from_extractor::<AuthUser>())
}

pub fn item_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::items::list_items).post(handlers::items::create_item))
        .route(
            "/:id",
            put(handlers::items::update_item).delete(handlers::items::delete_item),
        )
        .route_layer(axum::middleware::from_extractor::<AuthUser>())
}
