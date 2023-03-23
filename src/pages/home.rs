use crate::model::{category::Category, forum::*};
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let home_data = create_resource(cx, || (), |_| async { get_home_data().await });
    view! { cx,
        <div class="h-full w-full">
            <Suspense fallback=move || view! { cx, <p>""</p> }>
                {move || {
                    home_data.read(cx).map(|data| {
                        match data {
                            Err(e) => {
                                log!("Error: {:#?}", e);
                                view! {cx, <div>"Error"</div>}
                            },
                            Ok(data) => {
                                view! {cx,
                                    <div class="flex flex-col w-full">
                                        <div class="flex flex-col w-full">
                                            <For
                                                each=move || data.clone()
                                                key=|n| n.category.id
                                                view = move |cx, data| {
                                                    view! {cx,
                                                        <CategoryCard category={data.category} forums={data.forums}/>
                                                    }
                                                }
                                            />
                                        </div>
                                </div>
                                }
                            }
                        }

                    })
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn CategoryCard(cx: Scope, category: Category, forums: Option<Vec<Forum>>) -> impl IntoView {
    let forums = if let Some(forums) = forums {
        forums
    } else {
        vec![]
    };
    view! {cx,
        <div class="bg-neutral-800 rounded-lg shadow-lg p-4 mb-3">
            <h2 class="text-2xl font-bold">{category.title}</h2>
            <p class="text-sm text-text_secondary">{category.description}</p>
            <div class="flex flex-col">
                    <For
                        each=move || forums.clone()
                        key=|n| n.id
                        view = move |cx, forum| {
                            view! {cx,
                                <ForumCard forum={forum}/>
                            }
                        }
                    />
            </div>
        </div>
    }
}

#[component]
fn ForumCard(cx: Scope, forum: Forum) -> impl IntoView {
    view! {cx,
        <div class="bg-neutral-700 rounded-sm shadow-lg p-4 flex">
            <div class="w-3/5">
                <A href=move || format!("/forum/{}.{}", forum.slug, forum.id)>
                    <h2 class="text-xl font-bold">{forum.title}</h2>
                </A>
                <p class="text-sm text-text_secondary">{forum.description}</p>
            </div>
            <div class="flex">
                <div class="flex flex-col items-center">
                    <p>"Threads"</p>
                    <p>"1"</p>
                </div>
                <div class="flex flex-col items-center ml-6">
                    <p>"Messages"</p>
                    <p>"1"</p>
                </div>
            </div>
        </div>
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CategoryWithForums {
    pub category: Category,
    pub forums: Option<Vec<Forum>>,
}

#[server(GetHomePage)]
pub async fn get_home_data() -> Result<Vec<CategoryWithForums>, ServerFnError> {
    use crate::error::server_error;
    let mut conn = crate::database::get_db_pool().await.unwrap();

    //  retrieves all categories and their forums
    let query_result = sqlx::query!(
        r#"
        SELECT
        json_agg(
            json_build_object(
                'category',
                json_build_object(
                    'id', category.id,
                    'title', category.title,
                    'description', category.description,
                    'created_at', category.created_at,
                    'creator_id', category.creator_id
                ),
                'forums', forums.forums
            )
        ) AS result
    FROM
        category
        LEFT JOIN (
            SELECT
                category_id,
                json_agg(
                    json_build_object(
                        'id', id,
                        'title', title,
                        'description', description,
                        'slug', slug,
                        'category_id', category_id,
                        'created_at', created_at
                    ) ORDER BY id
                ) AS forums
            FROM
                forum
            GROUP BY
                category_id
        ) AS forums ON category.id = forums.category_id
        "#
    )
    .fetch_one(&conn)
    .await;
    match query_result {
        Ok(res) => {
            let data: Result<Vec<CategoryWithForums>, ServerFnError> = match res.result {
                Some(data) => data,
                None => return Ok(vec![]),
            }
            .as_array()
            .ok_or(ServerFnError::ServerError(
                "Internal server error".to_string(),
            ))?
            .into_iter()
            .map(|x| {
                serde_json::from_value::<CategoryWithForums>(x.clone()).map_err(|e| {
                    tracing::error!(
                        "Error serializing CategoryWithForums from the database rows: {}",
                        e.to_string()
                    );
                    ServerFnError::ServerError("Internal server error".to_string())
                })
            })
            .collect();
            return data;
        }
        Err(e) => {
            tracing::error!("Couldn't fetch the categories and its forums: {:#?}", e);
        }
    }
    Ok(vec![])
}
