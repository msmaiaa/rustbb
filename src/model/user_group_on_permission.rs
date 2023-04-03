use serde::{Deserialize, Serialize};

use crate::permission_entries::StaticPermissionEntries;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserGroupOnPermission {
    pub id: i32,
    pub user_group_id: i32,
    pub permission_id: String,
    pub value: String,
}

#[cfg(feature = "ssr")]
impl UserGroupOnPermission {
    pub async fn insert_default_entries_for_group(
        pool: &sqlx::Pool<sqlx::Postgres>,
        user_group_id: i32,
        permissions: &StaticPermissionEntries,
    ) -> Result<(), sqlx::Error> {
        use crate::model::permission::Permission;
        use crate::permission_entries::ValueType;

        //  TODO: use only one query instead of this
        for (id, label, value_type) in &permissions.0 {
            let value = match value_type {
                ValueType::Boolean => "false".to_string(),
                ValueType::Integer => "0".to_string(),
            };

            UserGroupOnPermission::create_ignore(pool, user_group_id, id.to_string(), value)
                .await?;
        }

        Ok(())
    }

    pub async fn create(
        pool: &sqlx::Pool<sqlx::Postgres>,
        user_group_id: i32,
        permission_id: String,
        value: String,
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO user_group_on_permission (user_group_id, permission_id, value)
            VALUES ($1, $2, $3)
            "#,
            user_group_id,
            permission_id,
            value
        )
        .execute(pool)
        .await
    }

    pub async fn create_ignore(
        pool: &sqlx::Pool<sqlx::Postgres>,
        user_group_id: i32,
        permission_id: String,
        value: String,
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO user_group_on_permission (user_group_id, permission_id, value)
            VALUES ($1, $2, $3)
            ON CONFLICT DO NOTHING
            "#,
            user_group_id,
            permission_id,
            value
        )
        .execute(pool)
        .await
    }
}
