use cfg_if::cfg_if;
use leptos::*;

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "hydrate")] {
        #[wasm_bindgen(module = "/public/sun_editor.js")]
        extern "C" {
            fn get_editor_text(id: String) -> String;
            fn create_editor(id: String) -> bool;
        }
    }

    else if #[cfg(feature = "ssr")] {

        #[allow(dead_code)]
        fn get_editor_text(id: String) -> String {
            "".into()
        }

        #[allow(dead_code)]
        fn create_editor(id: String) -> bool {
            false
        }
    }
}

#[component]
pub fn TextEditor<F>(
    cx: Scope,
    id: String,
    #[prop(optional)] class: String,
    on_submit: F,
) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    let _id = id.clone();
    let __id = id.clone();
    let on_click = move |_| {
        let id = _id.clone();
        let editor_value = get_editor_text(id);
        on_submit(editor_value);
    };

    create_effect(cx, move |_| {
        cfg_if::cfg_if! {
            if #[cfg(feature = "hydrate")] {
                create_editor(id.clone());
            }
        }
    });

    //  TODO: listen to the editor's change event and update a local state
    view! {cx,
        <textarea id=__id class=format!("{class}")/>
        <button on:click=on_click>"Submit"</button>
    }
}
