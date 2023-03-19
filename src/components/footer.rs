use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! {cx,
        <footer id="footer" class="w-full">"Hello from the footer"</footer>
    }
}
