//! Projects index (`/projects`).

use crate::components::cards::ProjectCard;
use crate::components::controls::TagTone;
use crate::components::layout::{Container, Grid, GridMin, Section, Stack, StackGap};
use crate::components::typography::{Body, BodyTone, Display, DisplaySize, Eyebrow};
use crate::data::projects::{EC_SERVICES, EMBEDDED_CONTROLLER, PATINA};
use leptos::prelude::*;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <Section class="pt-16 md:pt-24">
            <Container>
                <Stack gap=StackGap::Lg>
                    <Eyebrow>"Projects"</Eyebrow>
                    <Display size=DisplaySize::Lg>"What we are building."</Display>
                    <Body tone=BodyTone::Secondary class="max-w-[60ch]">
                        "ODP is currently focused on three projects across the device firmware stack. Each one is independently useful, and they are designed to compose."
                    </Body>
                </Stack>
            </Container>
        </Section>

        <Section>
            <Container>
                <Grid min=GridMin::Md gap=StackGap::Lg>
                    <ProjectCard project=&PATINA tone=TagTone::Patina />
                    <ProjectCard project=&EMBEDDED_CONTROLLER tone=TagTone::Ec />
                    <ProjectCard project=&EC_SERVICES tone=TagTone::Services />
                </Grid>
            </Container>
        </Section>
    }
}
