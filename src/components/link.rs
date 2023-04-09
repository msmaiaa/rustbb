use leptos::{ev::MouseEvent, *};

#[component]
pub fn RouteLink(
    cx: Scope,
    to: String,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] reload: bool,
    children: Children,
) -> impl IntoView {
    let href = to.clone();
    let on_click = move |e: MouseEvent| {
        if reload {
            e.prevent_default();
            log!("{:?}", leptos_dom::helpers::location().replace(&to));
        }
    };

    view! {cx,
        <a href=href on:click=on_click class=class>
            {children(cx)}
        </a>
    }
}
