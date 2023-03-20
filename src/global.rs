use cfg_if::cfg_if;

cfg_if!(
    if #[cfg(feature = "ssr")] {
        use static_init::dynamic;

        #[dynamic]
        pub static ARGON2_SALT: String = dotenv::var("ARGON2_SALT").expect("ARGON2_SALT must be set");
        #[dynamic]
        pub static JWT_KEY: String = dotenv::var("JWT_KEY").expect("JWT_KEY must be set");
        #[dynamic]
        pub static DATABASE_URL: String = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    }
);
