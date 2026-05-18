//! Content cards used across pages.

use crate::components::controls::{Tag, TagTone};
use crate::components::layout::{Cluster, Stack, StackGap, Surface, SurfaceElevation, SurfaceTone};
use crate::components::media::Icon;
use crate::components::typography::{Body, BodyTone, Eyebrow, Heading, HeadingLevel};
use crate::data::partners::PARTNERS;
use crate::data::projects::ProjectCopy;
use crate::data::teams::TeamMember;
use leptos::prelude::*;
use leptos_router::components::A;
use unocss_classes::uno;

/// A project card used on `/` and `/projects`.
///
/// The square media slot keeps every project icon visually
/// consistent regardless of the source image's intrinsic ratio.
#[component]
pub fn ProjectCard(project: &'static ProjectCopy, tone: TagTone) -> impl IntoView {
    let monogram_class = match tone {
        TagTone::Patina => "bg-[var(--color-project-patina)] text-[var(--color-project-patina-ink)]",
        TagTone::Ec => "bg-[var(--color-project-ec)] text-[var(--color-project-ec-ink)]",
        TagTone::Services => "bg-[var(--color-project-services)] text-[var(--color-project-services-ink)]",
        _ => "bg-surface-sunken text-ink-primary",
    };
    view! {
        <A href=project.route attr:class="group block">
            <Surface
                tone=SurfaceTone::Raised
                elevation=SurfaceElevation::E1
                class="h-full flex flex-col gap-5 group-hover:(shadow-elev-3 -translate-y-0.5) transition-all duration-300"
            >
                <div class=format!(
                    "{} {monogram_class}",
                    uno!(
                        "aspect-square w-full overflow-hidden rounded-md",
                        "flex items-center justify-center select-none"
                    ),
                )>
                    <span
                        class=uno!(
                            "font-display font-medium leading-none tracking-tight text-[clamp(3rem,7vw,5rem)]"
                        )
                        aria-hidden="true"
                    >
                        {project.monogram}
                    </span>
                </div>
                <div class=uno!("flex flex-col gap-3")>
                    <Tag tone=tone>{project.short_label}</Tag>
                    <Heading level=HeadingLevel::H3>{project.title}</Heading>
                    <Body tone=BodyTone::Secondary>{project.summary}</Body>
                </div>
                <span class=uno![
                    "mt-auto inline-flex items-center gap-2 text-small font-medium text-ink-accent",
                    "group-hover:underline underline-offset-4"
                ]>
                    "Explore project"
                    <span class="i-lucide-arrow-right w-4 h-4" aria-hidden="true"></span>
                </span>
            </Surface>
        </A>
    }
}

/// A condensed value-proposition card (icon + title + body).
#[component]
pub fn ValueCard(#[prop(into)] icon: String, #[prop(into)] title: String, #[prop(into)] body: String) -> impl IntoView {
    view! {
        <Surface
            tone=SurfaceTone::Raised
            elevation=SurfaceElevation::E1
            class="h-full flex flex-col gap-4"
        >
            <span class=uno![
                "inline-flex items-center justify-center w-12 h-12 rounded-md",
                "bg-accent-soft text-ink-accent"
            ]>
                <Icon name=icon class="w-6 h-6".to_string() />
            </span>
            <Heading level=HeadingLevel::H3>{title}</Heading>
            <Body tone=BodyTone::Secondary>{body}</Body>
        </Surface>
    }
}

/// A small team-member card with avatar, name, role, and GitHub
/// handle.
#[component]
pub fn TeamCard(member: TeamMember) -> impl IntoView {
    let full_name = format!("{} {}", member.first_name, member.last_name);
    view! {
        <Surface
            tone=SurfaceTone::Raised
            elevation=SurfaceElevation::Flat
            class="flex flex-col items-start gap-3"
        >
            <img
                src=member.image_url
                alt=full_name.clone()
                loading="lazy"
                class=uno![
                    "w-20 h-20 rounded-full object-cover bg-surface-sunken",
                    "border border-border-subtle"
                ]
            />
            <div class=uno!("flex flex-col gap-1")>
                <p class=uno!("text-body font-semibold text-ink-primary")>{full_name}</p>
                <p class=uno!("text-small text-ink-secondary")>{member.role}</p>
            </div>
            <a
                href=member.github_url
                target="_blank"
                rel="noopener noreferrer"
                class=uno![
                    "inline-flex items-center gap-1 text-small text-ink-muted",
                    "hover:text-ink-accent transition-colors"
                ]
            >
                <span class="i-lucide-github w-4 h-4" aria-hidden="true"></span>
                <span class="font-mono">{member.github_username}</span>
            </a>
        </Surface>
    }
}

