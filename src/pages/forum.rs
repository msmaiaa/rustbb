use leptos::*;
use leptos_router::*;

#[derive(Params, Clone, Debug, PartialEq)]
pub struct ForumParams {
    pub id: String,
}

#[component]
pub fn ForumPage(cx: Scope) -> impl IntoView {
    let params = use_params::<ForumParams>(cx);
    log::debug!("forum params: {:?}", params);
    view! { cx,
        <p>"this is a forum page"</p>
    }
}
