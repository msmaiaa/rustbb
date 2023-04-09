use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use wildmatch::WildMatch;

pub mod create_thread;
pub mod forum;
pub mod home;
pub mod login;
pub mod register;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, EnumIter)]
pub enum Page {
    Home,
    Forum,
    Login,
    Register,
    CreateThread,
}

impl Page {
    #[allow(dead_code)]
    pub fn path(&self) -> &'static str {
        match self {
            Page::Home => "/",
            Page::Forum => "/forum/:slug_dot_id",
            Page::Login => "/login",
            Page::Register => "/register",
            Page::CreateThread => "/forum/:slug_dot_id/create_thread",
        }
    }

    // #[cfg(feature = "ssr")]
    // pub fn preload_fn(
    //     &self,
    //     pool: sqlx::Pool<sqlx::Postgres>,
    //     uri: http::Uri,
    // ) -> Option<impl futures::Future<Output = Option<impl Fn(leptos::Scope) + Clone>>> {
    //     use itertools::Itertools;
    //     match self {
    //         Page::Home => None,
    //         Page::Forum => None,
    //         Page::Login => None,
    //         Page::Register => None,
    //         Page::CreateThread => None,
    //     }
    // }

    #[allow(dead_code)]
    pub fn from_uri(uri: &str) -> Option<Page> {
        Page::iter().find(|page| {
            let path_with_wc = page
                .path()
                .split("/")
                .map(|part| match part.starts_with(":") {
                    true => "*".to_string(),
                    false => part.to_string(),
                })
                .collect::<Vec<_>>()
                .join("/");
            WildMatch::new(&path_with_wc).matches(uri)
        })
    }
}
