use leptos::*;

#[component]
pub fn Button(
    cx: Scope,
    #[prop(optional)] _type: &'static str,
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <button type=_type class=format!("bg-blue-500 hover:bg-blue-700 text-white py-1 px-4 rounded {class}")>
            {children(cx)}
        </button>
    }
}
