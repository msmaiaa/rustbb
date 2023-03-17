use leptos::*;
use leptos::{For, ForProps, *};
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/rustbb.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <div class="text-text_primary bg-bg_primary min-h-screen">
                <Header/>
                <Navbar/>
                <Layout>
                    <Routes>
                        <Route path="" view=  move |cx| view! { cx, <Home/> }/>
                    </Routes>
                    <RightSidebar/>
                </Layout>
                <Footer/>
            </div>
        </Router>
    }
}

#[component]
pub fn RightSidebar(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="flex flex-col w-64 ml-3">
            <Card title="Members online">
                <p class="text-sm text-text_secondary">"No members are currently online"</p>
            </Card>
        </div>
    }
}

#[component]
pub fn Card(cx: Scope, title: &'static str, children: Children) -> impl IntoView {
    view! {cx,
        <div class="bg-neutral-800 rounded-lg shadow-lg p-4">
            <h2 class="text-2xl font-bold">{title}</h2>
            <div class="flex flex-col">
                {children(cx)}
            </div>
        </div>
    }
}

#[component]
fn Layout(cx: Scope, children: Children) -> impl IntoView {
    provide_meta_context(cx);
    view! {cx,
        <main class="text-inherit bg-inherit h-full flex items-center justify-center">
            <div class="w-[1100px] flex">
                {children(cx)}
            </div>
        </main>
    }
}

#[derive(Clone)]
struct Category {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub forum_id: String,
    pub creator_id: String,
}

#[derive(Clone)]
struct CategoryNode {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub category_id: i64,
}

#[component]
fn CategoryCard(cx: Scope, category: Category, nodes: Vec<CategoryNode>) -> impl IntoView {
    view! {cx,
        <div class="bg-neutral-800 rounded-lg shadow-lg p-4 mb-3">
            <h2 class="text-2xl font-bold">{category.name}</h2>
            <p class="text-sm text-text_secondary">{category.description}</p>
            <div class="flex flex-col">
                <For
                    each=move || nodes.clone()
                    key=|n| n.id
                    view = move |cx, node| {
                        view! {cx,
                            <div class="bg-neutral-700 rounded-sm shadow-lg p-4">
                                <h2 class="text-2xl font-bold">{node.name}</h2>
                                <p class="text-sm text-text_secondary">{node.description}</p>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

fn get_home_data() -> Vec<(Category, Vec<CategoryNode>)> {
    let mock_categories = vec![
        Category {
            id: 1,
            name: "General".to_string(),
            forum_id: "1".to_string(),
            description: "General discussion about the forum".to_string(),
            creator_id: "1".to_string(),
        },
        Category {
            id: 2,
            name: "Most popular".to_string(),
            forum_id: "1".to_string(),
            description: "Most popular stuff".to_string(),
            creator_id: "1".to_string(),
        },
        Category {
            id: 3,
            name: "Off-Topic".to_string(),
            forum_id: "1".to_string(),
            description: "Off-Topic discussion about the forum".to_string(),
            creator_id: "1".to_string(),
        },
    ];

    mock_categories
        .into_iter()
        .map(|c| {
            let mock_node = CategoryNode {
                id: 1,
                name: "Random tutorials".to_string(),
                category_id: c.id,
                description: "Random tutorials".to_string(),
            };
            (c, vec![mock_node])
        })
        .collect()
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    let home_page_data = get_home_data();
    view! { cx,
        <div class="h-full w-full">
            <For
                each=move || home_page_data.clone()
                key=|(c, _)| c.id
                view = move |cx, (category, nodes)| {
                    view! {cx,
                        <CategoryCard category={category} nodes={nodes}/>
                    }
                }
            />
            // <h2 class="p-6 text-4xl text-green-400">"Welcome to Leptos with Tailwind"</h2>
            // <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            // <button
            //     class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
            //     on:click=move |_| set_count.update(|count| *count += 1)
            // >
            //     "Something's here | "
            //     {move || if count() == 0 {
            //         "Click me!".to_string()
            //     } else {
            //         count().to_string()
            //     }}
            //     " | Some more text"
            // </button>
        </div>
    }
}

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! {cx,
        <footer id="footer">"Hello from the footer"</footer>
    }
}

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! {cx,
        <header id="header">"Hello from the header"</header>
    }
}

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    view! {cx,
        <nav id="navbar" class="bg-bg_primary">"Hello from the navbar"</nav>
    }
}
