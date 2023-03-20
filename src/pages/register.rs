use crate::components::register_form::*;
use leptos::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn Register(cx: Scope) -> impl IntoView {
    let (error, set_error) = create_signal(cx, "".to_string());
    let (success, set_success) = create_signal(cx, "".to_string());

    let try_register_user = create_action(cx, move |payload: &RegisterUserPayload| {
        let payload = payload.to_owned();
        async move {
            let response = register_user(payload.username, payload.email, payload.password).await;
            match response {
                Ok(_) => {
                    set_success("Successfully registered".to_string());
                    set_error("".to_string());
                }
                Err(e) => {
                    //  FIXME: i couldn't figure out how to get the actual message that i sent from the server
                    //  it justs says "Internal Server Error"
                    set_error(e.to_string());
                    set_success("".to_string());
                }
            }
        }
    });

    let on_register = move |payload: RegisterUserPayload| {
        try_register_user.dispatch(payload);
    };

    view! {cx,
        <div class="flex flex-col items-center justify-center h-full w-full">
            <RegisterForm on_register/>
            {move || {
                let err = error.get();
                let success_msg = success.get();
                if err != "" {
                    view! {cx,
                        <div class="bg-red-500 text-white rounded-sm p-2 mt-2">
                            {err}
                        </div>
                    }
                } else if success_msg != "" {
                    view! {cx,
                        <div class="bg-green-500 text-white rounded-sm p-2 mt-2">
                            {success_msg}
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
    username: String,
    email: String,
    password: String,
) -> Result<(), ServerFnError> {
    use crate::auth::*;
    use crate::database::get_db_pool;
    use crate::error::server_error;
    use crate::global;
    use crate::model::user::*;

    let db = get_db_pool().await.unwrap();
    let found_user = ForumUser::find_by_username_or_email(&db, &username, &email).await;
    if let Ok(u) = found_user {
        if u.username == username {
            return server_error!("Username already in use");
        }
        if u.email == email {
            return server_error!("Email already in use");
        }
    }

    let hashed_pass;
    hashed_pass = match HashedString::new(&global::ARGON2_SALT.clone(), &password) {
        Ok(h) => h,
        Err(e) => {
            tracing::error!("Error while trying to hash password: {e}");
            return server_error!("Internal server error");
        }
    };

    let created_user = ForumUser::create(&db, &username, &email, hashed_pass).await;
    Ok(())
}
