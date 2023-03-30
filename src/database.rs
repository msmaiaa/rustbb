use cfg_if::cfg_if;

use crate::auth::HashedString;

cfg_if!(
    if #[cfg(feature = "ssr")] {
        use crate::global;
        use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

        type PgPool = Pool<Postgres>;

        pub async fn get_db_pool() -> Result<PgPool, sqlx::error::Error> {
            PgPoolOptions::new().connect(global::DATABASE_URL.as_ref()).await
        }

        async fn migrate(db_pool: &PgPool) {
            match sqlx::migrate!("./migrations")
            .run(db_pool)
            .await {
                Ok(_) => tracing::info!("[sqlx] Database migrations ran successfully"),
                Err(e) => tracing::error!("[sqlx] Database migrations failed: {}", e),
            };
        }


        /// Creates the main forum row if it doesn't exist.
        /// The main forum is a table with a single row that contains configuration data for the website.
        async fn init_forum(db_pool: &PgPool) {
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

            //  TODO: create a flag to check if the default admin user has been created.
            //  If the user changes the default admin username, it will just create another one
            match ForumUser::find_by_username(db_pool, "admin").await {
                Ok(_) => tracing::info!("The default admin user already exists."),
                Err(e) => {
                    match e {
                        sqlx::Error::RowNotFound => {
                            tracing::info!("Default admin user not found. Creating it now.");
                            //  TODO: move the default admin credentials to a config file
                            let hashed_pass = HashedString::new(crate::global::ARGON2_SALT.as_ref(), "admin").unwrap();
                            if let Err(e) = ForumUser::create(db_pool, "admin", "admin@mail.com", hashed_pass).await {
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

        pub async fn setup() {
            let db_pool = get_db_pool().await.expect("Couldn't connect to the database");
            migrate(&db_pool).await;
            init_forum(&db_pool).await;
            init_admin(&db_pool).await;
        }
    }
);
