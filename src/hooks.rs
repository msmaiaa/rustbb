use leptos::*;

use crate::app::LoggedUserData;

pub fn use_user(cx: Scope) -> RwSignal<Option<LoggedUserData>> {
    let user_data = leptos::use_context::<leptos::RwSignal<Option<LoggedUserData>>>(cx)
        .expect("user_data context is not set");
    user_data
}
