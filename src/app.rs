use crate::pages::forum::{ForumPage, ForumPageProps};
use crate::pages::home::{Home, HomeProps};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/rustbb.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Title text="rustbb"/>
        <Router>
            <div class="text-text_primary bg-bg_primary min-h-screen">
                <Header/>
                <Navbar/>
                <Layout>
                    <Routes>
                        <Route path="" view=  move |cx| view! { cx, <Home/> }/>
                        <Route path="/forum/:id" view=  move |cx| view! { cx, <ForumPage/> }/>
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
                <p class="text-sm">"No members are currently online"</p>
            </Card>
            <ForumStatisticsCard/>
        </div>
    }
}

#[component]
pub fn ForumStatisticsCard(cx: Scope) -> impl IntoView {
    view! {cx,
        <Card title="Forum statistics" class="mt-3">
            <div class="flex flex-col">
                <div class="flex justify-between">
                    <p>"Threads:"</p>
                    <p>"1"</p>
                </div>
                <div class="flex justify-between">
                    <p>"Messages:"</p>
                    <p>"1"</p>
                </div>
                <div class="flex justify-between">
                    <p>"Members:"</p>
                    <p>"1"</p>
                </div>
            </div>
        </Card>
    }
}

#[component]
pub fn Card(
    cx: Scope,
    title: &'static str,
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <div class=format!("bg-neutral-800 rounded-md shadow-lg p-3 {class}")>
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
        <nav id="navbar" class="bg-bg_primary">
            <div class="flex justify-between items-center">
                <div class="flex items-center">
                    <a href="/" class="text-2xl font-bold text-text_primary">"Leptos"</a>
                </div>
                <div class="flex items-center">
                    <a href="/login" class="text-lg font-bold text-text_primary">"Login"</a>
                    <a href="/register" class="text-lg font-bold text-text_primary ml-4">"Register"</a>
                </div>
            </div>
        </nav>
    }
}
