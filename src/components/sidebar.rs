use crate::components::card::*;
use leptos::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn RightSidebar(cx: Scope) -> impl IntoView {
    //  TODO: implement a server function to get the number of members online
    view! {cx,
        <div class="flex flex-col w-64 ml-3">
            <Card title="Members online">
                <p class="text-sm">"No members are currently online"</p>
            </Card>
            <ForumStatistics/>
        </div>
    }
}

#[component]
pub fn ForumStatistics(cx: Scope) -> impl IntoView {
    //  TODO: make all this work instead of being static data
    let stats = create_resource(
        cx,
        || (),
        move |_| async move { get_forum_statistics(cx).await },
    );

    view! {cx,
        <Card title="Forum statistics" class="mt-3">
            <div class="flex flex-col">
                <Suspense fallback= || ()>
                    {move || {
                        stats.read(cx).map(|data| {
                            match data {
                                Ok(data) => {
                                    view! {cx,
                                        <div class="flex justify-between">
                                            <p>"Threads:"</p>
                                            <p>{data.threads}</p>
                                        </div>
                                        <div class="flex justify-between">
                                            <p>"Messages:"</p>
                                            <p>"1"</p>
                                        </div>
                                        <div class="flex justify-between">
                                            <p>"Members:"</p>
                                            <p>"1"</p>
                                        </div>
                                    }.into_view(cx)
                                },
                                Err(e) => {
                                    log!("{}", e.to_string());
                                    let _view = move || {
                                    view! {cx,
                                            <></>
                                        }
                                    };
                                    _view.into_view(cx)
                                }
                            }
                        })
                    }}
                </Suspense>
            </div>
        </Card>
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ForumStatistics {
    threads: i32,
}

#[server(GetSidebarData, "/api")]
pub async fn get_forum_statistics(cx: Scope) -> Result<ForumStatistics, ServerFnError> {
    use crate::database::get_db;
    let db = get_db(cx).await.map_err(|e| {
        ServerFnError::ServerError(format!("Failed to get database: {}", e.to_string()))
    })?;
    let data = db
        .query("SELECT count() AS threads FROM thread GROUP BY threads")
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .take::<Option<ForumStatistics>>(0)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    return match data {
        Some(data) => Ok(data),
        None => Err(ServerFnError::ServerError("No data found".to_string())),
    };
}
