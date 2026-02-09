use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::{
    middleware::auth::{issue_token, Claims},
    models::{Role, User, UserPublic},
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserPublic,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<impl IntoResponse, AppError> {
    let password_hash = hash_password(&payload.password)?;
    let user_id = Uuid::new_v4();

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, email, password_hash, role)
        VALUES ($1, $2, $3, 'User')
        RETURNING id, email, password_hash, role, created_at
        "#,
    )
    .bind(user_id)
    .bind(&payload.email)
    .bind(&password_hash)
    .fetch_one(&state.db)
    .await
    .map_err(AppError::Database)?;

    let claims = Claims::new(user.id, user.role.clone(), std::time::Duration::from_secs(60 * 60 * 24));
    let token = issue_token(&state.jwt, &claims)?;

    Ok((StatusCode::CREATED, Json(AuthResponse {
        token,
        user: user.into(),
    })))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, role, created_at
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(&payload.email)
    .fetch_optional(&state.db)
    .await
    .map_err(AppError::Database)?
    .ok_or(AppError::Unauthorized)?;

    verify_password(&payload.password, &user.password_hash)?;

    let claims = Claims::new(user.id, user.role.clone(), std::time::Duration::from_secs(60 * 60 * 24));
    let token = issue_token(&state.jwt, &claims)?;

    Ok(Json(AuthResponse {
        token,
        user: user.into(),
    }))
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppError::Crypto)?
        .to_string();
    Ok(hash)
}

fn verify_password(password: &str, password_hash: &str) -> Result<(), AppError> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|_| AppError::Crypto)?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized)
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("database error")]
    Database(#[from] sqlx::Error),
    #[error("crypto error")]
    Crypto,
    #[error("token error")]
    Token,
}

impl From<crate::middleware::auth::AuthError> for AppError {
    fn from(_: crate::middleware::auth::AuthError) -> Self {
        AppError::Token
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::Crypto => (StatusCode::INTERNAL_SERVER_ERROR, "Crypto error"),
            AppError::Token => (StatusCode::UNAUTHORIZED, "Token error"),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
