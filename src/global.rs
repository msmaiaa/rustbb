use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use lazy_static::lazy_static;
    lazy_static! {
        pub static ref ARGON2_SALT: String = dotenv::var("ARGON2_SALT").expect("ARGON2_SALT must be set");
        pub static ref JWT_KEY: String = dotenv::var("JWT_KEY").expect("JWT_KEY must be set");
        pub static ref DATABASE_URL: String = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    }
}
}
