//! Shared layout for the three project pages.
//!
//! Every project page renders an editorial hero (eyebrow + title +
//! lead + CTA cluster), an "at-a-glance facts" rail, the project's
//! repository graph, and a tab-based switcher to the other
//! projects. Per-page differences come from `data::projects`.

use crate::components::cards::ProjectCard;
use crate::components::controls::{ArrowLink, LinkButton, Tag, TagTone};
use crate::components::layout::{Cluster, Container, Grid, GridMin, Section, SectionSurface, Stack, StackGap};
use crate::components::repo_view::RepositoryGraph;
use crate::components::typography::{Body, BodyTone, Display, DisplaySize, Eyebrow, Heading, HeadingLevel};
use crate::data::projects::{ProjectCopy, EC_SERVICES, EMBEDDED_CONTROLLER, PATINA};
use leptos::prelude::*;
use unocss_classes::uno;

/// Renders the entire project page given the canonical `ProjectCopy`
/// row from `data::projects`.
#[component]
pub fn ProjectLayout(project: &'static ProjectCopy, tone: TagTone) -> impl IntoView {
    view! {
        <Section>
            <Container>
                <Stack gap=StackGap::Lg>
                    <Eyebrow>"Project"</Eyebrow>
                    <Display size=DisplaySize::Lg>{project.title}</Display>
                    <p class=uno!("text-lead text-ink-secondary max-w-[60ch]")>{project.summary}</p>
                    <Cluster gap=StackGap::Sm>
                        <Tag tone=tone>{project.short_label}</Tag>
                        <Tag tone=TagTone::Trust>"Open source"</Tag>
                        <Tag tone=TagTone::Neutral>"Rust-first"</Tag>
                    </Cluster>
                    <Cluster gap=StackGap::Sm>
                        <LinkButton href=project
                            .team_route
                            .to_string()>
                            "Meet the team"
                            <span class="i-lucide-arrow-right w-4 h-4" aria-hidden="true"></span>
                        </LinkButton>
                        <ArrowLink href="https://github.com/OpenDevicePartnership" external=true>
                            "Browse repositories"
                        </ArrowLink>
                    </Cluster>
                </Stack>
            </Container>
        </Section>

        <Section surface=SectionSurface::Sunken>
            <Container>
                <Grid min=GridMin::Md gap=StackGap::Lg>
                    <ProjectFact eyebrow="What" inner_html=project.what />
                    <ProjectFact eyebrow="Why" body=project.why.to_string() />
                </Grid>
            </Container>
        </Section>

        <Section>
            <Container>
                <Stack gap=StackGap::Lg>
                    <Eyebrow>"Repository graph"</Eyebrow>
                    <Heading level=HeadingLevel::H2>"Inside the codebase"</Heading>
                    <Body tone=BodyTone::Secondary class="max-w-[60ch]">
                        "An interactive map of the crates and components that make up this project. Drag, zoom, and click to inspect."
                    </Body>
                </Stack>
            </Container>
            <div class=uno!("mt-10 w-full overflow-x-auto")>
                <div class=uno!("max-w-[1536px] mx-auto px-section-x")>
                    <RepositoryGraph nodes=project.nodes_json links=project.links_json />
                </div>
            </div>
        </Section>

        <Section surface=SectionSurface::Sunken>
            <Container>
                <Stack gap=StackGap::Lg>
                    <Eyebrow>"Other projects"</Eyebrow>
                    <Heading level=HeadingLevel::H2>"Continue exploring"</Heading>
                    <Grid min=GridMin::Md gap=StackGap::Lg>
                        {[
                            (&PATINA, TagTone::Patina),
                            (&EMBEDDED_CONTROLLER, TagTone::Ec),
                            (&EC_SERVICES, TagTone::Services),
                        ]
                            .into_iter()
                            .filter(|(p, _)| p.route != project.route)
                            .map(|(p, t)| view! { <ProjectCard project=p tone=t /> })
                            .collect_view()}
                    </Grid>
                </Stack>
            </Container>
        </Section>
    }
}

/// "What" / "Why" fact panel. The "What" copy from `data::projects`
/// is HTML (it embeds inline anchors), so we render it via
/// `inner_html`. "Why" is plain text, rendered via `body`.
#[component]
fn ProjectFact(
    eyebrow: &'static str,
    #[prop(into, optional)] inner_html: &'static str,
    #[prop(into, optional)] body: String,
) -> impl IntoView {
    view! {
        <div class=uno!("flex flex-col gap-4")>
            <Eyebrow>{eyebrow}</Eyebrow>
            {if !inner_html.is_empty() {
                view! {
                    <div
                        class=uno![
                            "text-body text-ink-primary leading-relaxed max-w-[60ch]",
                            "[&_a]:underline [&_a]:underline-offset-4 [&_a]:text-ink-accent",
                            "whitespace-pre-line"
                        ]
                        inner_html=inner_html
                    ></div>
                }
                    .into_any()
            } else {
                view! {
                    <p class=uno!(
                        "text-body text-ink-primary leading-relaxed max-w-[60ch] whitespace-pre-line"
                    )>{body}</p>
                }
                    .into_any()
            }}
        </div>
    }
}
