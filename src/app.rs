#![allow(dead_code)]
use crate::components::{footer::*, header::*, nav::*, sidebar::*};
use crate::pages::create_thread::*;
use crate::pages::forum::*;
use crate::pages::home::*;
use crate::pages::login::*;
use crate::pages::register::*;
use crate::pages::Page;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub struct LoggedUserData {
    pub username: String,
    pub avatar_url: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetCurrentUserResponse {
    pub username: String,
    pub avatar_url: Option<String>,
}

#[cfg(feature = "ssr")]
pub fn token_from_cookie(
    req: &http::Request<axum::body::Body>,
) -> Result<crate::auth::AccessToken, ServerFnError> {
    use crate::auth::*;
    use crate::error::server_error;
    use cookie::Cookie;

    let cookies = req
        .headers()
        .get(http::header::COOKIE)
        .ok_or(ServerFnError::ServerError(
            "Couldn't get the request's cookies.".to_string(),
        ))?
        .to_str()
        .ok()
        .map(|cookies| cookies.to_owned())
        .unwrap();
    let mut found_cookie = None;
    for cookie in Cookie::split_parse(cookies) {
        let cookie = cookie.unwrap();
        match cookie.name() {
            "auth_token" => {
                found_cookie = Some(cookie);
                break;
            }
            _ => {}
        }
    }
    let token = match found_cookie {
        Some(token) => token.value().to_owned(),
        None => return server_error!("Couldn't find the authorization token cookie."),
    };

    let token_data: AccessToken = match decode_access_token(&token, &crate::global::JWT_KEY) {
        Ok(token) => token,
        Err(e) => return server_error!(e),
    };
    Ok(token_data)
}

#[server(GetCurrentUser, "/api")]
pub async fn get_current_user(cx: Scope) -> Result<GetCurrentUserResponse, ServerFnError> {
    use crate::database::get_db;
    use crate::error::server_error;

    let req = match use_context::<leptos_axum::LeptosRequest<axum::body::Body>>(cx) {
        Some(req) => req.take_request().unwrap(),
        None => return server_error!("Couldn't get the request's info."),
    };

    let token_data = token_from_cookie(&req)?;

    let user =
        match crate::model::user::ForumUser::find_by_id(&get_db(cx).await?, token_data.user_id)
            .await
        {
            Ok(user) => match user {
                Some(user) => user,
                None => return server_error!("Couldn't find the user."),
            },
            Err(e) => {
                tracing::info!("{}", e.to_string());
                return server_error!("Internal server error");
            }
        };

    return Ok(GetCurrentUserResponse {
        username: user.username,
        avatar_url: user.avatar_url,
    });
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    let user_data = create_rw_signal(cx, None::<LoggedUserData>);

    provide_context(cx, user_data);

    //  FIXME: we need to check if we have a user before rendering anything
    leptos::spawn_local(async move {
        match get_current_user(cx).await {
            Ok(user) => {
                user_data.update(|v| {
                    *v = Some(LoggedUserData {
                        username: user.username,
                        avatar_url: user.avatar_url,
                    })
                });
            }
            Err(_) => {}
        }
    });

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/rustbb.css"/>
        <Link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.3.0/css/all.min.css" crossorigin="anonymous" referrerpolicy="no-referrer" />
        <Script src="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.3.0/js/all.min.js" crossorigin="anonymous" referrerpolicy="no-referrer"></Script>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>

        /* sun text editor */
        <Link fetchpriority="high" href="/vendor/suneditor.min.css" rel="stylesheet"/>
        <Script fetchpriority="high" src="/vendor/suneditor.min.js"></Script>
        <Script fetchpriority="high" src="/vendor/suneditor.en.js"></Script>

        <Title text="rustbb"/>
        <Router>
            <div class="text-text_primary bg-bg_primary min-h-screen relative">
                <Header/>
                <Navbar/>
                <Layout>
                    <Routes>
                        <Route path={Page::Home.path()} view=move |cx| view! { cx, <Home/> }/>
                        <Route path={Page::CreateThread.path()} view=move |cx| view! { cx, <CreateThreadPage/> }/>
                        <Route path={Page::Forum.path()} view=move |cx| view! { cx, <ForumPage/> }/>
                        <Route path={Page::Login.path()} view=move |cx| view! { cx, <Login/> }/>
                        <Route path={Page::Register.path()} view=move |cx| view! { cx, <Register/> }/>
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
