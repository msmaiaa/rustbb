use crate::components::register_form::*;
use leptos::*;

#[component]
pub fn Register(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="flex flex-col items-center justify-center h-full w-full">
            <RegisterForm/>
        </div>
    }
}
