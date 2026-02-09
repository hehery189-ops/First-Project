use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    handlers::auth::AppError,
    middleware::auth::AuthUser,
    models::Item,
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct ItemPayload {
    pub title: String,
    pub description: Option<String>,
}

pub async fn list_items(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let items = sqlx::query_as::<_, Item>(
        r#"
        SELECT id, owner_id, title, description, created_at
        FROM items
        WHERE owner_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(auth.id)
    .fetch_all(&state.db)
    .await
    .map_err(AppError::Database)?;

    Ok(Json(items))
}

pub async fn create_item(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(payload): Json<ItemPayload>,
) -> Result<impl IntoResponse, AppError> {
    let item = sqlx::query_as::<_, Item>(
        r#"
        INSERT INTO items (id, owner_id, title, description)
        VALUES ($1, $2, $3, $4)
        RETURNING id, owner_id, title, description, created_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(auth.id)
    .bind(&payload.title)
    .bind(&payload.description)
    .fetch_one(&state.db)
    .await
    .map_err(AppError::Database)?;

    Ok((StatusCode::CREATED, Json(item)))
}

pub async fn update_item(
    State(state): State<AppState>,
    auth: AuthUser,
    axum::extract::Path(item_id): axum::extract::Path<Uuid>,
    Json(payload): Json<ItemPayload>,
) -> Result<impl IntoResponse, AppError> {
    let item = sqlx::query_as::<_, Item>(
        r#"
        UPDATE items
        SET title = $1,
            description = $2
        WHERE id = $3 AND owner_id = $4
        RETURNING id, owner_id, title, description, created_at
        "#,
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(item_id)
    .bind(auth.id)
    .fetch_one(&state.db)
    .await
    .map_err(AppError::Database)?;

    Ok(Json(item))
}

pub async fn delete_item(
    State(state): State<AppState>,
    auth: AuthUser,
    axum::extract::Path(item_id): axum::extract::Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    sqlx::query(
        r#"
        DELETE FROM items
        WHERE id = $1 AND owner_id = $2
        "#,
    )
    .bind(item_id)
    .bind(auth.id)
    .execute(&state.db)
    .await
    .map_err(AppError::Database)?;

    Ok(StatusCode::NO_CONTENT)
}
