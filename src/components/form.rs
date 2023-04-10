use leptos::*;

use crate::components::input::Input;

#[component]
pub fn FormRow(
    cx: Scope,
    label: &'static str,
    _type: &'static str,
    id: &'static str,
    signal: Option<RwSignal<String>>,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] required: bool,
) -> impl IntoView {
    //  TODO:   clear inputs on submit
    view! {cx,
        <div class=format!("flex {class}")>
            <label for=id class="w-1/3 text-right mr-2">{label}</label>
            <Input
                _type=_type
                id=id
                signal=signal
                class="w-2/3 pl-1 h-[28px] rounded-sm bg-zinc-800"
                required=required
            />
        </div>
    }
}
