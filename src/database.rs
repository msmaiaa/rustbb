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

    pub type SurrealClient = Surreal<Client>;

    pub struct DefaultUserGroups {
        pub admin: UserGroup,
        pub moderator: UserGroup,
        pub member: UserGroup,
        pub unconfirmed: UserGroup,
    }

    pub async fn setup() -> Surreal<Client> {
        let db = get_db_client().await.expect("Couldn't connect to the database");
        init_main_forum(&db).await;
        init_category_and_forum(&db).await;
        init_default_permissions(&db).await;
        let groups = init_default_groups(&db).await;
        seed_entries_for_groups(&db).await;
        init_admin(&db, groups.admin).await;
        db
    }

    pub async fn get_db_client() -> Result<Surreal<Client>, surrealdb::Error> {
        let db: Surreal<Client> = Surreal::new::<Ws>(global::DATABASE_URL.as_ref()).await?;
        db.signin(Root {
            username: global::DB_USER.as_ref(),
            password: global::DB_PASS.as_ref()
        }).await?;

        db.use_ns("rustbb").use_db("rustbb").await?;
        Ok(db)
    }

    pub async fn get_db(cx: leptos::Scope) -> Result<Surreal<Client>, leptos::ServerFnError> {
        let db = use_context::<Surreal<Client>>(cx)
        .unwrap_or(get_db_client().await.map_err(|e| leptos::ServerFnError::ServerError(e.to_string()))?);
        Ok(db)
    }

    async fn init_category_and_forum(db: &SurrealClient) {
        use crate::model::{category::Category, forum::Forum};
        //  TODO: this is temporary, we need to find a better solution
        let is_empty = match Category::count(db).await {
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
            let category = match Category::create(db, "Main category").await {
                Ok(category) => category,
                Err(e) => {
                    tracing::error!("Error while creating the default category: {}", e);
                    return ();
                }
            };

            let forum = match Forum::create(db, "Main forum", &slug::slugify("Main forum"), category.id.clone()).await {
                Ok(data) => {
                    tracing::info!("The default category and forum were created successfully.");
                    data
                },
                Err(e) => {
                    tracing::error!("Error while creating the default forum: {}", e);
                    return ();
                },
            };
            category.add_forum(db, forum.id).await.expect("Couldn't add the default forum to the default category");
        }
    }

    async fn init_default_permissions(db: &SurrealClient) {
        use crate::model::permission::Permission;
        let entries = PERMISSION_ENTRIES.clone().0;
        for (i, (id, description, kind)) in entries.iter().enumerate() {
            Permission::create_if_not_exists(db, id, description, kind.clone()).await.expect("Couldn't create the default permissions");
        }
    }

    async fn init_main_forum(db: &SurrealClient) {
        use crate::model::main_forum::MainForum;

        match MainForum::get_main_forum(db).await {
            Ok(d) => {
                if d.is_none() {
                   if let Err(e) = MainForum::create(db, "rustbb").await {
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
    async fn init_admin(db: &SurrealClient, user_group: UserGroup) {
        use crate::model::user::ForumUser;

        //  TODO: create a flag to check if the default admin user has been created.
        match ForumUser::find_by_username(db, "admin").await {
            Ok(result) => {
                match result {
                    None => {
                        let hashed_pass = HashedString::new(crate::global::ARGON2_SALT.as_ref(), "admin").unwrap();
                        if let Err(e) = ForumUser::create(db, "admin", "admin@mail.com", hashed_pass, user_group.id).await {
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

    async fn init_default_groups(db: &SurrealClient) -> DefaultUserGroups {

        //  TODO: same thing as above, create a flag to check if the default groups and permissions have been created.
        let admin = UserGroup::create_if_not_exists(db, "Admin", "Administrator", Some("Administrators have full control over the forum.".to_string())).await.expect("Couldn't create the Admin group");
        let moderator = UserGroup::create_if_not_exists(db, "Moderator", "Moderator", Some("Moderators have control over the forum.".to_string())).await.expect("Couldn't create the Moderator group");
        let member = UserGroup::create_if_not_exists(db, "Member", "Member", Some("Members are regular users.".to_string())).await.expect("Couldn't create the Member group");
        let unconfirmed = UserGroup::create_if_not_exists(db, "Unconfirmed", "Unconfirmed", Some("A user that has a pending email confirmation.".to_string())).await.expect("Couldn't create the Unconfirmed group");
        DefaultUserGroups {
            admin,
            moderator,
            member,
            unconfirmed,
        }
    }

    async fn seed_entries_for_groups(db: &SurrealClient) {
        let groups: Vec<UserGroup> = UserGroup::select_all(db).await.expect("Couldn't select the groups");
        let permissions: Vec<Permission> = Permission::select_all(db).await.expect("Couldn't select the permissions");
        for group in groups {
            for permission in permissions.iter() {
                if group.permissions.iter().find(|p| p.id == permission.id).is_none() {
                    group.add_permission(db, permission.clone()).await.expect("Couldn't add the permission to the group");
                }
            }
        }
    }
}
}
