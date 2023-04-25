#![allow(unused)]
use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::global;
    use surrealdb::engine::remote::ws::Ws;
    use surrealdb::opt::auth::Database;
    use surrealdb::sql::Thing;
    use surrealdb::engine::remote::ws::Client;
    use surrealdb::Surreal;
    use crate::model::user_group::UserGroup;
    use crate::auth::HashedString;
    use leptos::use_context;
    use crate::model::permission::Permission;
    use surrealdb::opt::auth::Root;
    use crate::model::permission::PERMISSION_ENTRIES;

    pub type SurrealPool = Surreal<Client>;

    pub struct DefaultUserGroups {
        pub admin: UserGroup,
        pub moderator: UserGroup,
        pub member: UserGroup,
        pub unconfirmed: UserGroup,
    }

    pub async fn setup() -> Surreal<Client> {
        let db_pool = get_db_pool().await.expect("Couldn't connect to the database");
        init_main_forum(&db_pool).await;
        init_category_and_forum(&db_pool).await;
        init_default_permissions(&db_pool).await;
        let groups = init_default_groups(&db_pool).await;
        seed_entries_for_groups(&db_pool).await;
        init_admin(&db_pool, groups.admin).await;
        db_pool
    }

    pub async fn get_db_pool() -> Result<Surreal<Client>, surrealdb::Error> {
        let db: Surreal<Client> = Surreal::new::<Ws>(global::DATABASE_URL.as_ref()).await?;
        db.signin(Root {
            username: global::DB_USER.as_ref(),
            password: global::DB_PASS.as_ref()
        }).await?;

        db.use_ns("rustbb").use_db("rustbb").await?;
        Ok(db)
    }

    pub async fn get_db(cx: leptos::Scope) -> Result<Surreal<Client>, leptos::ServerFnError> {
        let pool = use_context::<Surreal<Client>>(cx)
        .unwrap_or(get_db_pool().await.map_err(|e| leptos::ServerFnError::ServerError(e.to_string()))?);
        Ok(pool)
    }

    async fn init_category_and_forum(db_pool: &SurrealPool) {
        use crate::model::{category::Category, forum::Forum};
        //  TODO: this is temporary, we need to find a better solution
        let is_empty = match Category::count(db_pool).await {
            Ok(result) => match result {
                    Some(_) => false,
                    None => true,
            },
            Err(e) => {
                tracing::error!("Error while querying for categories: {}", e);
                return ();
            }
        };

        if is_empty {
            let category = match Category::create(db_pool, "Main category").await {
                Ok(category) => category,
                Err(e) => {
                    tracing::error!("Error while creating the default category: {}", e);
                    return ();
                }
            };

            match Forum::create(db_pool, "Main forum", &slug::slugify("Main forum"), category.id).await {
                Ok(_) => tracing::info!("The default category and forum were created successfully."),
                Err(e) => tracing::error!("Error while creating the default forum: {}", e),
            }
        }
    }

    async fn init_default_permissions(db_pool: &SurrealPool) {
        use crate::model::permission::Permission;
        let entries = PERMISSION_ENTRIES.clone().0;
        for (i, (id, description, kind)) in entries.iter().enumerate() {
            Permission::create_if_not_exists(db_pool, id, description, kind.clone()).await.expect("Couldn't create the default permissions");
        }
    }

    async fn init_main_forum(db_pool: &SurrealPool) {
        use crate::model::main_forum::MainForum;

        match MainForum::get_main_forum(db_pool).await {
            Ok(d) => {
                if d.is_none() {
                   if let Err(e) = MainForum::create(db_pool, "rustbb").await {
                       tracing::error!("Couldn't create the main forum :( {}", e);
                   }
                }
            },
            Err(e) => {
                tracing::error!("Error while querying for the main forum: {}", e);
            }
        }
    }

    /// Creates the default admin user if it doesn't exist.
    async fn init_admin(db_pool: &SurrealPool, user_group: UserGroup) {
        use crate::model::user::ForumUser;

        //  TODO: create a flag to check if the default admin user has been created.
        match ForumUser::find_by_username(db_pool, "admin").await {
            Ok(result) => {
                match result {
                    None => {
                        let hashed_pass = HashedString::new(crate::global::ARGON2_SALT.as_ref(), "admin").unwrap();
                        if let Err(e) = ForumUser::create(db_pool, "admin", "admin@mail.com", hashed_pass, user_group.id).await {
                            tracing::error!("Couldn't create the default admin user :( {}", e);
                        }
                    }
                    Some(_) => {}
                }
            },
            Err(e) => {
                tracing::error!("Error while querying for the default admin user: {}", e);
            }
        }
    }

    async fn init_default_groups(db_pool: &SurrealPool) -> DefaultUserGroups {

        //  TODO: same thing as above, create a flag to check if the default groups and permissions have been created.
        let admin = UserGroup::create_if_not_exists(db_pool, "Admin", "Administrator", Some("Administrators have full control over the forum.".to_string())).await.expect("Couldn't create the Admin group");
        let moderator = UserGroup::create_if_not_exists(db_pool, "Moderator", "Moderator", Some("Moderators have control over the forum.".to_string())).await.expect("Couldn't create the Moderator group");
        let member = UserGroup::create_if_not_exists(db_pool, "Member", "Member", Some("Members are regular users.".to_string())).await.expect("Couldn't create the Member group");
        let unconfirmed = UserGroup::create_if_not_exists(db_pool, "Unconfirmed", "Unconfirmed", Some("A user that has a pending email confirmation.".to_string())).await.expect("Couldn't create the Unconfirmed group");
        DefaultUserGroups {
            admin,
            moderator,
            member,
            unconfirmed,
        }
    }

    async fn seed_entries_for_groups(db_pool: &SurrealPool) {
        let groups: Vec<UserGroup> = UserGroup::select_all(db_pool).await.expect("Couldn't select the groups");
        let permissions: Vec<Permission> = Permission::select_all(db_pool).await.expect("Couldn't select the permissions");
        for group in groups {
            for permission in permissions.iter() {
                if group.permissions.iter().find(|p| p.id == permission.id).is_none() {
                    group.add_permission(db_pool, permission.clone()).await.expect("Couldn't add the permission to the group");
                }
            }
        }
    }
}
}
