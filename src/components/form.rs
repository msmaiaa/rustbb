use leptos::*;

#[component]
pub fn FormRow(
    cx: Scope,
    label: &'static str,
    _type: &'static str,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] required: bool,
) -> impl IntoView {
    view! {cx,
        <div class=format!("flex {class}")>
            <label for=label class="w-1/3 text-right mr-2">{label}</label>
            <Input required=required _type=_type name=label id=label class="w-2/3"/>
        </div>
    }
}

#[component]
pub fn Input(
    cx: Scope,
    _type: &'static str,
    name: &'static str,
    id: &'static str,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] required: bool,
) -> impl IntoView {
    view! {cx,
        <input required=required class=format!("pl-1 h-[28px] rounded-sm {class}") type=_type name=name id=id/>
    }
}
