#![allow(unused)]
use crate::components::button::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::model::thread::Thread;
use itertools::Itertools;
use surrealdb::sql::Thing;

#[derive(Clone, Debug, PartialEq, Params)]
pub struct ForumPageParams {
    pub id: String,
}

enum ThreadKind {
    Sticky,
    Normal,
}

#[component]
pub fn ForumPage(cx: Scope) -> impl IntoView {
    use crate::components::link::*;
    let forum_id = match use_params::<ForumPageParams>(cx).get() {
        Ok(params) => params.id,
        Err(e) => {
            _ = use_navigate(cx)("/", Default::default());
            "".to_string()
        }
    };

    let data = create_resource(
        cx,
        || (),
        move |_| {
            let _forum_id = forum_id.clone();
            async move { get_forum_page_data(cx, _forum_id).await }
        },
    );

    let no_threads_view = move |cx| {
        view! {cx,
            <>
                <p class="text-xl mb-6">"No threads here, mate"</p>
            </>
        }
    };

    let threads_view = move |cx: Scope, kind: ThreadKind, threads: Vec<ForumPageThread>| {
        view! {cx,
            <div class="flex flex-col w-full">
                <h2>{match kind {
                    ThreadKind::Sticky => "Sticky threads",
                    ThreadKind::Normal => "Normal threads",
                }}</h2>
                <div>
                    <For
                        each= move || threads.clone()
                        key=|n| n.id.to_raw()
                        view = move |cx, thread| {
                            view! {cx,
                                <ThreadCard thread={thread}/>
                            }
                        }
                    />
                </div>
            </div>
        }
    };

    view! { cx,
        <div class="flex flex-col w-full">
            <Suspense fallback=|| ()>{
                view!{cx,
                    <div>
                        {move || {
                            data.read(cx).map(|data| {
                                match data {
                                    Ok(data) => {
                                        let (sticky_threads, normal_threads) = data.threads.clone().into_iter().partition(|t| t.sticky);
                                        view! {cx,
                                            <Title text={data.forum_title.clone()}/>
                                            <div class="flex mb-6">
                                                <h2 class="text-2xl mr-2">{&data.forum_title}</h2>
                                                <RouteLink class="flex items-center" to=format!("{}/create_thread", use_route(cx).path())>"Create thread"</RouteLink>
                                            </div>
                                            <div class="bg-gray-900 rounded-sm">
                                                {match data.threads.is_empty() {
                                                    true => no_threads_view(cx),
                                                    false => {
                                                        view! {cx,
                                                            <>
                                                                {threads_view(cx, ThreadKind::Sticky, sticky_threads)}
                                                                {threads_view(cx, ThreadKind::Normal, normal_threads)}
                                                            </>
                                                        }
                                                    },
                                                }}
                                            </div>
                                        }
                                    }
                                    Err(_) => {
                                        //  TODO: redirect?
                                        view! {cx,
                                            <>
                                                <p></p>
                                            </>
                                        }
                                    }
                                }
                            })
                        }}
                    </div>
                }}
            </Suspense>
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
    pub id: Thing,
}

#[derive(Serialize, Deserialize)]
pub struct ForumTitle {
    pub title: String,
}

#[server(GetForumPageData, "/api")]
pub async fn get_forum_page_data(cx: Scope, id: String) -> Result<ForumPageData, ServerFnError> {
    let db = crate::database::get_db(cx).await?;

    let threads = db
        .query(format!(
            "SELECT title, slug, sticky, id FROM thread WHERE forum_id = '{}'",
            id
        ))
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .take(0)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    let forum_title: Option<ForumTitle> = db
        .query(format!("SELECT title FROM forum:{}", id))
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .take(0)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    return match forum_title {
        None => Err(ServerFnError::ServerError("Forum not found".to_string())),
        _ => Ok(ForumPageData {
            forum_title: forum_title.unwrap().title,
            threads,
        }),
    };
}
