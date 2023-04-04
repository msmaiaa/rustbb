mod app;
mod auth;
mod components;
mod database;
mod error;
mod error_template;
mod fallback;
mod global;
mod model;
mod pages;
mod permission_entries;
use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use leptos::*;
    use crate::app::*;
    use std::sync::Arc;
    use axum::{
        response::{Response, IntoResponse},
        routing::{post, get},
        extract::{Path, Extension},
        http::{Request, header::HeaderMap, Uri},
        body::Body as AxumBody,
        Router,
    };
    use crate::fallback::*;
    use strum::{IntoEnumIterator};
    use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};

    fn register_server_functions() {
        use crate::pages::register::RegisterUser;
        use crate::components::login_form::Login;
        _ = RegisterUser::register();
        _ = Login::register();
        _ = GetCurrentUser::register();
    }

    // async fn server_fn_handler(path: Path<String>, headers: HeaderMap, request: Request<AxumBody>) -> impl IntoResponse {
    //     handle_server_fns_with_context(path, headers, move |_| {}, request).await
    // }

    async fn render_frontend(req: Request<AxumBody>, options: Arc<LeptosOptions>, app_fn: impl Fn(Scope) + Clone + Send + Sync + 'static) -> Response
        {
        let handler = leptos_axum::render_app_to_stream_with_context((*options).clone(),
        app_fn,
            |cx| view! { cx, <App/> }
        );
        handler(req).await.into_response()
    }

    async fn frontend_routes_handler(Extension(options): Extension<Arc<LeptosOptions>>, uri: Uri, req: Request<AxumBody>) -> Response{
        let uri = uri.clone();
        for page in crate::pages::Page::iter() {
            if uri.path().eq(&page.path()) {
                if let Some(f) = page.preload_fn() {
                    let cb = f.await;
                    return render_frontend(req, options, move |cx| {
                        cb(cx);
                    }).await;
                }
            }
        }
        return render_frontend(req, options, move |_| {}).await;
    }

    #[tokio::main]
    async fn main() {
        use dotenv::dotenv;
        dotenv().ok();
        use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};

        tracing_subscriber::fmt::init();

        database::setup().await;

        let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
        let addr = conf.leptos_options.site_addr.clone();
        let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

        register_server_functions();

        let app = Router::new()
        .leptos_routes_with_handler(routes, get(frontend_routes_handler) )
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .fallback(file_and_error_handler)
        .layer(Extension(Arc::new(conf.leptos_options)));

        axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    }
}
else {
    pub fn main() {}
}
}
