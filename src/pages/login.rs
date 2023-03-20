use crate::components::login_form::*;
use leptos::*;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[server(Login, "/api")]
pub async fn login(email: String, password: String) -> Result<String, ServerFnError> {
    use crate::auth::*;
    use crate::database::get_db_pool;
    use crate::error::server_error;
    use crate::global;
    use crate::model::user::*;

    let db = get_db_pool().await.unwrap();
    let found_user;
    found_user = match ForumUser::find_by_email(&db, &email).await {
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                return server_error!("User not found");
            }
            _ => {
                return server_error!("Internal server error");
            }
        },
        Ok(user) => user,
    };

    let hashed_pass;
    hashed_pass = match hash(global::ARGON2_SALT.as_ref(), &password) {
        Ok(h) => h,
        Err(e) => return server_error!(e),
    };
    if found_user.password == hashed_pass {
        match generate_access_token(found_user.id, global::JWT_KEY.as_ref()) {
            Ok(token) => {
                return Ok(token);
            }
            Err(e) => return server_error!(e),
        }
    } else {
        return server_error!("Incorrect password");
    }
}
