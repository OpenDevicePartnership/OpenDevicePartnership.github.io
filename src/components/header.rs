use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="w-full h-[160px] px-[120px] bg-white dark:bg-black flex items-center justify-between">
            <div class="flex items-center space-x-6">
                <picture>
                    <source srcset="/images/dark/odplogo.svg" media="(prefers-color-scheme: dark)" />
                    <img
                        src="/images/light/odplogo.svg"
                        alt="ODP Logo"
                        class="w-[149px] h-[51.43px] object-contain"
                    />
                </picture>
            </div>

            <nav class="flex [column-gap:25px]">
                <NavButton href="/getting-started" label="Getting Started"/>
                <NavButton href="/projects" label="Projects"/>
                <ExternalNavButton href="https://opendevicepartnership.github.io/documentation/" label="Library"/>
                <NavButton href="/community" label="Community"/>
                <NavButton href="/home" label="Home"/>
            </nav>
        </header>
    }
}

#[component]
fn NavButton(href: &'static str, label: &'static str) -> impl IntoView {
    let location = leptos_router::hooks::use_location();
    let is_active = move || location.pathname.get().starts_with(href);

    view! {
        <A
            href=href
            class:odp-header-btn=true
            class:odp-header-btn-text=true
            class:odp-header-btn-active=is_active
            class:odp-header-btn-active-text=is_active
        >
            {label}
        </A>
    }
}

#[component]
fn ExternalNavButton(href: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <a
            href=href
            class="odp-header-btn odp-header-btn-text"
            target="_blank"
        >
            {label}
        </a>
    }
}
