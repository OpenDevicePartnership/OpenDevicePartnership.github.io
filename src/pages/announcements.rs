//! Announcements pages.
//!
//! - [`AnnouncementsPage`] (`/announcements`): blog-style index of
//!   announcement cards in reverse-chronological order.
//! - [`AnnouncementDetailPage`] (`/announcements/<slug>`): the
//!   permalink view for a single announcement. The prose body is
//!   co-located here as a per-slug match arm so adding a new
//!   announcement is one struct literal in `data::announcements`
//!   plus one match arm here.
//!
//! Legacy `/announcements?id=<slug>` query-string permalinks are
//! transparently redirected to `/announcements/<slug>` on mount.

use crate::components::cards::{AnnouncementCard, AnnouncementDetail};
use crate::components::controls::{ArrowLink, InlineLink};
use crate::components::layout::{Container, ContainerWidth, Section, Stack, StackGap};
use crate::components::typography::{Body, BodyTone, Display, DisplaySize, Eyebrow};
use crate::data::announcements::{find, ANNOUNCEMENTS};
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_location, use_navigate, use_params_map};
use unocss_classes::uno;

#[component]
pub fn AnnouncementsPage() -> impl IntoView {
    // Redirect legacy `?id=<slug>` permalinks to the new path-based
    // form. Cheap to keep — old links in chat history, bookmarks,
    // etc. still land in the right place.
    let location = use_location();
    let navigate = use_navigate();
    Effect::new(move |_| {
        let search = location.search.get();
        if let Some(slug) = slug_from_query(&search) {
            if find(slug).is_some() {
                navigate(&format!("/announcements/{slug}"), Default::default());
            }
        }
    });

    view! {
        <Section class="pt-16 md:pt-24">
            <Container width=ContainerWidth::Narrow>
                <Stack gap=StackGap::Md>
                    <Eyebrow>"Announcements"</Eyebrow>
                    <Display size=DisplaySize::Lg>"News from the partnership."</Display>
                </Stack>
            </Container>
        </Section>

        <Section>
            <Container width=ContainerWidth::Narrow>
                {if ANNOUNCEMENTS.is_empty() {
                    view! {
                        <Body tone=BodyTone::Secondary>
                            "No announcements yet — check back soon."
                        </Body>
                    }
                        .into_any()
                } else {
                    view! {
                        <Stack gap=StackGap::Lg>
                            {ANNOUNCEMENTS
                                .iter()
                                .map(|a| view! { <AnnouncementCard announcement=a /> })
                                .collect_view()}
                        </Stack>
                    }
                        .into_any()
                }}
            </Container>
        </Section>
    }
}

/// Permalink view for a single announcement.
#[component]
pub fn AnnouncementDetailPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();

    view! {
        <Section class="pt-16 md:pt-24">
            <Container width=ContainerWidth::Narrow>
                {move || {
                    let slug = slug();
                    match find(&slug) {
                        Some(a) => {
                            view! {
                                <Stack gap=StackGap::Lg>
                                    <A
                                        href="/announcements"
                                        attr:class=uno![
                                            "group inline-flex items-center gap-2 text-small font-medium",
                                            "text-ink-muted hover:text-ink-accent transition-colors duration-200"
                                        ]
                                    >
                                        <span
                                            class=uno![
                                                "i-lucide-arrow-right w-4 h-4 rotate-180",
                                                "group-hover:-translate-x-0.5 transition-transform duration-200"
                                            ]
                                            aria-hidden="true"
                                        ></span>
                                        "All announcements"
                                    </A>
                                    <AnnouncementDetail announcement=a>
                                        {render_content(a.slug)}
                                    </AnnouncementDetail>
                                </Stack>
                            }
                                .into_any()
                        }
                        None => {
                            view! {
                                <Stack gap=StackGap::Md>
                                    <Eyebrow>"Not found"</Eyebrow>
                                    <Display size=DisplaySize::Md>
                                        "That announcement isn't here."
                                    </Display>
                                    <Body tone=BodyTone::Secondary>
                                        "The link may be out of date — head back to the index for the latest news."
                                    </Body>
                                    <ArrowLink href="/announcements"
                                        .to_string()>"View all announcements"</ArrowLink>
                                </Stack>
                            }
                                .into_any()
                        }
                    }
                }}
            </Container>
        </Section>
    }
}

fn slug_from_query(search: &str) -> Option<&str> {
    // Strip an optional leading '?'.
    let s = search.strip_prefix('?').unwrap_or(search);
    let id_start = s.find("id=")?;
    let rest = &s[id_start + 3..];
    let slug = match rest.find('&') {
        Some(end) => &rest[..end],
        None => rest,
    };
    if slug.is_empty() {
        None
    } else {
        Some(slug)
    }
}

fn render_content(slug: &str) -> AnyView {
    match slug {
        "welcome-patina-announcement" => patina_press_release().into_any(),
        _ => view! { <p>"Content not found."</p> }.into_any(),
    }
}

fn patina_press_release() -> impl IntoView {
    view! {
        <div>
            <p>
                <strong>"October 7, 2025 - Redmond, WA"</strong>
                " - The "
                <strong>"Open Device Partnership (ODP)"</strong>
                " is announcing "
                <strong>"Patina"</strong>
                ", a new open-source firmware project, with details shared at the "
                <InlineLink
                    href="https://uefi.org/events/uefi-2025-developers-conference-and-plugfest"
                        .to_string()
                    external=true
                >
                    "UEFI 2025 Developer Conference & Plugfest"
                </InlineLink>
                ", October 7-10 in Sunnyvale, California. Patina is a Rust-based, UEFI-compatible firmware designed for memory safety and to address long-standing challenges in the PC firmware ecosystem. Patina joins a growing portfolio of ODP projects aimed at building a secure, modern foundation for device enablement. Learn more at "
                <InlineLink
                    href="https://opendevicepartnership.github.io/patina".to_string()
                    external=true
                >
                    "Patina Documentation"
                </InlineLink>
                "."
            </p>
            <p>
                "ODP is an industry-wide, open-source initiative focused on advancing "
                <strong>"security, fundamentals, and standardisation"</strong>
                " in device software. The partnership leverages "
                <strong>"memory-safe programming languages like Rust"</strong> " and "
                <strong>"hardware-rooted security features"</strong>
                ", grounded in standards that work across an entire device portfolio."
            </p>
            <p>"In addition to Patina, ODP is currently focused on three other major projects:"</p>
            <ul>
                <li>
                    <strong>"Secure EC firmware"</strong>
                    " - a modern, security-focused embedded controller implementation designed to eliminate classes of bugs prevalent in legacy EC codebases."
                </li>
                <li>
                    <strong>"Unified OS-EC service interface"</strong>
                    " - so operating systems can interact with embedded controllers in a consistent, well-defined way across devices."
                </li>
                <li>
                    <strong>"MPTF"</strong>
                    " - extending recent advancements in the Windows power-thermal framework to meet partner needs."
                </li>
            </ul>
            <p>
                "Together these efforts give hardware makers the ability to standardise firmware and device software across their entire portfolios - improving reliability, accelerating time-to-market, and reducing redundant engineering work."
            </p>
            <p>
                "Learn more and get involved at "
                <InlineLink href="https://opendevicepartnership.org/".to_string() external=true>
                    "opendevicepartnership.org"
                </InlineLink> "."
            </p>
        </div>
    }
}
