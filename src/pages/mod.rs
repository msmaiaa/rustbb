use std::future::Future;

use leptos::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use wildmatch::WildMatch;

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
}

impl Page {
    pub fn path(&self) -> String {
        match self {
            Page::Home => "/".to_string(),
            Page::Forum => "/forum/:id".to_string(),
            Page::Login => "/login".to_string(),
            Page::Register => "/register".to_string(),
        }
    }

    #[cfg(feature = "ssr")]
    pub fn preload_fn(&self) -> Option<impl Future<Output = impl Fn(Scope) + Clone>> {
        match self {
            Page::Home => Some(async {
                let home_data = crate::pages::home::get_home_data().await.unwrap();
                return move |cx| {
                    provide_context(cx, home_data.clone());
                };
            }),
            Page::Forum => None,
            Page::Login => None,
            Page::Register => None,
        }
    }

    pub fn from_uri(uri: &str) -> Option<Page> {
        Page::iter().find(|page| {
            let path_with_wc = page
                .path()
                .split("/")
                .map(|part| {
                    if part.starts_with(":") {
                        "*".to_string()
                    } else {
                        part.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("/");
            WildMatch::new(&path_with_wc).matches(uri)
        })
    }
}
