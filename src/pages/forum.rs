use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::model::forum::Forum;
use crate::model::thread::{Thread, ThreadStatus};

#[derive(Params, Clone, Debug, PartialEq)]
pub struct ForumParams {
    pub id: String,
}

fn get_forum_page() -> (Forum, Vec<Thread>) {
    let mock_forum = Forum {
        id: 1,
        title: "mock forum".to_string(),
        description: "mock forum description".to_string(),
        slug: "mock-forum".to_string(),
        category_id: 1,
        created_at: chrono::NaiveDateTime::from_timestamp(0, 0),
    };

    let mock_threads = vec![
        Thread {
            id: 1,
            title: "mock thread".to_string(),
            slug: "mock-thread".to_string(),
            status: ThreadStatus::Unlocked,
            sticky: false,
            forum_id: 1,
            creator_id: 1,
        },
        Thread {
            id: 2,
            title: "mock thread 2".to_string(),
            slug: "mock-thread-2".to_string(),
            status: ThreadStatus::Unlocked,
            sticky: true,
            forum_id: 1,
            creator_id: 1,
        },
        Thread {
            id: 3,
            title: "mock thread 3".to_string(),
            slug: "mock-thread-3".to_string(),
            status: ThreadStatus::Unlocked,
            sticky: false,
            forum_id: 1,
            creator_id: 1,
        },
    ];
    (mock_forum, mock_threads)
}

#[component]
pub fn ForumPage(cx: Scope) -> impl IntoView {
    let params = use_params::<ForumParams>(cx);
    let (forum, threads) = get_forum_page();
    let sticky_threads = threads.clone().into_iter().filter(|t| t.sticky);
    let normal_threads = threads.into_iter().filter(|t| !t.sticky);

    view! { cx,
        <div class="flex flex-col w-full">
            <Title text={forum.title.clone()}/>
                <h2 class="text-2xl mb-6">{forum.title}</h2>
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
        </div>
    }
}

#[component]
fn ThreadCard(cx: Scope, thread: Thread) -> impl IntoView {
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
