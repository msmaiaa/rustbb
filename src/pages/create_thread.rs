use crate::{components::text_editor::*, pages::forum::get_slug_and_id_ctx};
use leptos::*;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateThreadPayload {
    pub title: String,
    pub content: String,
    pub forum_id: i32,
}

#[component]
pub fn CreateThreadPage(cx: Scope) -> impl IntoView {
    let (title, set_title) = create_signal(cx, String::new());

    let try_create_thread = create_action(cx, move |payload: &CreateThreadPayload| {
        let payload = payload.to_owned();
        async move {
            match create_thread(cx, payload.title, payload.content, payload.forum_id).await {
                Ok(_) => {}
                Err(e) => {
                    log!("error!: {}", e.to_string());
                }
            }
        }
    });

    let on_submit_editor = move |content: String| {
        let title = title.get();
        if title.is_empty() || content.is_empty() {
            log!("Title or content is empty.");
            return;
        }
        let (_, forum_id) = match get_slug_and_id_ctx(cx) {
            Some((slug, id)) => (slug, id),
            None => {
                return;
            }
        };
        try_create_thread.dispatch(CreateThreadPayload {
            title,
            content,
            forum_id: forum_id.parse::<i32>().unwrap(),
        });
    };
    view! {cx,
        <div class="w-full">
            <h2 class="text-2xl mb-3">"Create thread"</h2>
            <input
                on:change = move |ev| {
                    let val = event_target_value(&ev);
                    set_title(val);
                }
             class="w-full bg-zinc-800 rounded-sm mb-2 h-8 pl-2 pr-8" type="text" placeholder="Title" autocomplete="off"/>
            <TextEditor on_submit=on_submit_editor class="w-full h-52".to_string() id="create_thread".to_string()/>
        </div>
    }
}

#[server(CreateThread, "/api")]
pub async fn create_thread(
    cx: Scope,
    title: String,
    content: String,
    forum_id: i32,
) -> Result<(), ServerFnError> {
    use crate::app::token_from_cookie;
    use crate::error::server_error;

    let req = match use_context::<leptos_axum::LeptosRequest<axum::body::Body>>(cx) {
        Some(req) => req.take_request().unwrap(),
        None => return server_error!("Couldn't get the request's info."),
    };

    let token_data = token_from_cookie(&req)?;
    let db = crate::database::get_db(cx).await?;
    let slug = slug::slugify(&title);

    // sqlx::query!(
    //     r#"
    //     INSERT INTO thread (title, slug, content, forum_id, creator_id)
    //     VALUES ($1, $2, $3, $4, $5)
    //     "#,
    //     title,
    //     slug,
    //     content,
    //     forum_id,
    //     token_data.user_id
    // )
    // .execute(&db)
    // .await
    // .map_err(|_| ServerFnError::ServerError("Database error".to_string()))?;
    Ok(())
}
