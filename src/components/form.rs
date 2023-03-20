use leptos::*;

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
    view! {cx,
        <div class=format!("flex {class}")>
            <label for=label class="w-1/3 text-right mr-2">{label}</label>
            <input
                on:keyup = move |ev: ev::KeyboardEvent| {
                    let val = event_target_value(&ev);
                    if let Some(signal) = signal {
                        signal.set(val);
                    }
                }
                on:change = move |ev| {
                    let val = event_target_value(&ev);
                    if let Some(signal) = signal {
                        signal.set(val);
                    }
                }
            required=required class="w-2/3 pl-1 h-[28px] rounded-sm" type=_type name=label id=id/>
        </div>
    }
}
