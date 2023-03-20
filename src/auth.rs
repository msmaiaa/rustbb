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

        #[derive(serde::Serialize, serde::Deserialize)]
        pub struct AccessToken {
            pub user_id: i32,
            pub iat: i64,
            pub exp: i64,
        }

        pub fn generate_access_token(user_id: i32, jwt_key: &str) -> Result<String, jsonwebtoken::errors::Error> {
            use jsonwebtoken::{encode, Header, EncodingKey, Algorithm};
            let iat = chrono::Utc::now();
            //  TODO: get the expiration time from a environment variable
            let exp = iat + chrono::Duration::seconds(3600);
            let iat = iat.timestamp_millis();
            let exp = exp.timestamp_millis();

            let key =
                EncodingKey::from_secret(jwt_key.as_bytes());
            let claims = AccessToken { user_id, iat, exp };
            let header = Header::new(Algorithm::HS256);
            encode(&header, &claims, &key)
        }
    }
);
