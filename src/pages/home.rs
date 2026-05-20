//! Home page (`/`).
//!
//! Editorial layout: oversized hero, three-pillar value props, the
//! project trio, a "what we believe" trust block, and a final CTA.
//! No partners section in the body -- the slim trust strip lives in
//! the footer.

use crate::components::cards::{ProjectCard, ValueCard};
use crate::components::controls::{ArrowLink, LinkButton, TagTone};
use crate::components::layout::{
    Cluster, Container, ContainerWidth, Grid, GridMin, Section, SectionSurface, Stack, StackGap,
};
use crate::components::media::VideoFacade;
use crate::components::typography::{Body, BodyTone, Display, DisplaySize, Eyebrow, Heading, HeadingLevel};
use crate::data::projects::{EC_SERVICES, EMBEDDED_CONTROLLER, PATINA};
use leptos::prelude::*;
use unocss_classes::uno;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <HomeHero />
        <ValueProposition />
        <ProjectTrio />
        <TrustBlock />
        <CallToAction />
    }
}

#[component]
fn HomeHero() -> impl IntoView {
    view! {
        <Section class="pt-16 md:pt-24">
            <Container>
                <div class=uno!("grid gap-10 md:gap-16 md:grid-cols-[1fr_1.1fr] items-center")>
                    <Stack gap=StackGap::Lg>
                        <Eyebrow>"Open Device Partnership"</Eyebrow>
                        <Display size=DisplaySize::Xl>
                            "An open collaboration for secure, modern devices."
                        </Display>
                        <p class=uno!(
                            "text-lead text-ink-secondary max-w-[50ch]"
                        )>
                            "A global initiative making it easier to ship secure, efficient, and reliable client devices across silicon, OS, and platform boundaries."
                        </p>
                        <Cluster gap=StackGap::Sm>
                            <LinkButton href="/getting-started"
                                .to_string()>
                                "Get started"
                                <span
                                    class="i-lucide-arrow-right w-4 h-4"
                                    aria-hidden="true"
                                ></span>
                            </LinkButton>
                            <ArrowLink href="/projects".to_string()>"See the projects"</ArrowLink>
                        </Cluster>
                    </Stack>
                    <VideoFacade
                        youtube_id="FMlPxYSY1LM"
                        title="Open Device Partnership — introduction"
                    />
                </div>
            </Container>
        </Section>
    }
}

#[component]
fn ValueProposition() -> impl IntoView {
    view! {
        <Section surface=SectionSurface::Sunken>
            <Container>
                <Stack gap=StackGap::Xl>
                    <div class=uno!("grid gap-6 md:gap-12 md:grid-cols-[auto_1fr] items-baseline")>
                        <Eyebrow>"Why ODP"</Eyebrow>
                        <Heading level=HeadingLevel::H2 class="max-w-[34ch]">
                            "Three principles. One foundation for the next decade of device firmware."
                        </Heading>
                    </div>
                    <Grid min=GridMin::Md gap=StackGap::Lg>
                        <ValueCard
                            icon="shield-check"
                            title="Security at the root"
                            body="Threats keep evolving. ODP cuts attack surface, leans on hardware-rooted features, and uses memory-safe Rust so whole classes of bugs are gone before they ship."
                        />
                        <ValueCard
                            icon="layers"
                            title="Standardised plumbing"
                            body="Most firmware is invisible plumbing - necessary, expensive, duplicated. Standards-based components maximise reuse across devices, architectures, and product generations."
                        />
                        <ValueCard
                            icon="zap"
                            title="Faster, together"
                            body="Open collaboration means shared solutions, fewer reinvented wheels, and faster paths from prototype to ship-quality firmware."
                        />
                    </Grid>
                </Stack>
            </Container>
        </Section>
    }
}

#[component]
fn ProjectTrio() -> impl IntoView {
    view! {
        <Section>
            <Container>
                <Stack gap=StackGap::Xl>
                    <div class=uno!("grid gap-6 md:gap-12 md:grid-cols-[auto_1fr] items-baseline")>
                        <Eyebrow>"Projects"</Eyebrow>
                        <Heading level=HeadingLevel::H2 class="max-w-[34ch]">
                            "Three projects, one open foundation."
                        </Heading>
                    </div>
                    <Body tone=BodyTone::Secondary class="max-w-[60ch]">
                        "Boot firmware, embedded controller software, and a unified EC service layer - shaped by partners across the industry, built in the open."
                    </Body>
                    <Grid min=GridMin::Md gap=StackGap::Lg>
                        <ProjectCard project=&PATINA tone=TagTone::Patina />
                        <ProjectCard project=&EMBEDDED_CONTROLLER tone=TagTone::Ec />
                        <ProjectCard project=&EC_SERVICES tone=TagTone::Services />
                    </Grid>
                </Stack>
            </Container>
        </Section>
    }
}

#[component]
fn TrustBlock() -> impl IntoView {
    view! {
        <Section surface=SectionSurface::Inverse class="py-section-y">
            <Container>
                <Stack gap=StackGap::Lg>
                    <span class=uno!(
                        "text-caption font-mono uppercase tracking-[0.18em] text-ink-inverse/70"
                    )>"What we believe"</span>
                    <p class=uno!(
                        "text-display font-display text-ink-inverse max-w-[24ch]"
                    )>"Trust is engineered, not declared."</p>
                    <div class=uno!("grid gap-6 md:grid-cols-3 max-w-[80ch] mt-4")>
                        <Pillar
                            title="Memory-safe by default"
                            body="Rust everywhere it matters - eliminating whole classes of CVEs from the firmware stack."
                        />
                        <Pillar
                            title="Hardware-rooted"
                            body="We lean on modern silicon security primitives instead of bolting protection on after the fact."
                        />
                        <Pillar
                            title="Standards-first"
                            body="Open specifications keep the ecosystem honest and reusable across an entire device portfolio."
                        />
                    </div>
                </Stack>
            </Container>
        </Section>
    }
}

#[component]
fn Pillar(title: &'static str, body: &'static str) -> impl IntoView {
    view! {
        <div class=uno!("flex flex-col gap-2")>
            <p class=uno!("text-h3 font-semibold text-ink-inverse")>{title}</p>
            <p class=uno!("text-small text-ink-inverse/70 leading-relaxed")>{body}</p>
        </div>
    }
}

#[component]
fn CallToAction() -> impl IntoView {
    view! {
        <Section>
            <Container width=ContainerWidth::Narrow>
                <div class=uno!("flex flex-col items-start gap-6 text-left")>
                    <Eyebrow>"Get involved"</Eyebrow>
                    <Display size=DisplaySize::Md tag="h2".to_string()>
                        "Help build the foundation."
                    </Display>
                    <Body tone=BodyTone::Secondary>
                        "ODP is an inclusive partnership for OEMs, ODMs, silicon vendors, hardware developers, security researchers - anyone willing to make device firmware better. Read the docs, clone a repo, or join a working group."
                    </Body>
                    <Cluster gap=StackGap::Sm>
                        <LinkButton href="/community".to_string()>"Join the community"</LinkButton>
                        <ArrowLink
                            href="https://opendevicepartnership.github.io/documentation/"
                                .to_string()
                            external=true
                        >
                            "Read the documentation"
                        </ArrowLink>
                    </Cluster>
                </div>
            </Container>
        </Section>
    }
}
