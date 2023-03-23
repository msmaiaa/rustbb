use super::button::*;
use super::card::*;
use super::form::*;
use leptos::*;
use serde::Deserialize;
use serde::Serialize;

const JWT_STORAGE_KEY: &str = "auth_token";

#[component]
pub fn LoginForm(cx: Scope) -> impl IntoView {
    use gloo_storage::{LocalStorage, Storage};

    let (error, set_error) = create_signal(cx, "".to_string());
    let email = create_rw_signal(cx, "".to_string());
    let password = create_rw_signal(cx, "".to_string());

    let try_login = create_action(cx, move |payload: &LoginPayload| {
        let payload = payload.to_owned();
        async move {
            match login(payload.email, payload.password).await {
                Ok(response) => {
                    if LocalStorage::set(JWT_STORAGE_KEY, response.token).is_err() {
                        log::error!("Failed to save token to local storage");
                    }
                }
                Err(e) => {
                    set_error(e.to_string());
                }
            }
        }
    });

    let on_submit = move || {
        if !email.get().is_empty() && !password.get().is_empty() {
            set_error("".to_string());
            let payload = LoginPayload {
                email: email.get(),
                password: password.get(),
            };
            try_login.dispatch(payload);
        }
    };

    view! {cx,
        <Card title="Login" class="w-full flex flex-col">
            <form on:submit= move |ev| {
                ev.prevent_default();
                on_submit();
            }>
                <FormRow signal=Some(email) required=true label="Email" id="email" _type="email" class="mb-2"/>
                <FormRow signal=Some(password) required=true label="Password" id="password" _type="password"/>
                <Button _type="submit">"Login"</Button>
            </form>

            {move|| {
                if !error().is_empty() {
                    view! {cx,
                        <div class="bg-red-500 text-white rounded-sm p-2 mt-2">
                            {error()}
                        </div>
                    }
                } else {
                    view! {cx, <div></div>}
                }
            }}
        </Card>
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
    pub username: String,
}

#[server(Login, "/api")]
pub async fn login(email: String, password: String) -> Result<LoginResponse, ServerFnError> {
    use crate::auth::*;
    use crate::database::get_db_pool;
    use crate::error::server_error;
    use crate::global;
    use crate::model::user::*;

    let db = get_db_pool().await.unwrap();

    let found_user = match ForumUser::find_by_email(&db, &email).await {
        Err(e) => match e {
            sqlx::Error::RowNotFound => return server_error!("User not found"),
            _ => return server_error!("Internal server error"),
        },
        Ok(user) => user,
    };

    let hashed_pass = match hash(global::ARGON2_SALT.as_ref(), &password) {
        Ok(h) => h,
        Err(e) => return server_error!(e),
    };

    if found_user.password == hashed_pass {
        match generate_access_token(found_user.id, global::JWT_KEY.as_ref()) {
            Ok(token) => {
                return Ok(LoginResponse {
                    token,
                    username: found_user.username,
                });
            }
            Err(e) => return server_error!(e),
        }
    } else {
        return server_error!("Incorrect password");
    }
}
