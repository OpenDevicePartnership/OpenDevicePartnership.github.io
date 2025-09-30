use leptos::prelude::*;

#[component]
pub fn AnnounceBanner() -> impl IntoView {
    view! {
        <div class="background_announcement w-full flex items-center gap-3 px-6 py-3">
            <img src="/images/flag.svg" alt="Flag Icon" class="w-6 h-6" style="display: inline-block;" />
            <span class="p3">
                <span class="font-bold">{"UPDATE: "}</span>
                <span>{"Open Device Partnership Expands Open-Source Efforts; "}</span>
                <a href="/announcements" class="underline hover:text-blue-600 transition-colors duration-150">
                    {"Patina Project to Launch at UEFI 2025 Developer Conference & Plugfest - read more!"}
                </a>
            </span>
        </div>
    }
}
