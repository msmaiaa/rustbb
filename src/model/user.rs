use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};
use crate::database;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ForumUser {
    pub id: Thing,
    pub username: String,
    pub email: String,
    pub password: String,
    pub user_group: Thing,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::auth::HashedString;
    use crate::database::SurrealPool;
    impl ForumUser {
        pub async fn find_by_username_or_email(pool: &SurrealPool, username: &str, email: &str) -> Result<Option<ForumUser>, surrealdb::Error> {
            pool.query(format!("SELECT * FROM forum_user WHERE username = '{}' OR email = '{}'", username, email)).await?.take(0)
        }

        pub async fn create(pool: &SurrealPool, username: &str, email: &str, password: HashedString, user_group: Thing) -> Result<ForumUser, surrealdb::Error> {
            pool
                .create("user")
                .content(Self {
                    id: Thing {
                        id: Id::ulid(),
                        tb: "user".to_string()
                    },
                    username: username.to_string(),
                    email: email.to_string(),
                    password: password.to_string(),
                    user_group: user_group,
                    avatar_url: None,
                    created_at: chrono::offset::Utc::now()
                })
                .await
        }

        pub async fn find_by_email(pool: &SurrealPool, email: &str) -> Result<Option<ForumUser>, surrealdb::Error> {
            pool.query(format!("SELECT * FROM user WHERE email = '{}'", email)).await?.take(0)
        }

        // #[allow(dead_code)]
        // pub async fn find_by_username(pool: &SurrealPool, username: &str) -> Result<ForumUser, surrealdb::Error> {
        //     sqlx::query_as!(Self,
        //         r#"
        //         SELECT * FROM forum_user WHERE username = $1
        //         "#,
        //         username
        //     )
        //     .fetch_one(pool).await
        // }
        fn test() {

            }
        pub async fn find_by_id(pool: &SurrealPool, id: String) -> Result<ForumUser, surrealdb::Error> {
            pool.select::<Option<ForumUser>>(("user", id)).await
        }
    }
}
}
