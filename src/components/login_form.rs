use super::button::*;
use super::card::*;
use super::form::*;
use crate::app::LoggedUserData;
use leptos::*;
use serde::Deserialize;
use serde::Serialize;

const JWT_COOKIE_KEY: &str = "auth_token";

#[component]
pub fn LoginForm(cx: Scope) -> impl IntoView {
    let user_data =
        use_context::<RwSignal<Option<LoggedUserData>>>(cx).expect("user_data context is not set");
    let (error, set_error) = create_signal(cx, "".to_string());
    let email = create_rw_signal(cx, "".to_string());
    let password = create_rw_signal(cx, "".to_string());

    let try_login = create_action(cx, move |payload: &LoginPayload| {
        let payload = payload.to_owned();
        async move {
            match login(cx, payload.email, payload.password).await {
                Ok(response) => {
                    user_data.set(Some(LoggedUserData {
                        username: response.username,
                        avatar_url: response.avatar_url,
                    }));
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
    pub username: String,
    pub avatar_url: Option<String>,
}

#[server(Login)]
pub async fn login(
    cx: Scope,
    email: String,
    password: String,
) -> Result<LoginResponse, ServerFnError> {
    use crate::auth::*;
    use crate::database::get_db_pool;
    use crate::error::server_error;
    use crate::global;
    use crate::model::user::*;
    use actix_web::http::header::{HeaderName, HeaderValue};
    use leptos_actix::ResponseOptions;

    let db = get_db_pool().await.unwrap();

    let response = match use_context::<ResponseOptions>(cx) {
        Some(r) => r,
        None => return server_error!("Couldn't get response options"),
    };

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
                response.insert_header(
                    HeaderName::from_static("set-cookie"),
                    HeaderValue::from_str(&format!(
                        "{}={}; HttpOnly; SameSite=Strict",
                        JWT_COOKIE_KEY, token
                    ))
                    .unwrap(),
                );
                Ok(LoginResponse {
                    username: found_user.username,
                    avatar_url: found_user.avatar_url,
                })
            }
            Err(e) => return server_error!(e),
        }
    } else {
        return server_error!("Incorrect password");
    }
}
