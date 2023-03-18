mod app;
mod db;
mod model;
mod pages;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use actix_files::Files;
        use actix_web::*;
        use leptos::*;
        use crate::app::*;
        use leptos_actix::{generate_route_list, LeptosRoutes};
        use dotenv::dotenv;

        #[get("/style.css")]
        async fn css() -> impl Responder {
            actix_files::NamedFile::open_async("./style/output.css").await
        }

        async fn migrate(db_pool: &sqlx::Pool<sqlx::Postgres>) {
            match sqlx::migrate!("./migrations")
            .run(db_pool)
            .await {
                Ok(_) => tracing::info!("[sqlx] Database migrations ran successfully"),
                Err(e) => tracing::error!("[sqlx] Database migrations failed: {}", e),
            };
        }


        async fn init_forum(db_pool: &sqlx::Pool<sqlx::Postgres>) {
            use model::main_forum::MainForum;
            match MainForum::get_main_forum(db_pool).await {
                Ok(_) => tracing::info!("The main forum is already set."),
                Err(e) => {
                    match e {
                        sqlx::Error::RowNotFound => {
                            tracing::info!("The main forum is not set. Creating it now.");
                            if let Err(e) = MainForum::create(db_pool, "rustbb").await {
                                tracing::error!("Couldn't create the main forum :( {}", e);
                            }
                        }
                        _ => {
                            tracing::error!("Error while getting the main forum: {}", e);
                        }
                    }
                }
            }
        }

        #[actix_web::main]
        async fn main() -> std::io::Result<()> {
            use crate::pages::home::GetHomePage;

            dotenv().ok();
            tracing_subscriber::fmt::init();


            let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let db_pool = db::get_db_pool(&database_url).await.expect("Couldn't connect to the database");
            migrate(&db_pool).await;

            init_forum(&db_pool).await;

            // Setting this to None means we'll be using cargo-leptos and its env vars.
            let conf = get_configuration(None).await.unwrap();
            let addr = conf.leptos_options.site_addr.clone();
            // Generate the list of routes in your Leptos App
            let routes = generate_route_list(|cx| view! { cx, <App/> });


            _ = GetHomePage::register();

            HttpServer::new(move || {
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;
                let routes = &routes;
                App::new()
                    .service(css)
                    .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
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
