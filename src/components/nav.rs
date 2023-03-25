use leptos::*;
use leptos_router::*;

use crate::app::LoggedUserData;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    //  TODO: decent dropdown menu like xenforo?
    let user_data =
        use_context::<RwSignal<Option<LoggedUserData>>>(cx).expect("user_data context is not set");

    view! {cx,
        <div class="w-full flex justify-center">
            <div class="w-[1100px]">
                {move || {
                    match user_data.get() {
                        Some(data) => {
                            let img_url = match data.avatar_url {
                                Some(url) => url,
                                None => "/assets/default_avatar.png".to_string(),
                            };
                            view! {cx,
                                    <div>
                                        <img src=img_url alt="Avatar" height="25" width="25" class="rounded-full"/>
                                        <p>{data.username}</p>
                                    </div>
                            }
                        }
                        None => {
                            view! {cx,
                                <div>
                                    <A href="/login" class="text-lg font-bold text-text_primary">"Login"</A>
                                    <A href="/register" class="text-lg font-bold text-text_primary ml-4">"Register"</A>
                                </div>
                            }
                        }
                    }
                }}
            </div>
        </div>
    }
}
