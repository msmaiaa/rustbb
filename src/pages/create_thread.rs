use crate::components::button::Button;
use crate::components::text_editor::*;
use crate::pages::forum::ForumPageParams;
use leptos::*;
use leptos_router::*;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateThreadPayload {
    pub title: String,
    pub content: String,
    pub forum_id: String,
}

#[component]
pub fn CreateThreadPage(cx: Scope) -> impl IntoView {
    let (title, set_title) = create_signal(cx, String::new());
    let editor_content = create_rw_signal(cx, String::new());

    let navigate = use_navigate(cx);
    let try_create_thread = create_action(cx, move |payload: &CreateThreadPayload| {
        let payload = payload.to_owned();
        async move {
            if let Err(e) =
                create_thread(cx, payload.title, payload.content, payload.forum_id).await
            {
                log!("error on create_thread: {}", e.to_string());
            }
            set_title.update(|t| *t = "".to_string());
            editor_content.update(|c| *c = "".to_string());
        }
    });

    let on_submit = move |e: leptos::ev::MouseEvent| {
        let title = title.get();
        let content = editor_content.get();
        if title.is_empty() || content.is_empty() {
            log!("Title or content is empty.");
            return;
        }

        let forum_id = match use_params::<ForumPageParams>(cx).get() {
            Ok(params) => params.id,
            Err(_) => {
                let _ = navigate("/", Default::default());
                return;
            }
        };
        try_create_thread.dispatch(CreateThreadPayload {
            title,
            content,
            forum_id,
        });
    };
    view! {cx,
        <div class="w-full">
            <h2 class="text-2xl mb-3 font-light">"New thread"</h2>
            <input
                on:change = move |ev| {
                    let val = event_target_value(&ev);
                    set_title(val);
                }
                prop:value=title
             class="w-full bg-zinc-800 rounded-sm mb-2 h-8 pl-2 pr-8" type="text" placeholder="Title" autocomplete="off"/>
            <TextEditor content_signal=editor_content class="w-full".to_string() key="create_thread".to_string()/>
            <Button class="text-center rounded bg-green-400 text-black px-4 text-sm mt-2" on_click=on_submit>"Submit"</Button>
        </div>
    }
}

#[server(CreateThread, "/api")]
pub async fn create_thread(
    cx: Scope,
    title: String,
    content: String,
    forum_id: String,
) -> Result<(), ServerFnError> {
    use crate::app::token_from_cookie;
    use crate::error::server_error;
    use crate::model::thread::Thread;
    use itertools::Itertools;
    use surrealdb::sql::{Id, Thing};

    let forum = Thing::from(("forum".to_string(), forum_id));

    let req = match use_context::<leptos_axum::LeptosRequest<axum::body::Body>>(cx) {
        Some(req) => req.take_request().unwrap(),
        None => return server_error!("Couldn't get the request's info."),
    };

    let token_data = token_from_cookie(&req)?;
    let db = crate::database::get_db(cx).await?;
    let slug = slug::slugify(&title);

    let uid: (&str, &str) = token_data.user_id.split(":").collect_tuple().unwrap();
    let _thread: Thread = db
        .create("thread")
        .content(Thread {
            id: Thing {
                id: Id::ulid(),
                tb: "thread".to_string(),
            },
            title,
            slug,
            content,
            sticky: false,
            locked: false,
            forum,
            created_by: Thing::from(uid),
            created_at: chrono::Utc::now(),
        })
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(())
}
