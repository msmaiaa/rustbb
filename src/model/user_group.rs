#[allow(unused)]
use crate::model::permission::Permission;
use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserGroup {
    pub id: Thing,
    pub name: String,
    pub user_title: String,
    pub description: Option<String>,
    pub permissions: Vec<UserGroupPermission>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserGroupPermission {
    pub id: Thing,
    pub value: String,
}

cfg_if! {
if #[cfg(feature="ssr")] {
    use crate::database::SurrealClient;
    use surrealdb::sql::{Id};

    impl UserGroup {
        pub async fn find_by_name(db: &SurrealClient, name: String) -> Result<Option<UserGroup>, surrealdb::Error> {
            db.query(format!("SELECT * FROM user_group WHERE name = '{}'", name)).await?.take(0)
        }

        pub async fn create(
            db: &SurrealClient,
            name: &str,
            user_title: &str,
            description: Option<String>,
        ) -> Result<UserGroup, surrealdb::Error> {
            db
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

        pub async fn select_all(db: &SurrealClient) -> Result<Vec<UserGroup>, surrealdb::Error> {
            db.query("SELECT * FROM user_group").await?.take(0)
        }

        pub async fn add_permission(&self, db: &SurrealClient, permission: Permission) -> Result<Option<Self>, surrealdb::Error> {
            db.query(format!("UPDATE {} SET permissions += $permission", self.id.to_raw()))
                .bind(("permission", UserGroupPermission{
                    value: permission.value_kind.default_value().to_string(),
                    id: permission.id
                }))
                .await?
                .take(0)
        }

        pub async fn create_if_not_exists(
            db: &SurrealClient,
            name: &str,
            user_title: &str,
            description: Option<String>,
        ) -> Result<UserGroup, surrealdb::Error> {
            use crate::model::user_group::UserGroup;

            match UserGroup::find_by_name(db, name.to_string()).await {
                Ok(data) => {
                    match data {
                            Some(data) => {
                                return Ok(data)
                            }
                            None => {
                                tracing::info!("{} group not found. Creating it now.", name);
                                return UserGroup::create(db, name, user_title, description).await
                            }
                        }
                },
                Err(e) => {
                    tracing::error!("Couldn't create the group: {}", name);
                    return Err(e);
                },
            }
        }
    }
}
}
