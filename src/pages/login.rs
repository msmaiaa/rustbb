use crate::{app::LoggedUserData, components::login_form::*};
use leptos::*;
use leptos_router::use_navigate;

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    //  TODO: add contact us/terms and rules/etc links
    //  TODO: add a list of social media links (fontawesome?)
    let user_data =
        use_context::<RwSignal<Option<LoggedUserData>>>(cx).expect("user_data context is not set");

    create_effect(cx, move |_| match user_data.get() {
        Some(_) => {
            let _ = use_navigate(cx)("/", Default::default());
        }
        None => {}
    });

    view! {cx,
        <div class="w-full">
            <LoginForm on_login=move || {
                let _ = use_navigate(cx)("/", Default::default());
            }/>
        </div>
    }
}
