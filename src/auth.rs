use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")]{
    pub struct HashedString(String);
    use serde::{de::DeserializeOwned, Deserialize, Serialize};
    use argon2::{self, Config, Error as Argon2Error};
    use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Algorithm, errors::Error as JwtError};

    impl HashedString {
        pub fn new(salt: &str, string: &str) -> Result<Self, Argon2Error> {
            let hashed = hash_str(salt, string)?;
            Ok(Self(hashed))
        }
    }

    impl AsRef<str> for HashedString {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    impl ToString for HashedString {
        fn to_string(&self) -> String {
            self.0.clone()
        }
    }

    pub fn hash_str(salt: &str, content: &str) -> Result<String, Argon2Error> {
        let config = Config::default();
        argon2::hash_encoded(content.as_bytes(), salt.as_bytes(), &config)
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AccessToken {
        pub user_id: String,
        pub exp: i64,
    }

    pub fn generate_access_token(user_id: String, jwt_key: &str) -> Result<String, JwtError> {
        let iat = chrono::Utc::now();
        //  TODO: get the expiration time from a environment variable
        let exp = iat + chrono::Duration::weeks(1);
        let exp = exp.timestamp();

        let key =
            EncodingKey::from_secret(jwt_key.as_bytes());
        let claims = AccessToken { user_id, exp };
        let header = Header::new(Algorithm::HS256);
        encode(&header, &claims, &key)
    }

    pub fn decode_access_token<T: DeserializeOwned>(token: &str, jwt_key: &str) -> Result<T, JwtError> {
        let key = DecodingKey::from_secret(jwt_key.as_bytes());
        decode::<T>(&token, &key, &jsonwebtoken::Validation::default()).map(|data| data.claims)
    }
}
}
