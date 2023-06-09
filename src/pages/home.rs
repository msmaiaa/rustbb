#![allow(unused)]

use crate::components::link::*;
use crate::model::{category::Category, forum::*};
use cfg_if::cfg_if;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let home_data = create_resource(cx, || (), move |_| async move { get_home_data(cx).await });

    let home_view = move |cx| {
        home_data.read(cx).map(|data| match data {
            Ok(data) => {
                view! {cx,
                    <div class="h-full w-full">
                        <div class="flex flex-col w-full">
                        <For
                            each=move || data.clone()
                            key=|n| n.id.id.to_raw()
                            view = move |cx, data| {
                                view! {cx,
                                    <CategoryCard category={data}/>
                                }
                            }
                        />
                        </div>
                    </div>
                }
            }
            Err(_) => {
                view! {cx,
                    <div class="h-full w-full">
                        ""
                    </div>
                }
            }
        })
    };

    view! { cx,
        <Suspense fallback= move || {
            view! {cx,
                <div class="h-full w-full">
                    ""
                </div>
            }
        }>{home_view(cx)}</Suspense>
    }
}

#[component]
fn CategoryCard(cx: Scope, category: HomeCategory) -> impl IntoView {
    let forums = category.forums.clone();
    view! {cx,
        <div class="bg-neutral-800 rounded-lg shadow-lg p-4 mb-3">
            <h2 class="text-2xl">{category.title}</h2>
            <p class="text-sm text-text_secondary">{category.description}</p>
            <div class="flex flex-col">
                    <For
                        each=move || forums.clone()
                        key=|n| n.id.id.to_raw()
                        view = move |cx, forum| {
                            view! {cx,
                                <ForumCard class="mt-2" forum={forum}/>
                            }
                        }
                    />
            </div>
        </div>
    }
}

#[component]
fn ForumCard(cx: Scope, forum: HomeForum, #[prop(optional)] class: &'static str) -> impl IntoView {
    let mut forum_path = format!("/forum/{}", forum.get_id());
    let description = forum.description.clone().unwrap_or_default();
    let title_container_class = if description.is_empty() {
        "flex items-center"
    } else {
        ""
    };
    view! {cx,
        <div class=format!("bg-neutral-700 rounded shadow-lg p-4 flex {class}")>
            <div class=format!("w-3/5 {title_container_class}")>
                <h2 class="text-lg font-normal">
                    <RouteLink to=forum_path>
                        {forum.title}
                    </RouteLink>
                </h2>

                <p class="text-sm text-text_secondary bg-red-200">{forum.description.unwrap_or_default()}</p>
            </div>
            <div class="flex">
                <div class="flex flex-col items-center">
                    <p>"Threads"</p>
                    <p>{forum.threads_amt}</p>
                </div>
                <div class="flex flex-col items-center ml-6">
                    <p>"Messages"</p>
                    <p>"1"</p>
                </div>
            </div>
        </div>
    }
}

//  use surrealdb, they said
//  it will be fun, they said
macro_rules! impl_get_id {
    ($ty:ty, $tb: literal) => {
        impl $ty {
            pub fn get_id(&self) -> String {
                let mut id = self.id.id.to_raw();
                cfg_if! {
                    if #[cfg(not(feature="ssr"))] {
                        id = id.replace(&format!("{}:", $tb), "");
                    }
                }
                id
            }
        }
    };
}

#[derive(Serialize, Clone, Debug)]
pub struct HomeForum {
    pub slug: String,
    pub title: String,
    pub id: surrealdb::sql::Thing,
    pub description: Option<String>,
    pub threads_amt: i32,
}

impl_get_id!(HomeForum, "forum");

#[macro_export]
macro_rules! deser_map {
    ($map:ident, $field:ident, $ty:ty) => {
        let $field = $map
            .remove(stringify!($field))
            .ok_or_else(|| serde::de::Error::missing_field(stringify!($field)))?;
        let $field = serde_json::from_value::<$ty>($field).map_err(serde::de::Error::custom)?;
    };
}

pub(crate) use deser_map;

//  "Thing" deserialization is fucked up TODO: create a better macro
impl<'de> Deserialize<'de> for HomeForum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;
        deser_map!(map, slug, String);
        deser_map!(map, title, String);
        deser_map!(map, threads_amt, i32);
        deser_map!(map, description, Option<String>);

        let id = map
            .remove("id")
            .ok_or_else(|| serde::de::Error::missing_field("id"))?;

        cfg_if! {
            if #[cfg(not(feature="ssr"))] {
                Ok(Self {
                    id: surrealdb::sql::Thing {
                        id: surrealdb::sql::Id::String(serde_json::from_value(id.clone()).unwrap()),
                        tb: "forum".to_string(),
                    },
                    title,
                    slug,
                    description,
                    threads_amt
                })
            } else {
                let id = id.get("id").unwrap().get("String").unwrap();
                let id = surrealdb::sql::Thing {
                    id: surrealdb::sql::Id::String(serde_json::from_value(id.clone()).unwrap()),
                    tb: "forum".to_string(),
                };
                Ok(Self {
                    id,
                    title,
                    slug,
                    description,
                    threads_amt
                })
            }
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct HomeCategory {
    pub id: surrealdb::sql::Thing,
    pub title: String,
    pub description: Option<String>,
    pub forums: Vec<HomeForum>,
}

impl_get_id!(HomeCategory, "category");

impl<'de> Deserialize<'de> for HomeCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;
        deser_map!(map, forums, Vec<HomeForum>);
        deser_map!(map, title, String);
        deser_map!(map, description, Option<String>);
        let id = map
            .remove("id")
            .ok_or_else(|| serde::de::Error::missing_field("id"))?;

        cfg_if! {
            if #[cfg(not(feature="ssr"))] {
                Ok(Self {
                    id: surrealdb::sql::Thing {
                        id: surrealdb::sql::Id::String(serde_json::from_value(id.clone()).unwrap()),
                        tb: "category".to_string(),
                    },
                    title,
                    description,
                    forums,
                })
            } else {
                let id = id.get("id").unwrap().get("String").unwrap();
                let id = surrealdb::sql::Thing {
                    id: surrealdb::sql::Id::String(serde_json::from_value(id.clone()).unwrap()),
                    tb: "category".to_string(),
                };
                Ok(Self {
                    id,
                    title,
                    description,
                    forums,
                })
            }
        }
    }
}

#[server(GetHomeData, "/api")]
pub async fn get_home_data(cx: Scope) -> Result<Vec<HomeCategory>, ServerFnError> {
    use crate::database::get_db;
    use crate::error::server_error;
    let db = get_db(cx).await?;
    let result = db.query("SELECT id, title, description, 
                           (SELECT title, slug, id, description, count((SELECT * FROM thread WHERE forum == $parent.id)) as threads_amt from forums.*.*) AS forums FROM category")
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .take::<Vec<HomeCategory>>(0)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(result)
}
