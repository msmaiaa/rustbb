use leptos::*;

#[component]
pub fn Input(
    cx: Scope,
    signal: Option<RwSignal<String>>,
    #[prop(optional)] _type: &'static str,
    #[prop(optional)] id: &'static str,
    #[prop(optional)] name: &'static str,
    #[prop(optional)] value: &'static str,
    #[prop(optional)] placeholder: &'static str,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] required: bool,
) -> impl IntoView {
    view! {cx,
        <input
            on:keyup = move |ev| {
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
            type=_type
            id=id
            name=name
            value=value
            placeholder=placeholder
            class=format!("bg-neutral-700 rounded-sm shadow-lg p-2 w-full {class}")
            required=required
        />
    }
}
