use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    handlers::auth::AppError,
    middleware::auth::AuthUser,
    models::{User, UserPublic},
    AppState,
};

pub async fn me(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, role, created_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(auth.id)
    .fetch_one(&state.db)
    .await
    .map_err(AppError::Database)?;

    Ok(Json(UserPublic::from(user)))
}
