//! Navigation chrome: `NavBar`, `MobileDrawer`, `Footer`.

use crate::components::media::{BrandIcon, Logo};
use crate::components::theme::ThemeToggle;
use leptos::ev;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;
use unocss_classes::uno;

const NAV_LINKS: &[(&str, &str, bool)] = &[
    ("/getting-started", "Getting started", false),
    ("/projects", "Projects", false),
    (
        "https://opendevicepartnership.github.io/documentation/",
        "Library",
        true,
    ),
    ("/community", "Community", false),
    ("/announcements", "Announcements", false),
];

/// Sticky, slim nav bar with the logo, primary links, theme toggle,
/// and a "Get involved" CTA. Collapses to a hamburger + drawer on
/// narrow viewports.
#[component]
pub fn NavBar() -> impl IntoView {
    let drawer_open = RwSignal::new(false);
    let close_drawer = move || drawer_open.set(false);

    // ESC closes the drawer.
    window_event_listener(ev::keydown, move |e| {
        if e.key() == "Escape" && drawer_open.get_untracked() {
            drawer_open.set(false);
        }
    });

    view! {
        <header class=uno![
            "sticky top-0 z-40 w-full",
            "bg-surface-page/85 backdrop-blur",
            "border-b border-border-subtle"
        ]>
            <div class=uno![
                "max-w-[1536px] mx-auto px-section-x",
                "h-16 md:h-20 flex items-center justify-between gap-6"
            ]>
                <A
                    href="/"
                    attr:aria-label="Open Device Partnership home"
                    attr:class="flex-shrink-0"
                >
                    <Logo />
                </A>

                <nav class=uno!("hidden lg:flex items-center gap-1") aria-label="Primary">
                    {NAV_LINKS
                        .iter()
                        .copied()
                        .map(|(href, label, external)| {
                            view! { <DesktopLink href=href label=label external=external /> }
                        })
                        .collect_view()}
                </nav>

                <div class=uno!("flex items-center gap-2")>
                    <ThemeToggle />
                    <a
                        href="https://github.com/OpenDevicePartnership"
                        target="_blank"
                        rel="noopener noreferrer"
                        aria-label="GitHub organisation"
                        class=uno![
                            "inline-flex items-center justify-center w-10 h-10 rounded-md",
                            "text-ink-secondary hover:(text-ink-primary bg-surface-sunken)",
                            "transition-colors duration-200"
                        ]
                    >
                        <BrandIcon name="github" />
                    </a>
                    <button
                        type="button"
                        class=uno![
                            "lg:hidden inline-flex items-center justify-center w-10 h-10 rounded-md",
                            "text-ink-primary hover:bg-surface-sunken",
                            "transition-colors duration-200"
                        ]
                        aria-label=move || {
                            if drawer_open.get() { "Close menu" } else { "Open menu" }
                        }
                        aria-expanded=move || if drawer_open.get() { "true" } else { "false" }
                        aria-controls="primary-mobile-nav"
                        on:click=move |_| drawer_open.update(|v| *v = !*v)
                    >
                        <span
                            class=move || {
                                if drawer_open.get() {
                                    uno!("i-lucide-x w-6 h-6 block")
                                } else {
                                    uno!("i-lucide-menu w-6 h-6 block")
                                }
                            }
                            aria-hidden="true"
                        ></span>
                    </button>
                </div>
            </div>

            <MobileDrawer open=drawer_open close=Callback::new(move |_| close_drawer()) />
        </header>
    }
}

#[component]
fn DesktopLink(href: &'static str, label: &'static str, external: bool) -> impl IntoView {
    let location = use_location();
    let is_active = move || {
        if external {
            return false;
        }
        let path = location.pathname.get();
        if href == "/" {
            path == "/"
        } else {
            path == href || path.starts_with(&format!("{href}/"))
        }
    };

    let base = uno!(
        "px-3 py-2 rounded-md text-small font-medium",
        "transition-colors duration-200"
    );

    if external {
        view! {
            <a
                href=href
                target="_blank"
                rel="noopener noreferrer"
                class=format!(
                    "{base} text-ink-secondary hover:(text-ink-primary bg-surface-sunken)",
                )
            >
                {label}
            </a>
        }
        .into_any()
    } else {
        view! {
            <A
                href=href
                attr:class=move || {
                    if is_active() {
                        format!("{base} text-ink-primary bg-surface-sunken")
                    } else {
                        format!(
                            "{base} text-ink-secondary hover:(text-ink-primary bg-surface-sunken)",
                        )
                    }
                }
                attr:aria-current=move || if is_active() { Some("page") } else { None }
            >
                {label}
            </A>
        }
        .into_any()
    }
}

#[component]
fn MobileDrawer(open: RwSignal<bool>, close: Callback<()>) -> impl IntoView {
    view! {
        <div
            class=uno![
                "fixed inset-0 z-30 lg:hidden",
                "backdrop-blur-md",
                "transition-opacity duration-200"
            ]
            style:display=move || if open.get() { "block" } else { "none" }
            on:click=move |_| close.run(())
            aria-hidden="true"
        ></div>
        <nav
            id="primary-mobile-nav"
            aria-label="Primary"
            class=uno![
                "fixed top-16 right-0 w-[85vw] max-w-sm z-40 lg:hidden",
                "bg-surface-raised border-l border-b border-border-subtle",
                "rounded-bl-lg shadow-elev-3",
                "p-6 flex flex-col gap-1"
            ]
            style:display=move || if open.get() { "flex" } else { "none" }
        >
            {NAV_LINKS
                .iter()
                .copied()
                .map(|(href, label, external)| {
                    view! { <MobileLink href=href label=label external=external close=close /> }
                })
                .collect_view()}
        </nav>
    }
}

