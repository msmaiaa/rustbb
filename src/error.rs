use http::status::StatusCode;
use thiserror::Error;

#[allow(unused_macros)]
macro_rules! server_error {
    ($e:expr) => {
        Err(ServerFnError::ServerError($e.to_string()))
    };
}

#[allow(unused_imports)]
pub(crate) use server_error;

#[allow(dead_code)]
#[derive(Debug, Clone, Error)]
pub enum RustbbError {
    #[error("Not Found")]
    NotFound,
}

impl RustbbError {
    #[allow(dead_code)]
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
        }
    }
}
