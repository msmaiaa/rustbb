use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: Thing,
    pub title: String,
    pub description: Option<String>,
    pub created_by: Thing,
    pub created_at: DateTime<Utc>,
}

cfg_if! {
    if #[cfg(feature="ssr")] {
        use crate::database::SurrealPool;
        struct Exists {
            exists: Option<bool>
        }
        // impl Category {
        //     #[allow(dead_code)]
        //     pub async fn find_by_id(pool: &SurrealPool, category_id: String) -> Result<Category, surrealdb::Error> {
        //         pool.select::<Option<Category>>(("category", category_id)).await
        //     }
        //
        //     #[allow(dead_code)]
        //     pub async fn is_empty(db_pool: &SurrealPool) -> Result<bool, surrealdb::Error> {
        //         Ok(sqlx::query_as!(Exists,
        //             r#"
        //                 select exists (select * from category)
        //             "#
        //         )
        //         .fetch_one(db_pool)
        //         .await?
        //         .exists
        //         .map(|e| !e)
        //         .unwrap_or(false))
        //     }
        //
        //     #[allow(dead_code)]
        //     pub async fn create(db_pool: &SurrealPool, title: &str, creator_id: i32) -> Result<Category, surrealdb::Error> {
        //         sqlx::query_as!(
        //             Category,
        //             r#"
        //             INSERT INTO category (title, creator_id)
        //             VALUES ($1, $2)
        //             RETURNING *
        //             "#,
        //             title,
        //             creator_id
        //         )
        //         .fetch_one(db_pool)
        //         .await
        //     }
        //
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
        // }
    }
}
