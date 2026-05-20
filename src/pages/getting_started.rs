//! Getting started page (`/getting-started`) -- numbered three-step
//! walkthrough plus links to the deeper documentation.

use crate::components::cards::DocCard;
use crate::components::controls::ArrowLink;
use crate::components::layout::{
    Cluster, Container, Grid, GridMin, Section, SectionSurface, Stack, StackGap, Surface, SurfaceElevation, SurfaceTone,
};
use crate::components::typography::{Body, BodyTone, Display, DisplaySize, Eyebrow, Heading, HeadingLevel};
use leptos::prelude::*;
use unocss_classes::uno;

#[component]
pub fn GettingStarted() -> impl IntoView {
    view! {
        <Section class="pt-16 md:pt-24">
            <Container>
                <Stack gap=StackGap::Lg>
                    <Eyebrow>"Getting started"</Eyebrow>
                    <Display size=DisplaySize::Lg>
                        "From zero to your first ODP contribution."
                    </Display>
                    <Body tone=BodyTone::Secondary class="max-w-[60ch]">
                        "Three steps. The first is the hardest; the rest follow naturally."
                    </Body>
                </Stack>
            </Container>
        </Section>

        <Section>
            <Container>
                <Grid min=GridMin::Md gap=StackGap::Lg>
                    <Step
                        number="01"
                        title="Pick a project"
                        body="Patina (boot firmware), Secure EC, or Unified EC Services. Each project page has a short tour of the codebase and the working group."
                        cta_label="See the projects"
                        cta_href="/projects"
                    />
                    <Step
                        number="02"
                        title="Read the docs"
                        body="The library covers the why, the architecture, and the specs. Skim the 'Why ODP?' page first; it answers most early questions."
                        cta_label="Open the library"
                        cta_href="https://opendevicepartnership.github.io/documentation/"
                        external=true
                    />
                    <Step
                        number="03"
                        title="Join the conversation"
                        body="Working group chatter happens on Zulip. Drop in, lurk for a bit, then say hello in the relevant stream."
                        cta_label="Join Zulip"
                        cta_href="https://opendevicepartnership.zulipchat.com/"
                        external=true
                    />
                </Grid>
            </Container>
        </Section>

        <Section surface=SectionSurface::Sunken>
            <Container>
                <Stack gap=StackGap::Lg>
                    <Eyebrow>"Documentation"</Eyebrow>
                    <Heading level=HeadingLevel::H2>"Go deeper."</Heading>
                    <Grid min=GridMin::Md gap=StackGap::Lg>
                        <DocCard
                            href="https://opendevicepartnership.github.io/documentation/guide/why/why.html"
                            title="Why ODP?"
                            description="The rationale behind the partnership and its technical bets."
                            external=true
                        />
                        <DocCard
                            href="https://opendevicepartnership.github.io/documentation/guide/intro/getting_started.html"
                            title="Developer getting-started"
                            description="Tooling, repo layout, and your first build."
                            external=true
                        />
                        <DocCard
                            href="https://opendevicepartnership.github.io/documentation/guide/specs/specifications.html"
                            title="Specifications"
                            description="The public specs ODP implements and extends."
                            external=true
                        />
                        <DocCard
                            href="/community"
                            title="Community + governance"
                            description="How decisions get made, how working groups operate, and how to join."
                        />
                    </Grid>
                </Stack>
            </Container>
        </Section>
    }
}

#[component]
fn Step(
    number: &'static str,
    title: &'static str,
    body: &'static str,
    cta_label: &'static str,
    cta_href: &'static str,
    #[prop(optional, default = false)] external: bool,
) -> impl IntoView {
    view! {
        <Surface
            tone=SurfaceTone::Raised
            elevation=SurfaceElevation::E1
            class="h-full flex flex-col gap-4"
        >
            <span class=uno!(
                "text-display font-display font-medium text-accent leading-none"
            )>{number}</span>
            <Heading level=HeadingLevel::H3>{title}</Heading>
            <Body tone=BodyTone::Secondary>{body}</Body>
            <Cluster gap=StackGap::Sm class="mt-auto pt-2">
                <ArrowLink href=cta_href.to_string() external=external>
                    {cta_label}
                </ArrowLink>
            </Cluster>
        </Surface>
    }
}
