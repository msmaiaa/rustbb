use leptos::{ev::MouseEvent, *};

#[component]
pub fn Button<F>(
    cx: Scope,
    #[prop(optional)] _type: &'static str,
    #[prop(optional)] class: &'static str,
    on_click: F,
    children: Children,
) -> impl IntoView
where
    F: FnMut(MouseEvent) + 'static,
{
    view! {cx,
        <button on:click=on_click type=_type class=format!("py-1 px-4 rounded {class}")>
            {children(cx)}
        </button>
    }
}
