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
        use crate::database::SurrealPool;
        use surrealdb::sql::{Id};
        struct Exists {
            exists: Option<bool>
        }
        impl Category {
        //     #[allow(dead_code)]
        //     pub async fn find_by_id(pool: &SurrealPool, category_id: String) -> Result<Category, surrealdb::Error> {
        //         pool.select::<Option<Category>>(("category", category_id)).await
        //     }
        //
            pub async fn count(db_pool: &SurrealPool) -> Result<Option<Count>, surrealdb::Error> {
                db_pool
                .query("SELECT count() FROM category")
                .await?
                .take::<Option<Count>>(0)
            }

            pub async fn create(db_pool: &SurrealPool, title: &str) -> Result<Category, surrealdb::Error> {
                db_pool
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

        //     #[allow(dead_code)]
        //     pub async fn create_with_desc(db_pool: &SurrealPool, title: &str, description: &str, creator_id: i32) -> Result<Category, surrealdb::Error> {
        //         sqlx::query_as!(
        //             Category,
        //             r#"
        //             INSERT INTO category (title, description, creator_id)
        //             VALUES ($1, $2, $3)
        //             RETURNING *
        //             "#,
        //             title,
        //             description,
        //             creator_id
        //         )
        //         .fetch_one(db_pool)
        //         .await
        //     }
        }
    }
}
