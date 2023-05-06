use leptos::*;
use papelito::{Papelito, PapelitoClasses};

#[component]
pub fn TextEditor(
    cx: Scope,
    key: String,
    content_signal: RwSignal<String>,
    #[prop(optional)] class: String,
) -> impl IntoView {
    let classes = PapelitoClasses {
        actionbar: "bg-neutral-50 rounded-sm h-[30px] flex justify-center border-b-1 border-neutral-950 text-black".to_string(),
        button:
            "bg-transparent border-none cursor-pointer h-[30px] w-[30px] outline-0 align-bottom flex items-center justify-center"
                .to_string(),
        content:
            "h-[480px] p-[10px] w-full text-neutral-950 bg-neutral-700 rounded-sm shadow-lg border border-neutral-800 border-box outline-0 overflow-y-auto border-solid mt-4 focus:outline-none"
                .to_string(),
        editor: format!("border-solid {class}"),
        selected: "bg-neutral-300".to_string(),
    };

    view! {cx,
        <Papelito content_signal=content_signal key=key classes=classes/>
    }
}
