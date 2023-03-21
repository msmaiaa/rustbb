use crate::components::login_form::*;
use leptos::*;

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="w-full">
            <LoginForm/>
        </div>
    }
}
