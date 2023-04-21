use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserGroup {
    pub id: Thing,
    pub name: String,
    pub user_title: String,
    pub description: Option<String>,
    pub permissions: Vec<Thing>
}

cfg_if! {
if #[cfg(feature="ssr")] {
    use crate::database::SurrealPool;
    use surrealdb::sql::{Id};
    impl UserGroup {
        pub async fn find_by_name(pool: &SurrealPool, name: String) -> Result<Option<UserGroup>, surrealdb::Error> {
            pool.query(format!("SELECT * FROM user_group WHERE name = '{}'", name)).await?.take(0)
        }

        pub async fn create(
            pool: &SurrealPool,
            name: &str,
            user_title: &str,
            description: Option<String>,
        ) -> Result<Self, surrealdb::Error> {
            pool
                .create("user_group")
                .content(Self {
                    id: Thing {
                        id: Id::ulid(),
                        tb: "user_group".to_string()
                    },
                    name: name.to_string(),
                    user_title: user_title.to_string(),
                    description,
                    permissions: vec![]
                })
                .await
        }

        // #[allow(dead_code)]
        // pub async fn select_all(pool: &Pool<Postgres>) -> Result<Vec<Self>, Error> {
        //     sqlx::query_as!(Self, r#"SELECT * FROM user_group"#)
        //         .fetch_all(pool)
        //         .await
        // }


        pub async fn create_if_not_exists(
            db_pool: &SurrealPool,
            name: &str,
            user_title: &str,
            description: Option<String>,
        ) -> Result<(), surrealdb::Error> {
            use crate::model::user_group::UserGroup;

            match UserGroup::find_by_name(db_pool, name.to_string()).await {
                Ok(data) => {
                    if data.is_none() {
                        tracing::info!("{} group not found. Creating it now.", name);
                        if let Err(e) = UserGroup::create(db_pool, name, user_title, description).await {
                            tracing::error!("Couldn't create the {} group :( {}", name, e);
                        }
                    }
                },
                Err(e) => {
                    tracing::error!("Couldn't create the group: {}", name);
                },
            }
            Ok(())
        }
    }
}
}
