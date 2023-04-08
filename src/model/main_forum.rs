use cfg_if::cfg_if;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MainForum {
    pub id: i32,
    pub title: String,
    pub created_at: NaiveDateTime,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    impl MainForum {
        #[allow(dead_code)]
        pub async fn get_main_forum(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<MainForum, sqlx::Error> {
            let forum = sqlx::query_as!(
                MainForum,
                r#"
                    SELECT id, title, created_at
                    FROM main_forum
                    LIMIT 1
                "#
            )
            .fetch_one(pool)
            .await?;
            Ok(forum)
        }

        #[allow(dead_code)]
        pub async fn create(pool: &sqlx::Pool<sqlx::Postgres>, title: &str) -> Result<MainForum, sqlx::Error> {
            let forum = sqlx::query_as!(
                MainForum,
                r#"
                    INSERT INTO main_forum (title)
                    VALUES ($1)
                    RETURNING id, title, created_at
                "#,
                title
            )
            .fetch_one(pool)
            .await?;
            Ok(forum)
        }
    }
}
}
