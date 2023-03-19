use crate::components::card::*;
use leptos::*;

#[component]
pub fn RightSidebar(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="flex flex-col w-64 ml-3">
            <Card title="Members online">
                <p class="text-sm">"No members are currently online"</p>
            </Card>
            <ForumStatistics/>
        </div>
    }
}

#[component]
pub fn ForumStatistics(cx: Scope) -> impl IntoView {
    view! {cx,
        <Card title="Forum statistics" class="mt-3">
            <div class="flex flex-col">
                <div class="flex justify-between">
                    <p>"Threads:"</p>
                    <p>"1"</p>
                </div>
                <div class="flex justify-between">
                    <p>"Messages:"</p>
                    <p>"1"</p>
                </div>
                <div class="flex justify-between">
                    <p>"Members:"</p>
                    <p>"1"</p>
                </div>
            </div>
        </Card>
    }
}
