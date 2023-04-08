use cfg_if::cfg_if;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub creator_id: i32,
    pub created_at: NaiveDateTime,
}

cfg_if! {
    if #[cfg(feature="ssr")] {
        use sqlx::{Pool, Postgres};
        struct Exists {
            exists: Option<bool>
        }
        impl Category {
            #[allow(dead_code)]
            pub async fn find_by_id(db_pool: &Pool<Postgres>, category_id: i32) -> Result<Category, sqlx::Error> {
                sqlx::query_as!(
                    Category,
                    r#"
                    SELECT * FROM category WHERE id = $1
                    "#,
                    category_id
                )
                .fetch_one(db_pool)
                .await
            }

            #[allow(dead_code)]
            pub async fn is_empty(db_pool: &Pool<Postgres>) -> Result<bool, sqlx::Error> {
                Ok(sqlx::query_as!(Exists,
                    r#"
                        select exists (select * from category)
                    "#
                )
                .fetch_one(db_pool)
                .await?
                .exists
                .map(|e| !e)
                .unwrap_or(false))
            }

            #[allow(dead_code)]
            pub async fn create(db_pool: &Pool<Postgres>, title: &str, creator_id: i32) -> Result<Category, sqlx::Error> {
                sqlx::query_as!(
                    Category,
                    r#"
                    INSERT INTO category (title, creator_id)
                    VALUES ($1, $2)
                    RETURNING *
                    "#,
                    title,
                    creator_id
                )
                .fetch_one(db_pool)
                .await
            }

            #[allow(dead_code)]
            pub async fn create_with_desc(db_pool: &Pool<Postgres>, title: &str, description: &str, creator_id: i32) -> Result<Category, sqlx::Error> {
                sqlx::query_as!(
                    Category,
                    r#"
                    INSERT INTO category (title, description, creator_id)
                    VALUES ($1, $2, $3)
                    RETURNING *
                    "#,
                    title,
                    description,
                    creator_id
                )
                .fetch_one(db_pool)
                .await
            }
        }
    }
}
