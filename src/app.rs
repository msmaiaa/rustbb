use crate::components::{footer::*, header::*, nav::*, sidebar::*};
use crate::pages::forum::*;
use crate::pages::home::*;
use crate::pages::login::*;
use crate::pages::register::*;
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
            <div class="text-text_primary bg-bg_primary min-h-screen relative">
                <Header/>
                <Navbar/>
                <Layout>
                    <Routes>
                        <Route path="" view=move |cx| view! { cx, <Home/> }/>
                        <Route path="forum/:id" view=move |cx| view! { cx, <ForumPage/> }/>
                        <Route path="login" view=move |cx| view! { cx, <Login/> }/>
                        <Route path="register" view=move |cx| view! { cx, <Register/> }/>
                    </Routes>
                    <RightSidebar/>
                </Layout>
                <Footer/>
            </div>
        </Router>
    }
}

#[component]
fn Layout(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <main class="text-inherit bg-inherit h-full flex items-center justify-center">
            <div class="w-[1100px] flex">
                {children(cx)}
            </div>
        </main>
    }
}
