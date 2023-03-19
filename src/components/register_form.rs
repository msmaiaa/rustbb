use super::button::*;
use super::card::*;
use super::form::*;
use leptos::*;

#[component]
pub fn RegisterForm(cx: Scope) -> impl IntoView {
    view! {cx,
        <Card title="Register" class="w-full flex flex-col">
            <form>
                <FormRow required=true label="Username" _type="text" class="mb-2"/>
                <FormRow required=true label="Email" _type="email" class="mb-2"/>
                <FormRow required=true label="Password" _type="password" class="mb-2"/>
                <FormRow required=true label="Confirm password" _type="password"/>
                <Button _type="submit">"Register"</Button>
            </form>
        </Card>
    }
}
