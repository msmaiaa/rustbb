use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

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

        #[allow(dead_code)]
        pub async fn create(pool: &SurrealPool, title: &str) -> Result<MainForum, surrealdb::Error> {
            pool.create("main_forum").content(Self {
                id: Thing {
                    id: Id::ulid(),
                    tb: "main_forum".to_string()
                },
                title: title.to_string(),
                created_at: Utc::now()
            }).await
        }
    }
}
}
