use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::{postgres::PgQueryResult, Error, Pool, Postgres};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserGroup {
    pub id: i32,
    pub name: String,
    pub user_title: String,
    pub description: Option<String>,
}

#[cfg(feature = "ssr")]
impl UserGroup {
    pub async fn create(
        pool: &Pool<Postgres>,
        name: &str,
        user_title: &str,
        description: Option<String>,
    ) -> Result<Self, Error> {
        sqlx::query_as!(
            Self,
            r#"
            INSERT INTO user_group (name, user_title, description)
            VALUES ($1, $2, $3)
            RETURNING id, name, user_title, description
            "#,
            name,
            user_title,
            description
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_name(pool: &Pool<Postgres>, name: &str) -> Result<Self, Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM user_group WHERE name = $1
            "#,
            name
        )
        .fetch_one(pool)
        .await
    }

    pub async fn select_all(pool: &Pool<Postgres>) -> Result<Vec<Self>, Error> {
        sqlx::query_as!(Self, r#"SELECT * FROM user_group"#)
            .fetch_all(pool)
            .await
    }

    pub async fn create_if_not_exists(
        db_pool: &Pool<Postgres>,
        name: &str,
        user_title: &str,
        description: Option<String>,
    ) -> Result<(), sqlx::Error> {
        use crate::model::user_group::UserGroup;

        match UserGroup::find_by_name(db_pool, name).await {
            Ok(_) => tracing::info!("The {} group already exists.", name),
            Err(e) => match e {
                sqlx::Error::RowNotFound => {
                    tracing::info!("{} group not found. Creating it now.", name);
                    if let Err(e) = UserGroup::create(db_pool, name, user_title, description).await
                    {
                        tracing::error!("Couldn't create the {} group :( {}", name, e);
                    }
                }
                _ => {
                    tracing::error!("Error while querying for the {} group: {}", name, e);
                    return Err(e);
                }
            },
        }
        Ok(())
    }
}
