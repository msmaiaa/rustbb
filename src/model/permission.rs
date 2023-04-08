use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum ValueType {
    Boolean,
    Integer,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Permission {
    pub id: String,
    pub label: String,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    impl Permission {

        #[allow(dead_code)]
        pub async fn create(
            pool: &sqlx::Pool<sqlx::Postgres>,
            id: &str,
            label: &str,
        ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
            sqlx::query!(
                r#"
                INSERT INTO permission (id, label)
                VALUES ($1, $2)
                "#,
                id,
                label
            )
            .execute(pool)
            .await
        }

        #[allow(dead_code)]
        pub async fn create_if_not_exists(
            pool: &sqlx::Pool<sqlx::Postgres>,
            id: &str,
            label: &str,
        ) -> Result<(), sqlx::Error> {
            use crate::model::permission::Permission;

            match Permission::find_by_id(pool, id).await {
                Ok(_) => {}
                Err(e) => match e {
                    sqlx::Error::RowNotFound => {
                        Permission::create(pool, id, label).await?;
                    }
                    _ => return Err(e),
                },
            }

            Ok(())
        }

        #[allow(dead_code)]
        pub async fn find_by_id(
            pool: &sqlx::Pool<sqlx::Postgres>,
            id: &str,
        ) -> Result<Self, sqlx::Error> {
            sqlx::query_as!(
                Self,
                r#"
                SELECT * FROM permission WHERE id = $1
                "#,
                id
            )
            .fetch_one(pool)
            .await
        }
    }
}
}
