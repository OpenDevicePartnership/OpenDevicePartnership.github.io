//! Community page (`/`) -- merges governance, contributing, docs &
//! training, and team directory into one editorial page with
//! anchored sections.

use crate::components::cards::{DocCard, TeamCard};
use crate::components::controls::InlineLink;
use crate::components::layout::{
    Container, Grid, GridMin, Section, SectionSurface, Stack, StackGap, Surface, SurfaceElevation, SurfaceTone,
};
use crate::components::typography::{Body, BodyTone, Display, DisplaySize, Eyebrow, Heading, HeadingLevel};
use crate::data::teams::steering_committee;
use leptos::prelude::*;
use leptos_router::components::A;
use unocss_classes::uno;

#[component]
pub fn Community() -> impl IntoView {
    view! {
        <Section class="pt-16 md:pt-24">
            <Container>
                <Stack gap=StackGap::Lg>
                    <Eyebrow>"Community"</Eyebrow>
                    <Display size=DisplaySize::Lg>"How ODP is built by its people."</Display>
                    <Body tone=BodyTone::Secondary class="max-w-[60ch]">
                        "ODP is a collaborative open-source initiative. We work in the open, govern in the open, and welcome anyone willing to make device firmware safer and more reusable."
                    </Body>
                </Stack>
            </Container>
        </Section>

        <GovernanceSection />
        <SteeringSection />
        <WorkingGroupsSection />
        <ContributeSection />
        <DocsSection />
    }
}

#[component]
fn GovernanceSection() -> impl IntoView {
    view! {
        <Section id="governance" surface=SectionSurface::Sunken>
            <Container>
                <Stack gap=StackGap::Lg>
                    <Eyebrow>"Governance"</Eyebrow>
                    <Heading level=HeadingLevel::H2>"Lightweight, formal, transparent."</Heading>
                    <div class=uno!(
                        "max-w-[68ch] text-body text-ink-primary leading-relaxed flex flex-col gap-4"
                    )>
                        <p>
                            "ODP has adopted a lightweight yet formal governance model that clarifies how decisions are made, how contributions are recognised, and how the community remains focused on shared goals."
                        </p>
                        <p>
                            "A Technical Steering Committee (TSC) of industry-experienced contributors guides the technical direction. Working Groups handle development and specification within particular areas of concern (Patina, EC Services, ...)."
                        </p>
                        <p>
                            "Decisions are made via public discussion followed by majority vote among TSC members. All official actions, road maps, and meeting notes are published, and all project materials are hosted openly on GitHub."
                        </p>
                        <p>
                            "Read the full policies in the "
                            <InlineLink
                                href="https://github.com/OpenDevicePartnership/governance/blob/main/README.md"
                                    .to_string()
                                external=true
                            >
                                "ODP Governance repository"
                            </InlineLink> ", or join the discussion on "
                            <InlineLink
                                href="https://opendevicepartnership.zulipchat.com/".to_string()
                                external=true
                            >
                                "Zulip"
                            </InlineLink> "."
                        </p>
                    </div>
                </Stack>
            </Container>
        </Section>
    }
}

#[component]
fn SteeringSection() -> impl IntoView {
    let members = steering_committee();
    view! {
        <Section id="steering">
            <Container>
                <Stack gap=StackGap::Lg>
                    <Eyebrow>"Technical Steering Committee"</Eyebrow>
                    <Heading level=HeadingLevel::H2>"The people who steer the ship."</Heading>
                    <Grid min=GridMin::Sm gap=StackGap::Md>
                        {members
                            .into_iter()
                            .map(|m| view! { <TeamCard member=m /> })
                            .collect_view()}
                    </Grid>
                </Stack>
            </Container>
        </Section>
    }
}

