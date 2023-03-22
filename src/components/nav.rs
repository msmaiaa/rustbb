use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    //  TODO: decent dropdown menu like xenforo?
    view! {cx,
        <nav id="navbar" class="bg-bg_primary">
            <div class="flex justify-between items-center">
                <div class="flex items-center">
                    <A href="/login" class="text-lg font-bold text-text_primary">"Login"</A>
                    <A href="/register" class="text-lg font-bold text-text_primary ml-4">"Register"</A>
                </div>
            </div>
        </nav>
    }
}
