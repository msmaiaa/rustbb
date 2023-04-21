use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MainForum {
    pub id: Thing,
    pub title: String,
    pub created_at: DateTime<Utc>,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::database::SurrealPool;
    impl MainForum {
        pub async fn get_main_forum(pool: &SurrealPool) -> Result<Option<MainForum>, surrealdb::Error> {
            let forum: Vec<MainForum> = pool.select("main_forum").await?;
            Ok(forum.first().cloned())
        }

        // #[allow(dead_code)]
        // pub async fn create(pool: &SurrealPool, title: &str) -> Result<MainForum, surrealdb::Error> {
        //     let forum = sqlx::query_as!(
        //         MainForum,
        //         r#"
        //             INSERT INTO main_forum (title)
        //             VALUES ($1)
        //             RETURNING id, title, created_at
        //         "#,
        //         title
        //     )
        //     .fetch_one(pool)
        //     .await?;
        //     Ok(forum)
        // }
    }
}
}
