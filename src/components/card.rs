use leptos::*;

#[component]
pub fn Card(
    cx: Scope,
    title: &'static str,
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <div class=format!("bg-neutral-800 rounded-md shadow-lg p-3 {class}")>
            <h2 class="text-2xl font-bold">{title}</h2>
            {children(cx)}
        </div>
    }
}
