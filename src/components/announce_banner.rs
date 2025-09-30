use leptos::prelude::*;

#[component]
pub fn AnnounceBanner() -> impl IntoView {
    view! {
        <div class="background_announcement w-full flex items-center gap-3 px-6 py-3">
            <img src="/images/flag.svg" alt="Flag Icon" class="w-6 h-6" style="display: inline-block;" />
            <a href="/announcements" class="p3 underline hover:text-blue-600 transition-colors duration-150">
                {"This is an important announcement!"}
            </a>
        </div>
    }
}
