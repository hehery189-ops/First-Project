use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: Role,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub email: String,
    pub role: Role,
}

impl From<User> for UserPublic {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            role: user.role,
        }
    }
}
