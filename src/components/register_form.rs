use crate::pages::register::RegisterUserPayload;

use super::button::*;
use super::card::*;
use super::form::*;
use leptos::*;

#[component]
pub fn RegisterForm<F>(cx: Scope, on_register: F) -> impl IntoView
where
    F: Fn(RegisterUserPayload) + 'static,
{
    let username = create_rw_signal(cx, "".to_string());
    let email = create_rw_signal(cx, "".to_string());
    let password = create_rw_signal(cx, "".to_string());
    let password_confirmation = create_rw_signal(cx, "".to_string());
    let (error, set_error) = create_signal(cx, "".to_string());

    let on_submit = move || {
        if password.get() != password_confirmation.get() {
            set_error("Passwords do not match".to_string());
            return;
        } else if !error().is_empty() {
            set_error("".to_string());
        }
        if !username.get().is_empty() && !email.get().is_empty() {
            on_register(RegisterUserPayload {
                username: username.get(),
                email: email.get(),
                password: password.get(),
                password_confirmation: password_confirmation.get(),
            });
        }
    };

    view! {cx,
        <Card title="Register" class="w-full flex flex-col">
            <form on:submit= move |ev| {
                ev.prevent_default();
                on_submit();
            }>
                <FormRow signal=Some(username) required=true label="Username" id="username" _type="text" class="mb-2"/>
                <FormRow signal=Some(email) required=true label="Email" id="email" _type="email" class="mb-2"/>
                <FormRow signal=Some(password) required=true label="Password" id="password" _type="password" class="mb-2"/>
                <FormRow signal=Some(password_confirmation) required=true label="Confirm password" id="password_confirmation" _type="password"/>
                {move || {
                    if !error().is_empty() {
                        view! {cx,
                            <div class="text-red-500 text-sm">{error()}</div>
                        }
                    } else {
                        view! {cx, <div></div>}
                    }
                }}
                <Button _type="submit">"Register"</Button>
            </form>
        </Card>
    }
}
