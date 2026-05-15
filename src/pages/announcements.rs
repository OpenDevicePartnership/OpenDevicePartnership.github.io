//! Announcements page (`/announcements`).
//!
//! Sidebar list + detail panel. Detail content is co-located here
//! (one match arm per slug) so adding an announcement is a single
//! struct literal in `data::announcements` plus a match arm.

use crate::components::cards::AnnouncementDetail;
use crate::components::controls::InlineLink;
use crate::components::layout::{Container, Section, Stack, StackGap, Surface, SurfaceElevation, SurfaceTone};
use crate::components::typography::{Display, DisplaySize, Eyebrow};
use crate::data::announcements::{index_of, ANNOUNCEMENTS};
use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};
use unocss_classes::uno;

#[component]
pub fn AnnouncementsPage() -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();

    let (selected, set_selected) = signal(0_usize);

    {
        let location = location.clone();
        Effect::new(move |_| {
            let search = location.search.get();
            if let Some(slug) = slug_from_query(&search) {
                if let Some(idx) = index_of(slug) {
                    set_selected.set(idx);
                }
            }
        });
    }

    view! {
        <Section class="pt-16 md:pt-24">
            <Container>
                <Stack gap=StackGap::Md>
                    <Eyebrow>"Announcements"</Eyebrow>
                    <Display size=DisplaySize::Lg>"News from the partnership."</Display>
                </Stack>
            </Container>
        </Section>

        <Section>
            <Container>
                <div class=uno!("grid gap-8 md:gap-12 md:grid-cols-[280px_1fr] items-start")>
                    <nav aria-label="Announcements" class=uno!("flex flex-col gap-2")>
                        {ANNOUNCEMENTS
                            .iter()
                            .enumerate()
                            .map(|(i, a)| {
                                let navigate = navigate.clone();
                                let slug = a.slug;
                                let label = a.link_label;
                                let active = move || selected.get() == i;
                                view! {
                                    <button
                                        type="button"
                                        class=move || {
                                            let base = uno!(
                                                "w-full text-left px-4 py-3 rounded-md text-small font-medium transition-colors duration-200"
                                            );
                                            if active() {
                                                format!("{base} bg-accent-soft text-ink-accent")
                                            } else {
                                                format!(
                                                    "{base} text-ink-secondary hover:(text-ink-primary bg-surface-sunken)",
                                                )
                                            }
                                        }
                                        on:click={
                                            let navigate = navigate.clone();
                                            move |_| {
                                                set_selected.set(i);
                                                navigate(
                                                    &format!("/announcements?id={slug}"),
                                                    Default::default(),
                                                );
                                            }
                                        }
                                    >
                                        {label}
                                    </button>
                                }
                            })
                            .collect_view()}
                    </nav>

                    <Surface
                        tone=SurfaceTone::Raised
                        elevation=SurfaceElevation::E1
                        class="!p-8 md:!p-12"
                    >
                        {move || {
                            let idx = selected.get();
                            if let Some(a) = ANNOUNCEMENTS.get(idx) {
                                view! {
                                    <AnnouncementDetail
                                        eyebrow="Press release".to_string()
                                        title=a.title.to_string()
                                    >
                                        {render_content(a.slug)}
                                    </AnnouncementDetail>
                                }
                                    .into_any()
                            } else {
                                view! { <p>"Select an announcement."</p> }.into_any()
                            }
                        }}
                    </Surface>
                </div>
            </Container>
        </Section>
    }
}

fn slug_from_query(search: &str) -> Option<&str> {
    let id_start = search.find("id=")?;
    let rest = &search[id_start + 3..];
    Some(match rest.find('&') {
        Some(end) => &rest[..end],
        None => rest,
    })
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
