use serde::{Deserialize, Serialize};

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

#[cfg(feature = "ssr")]
impl Permission {
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
