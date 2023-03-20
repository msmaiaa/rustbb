use cfg_if::cfg_if;

cfg_if!(
    if #[cfg(feature = "ssr")] {
        use crate::global;
        pub async fn get_db_pool() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::error::Error> {
            sqlx::postgres::PgPoolOptions::new().connect(global::DATABASE_URL.as_ref()).await
        }

        async fn migrate(db_pool: &sqlx::Pool<sqlx::Postgres>) {
            match sqlx::migrate!("./migrations")
            .run(db_pool)
            .await {
                Ok(_) => tracing::info!("[sqlx] Database migrations ran successfully"),
                Err(e) => tracing::error!("[sqlx] Database migrations failed: {}", e),
            };
        }


        /// Creates the main forum row if it doesn't exist.
        /// The main forum is a table with a single row that contains configuration data for the website.
        async fn init_forum(db_pool: &sqlx::Pool<sqlx::Postgres>) {
            use crate::model::main_forum::MainForum;

            match MainForum::get_main_forum(db_pool).await {
                Ok(_) => tracing::info!("The main forum is already set."),
                Err(e) => {
                    match e {
                        sqlx::Error::RowNotFound => {
                            tracing::info!("The main forum is not set. Creating it now.");
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

        pub async fn setup() {
            let db_pool = get_db_pool().await.expect("Couldn't connect to the database");
            migrate(&db_pool).await;
            init_forum(&db_pool).await;
        }
    }
);
