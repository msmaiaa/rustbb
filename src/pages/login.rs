use crate::components::login_form::*;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let (error, set_error) = create_signal(cx, "".to_string());
    let (token, set_token) = create_signal(cx, "".to_string());
    let try_login = create_action(cx, move |payload: &LoginPayload| {
        let payload = payload.to_owned();
        async move {
            let response = login(payload.email, payload.password).await;
            match response {
                Ok(token) => {
                    set_token(token);
                }
                Err(e) => {
                    set_error(e.to_string());
                }
            }
        }
    });

    let on_login = move |payload: LoginPayload| {
        set_error("".to_string());
        try_login.dispatch(payload);
    };

    view! {cx,
        <div class="w-full">
            <LoginForm on_login/>
            {move|| {
                if error() != "" {
                    view! {cx,
                        <div class="bg-red-500 text-white rounded-sm p-2 mt-2">
                            {error()}
                        </div>
                    }
                } else if token() != "" {
                    view! {cx,
                        <div class="bg-green-500 text-white rounded-sm p-2 mt-2">
                            {token()}
                        </div>
                    }
                } else {
                    view! {cx, <div></div>}
                }
            }}
        </div>
    }
}

#[server(Login, "/api")]
pub async fn login(email: String, password: String) -> Result<String, ServerFnError> {
    use crate::auth::*;
    use crate::database::get_db_pool;
    use crate::model::user::*;

    let db = get_db_pool().await.unwrap();
    let found_user = ForumUser::find_by_email(&db, &email).await;
    match found_user {
        Ok(db_user) => {
            let salt = std::env::var("SALT").unwrap_or("123451234512345123451235".to_string());
            let hashed_pass;
            hashed_pass = match hash(&salt, &password) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ServerFnError::ServerError(e.to_string()));
                }
            };
            if db_user.password == hashed_pass {
                match generate_access_token(db_user.id) {
                    Ok(token) => {
                        return Ok(token);
                    }
                    Err(e) => {
                        return Err(ServerFnError::ServerError(e.to_string()));
                    }
                }
            } else {
                return Err(ServerFnError::ServerError("Incorrect password".to_string()));
            }
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                return Err(ServerFnError::ServerError("User not found".to_string()));
            }
            _ => {
                return Err(ServerFnError::ServerError(
                    "Internal server error".to_string(),
                ));
            }
        },
    }
}
