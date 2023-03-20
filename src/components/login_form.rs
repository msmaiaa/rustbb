use super::button::*;
use super::card::*;
use super::form::*;
use crate::pages::login::LoginPayload;
use leptos::*;

#[component]
pub fn LoginForm<F>(cx: Scope, on_login: F) -> impl IntoView
where
    F: Fn(LoginPayload) + 'static,
{
    let email = create_rw_signal(cx, "".to_string());
    let password = create_rw_signal(cx, "".to_string());

    let on_submit = move || {
        if email.get() != "" && password.get() != "" {
            on_login(LoginPayload {
                email: email.get(),
                password: password.get(),
            });
        }
    };

    view! {cx,
        <Card title="Login" class="w-full flex flex-col">
            <form on:submit= move |ev| {
                ev.prevent_default();
                on_submit();
            }>
                <FormRow signal=Some(email) required=true label="Email" id="email" _type="email" class="mb-2"/>
                <FormRow signal=Some(password) required=true label="Password" id="password" _type="password"/>
                <Button _type="submit">"Login"</Button>
            </form>
        </Card>
    }
}
