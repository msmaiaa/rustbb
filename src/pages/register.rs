use crate::{components::register_form::*, hooks::use_user};
use leptos::*;
use leptos_router::use_navigate;
use serde::{Deserialize, Serialize};

#[component]
pub fn Register(cx: Scope) -> impl IntoView {
    let (error, set_error) = create_signal(cx, "".to_string());
    let logged_user = use_user(cx);

    let try_register_user = create_action(cx, move |payload: &RegisterUserPayload| {
        let payload = payload.to_owned();
        async move {
            let response =
                register_user(cx, payload.username, payload.email, payload.password).await;
            match response {
                Ok(_) => {
                    let _ = use_navigate(cx)("/login", Default::default());
                }
                Err(_) => {
                    //  FIXME: i couldn't figure out how to get the actual message that i sent from the server
                    //  it justs says "Internal Server Error"
                    set_error("Something went wrong".to_string());
                }
            }
        }
    });

    create_effect(cx, move |_| {
        if logged_user.get().is_some() {
            let _ = use_navigate(cx)("/", Default::default());
        }
    });

    let on_register = move |payload: RegisterUserPayload| {
        try_register_user.dispatch(payload);
    };

    view! {cx,
        <div class="flex flex-col items-center justify-center h-full w-full">
            <RegisterForm on_register/>
            {move || {
                let error = error.get();
                if !error.is_empty() {
                    view! {cx,
                        <div class="bg-red-500 text-white rounded-sm p-2 mt-2">
                            {error}
                        </div>
                    }
                } else {
                    view! {cx, <div></div>}
                }

            }}
        </div>
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegisterUserPayload {
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

//  for some reason it errors when i send a struct instead of primitives
#[server(RegisterUser, "/api")]
pub async fn register_user(
    cx: Scope,
    username: String,
    email: String,
    password: String,
) -> Result<(), ServerFnError> {
    use crate::auth::*;
    use crate::database::get_db;
    use crate::error::server_error;
    use crate::global;
    use crate::model::user::*;
    use crate::model::user_group::UserGroup;
    let db = get_db(cx).await?;
    let found_user = ForumUser::find_by_username_or_email(&db, &username, &email).await;
    if let Ok(u) = found_user {
        if let Some(u) = u {
            if u.username == username {
                return server_error!("Username already in use");
            }
            if u.email == email {
                return server_error!("Email already in use");
            }
        }
    }

    let hashed_pass = match HashedString::new(&global::ARGON2_SALT.clone(), &password) {
        Ok(h) => h,
        Err(e) => {
            tracing::error!("Error while trying to hash password: {e}");
            return server_error!("Internal server error");
        }
    };
    let member_group = UserGroup::find_by_name(&db, "Member".to_string()).await.map_err(|_| ServerFnError::ServerError("Internal server error".to_string()))?;
    let _ = ForumUser::create(&db, &username, &email, hashed_pass, member_group.unwrap().id).await;
    Ok(())
}
