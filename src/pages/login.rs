use crate::components::login_form::*;
use leptos::*;

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    //  TODO: add contact us/terms and rules/etc links
    //  TODO: add a list of social media links (fontawesome?)
    view! {cx,
        <div class="w-full">
            <LoginForm/>
        </div>
    }
}
