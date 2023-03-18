use cfg_if::cfg_if;

cfg_if!(
    if #[cfg(feature = "ssr")] {
        pub async fn get_db_pool(db_url: &str) -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::error::Error> {
            sqlx::postgres::PgPoolOptions::new().connect(db_url).await
        }
    }
);
