use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Forum {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub slug: String,
    pub category_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

cfg_if! {
    if #[cfg(feature="ssr")] {
        use sqlx::{Pool, Postgres};
        impl Forum {

            #[allow(dead_code)]
            pub async fn find_by_id(db_pool: &Pool<Postgres>, forum_id: i32) -> Result<Forum, sqlx::Error> {
                sqlx::query_as!(
                    Forum,
                    r#"
                    SELECT * FROM forum WHERE id = $1
                    "#,
                    forum_id
                )
                .fetch_one(db_pool)
                .await
            }

            #[allow(dead_code)]
            pub async fn create(db_pool: &Pool<Postgres>, title: &str, slug: &str, category_id: i32) -> Result<Forum, sqlx::Error> {
                sqlx::query_as!(
                    Forum,
                    r#"
                    INSERT INTO forum (title, slug, category_id)
                    VALUES ($1, $2, $3)
                    RETURNING *
                    "#,
                    title,
                    slug,
                    category_id
                )
                .fetch_one(db_pool)
                .await
            }

            #[allow(dead_code)]
            pub async fn create_with_desc(db_pool: &Pool<Postgres>, title: &str, slug: &str, category_id: i32, description: &str) -> Result<Forum, sqlx::Error> {
                sqlx::query_as!(
                    Forum,
                    r#"
                    INSERT INTO forum (title, slug, category_id, description)
                    VALUES ($1, $2, $3, $4)
                    RETURNING *
                    "#,
                    title,
                    slug,
                    category_id,
                    description
                )
                .fetch_one(db_pool)
                .await
            }
        }
    }
}
