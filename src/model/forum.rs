use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Forum {
    pub id: Thing,
    pub title: String,
    pub description: Option<String>,
    pub slug: String,
    pub category: Thing,
    pub created_at: DateTime<Utc>,
}

cfg_if! {
    if #[cfg(feature="ssr")] {
        use crate::database::SurrealClient;
        impl Forum {
            #[allow(dead_code)]
            pub async fn create(db: &SurrealClient, title: &str, slug: &str, category: Thing) -> Result<Forum, surrealdb::Error> {
                db.create("forum")
                    .content(Forum {
                        id: Thing {
                            id: surrealdb::sql::Id::ulid(),
                            tb: "forum".to_string()
                        },
                        title: title.to_string(),
                        description: None,
                        slug: slug.to_string(),
                        category: category,
                        created_at: Utc::now()
                    })
                    .await
            }
        }
    }
}
