use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum ValueType {
    Boolean,
    Integer,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Permission {
    pub id: Thing,
    pub label: String,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::database::SurrealPool;
        use surrealdb::sql::{Id};
    impl Permission {
        #[allow(dead_code)]
        pub async fn create(
            pool: &SurrealPool,
            id: &str,
            label: &str,
        ) -> Result<Self, surrealdb::Error> {
            pool
                .create("permission")
                .content(Self {
                    id: Thing {
                        id: Id::String(id.to_string()),
                        tb: "permission".to_string(),
                    },
                    label: label.to_string(),
                })
                .await
        }

        #[allow(dead_code)]
        pub async fn create_if_not_exists(
            pool: &SurrealPool,
            id: &str,
            label: &str,
        ) -> Result<(), surrealdb::Error> {
            use crate::model::permission::Permission;

            match Permission::find_by_id(pool, id).await {
                Ok(data) => {
                     if data.is_none() {
                            if let Err(e) = Permission::create(pool, id, label).await {
                                tracing::error!("Error creating permission: {}", e.to_string());
                            }
                        }
                }
                Err(e) => {
                    tracing::error!("Error finding permission: {}", e.to_string());
                },
            }

            Ok(())
        }

        #[allow(dead_code)]
        pub async fn find_by_id(
            pool: &SurrealPool,
            id: &str,
        ) -> Result<Option<Permission>, surrealdb::Error> {
            pool.query(format!("SELECT * FROM permission:⟨{}⟩", id)).await?.take(0)
        }
    }
}
}
