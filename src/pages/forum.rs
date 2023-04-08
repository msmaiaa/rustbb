use chrono::NaiveDateTime;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::error::server_error;
use crate::model::forum::Forum;
use crate::model::thread::{Thread, ThreadStatus};
use itertools::Itertools;

pub fn get_slug_and_id(path: &str) -> Option<(String, String)> {
    let path = path.replace("/forum/", "");
    let (slug, id) = match path.split('.').next_tuple() {
        Some((slug, id)) => {
            if slug.is_empty() || id.is_empty() {
                return None;
            }
            (slug, id)
        }
        None => {
            return None;
        }
    };
    Some((slug.to_string(), id.to_string()))
}

#[component]
pub fn ForumPage(cx: Scope) -> impl IntoView {
    let data = create_resource(
        cx,
        || (),
        move |_| async move {
            let route = use_route(cx);
            let path = route.path();
            let (slug, id) = get_slug_and_id(&path)
                .ok_or_else(|| ServerFnError::ServerError("Invalid path".to_string()))?;
            get_forum_page_data(cx, slug.to_string(), id.to_string()).await
        },
    );
    let navigate = use_navigate(cx);

    let view = move |cx| {
        data.read(cx).map(|data| {
            match data {
                Ok(data) => {
                    let sticky_threads = data
                        .clone()
                        .threads
                        .into_iter()
                        .filter(|t| t.sticky)
                        .collect::<Vec<ForumPageThread>>();
                    let normal_threads = data
                        .threads
                        .into_iter()
                        .filter(|t| !t.sticky)
                        .collect::<Vec<ForumPageThread>>();
                    view! {cx,
                        <Title text={data.forum_title.clone()}/>
                        <h2 class="text-2xl mb-6">{data.forum_title.clone()}</h2>
                        <div class="flex flex-col w-full">
                            <h2>"Sticky threads"</h2>
                            <div>
                                <For
                                    each= move || sticky_threads.clone()
                                    key=|n| n.id
                                    view = move |cx, thread| {
                                        view! {cx,
                                            <ThreadCard thread={thread}/>
                                        }
                                    }
                                />
                            </div>
                        </div>
                        <div class="flex flex-col w-full">
                            <h2>"Normal threads"</h2>
                            <div>
                                <For
                                    each= move || normal_threads.clone()
                                    key=|n| n.id
                                    view = move |cx, thread| {
                                        view! {cx,
                                            <ThreadCard thread={thread}/>
                                        }
                                    }
                                />
                            </div>
                        </div>
                    }
                }
                Err(e) => {
                    //  TODO: redirect?
                    view! {cx,
                        <>
                            <p></p>
                        </>
                    }
                }
            }
        })
    };
    view! { cx,
        <div class="flex flex-col w-full">
            <Suspense fallback=|| ()>{view(cx)}</Suspense>
        </div>
    }
}

#[component]
fn ThreadCard(cx: Scope, thread: ForumPageThread) -> impl IntoView {
    view! {cx,
        <div class="bg-neutral-700 rounded-sm shadow-lg p-4 flex w-full">
            <div class="w-3/5">
                <A href=format!("/thread/{}.{}", thread.slug, thread.id)>
                    <h2 class="text-xl font-bold">{thread.title}</h2>
                </A>
            </div>
        </div>
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ForumPageData {
    pub forum_title: String,
    pub threads: Vec<ForumPageThread>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ForumPageThread {
    pub title: String,
    pub slug: String,
    pub sticky: bool,
    pub id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ForumTitle {
    pub title: String,
}

#[server(GetForumPageData, "/api")]
pub async fn get_forum_page_data(
    cx: Scope,
    slug: String,
    id: String,
) -> Result<ForumPageData, ServerFnError> {
    let id = id
        .parse::<i32>()
        .map_err(|_| ServerFnError::ServerError("Invalid path".to_string()))?;
    let pool = crate::database::get_db(cx).await?;

    let threads = sqlx::query_as!(
        ForumPageThread,
        r#"
        SELECT
            title,
            slug,
            sticky,
            id
        FROM
            thread
        WHERE
            forum_id = $1
        "#,
        id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| ServerFnError::ServerError("Internal Server Error".to_string()))?;

    let forum_title = sqlx::query_as!(
        ForumTitle,
        r#"
        SELECT
            title
        FROM
            forum
        WHERE
            slug = $1
        AND 
            id = $2
        "#,
        slug,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| ServerFnError::ServerError("Internal Server Error".to_string()))?;

    Ok(ForumPageData {
        forum_title: forum_title.title,
        threads,
    })
}
