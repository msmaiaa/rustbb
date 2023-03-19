use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    view! {cx,
        <nav id="navbar" class="bg-bg_primary">
            <div class="flex justify-between items-center">
                <div class="flex items-center">
                    <A href="/" class="text-2xl font-bold text-text_primary">"Leptos"</A>
                </div>
                <div class="flex items-center">
                    <A href="/login" class="text-lg font-bold text-text_primary">"Login"</A>
                    <A href="/register" class="text-lg font-bold text-text_primary ml-4">"Register"</A>
                </div>
            </div>
        </nav>
    }
}
