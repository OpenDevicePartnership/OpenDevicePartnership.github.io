use leptos::prelude::RwSignal;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Header() -> impl IntoView {
    let menu_open = RwSignal::new(false);
    view! {
    <header class="w-full h-[80px] md:h-[160px] px-2 md:px-[120px] background_primary flex items-center justify-between z-50 m-0 p-0 relative">
            <div class="flex items-center space-x-6">
                <picture>
                    <source srcset="/images/dark/odplogo.svg" media="(prefers-color-scheme: dark)" />
                    <img
                        src="/images/light/odplogo.svg"
                        alt="ODP Logo"
                        class="w-[100px] h-[34.5px] md:w-[149px] md:h-[51.43px] object-contain"
                    />
                </picture>
            </div>

            {/* Hamburger icon for mobile */}
            <button
                class="md:hidden flex flex-col justify-center items-center w-10 h-10 p-2 focus:outline-none"
                aria-label="Open menu"
                on:click=move |_| menu_open.update(|v| *v = !*v)
            >
                <span class="block w-6 h-0.5 bg-black dark:bg-white mb-1"></span>
                <span class="block w-6 h-0.5 bg-black dark:bg-white mb-1"></span>
                <span class="block w-6 h-0.5 bg-black dark:bg-white"></span>
            </button>

            {/* Navigation */}
            <nav
                class="hidden md:flex [column-gap:25px]"
            >
                <NavButton href="/getting-started" label="Getting Started"/>
                <NavButton href="/projects" label="Projects"/>
                <ExternalNavButton href="https://opendevicepartnership.github.io/documentation/" label="Library"/>
                <NavButton href="/community" label="Community"/>
                <NavButton href="/home" label="Home"/>
            </nav>

            {/* Mobile nav menu, expands below header, right-aligned */}
            <nav
                class="absolute right-0 top-full w-[80vw] max-w-xs background_primary flex-col items-end px-4 py-4 space-y-2 shadow-lg md:hidden transition-all duration-200"
                style=move || if menu_open.get() { "display: flex;" } else { "display: none;" }
            >
                <NavButton href="/getting-started" label="Getting Started" mobile=true/>
                <NavButton href="/projects" label="Projects" mobile=true/>
                <ExternalNavButton href="https://opendevicepartnership.github.io/documentation/" label="Library" mobile=true/>
                <NavButton href="/community" label="Community" mobile=true/>
                <NavButton href="/home" label="Home" mobile=true/>
            </nav>
        </header>
    }
}

#[component]
fn NavButton(href: &'static str, label: &'static str, #[prop(optional)] mobile: bool) -> impl IntoView {
    let location = leptos_router::hooks::use_location();
    let is_active = move || location.pathname.get().starts_with(href);

    view! {
        <A
            href=href
            class:odp-header-btn=true
            class:odp-header-btn-text=true
            class:odp-header-btn-active=is_active
            class:odp-header-btn-active-text=is_active
            class:w-full=mobile
        >
            {label}
        </A>
    }
}

#[component]
fn ExternalNavButton(href: &'static str, label: &'static str, #[prop(optional)] mobile: bool) -> impl IntoView {
    view! {
        <a
            href=href
            class=format!("odp-header-btn odp-header-btn-text{}", if mobile { " w-full" } else { "" })
            target="_blank"
        >
            {label}
        </a>
    }
}
