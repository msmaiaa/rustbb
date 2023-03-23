mod app;
mod auth;
mod components;
mod database;
mod error;
mod global;
mod model;
mod pages;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use actix_web::*;
        use leptos::*;
        use crate::app::*;
        use actix_files::Files;

        #[get("/style.css")]
        async fn css() -> impl Responder {
            actix_files::NamedFile::open_async("./style/output.css").await
        }

        fn register_server_functions() {
            use crate::pages::home::GetHomePage;
            use crate::pages::register::RegisterUser;
            use crate::components::login_form::Login;
            _ = GetHomePage::register();
            _ = RegisterUser::register();
            _ = Login::register();
            _ = GetCurrentUser::register();
        }

        #[actix_web::main]
        async fn main() -> std::io::Result<()> {
            use dotenv::dotenv;
            dotenv().ok();
            use leptos_actix::{generate_route_list, LeptosRoutes};

            tracing_subscriber::fmt::init();

            database::setup().await;

            // Setting this to None means we'll be using cargo-leptos and its env vars.
            let conf = get_configuration(None).await.unwrap();
            let addr = conf.leptos_options.site_addr.clone();
            let routes = generate_route_list(|cx| view! { cx, <App/> });

            register_server_functions();

            HttpServer::new(move || {
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;
                let routes = &routes;
                App::new()
                    .service(css)
                    .route("/{tail:.*}", leptos_actix::handle_server_fns())
                    .leptos_routes(leptos_options.to_owned(), routes.to_owned(), |cx| view! { cx, <App/> })
                    .service(Files::new("/", &site_root))
                    .wrap(middleware::Compress::default())
            })
            .bind(&addr)?
            .run()
            .await
        }
    }
    else {
        pub fn main() {}
    }
}
