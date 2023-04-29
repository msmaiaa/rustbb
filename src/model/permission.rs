use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PermissionValueKind {
    Boolean,
    Integer,
    Float,
}

impl PermissionValueKind {
    #[allow(dead_code)]
    pub fn default_value(&self) -> &'static str {
        match self {
            PermissionValueKind::Boolean => "false",
            PermissionValueKind::Integer => "0",
            PermissionValueKind::Float => "0.0",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Permission {
    pub id: Thing,
    pub description: String,
    pub value_kind: PermissionValueKind,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::database::SurrealClient;

    pub type Id = &'static str;
    pub type Label = &'static str;

    use lazy_static::lazy_static;
    type StaticPermissionEntry = (Id, Label, PermissionValueKind);

    #[derive(Clone, Debug)]
    pub struct StaticPermissionEntries(pub Vec<StaticPermissionEntry>);

    impl Permission {
    #[allow(dead_code)]
    pub async fn create(
    db: &SurrealClient,
    id: &str,
    description: &str,
    value_kind: PermissionValueKind,
    ) -> Result<Self, surrealdb::Error> {
        db.create("permission").content(Self {
            id: Thing {
                id: id.into(),
                tb: "permission".into(),
            },
            description: description.to_string(),
            value_kind,
        }).await
    }

    #[allow(dead_code)]
    pub async fn create_if_not_exists(
    db: &SurrealClient,
    id: &str,
    description: &str,
    value_kind: PermissionValueKind,
    ) -> Result<(), surrealdb::Error> {
    use crate::model::permission::Permission;

    match Permission::find_by_id(db, id).await {
        Ok(data) => {
            if data.is_none() {
                if let Err(e) = Permission::create(db, id, description, value_kind).await {
                    tracing::error!("Error creating permission: {}", e.to_string());
                }
            }
        }
        Err(e) => {
            tracing::error!("Error finding permission: {}", e.to_string());
        }
    }
    Ok(())
    }

    #[allow(dead_code)]
    pub async fn find_by_id(
    db: &SurrealClient,
    id: &str,
    ) -> Result<Option<Permission>, surrealdb::Error> {
        db
        .query(format!("SELECT * FROM permission:⟨{id}⟩"))
        .await?
        .take(0)
    }

    pub async fn select_all(db: &SurrealClient) -> Result<Vec<Permission>, surrealdb::Error> {
        db.select("permission").await
    }
}
}
}

#[cfg(feature = "ssr")]
lazy_static! {
    pub static ref PERMISSION_ENTRIES: StaticPermissionEntries = StaticPermissionEntries(vec![
        (
            "thread.create",
            "Create threads",
            PermissionValueKind::Boolean
        ),
        (
            "thread.edit",
            "Edit own threads",
            PermissionValueKind::Boolean
        ),
        (
            "thread.delete",
            "Delete own threads",
            PermissionValueKind::Boolean
        ),
        (
            "thread.edit_any",
            "Edit threads from any user",
            PermissionValueKind::Boolean
        ),
        (
            "thread.delete_any",
            "Delete threads from any user",
            PermissionValueKind::Boolean
        ),
        (
            "thread.move_any",
            "Move threads",
            PermissionValueKind::Boolean
        ),
        (
            "thread.approve",
            "Approve threads",
            PermissionValueKind::Boolean
        ),
        (
            "thread.delete",
            "Delete threads",
            PermissionValueKind::Boolean
        ),
        (
            "thread.subscribe",
            "Subscribe to threads",
            PermissionValueKind::Boolean
        ),
        (
            "thread.unsubscribe",
            "Unsubscribe from threads",
            PermissionValueKind::Boolean
        ),
        (
            "category.create",
            "Create categories",
            PermissionValueKind::Boolean
        ),
        (
            "category.edit",
            "Edit categories",
            PermissionValueKind::Boolean
        ),
        (
            "category.delete",
            "Delete categories",
            PermissionValueKind::Boolean
        ),
        (
            "forum.create",
            "Create a forum",
            PermissionValueKind::Boolean
        ),
        ("forum.edit", "Edit forums", PermissionValueKind::Boolean),
        (
            "forum.delete",
            "Delete forums",
            PermissionValueKind::Boolean
        ),
        ("post.create", "Create posts", PermissionValueKind::Boolean),
        ("post.edit", "Edit own post", PermissionValueKind::Boolean),
        (
            "post.edit_any",
            "Edit posts from any user",
            PermissionValueKind::Boolean
        ),
        (
            "post.delete",
            "Delete own posts",
            PermissionValueKind::Boolean
        ),
        (
            "post.delete_any",
            "Delete posts from any user",
            PermissionValueKind::Boolean
        ),
    ]);
}
