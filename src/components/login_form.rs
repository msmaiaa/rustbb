use super::button::*;
use super::card::*;
use super::form::*;
use leptos::*;

#[component]
pub fn LoginForm(cx: Scope) -> impl IntoView {
    view! {cx,
        <Card title="Login" class="w-full flex flex-col">
            <form>
                <FormRow required=true label="Email" _type="email" class="mb-2"/>
                <FormRow required=true label="Password" _type="password"/>
                <Button _type="submit">"Login"</Button>
            </form>
        </Card>
    }
}
