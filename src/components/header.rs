use leptos::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! {cx,
        <header id="header">"Hello from the header"</header>
    }
}
