use leptos::*;

#[component]
pub fn Button(
    cx: Scope,
    #[prop(optional)] _type: &'static str,
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <button type=_type class=format!("py-1 px-4 rounded {class}")>
            {children(cx)}
        </button>
    }
}
