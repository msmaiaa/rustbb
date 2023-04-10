use crate::{app::LoggedUserData, hooks::use_user};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    //  TODO: decent dropdown menu like xenforo?
    //  TODO: make it sticky
    let user_data = use_user(cx);

    let logged_in_view = move |cx: Scope, user: LoggedUserData| {
        let img_url = match user.avatar_url {
            Some(url) => url,
            None => "/assets/default_avatar.png".to_string(),
        };
        view! {cx,
            <img src=img_url alt="Avatar" height="25" width="25" class="rounded-full mr-2"/>
            <p>{user.username}</p>
        }
    };

    let logged_off_view = move |cx: Scope| {
        view! {cx,
            <A href="/login" class="text-lg font-bold text-text_primary">"Login"</A>
            <A href="/register" class="text-lg font-bold text-text_primary ml-4">"Register"</A>
        }
    };
    view! {cx,
        <div class="w-full flex justify-center h-9">
            <div class="w-[1100px] flex items-center h-full">
                <div class="w-full flex justify-end">
                    {move || {
                        match user_data.get() {
                            Some(user) => logged_in_view(cx, user),
                            None => logged_off_view(cx)
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
