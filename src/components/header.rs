use super::link::*;
use leptos::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    use crate::pages::Page;
    //  TODO: forum logo instead of text
    //  TODO: use custom header background image
    view! {cx,
        <header id="header" class="bg-header w-full flex justify-center h-12">
            <div class="flex items-center w-[1100px]">
                <RouteLink to=Page::Home.path().to_string() reload=true class="text-2xl font-bold text-text_primary">"Leptos"</RouteLink>
            </div>
        </header>
    }
}
