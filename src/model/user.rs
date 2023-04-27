use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[allow(unused)]
use surrealdb::sql::{Id, Thing};

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
    use crate::database::SurrealClient;
    impl ForumUser {
        pub async fn find_by_username_or_email(db: &SurrealClient, username: &str, email: &str) -> Result<Option<ForumUser>, surrealdb::Error> {
            db
                .query("SELECT * FROM user WHERE username = $username OR email = $email")
                .bind(("username", username))
                .bind(("email", email))
                .await?.take(0)
        }

        pub async fn create(db: &SurrealClient, username: &str, email: &str, password: HashedString, user_group: Thing) -> Result<ForumUser, surrealdb::Error> {
            db
                .create("user")
                .content(Self {
                    id: Thing {
                        id: Id::ulid(),
                        tb: "user".to_string()
                    },
                    username: username.to_string(),
                    email: email.to_string(),
                    password: password.to_string(),
                    user_group,
                    avatar_url: None,
                    created_at: chrono::offset::Utc::now()
                })
                .await
        }

        pub async fn find_by_email(db: &SurrealClient, email: &str) -> Result<Option<ForumUser>, surrealdb::Error> {
            db
                .query("SELECT * FROM user WHERE email = $email")
                .bind(("email", email))
                .await?.take(0)
        }

        #[allow(dead_code)]
        pub async fn find_by_username(db: &SurrealClient, username: &str) -> Result<Option<ForumUser>, surrealdb::Error> {
            db.query("SELECT * FROM user WHERE username = $username")
                .bind(("username", username))
                .await?.take(0)
        }

        pub async fn find_by_id(db: &SurrealClient, id: String) -> Result<Option<ForumUser>, surrealdb::Error> {
         db
                .query(format!("SELECT * FROM {id}"))
                .await?.take(0);
        }
    }
}
}
