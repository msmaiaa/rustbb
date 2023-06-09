use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: Thing,
    pub title: String,
    pub forums: Vec<Thing>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct Count {
    pub count: i64,
}

cfg_if! {
    if #[cfg(feature="ssr")] {
        use crate::database::SurrealClient;
        use surrealdb::sql::{Id};
        impl Category {
            pub async fn count(db: &SurrealClient) -> Result<Option<Count>, surrealdb::Error> {
                db
                .query("SELECT count() FROM category")
                .await?
                .take::<Option<Count>>(0)
            }

            pub async fn create(db: &SurrealClient, title: &str) -> Result<Category, surrealdb::Error> {
                db
                    .create("category")
                    .content(Self {
                        id: Thing {
                            id: Id::ulid(),
                            tb: "category".to_string()
                        },
                        title: title.to_string(),
                        forums: vec![],
                        description: None,
                        created_at: Utc::now()
                    })
                    .await
            }

            pub async fn add_forum(self, db: &SurrealClient, forum_id: Thing) -> Result<Option<Category>, surrealdb::Error> {
                db.query(format!("UPDATE {} SET forums += $forum", self.id.to_raw()))
                .bind(("forum", forum_id))
                .await?
                .take(0)
            }
        }
    }
}
