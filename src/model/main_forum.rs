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
    use crate::database::SurrealClient;
    impl MainForum {
        pub async fn get_main_forum(db: &SurrealClient) -> Result<Option<MainForum>, surrealdb::Error> {
            let forum: Vec<MainForum> = db.select("main_forum").await?;
            Ok(forum.first().cloned())
        }

        #[allow(dead_code)]
        pub async fn create(db: &SurrealClient, title: &str) -> Result<MainForum, surrealdb::Error> {
            db.create("main_forum").content(Self {
                id: Thing {
                    id: surrealdb::sql::Id::ulid(),
                    tb: "main_forum".to_string()
                },
                title: title.to_string(),
                created_at: Utc::now()
            }).await
        }
    }
}
}
