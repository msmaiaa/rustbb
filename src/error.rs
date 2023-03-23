macro_rules! server_error {
    ($e:expr) => {
        Err(ServerFnError::ServerError($e.to_string()))
    };
}

pub(crate) use server_error;