#[component]
fn MobileLink(href: &'static str, label: &'static str, external: bool, close: Callback<()>) -> impl IntoView {
    let class = uno!(
        "block w-full px-4 py-3 rounded-md text-body font-medium",
        "text-ink-primary hover:bg-surface-sunken",
        "transition-colors duration-200"
    );
    if external {
        view! {
            <a
                href=href
                target="_blank"
                rel="noopener noreferrer"
                class=class
                on:click=move |_| close.run(())
            >
                {label}
            </a>
        }
        .into_any()
    } else {
        view! {
            <A href=href attr:class=class on:click=move |_| close.run(())>
                {label}
            </A>
        }
        .into_any()
    }
}

/// Compact, editorial footer with brand mark, link clusters, social
/// links, and the "members of" partner trust strip.
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class=uno![
            "w-full mt-auto",
            "bg-surface-sunken border-t border-border-subtle",
            "text-ink-secondary"
        ]>
            <div class=uno!(
                "max-w-[1536px] mx-auto px-section-x py-12 md:py-16 flex flex-col gap-12"
            )>
                <div class=uno!("grid gap-10 md:grid-cols-[1.4fr_1fr_1fr_1fr] items-start")>
                    <div class=uno!("flex flex-col gap-4")>
                        <Logo />
                        <p class=uno!(
                            "text-small text-ink-muted max-w-[40ch]"
                        )>
                            "An open collaboration for secure, modern device firmware. Built in the open, by the people who maintain it."
                        </p>
                    </div>

                    <FooterColumn title="Project">
                        <FooterLink href="/projects">"All projects"</FooterLink>
                        <FooterLink href="/boot-firmware">"Patina"</FooterLink>
                        <FooterLink href="/embedded-controller">"Secure EC"</FooterLink>
                        <FooterLink href="/windows-ec-services">"EC Services"</FooterLink>
                    </FooterColumn>

                    <FooterColumn title="Community">
                        <FooterLink href="/community">"Governance"</FooterLink>
                        <FooterLink href="/getting-started">"Getting started"</FooterLink>
                        <FooterLink href="/announcements">"Announcements"</FooterLink>
                        <FooterLink
                            href="https://opendevicepartnership.github.io/documentation/"
                            external=true
                        >
                            "Library"
                        </FooterLink>
                    </FooterColumn>

                    <FooterColumn title="Connect">
                        <FooterLink href="https://github.com/OpenDevicePartnership" external=true>
                            "GitHub"
                        </FooterLink>
                        <FooterLink
                            href="https://opendevicepartnership.zulipchat.com"
                            external=true
                        >
                            "Zulip"
                        </FooterLink>
                        <FooterLink href="https://discord.gg/a8cEfTDQN4" external=true>
                            "Discord"
                        </FooterLink>
                        <FooterLink
                            href="https://www.youtube.com/@OpenDevicePartnership"
                            external=true
                        >
                            "YouTube"
                        </FooterLink>
                    </FooterColumn>
                </div>

                <crate::components::cards::TrustStrip />

                <div class=uno![
                    "pt-8 border-t border-border-subtle",
                    "flex flex-col md:flex-row items-start md:items-center justify-between gap-4",
                    "text-caption text-ink-muted"
                ]>
                    <p>"© 2025 Open Device Partnership"</p>
                    <div class=uno!("flex items-center gap-4")>
                        <a
                            href="https://github.com/OpenDevicePartnership"
                            target="_blank"
                            rel="noopener noreferrer"
                            aria-label="GitHub"
                            class=uno!("hover:text-ink-primary transition-colors")
                        >
                            <BrandIcon name="github" class="w-4 h-4" />
                        </a>
                        <a
                            href="https://opendevicepartnership.zulipchat.com"
                            target="_blank"
                            rel="noopener noreferrer"
                            aria-label="Zulip"
                            class=uno!("hover:text-ink-primary transition-colors")
                        >
                            <BrandIcon name="zulip" class="w-4 h-4" />
                        </a>
                        <a
                            href="https://discord.gg/a8cEfTDQN4"
                            target="_blank"
                            rel="noopener noreferrer"
                            aria-label="Discord"
                            class=uno!("hover:text-ink-primary transition-colors")
                        >
                            <BrandIcon name="discord" class="w-4 h-4" />
                        </a>
                        <a
                            href="https://www.youtube.com/@OpenDevicePartnership"
                            target="_blank"
                            rel="noopener noreferrer"
                            aria-label="YouTube"
                            class=uno!("hover:text-ink-primary transition-colors")
                        >
                            <BrandIcon name="youtube" class="w-4 h-4" />
                        </a>
                    </div>
                </div>
            </div>
        </footer>
    }
}

#[component]
fn FooterColumn(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class=uno!("flex flex-col gap-3")>
            <h3 class=uno!(
                "text-caption font-mono uppercase tracking-wider text-ink-muted"
            )>{title}</h3>
            <ul class=uno!("flex flex-col gap-2")>{children()}</ul>
        </div>
    }
}

#[component]
fn FooterLink(
    #[prop(into)] href: String,
    #[prop(optional, default = false)] external: bool,
    children: Children,
) -> impl IntoView {
    let class = uno!("text-small text-ink-secondary hover:text-ink-primary transition-colors");
    if external {
        view! {
            <li>
                <a href=href target="_blank" rel="noopener noreferrer" class=class>
                    {children()}
                </a>
            </li>
        }
        .into_any()
    } else {
        view! {
            <li>
                <A href=href attr:class=class>
                    {children()}
                </A>
            </li>
        }
        .into_any()
    }
}
