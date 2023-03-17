use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let home_page_data = get_home_data();
    view! { cx,
        <div class="h-full w-full">
            <For
                each=move || home_page_data.clone()
                key=|(c, _)| c.id
                view = move |cx, (category, forums)| {
                    view! {cx,
                        <CategoryCard category={category} forums={forums}/>
                    }
                }
            />
        </div>
    }
}

#[derive(Clone)]
struct Category {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub creator_id: String,
}

#[derive(Clone)]
struct Forum {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub slug: String,
    pub category_id: i64,
}

#[component]
fn CategoryCard(cx: Scope, category: Category, forums: Vec<Forum>) -> impl IntoView {
    view! {cx,
        <div class="bg-neutral-800 rounded-lg shadow-lg p-4 mb-3">
            <h2 class="text-2xl font-bold">{category.name}</h2>
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
                <a href=format!("/forum/{}.{}", forum.slug, forum.id)>
                    <h2 class="text-xl font-bold">{forum.title}</h2>
                </a>
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

fn get_home_data() -> Vec<(Category, Vec<Forum>)> {
    let mock_categories = vec![
        Category {
            id: 1,
            name: "General".to_string(),
            description: "General discussion about the forum".to_string(),
            creator_id: "1".to_string(),
        },
        Category {
            id: 2,
            name: "Most popular".to_string(),
            description: "Most popular stuff".to_string(),
            creator_id: "1".to_string(),
        },
        Category {
            id: 3,
            name: "Off-Topic".to_string(),
            description: "Off-Topic discussion about the forum".to_string(),
            creator_id: "1".to_string(),
        },
    ];

    mock_categories
        .into_iter()
        .map(|c| {
            let mock_node = Forum {
                id: 1,
                title: "This is a forum title".to_string(),
                category_id: c.id,
                description: "This is a forum description".to_string(),
                slug: "this-is-a-forum-slug".to_string(),
            };
            let mock_node_2 = Forum {
                id: 2,
                title: "This is a forum title2".to_string(),
                category_id: c.id,
                description: "This is a forum 2".to_string(),
                slug: "this-is-a-forum-slug-2".to_string(),
            };
            (c, vec![mock_node, mock_node_2])
        })
        .collect()
}
