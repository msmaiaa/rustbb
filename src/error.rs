use http::status::StatusCode;
use thiserror::Error;

macro_rules! server_error {
    ($e:expr) => {
        Err(ServerFnError::ServerError($e.to_string()))
    };
}

pub(crate) use server_error;

#[derive(Debug, Clone, Error)]
pub enum RustbbError {
    #[error("Not Found")]
    NotFound,
}

impl RustbbError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
        }
    }
}
