#![allow(unused)]
use crate::components::button::*;
use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
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
        let (title, class) = match kind {
            ThreadKind::Sticky => ("Sticky threads", ""),
            ThreadKind::Normal => ("Normal threads", "mt-6"),
        };

        view! {cx,
            <>
                <div class=format!("flex flex-col w-full {class}")>
                    <h2>{title}</h2>
                    <div>
                        <For
                            each= move || threads.clone()
                            key=|n| n.id.to_raw()
                            view = move |cx, thread| {
                                view! {cx,
                                    <ThreadCard class="mt-2" thread={thread}/>
                                }
                            }
                        />
                    </div>
                </div>
            </>
        }
    };

    view! { cx,
        <div class="flex flex-col w-full">
            <Suspense fallback=|| ()>{
                move || {
                    data.read(cx).map(|data| {
                        match data {
                            Ok(data) => {
                                let (sticky_threads, normal_threads) = data.threads.clone().into_iter().partition(|t| t.sticky);
                                view! {cx,
                                    <Title text={data.forum_title.clone()}/>
                                    <div class="flex mb-6">
                                        <h2 class="text-2xl mr-2">{&data.forum_title}</h2>
                                        <RouteLink class="flex items-center ml-4 rounded bg-green-500 text-black px-4" to=format!("{}/create_thread", use_route(cx).path())>"Create thread"</RouteLink>
                                    </div>
                                    <div class="rounded-sm h-full">
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
                }}
            </Suspense>
        </div>
    }
}

fn format_creation_data(date: DateTime<Utc>) -> String {
    let now = Utc::now();
    let today = now.date_naive();
    let yesterday = today - chrono::Duration::days(1);

    if date.date_naive() == today {
        format!(
            "Today at {}",
            date.format("%l:%M %p").to_string().to_lowercase()
        )
    } else if date.date_naive() == yesterday {
        format!(
            "Yesterday at {}",
            date.format("%l:%M %p").to_string().to_lowercase()
        )
    } else {
        date.format("%d-%m-%Y %H:%M").to_string()
    }
}

#[component]
fn ThreadCard(
    cx: Scope,
    thread: ForumPageThread,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let avatar_url = match thread.author_avatar_url {
        Some(url) => url,
        None => "/assets/default_avatar.png".to_string(),
    };
    view! {cx,
        <div class=format!("bg-neutral-800 rounded-sm shadow-lg p-1.5 flex w-full shadow-lg {class}")>
            <div class="w-3/5 flex items-center">
                <img src=avatar_url alt="Avatar" height="35" width="35" class="rounded-full mr-3"/>
                <div class="flex flex-col">
                    <A href=format!("/thread/{}.{}", thread.slug, thread.id) class="text-lg font-normal h-6">
                        {thread.title}
                    </A>
                    <div class="flex items-center h-5">
                        <p class="text-xs mr-1">
                            {thread.author_username}
                        </p>
                        "·"
                        <p class="text-xs ml-1">
                            {format_creation_data(thread.created_at)}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ForumPageData {
    pub forum_title: String,
    pub threads: Vec<ForumPageThread>,
}
#[derive(Serialize, Clone)]
pub struct ForumPageThread {
    pub title: String,
    pub slug: String,
    pub sticky: bool,
    pub id: Thing,
    pub created_at: DateTime<Utc>,
    pub author_username: String,
    pub author_avatar_url: Option<String>,
}

impl<'de> Deserialize<'de> for ForumPageThread {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use crate::pages::home::deser_map;
        let mut map = serde_json::Map::deserialize(deserializer)?;
        deser_map!(map, title, String);
        deser_map!(map, slug, String);
        deser_map!(map, sticky, bool);
        deser_map!(map, created_at, DateTime<Utc>);
        deser_map!(map, author_username, String);
        deser_map!(map, author_avatar_url, Option<String>);

        let id = map
            .remove("id")
            .ok_or_else(|| serde::de::Error::missing_field("id"))?;

        cfg_if! {
            if #[cfg(not(feature="ssr"))] {
                Ok(Self {
                    id: surrealdb::sql::Thing {
                        id: surrealdb::sql::Id::String(serde_json::from_value(id.clone()).unwrap()),
                        tb: "thread".to_string(),
                    },
                    title,
                    slug,
                    sticky,
                    created_at,
                    author_username,
                    author_avatar_url,
                })
            } else {
                let id = id.get("id").unwrap().get("String").unwrap();
                let id = surrealdb::sql::Thing {
                    id: surrealdb::sql::Id::String(serde_json::from_value(id.clone()).unwrap()),
                    tb: "thread".to_string(),
                };
                Ok(Self {
                    id,
                    title,
                    slug,
                    sticky,
                    created_at,
                    author_username,
                    author_avatar_url,
                })
            }
        }
    }
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
            "SELECT title, slug, sticky, id, created_at, created_by.username as author_username, created_by.avatar_url as author_avatar_url FROM thread WHERE forum = 'forum:{}'",
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
