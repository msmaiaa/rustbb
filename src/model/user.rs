use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

use crate::auth::HashedString;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ForumUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

cfg_if!(
    if #[cfg(feature = "ssr")] {
        impl ForumUser {
            pub async fn find_by_username_or_email(pool: &sqlx::Pool<sqlx::Postgres>, username: &str, email: &str) -> Result<Self, sqlx::Error> {
                sqlx::query_as!(Self,
                    r#"
                    SELECT * FROM forum_user WHERE username = $1 OR email = $2
                    "#,
                    username,
                    email
                )
                .fetch_one(pool).await
            }

            pub async fn create(pool: &sqlx::Pool<sqlx::Postgres>, username: &str, email: &str, password: HashedString) -> Result<sqlx::postgres::PgRow, sqlx::Error> {
                sqlx::query_as!(Self,
                    r#"
                    INSERT INTO forum_user (username, email, password)
                    VALUES ($1, $2, $3)
                    "#,
                    username,
                    email,
                    password.as_ref()
                )
                .fetch_one(pool).await
            }

            pub async fn find_by_email(pool: &sqlx::Pool<sqlx::Postgres>, email: &str) -> Result<Self, sqlx::Error> {
                sqlx::query_as!(Self,
                    r#"
                    SELECT * FROM forum_user WHERE email = $1
                    "#,
                    email
                )
                .fetch_one(pool).await
            }

            pub async fn find_by_id(pool: &sqlx::Pool<sqlx::Postgres>, id: i32) -> Result<Self, sqlx::Error> {
                sqlx::query_as!(Self,
                    r#"
                    SELECT * FROM forum_user WHERE id = $1
                    "#,
                    id
                )
                .fetch_one(pool).await
            }
        }
    }
);
