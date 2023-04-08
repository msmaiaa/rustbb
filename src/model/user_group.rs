use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserGroup {
    pub id: String,
    pub user_title: String,
    pub description: Option<String>,
}

cfg_if! {
if #[cfg(feature="ssr")] {
    use sqlx::{Error, Pool, Postgres};
    impl UserGroup {
        #[allow(dead_code)]
        pub async fn create(
            pool: &Pool<Postgres>,
            id: &str,
            user_title: &str,
            description: Option<String>,
        ) -> Result<Self, Error> {
            sqlx::query_as!(
                Self,
                r#"
                INSERT INTO user_group (id, user_title, description)
                VALUES ($1, $2, $3)
                RETURNING id, user_title, description
                "#,
                id,
                user_title,
                description
            )
            .fetch_one(pool)
            .await
        }

        #[allow(dead_code)]
        pub async fn find_by_id(pool: &Pool<Postgres>, id: &str) -> Result<Self, Error> {
            sqlx::query_as!(
                Self,
                r#"
                SELECT * FROM user_group WHERE id = $1
                "#,
                id
            )
            .fetch_one(pool)
            .await
        }

        #[allow(dead_code)]
        pub async fn select_all(pool: &Pool<Postgres>) -> Result<Vec<Self>, Error> {
            sqlx::query_as!(Self, r#"SELECT * FROM user_group"#)
                .fetch_all(pool)
                .await
        }

        #[allow(dead_code)]
        pub async fn create_if_not_exists(
            db_pool: &Pool<Postgres>,
            id: &str,
            user_title: &str,
            description: Option<String>,
        ) -> Result<(), sqlx::Error> {
            use crate::model::user_group::UserGroup;

            match UserGroup::find_by_id(db_pool, id).await {
                Ok(_) => tracing::info!("The {} group already exists.", id),
                Err(e) => match e {
                    sqlx::Error::RowNotFound => {
                        tracing::info!("{} group not found. Creating it now.", id);
                        if let Err(e) = UserGroup::create(db_pool, id, user_title, description).await {
                            tracing::error!("Couldn't create the {} group :( {}", id, e);
                        }
                    }
                    _ => {
                        tracing::error!("Error while querying for the {} group: {}", id, e);
                        return Err(e);
                    }
                },
            }
            Ok(())
        }
    }
}
}
