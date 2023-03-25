use leptos::*;
use leptos_router::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    //  TODO: forum logo instead of text
    //  TODO: use custom header background image
    view! {cx,
        <header id="header" class="bg-header w-full flex justify-center">
            <div class="flex items-center w-[1100px]">
                <A href="/" class="text-2xl font-bold text-text_primary">"Leptos"</A>
            </div>
        </header>
    }
}
