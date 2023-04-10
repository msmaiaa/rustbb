use crate::{app::LoggedUserData, components::login_form::*, hooks::use_user};
use leptos::*;
use leptos_router::use_navigate;

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    //  TODO: add contact us/terms and rules/etc links
    //  TODO: add a list of social media links (fontawesome?)
    let user_data = use_user(cx);

    create_effect(cx, move |_| match user_data.get() {
        Some(_) => {
            let _ = use_navigate(cx)("/", Default::default());
        }
        None => {}
    });

    let on_login = move |data: LoggedUserData| {
        let _ = use_navigate(cx)("/", Default::default());
        user_data.set(Some(data));
    };

    view! {cx,
        <div class="w-full">
            <LoginForm on_login=on_login/>
        </div>
    }
}