/// Small grouping card for a documentation/training link.
#[component]
pub fn DocCard(
    #[prop(into)] href: String,
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(optional, default = false)] external: bool,
) -> impl IntoView {
    view! {
        <a
            href=href
            target=if external { Some("_blank") } else { None }
            rel=if external { Some("noopener noreferrer") } else { None }
            class="group block"
        >
            <Surface
                tone=SurfaceTone::Outline
                elevation=SurfaceElevation::Flat
                class="h-full flex flex-col gap-3 group-hover:(border-border-accent bg-surface-raised) transition-colors duration-200"
            >
                <div class=uno!("flex items-start justify-between gap-3")>
                    <Heading level=HeadingLevel::H3 class="!text-h3">
                        {title}
                    </Heading>
                    <span
                        class=uno![
                            "i-lucide-arrow-up-right w-5 h-5 text-ink-muted flex-shrink-0",
                        "group-hover:(text-accent translate-x-0.5 translate-y--0.5) transition-transform duration-200"
                        ]
                        aria-hidden="true"
                    ></span>
                </div>
                <Body tone=BodyTone::Secondary class="!text-small">
                    {description}
                </Body>
            </Surface>
        </a>
    }
}

/// Slim trust strip rendered in the footer.
#[component]
pub fn TrustStrip() -> impl IntoView {
    view! {
        <div class=uno!("flex flex-col gap-4")>
            <p class=uno!(
                "text-caption font-mono uppercase tracking-wider text-ink-muted"
            )>"Built with our partners"</p>
            <Cluster
                gap=StackGap::Lg
                class="opacity-70 hover:opacity-100 transition-opacity duration-300"
            >
                {PARTNERS
                    .iter()
                    .map(|p| {
                        view! {
                            <a
                                href=p.url
                                target="_blank"
                                rel="noopener noreferrer"
                                aria-label=p.name
                                class=uno![
                                    "inline-flex items-center justify-center h-8 md:h-10",
                                    "grayscale hover:grayscale-0 transition-[filter] duration-300"
                                ]
                            >
                                <img
                                    src=p.logo
                                    alt=p.name
                                    loading="lazy"
                                    class="h-full w-auto max-w-[160px] object-contain"
                                />
                            </a>
                        }
                    })
                    .collect_view()}
            </Cluster>
        </div>
    }
}

/// Announcement detail card. Used on `/announcements/<slug>`.
#[component]
pub fn AnnouncementDetail(
    announcement: &'static crate::data::announcements::Announcement,
    children: Children,
) -> impl IntoView {
    let date_long = announcement.published_at.long();
    let date_iso = announcement.published_at.iso();
    let kind_label = announcement.kind.label();
    let location = announcement.location;
    view! {
        <article>
            <Stack gap=StackGap::Md>
                <Eyebrow>{kind_label}</Eyebrow>
                <Heading level=HeadingLevel::H1>{announcement.title}</Heading>
                <p class=uno!("text-small text-ink-muted font-mono")>
                    <time datetime=date_iso>{date_long}</time>
                    {location
                        .map(|loc| {
                            view! {
                                " · "
                                {loc}
                            }
                        })}
                </p>
                <div class=uno![
                    "max-w-[68ch] text-body text-ink-primary",
                    "leading-relaxed [&>p]:mb-5 [&_a]:underline [&_a]:underline-offset-4 [&_a]:text-ink-accent",
                    "[&_strong]:font-semibold [&_ul]:list-disc [&_ul]:pl-6 [&_ul]:mb-5 [&_li]:mb-2"
                ]>{children()}</div>
            </Stack>
        </article>
    }
}

/// A blog-list card for the announcements index.
///
/// Renders the eyebrow (kind + date + optional location), the title,
/// an excerpt, and a "Read more →" affordance. The entire card is a
/// single anchor to the permalink so the click target is generous.
#[component]
pub fn AnnouncementCard(announcement: &'static crate::data::announcements::Announcement) -> impl IntoView {
    let href = format!("/announcements/{}", announcement.slug);
    let date_long = announcement.published_at.long();
    let date_iso = announcement.published_at.iso();
    let kind_label = announcement.kind.label();
    let location = announcement.location;
    view! {
        <A href=href attr:class="group block">
            <Surface
                tone=SurfaceTone::Raised
                elevation=SurfaceElevation::E1
                class="h-full flex flex-col gap-4 group-hover:(shadow-elev-3 -translate-y-0.5) transition-all duration-300"
            >
                <div class=uno!(
                    "flex flex-wrap items-baseline gap-x-3 gap-y-1 text-caption font-mono uppercase tracking-wider"
                )>
                    <span class=uno!("text-ink-accent")>{kind_label}</span>
                    <span class=uno!("text-ink-muted")>"·"</span>
                    <time class=uno!("text-ink-muted") datetime=date_iso>
                        {date_long}
                    </time>
                    {location
                        .map(|loc| {
                            view! {
                                <span class=uno!("text-ink-muted")>"·"</span>
                                <span class=uno!("text-ink-muted")>{loc}</span>
                            }
                        })}
                </div>
                <Heading level=HeadingLevel::H2 class="!text-h3">
                    {announcement.title}
                </Heading>
                <Body tone=BodyTone::Secondary>{announcement.excerpt}</Body>
                <span class=uno![
                    "mt-auto inline-flex items-center gap-2 text-small font-medium text-ink-accent",
                    "group-hover:underline underline-offset-4"
                ]>
                    "Read more"
                    <span class="i-lucide-arrow-right w-4 h-4" aria-hidden="true"></span>
                </span>
            </Surface>
        </A>
    }
}
