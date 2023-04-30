use leptos::*;

const REPO_URL: &'static str = "https://www.github.com/msmaiaa/rustbb";

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! {cx,
        <footer id="footer" class="w-full bg-bg_darker h-16 flex justify-center items-center">
            <a href=REPO_URL target="_blank">
                <svg class="fab fa-github" class="h-[25px] w-[25px]"></svg>
            </a>
        </footer>
    }
}
