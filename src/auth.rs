use cfg_if::cfg_if;

pub struct HashedString(String);

cfg_if!(
    if #[cfg(feature = "ssr")]{
        use argon2::{self, Config};

        impl HashedString {
            pub fn new(salt: &str, string: &str) -> Result<Self, argon2::Error> {
                let hashed = hash(salt, string)?;
                Ok(Self(hashed))
            }

        }

        impl AsRef<str> for HashedString {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        pub fn hash(salt: &str, content: &str) -> Result<String, argon2::Error> {
            let config = Config::default();
            argon2::hash_encoded(content.as_bytes(), salt.as_bytes(), &config)
        }
    }
);
