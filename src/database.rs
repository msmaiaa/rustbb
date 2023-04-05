use cfg_if::cfg_if;
use leptos::use_context;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::global;
    use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
    use crate::auth::HashedString;
    use crate::permission_entries::PERMISSION_ENTRIES;

    type PgPool = Pool<Postgres>;


    pub async fn setup() -> PgPool {
        let db_pool = get_db_pool().await.expect("Couldn't connect to the database");
        migrate(&db_pool).await;
        init_main_forum(&db_pool).await;
        init_category_and_forum(&db_pool).await;
        init_default_groups(&db_pool).await;
        init_default_permissions(&db_pool).await;
        seed_entries_for_groups(&db_pool).await;
        init_admin(&db_pool).await;
        db_pool
    }

    pub async fn get_db_pool() -> Result<PgPool, sqlx::error::Error> {
        PgPoolOptions::new().connect(global::DATABASE_URL.as_ref()).await
    }

    pub fn get_db(cx: leptos::Scope) -> Result<PgPool, leptos::ServerFnError> {
        use_context::<PgPool>(cx)
        .ok_or("Missing db pool")
        .map_err(|e| leptos::ServerFnError::ServerError(e.to_string()))
    }

    async fn migrate(db_pool: &PgPool) {
        match sqlx::migrate!("./migrations")
        .run(db_pool)
        .await {
            Ok(_) => tracing::info!("[sqlx] Database migrations ran successfully"),
            Err(e) => tracing::error!("[sqlx] Database migrations failed: {}", e),
        };
    }

    async fn init_category_and_forum(db_pool: &PgPool) {
        use crate::model::{category::Category, forum::Forum};
        let is_empty = match Category::is_empty(db_pool).await {
            Ok(exists) => exists,
            Err(e) => {
                tracing::error!("Error while querying for categories: {}", e);
                return ();
            }
        };

        if is_empty {
            let category = match Category::create(db_pool, "Main category", 1).await {
                Ok(category) => category,
                Err(e) => {
                    tracing::error!("Error while creating the default category: {}", e);
                    return ();
                }
            };
        }
    }

    /// Creates the main forum row if it doesn't exist.
    /// The main forum is a table with a single row that contains configuration data for the website.
    async fn init_main_forum(db_pool: &PgPool) {
        use crate::model::main_forum::MainForum;

        match MainForum::get_main_forum(db_pool).await {
            Ok(_) => tracing::info!("The main forum is already created."),
            Err(e) => {
                match e {
                    sqlx::Error::RowNotFound => {
                        tracing::info!("Main forum not found. Creating it now.");
                        if let Err(e) = MainForum::create(db_pool, "rustbb").await {
                            tracing::error!("Couldn't create the main forum :( {}", e);
                        }
                    }
                    _ => {
                        tracing::error!("Error while querying for the main forum: {}", e);
                    }
                }
            }
        }
    }

    /// Creates the default admin user if it doesn't exist.
    async fn init_admin(db_pool: &PgPool) {
        use crate::model::user::ForumUser;
        use crate::model::user_group::UserGroup;

        //  TODO: create a flag to check if the default admin user has been created.
        match ForumUser::find_by_username(db_pool, "admin").await {
            Ok(_) => tracing::info!("The default admin user already exists."),
            Err(e) => {
                match e {
                    sqlx::Error::RowNotFound => {
                        tracing::info!("Default admin user not found. Creating it now.");
                        //  TODO: move the default admin credentials to a config file
                        let hashed_pass = HashedString::new(crate::global::ARGON2_SALT.as_ref(), "admin").unwrap();
                        if let Err(e) = ForumUser::create(db_pool, "admin", "admin@mail.com", hashed_pass, "Admin").await {
                            tracing::error!("Couldn't create the default admin user :( {}", e);
                        }
                    }
                    _ => {
                        tracing::error!("Error while querying for the default admin user: {}", e);
                    }
                }
            }
        }
    }

    async fn init_default_permissions(db_pool: &PgPool) {
        use crate::model::permission::Permission;
        let mut query = "INSERT INTO permission (id, label) VALUES ".to_string();
        let entries = PERMISSION_ENTRIES.clone().0;
        for (i, (id, label, _)) in entries.iter().enumerate() {
            let last_char = match i == entries.len() - 1 {
                true => ' ',
                false => ',',
            };
            query = query + &format!("('{}', '{}'){}", id, label, last_char);
        }
        query = query + "ON CONFLICT DO NOTHING;";
        sqlx::query(&query).execute(db_pool).await.expect("Couldn't insert the default permissions");
    }

    async fn init_default_groups(db_pool: &PgPool) {
        use crate::model::user_group::UserGroup;

        //  TODO: same thing as above, create a flag to check if the default groups and permissions have been created.
        UserGroup::create_if_not_exists(db_pool, "Admin", "Administrator", Some("Administrators have full control over the forum.".to_string())).await.expect("Couldn't create the Admin group");
        UserGroup::create_if_not_exists(db_pool, "Moderator", "Moderator", Some("Moderators have control over the forum.".to_string())).await.expect("Couldn't create the Moderator group");
        UserGroup::create_if_not_exists(db_pool, "Member", "Member", Some("Members are regular users.".to_string())).await.expect("Couldn't create the Member group");
        UserGroup::create_if_not_exists(db_pool, "Unconfirmed", "Unconfirmed", Some("A user that has a pending email confirmation.".to_string())).await.expect("Couldn't create the Unconfirmed group");
    }

    async fn seed_entries_for_groups(db_pool: &PgPool) {
        use crate::model::user_group_on_permission::UserGroupOnPermission;
        use crate::model::user_group::UserGroup;
        let groups = UserGroup::select_all(db_pool).await.expect("Couldn't select the groups");
        let entries = PERMISSION_ENTRIES.clone();
        for group in groups {
            UserGroupOnPermission::insert_default_entries_for_group(db_pool, &group.id, &entries).await;
        }
    }
}
}