#[component]
fn WorkingGroupsSection() -> impl IntoView {
    view! {
        <Section id="working-groups" surface=SectionSurface::Sunken>
            <Container>
                <Stack gap=StackGap::Lg>
                    <Eyebrow>"Working groups"</Eyebrow>
                    <Heading level=HeadingLevel::H2>"Find your team."</Heading>
                    <Grid min=GridMin::Md gap=StackGap::Lg>
                        <WorkingGroupCard
                            title="Boot Firmware (Patina)"
                            description="Developing and managing a new modern UEFI."
                            href="/team-patina"
                        />
                        <WorkingGroupCard
                            title="Secure EC"
                            description="Developing and managing secure embedded controller internals."
                            href="/team-ec"
                        />
                        <WorkingGroupCard
                            title="Unified EC Services"
                            description="Designing and managing implementation of a unified EC services interface."
                            href="/team-ec-services"
                        />
                    </Grid>
                </Stack>
            </Container>
        </Section>
    }
}

#[component]
fn WorkingGroupCard(title: &'static str, description: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <A href=href attr:class="group block">
            <Surface
                tone=SurfaceTone::Raised
                elevation=SurfaceElevation::E1
                class="h-full flex flex-col gap-3 group-hover:(shadow-elev-3 -translate-y-0.5) transition-all duration-300"
            >
                <Heading level=HeadingLevel::H3>{title}</Heading>
                <Body tone=BodyTone::Secondary>{description}</Body>
                <span class=uno![
                    "mt-auto inline-flex items-center gap-2 text-small font-medium text-ink-accent",
                    "group-hover:underline underline-offset-4"
                ]>
                    "Members + contacts"
                    <span class="i-lucide-arrow-right w-4 h-4" aria-hidden="true"></span>
                </span>
            </Surface>
        </A>
    }
}

#[component]
fn ContributeSection() -> impl IntoView {
    view! {
        <Section id="contribute">
            <Container>
                <Stack gap=StackGap::Lg>
                    <Eyebrow>"Contribute"</Eyebrow>
                    <Heading level=HeadingLevel::H2>"Ways to start."</Heading>
                    <Grid min=GridMin::Md gap=StackGap::Lg>
                        <DocCard
                            href="https://github.com/OpenDevicePartnership"
                            title="Browse the code"
                            description="All projects live on GitHub. Pick a repo, read the contributing guide, file an issue, or open a PR."
                            external=true
                        />
                        <DocCard
                            href="https://opendevicepartnership.zulipchat.com/"
                            title="Join the conversation"
                            description="Zulip is the home for working group discussion, design reviews, and async Q&A."
                            external=true
                        />
                        <DocCard
                            href="/getting-started"
                            title="Read the getting-started guide"
                            description="A short walkthrough that orients new contributors to the projects, the docs, and the working groups."
                        />
                        <DocCard
                            href="https://www.youtube.com/@OpenDevicePartnership"
                            title="ODP on YouTube"
                            description="Talks, demos, and recordings from working group sessions."
                            external=true
                        />
                    </Grid>
                </Stack>
            </Container>
        </Section>
    }
}

#[component]
fn DocsSection() -> impl IntoView {
    view! {
        <Section id="docs" surface=SectionSurface::Sunken>
            <Container>
                <Stack gap=StackGap::Lg>
                    <Eyebrow>"Documentation"</Eyebrow>
                    <Heading level=HeadingLevel::H2>"Start developing with ODP."</Heading>
                    <Grid min=GridMin::Md gap=StackGap::Lg>
                        <DocCard
                            href="https://opendevicepartnership.github.io/documentation/guide/why/why.html"
                            title="Why ODP?"
                            description="The rationale behind the partnership, the projects, and the technical direction."
                            external=true
                        />
                        <DocCard
                            href="https://opendevicepartnership.github.io/documentation/guide/intro/getting_started.html"
                            title="Getting started with ODP"
                            description="Tooling, repos, and the lay of the land for new developers."
                            external=true
                        />
                        <DocCard
                            href="https://opendevicepartnership.github.io/documentation/guide/intro/welcome.html"
                            title="Tutorials"
                            description="Hands-on walkthroughs covering the core projects."
                            external=true
                        />
                        <DocCard
                            href="https://opendevicepartnership.github.io/documentation/guide/specs/specifications.html"
                            title="Specifications"
                            description="The public, versioned specifications that ODP projects implement and extend."
                            external=true
                        />
                    </Grid>
                </Stack>
            </Container>
        </Section>
    }
}
