use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Forum {
    pub id: Thing,
    pub title: String,
    pub description: Option<String>,
    pub slug: String,
    pub category: Thing,
    pub created_at: chrono::NaiveDateTime,
}

cfg_if! {
    if #[cfg(feature="ssr")] {
        use crate::database::SurrealPool;
        impl Forum {
            // #[allow(dead_code)]
            // pub async fn find_by_id(pool: &SurrealPool, forum_id: String) -> Result<Self, surrealdb::Error> {
            //     pool.select::<Option<Self>>(("forum", forum_id)).await
            // }
            //
            // #[allow(dead_code)]
            // pub async fn create(db_pool: &SurrealPool, title: &str, slug: &str, category_id: i32) -> Result<Forum, surrealdb::Error> {
            //     sqlx::query_as!(
            //         Forum,
            //         r#"
            //         INSERT INTO forum (title, slug, category_id)
            //         VALUES ($1, $2, $3)
            //         RETURNING *
            //         "#,
            //         title,
            //         slug,
            //         category_id
            //     )
            //     .fetch_one(db_pool)
            //     .await
            // }

            // #[allow(dead_code)]
            // pub async fn create_with_desc(db_pool: &SurrealPool, title: &str, slug: &str, category_id: i32, description: &str) -> Result<Forum, surrealdb::Error> {
            //     sqlx::query_as!(
            //         Forum,
            //         r#"
            //         INSERT INTO forum (title, slug, category_id, description)
            //         VALUES ($1, $2, $3, $4)
            //         RETURNING *
            //         "#,
            //         title,
            //         slug,
            //         category_id,
            //         description
            //     )
            //     .fetch_one(db_pool)
            //     .await
            // }
        }
    }
}
