//! Shared team-page layout. Used by `team-patina`, `team-ec`, and
//! `team-ec-services`.

use crate::components::cards::TeamCard;
use crate::components::controls::ArrowLink;
use crate::components::layout::{Container, Grid, GridMin, Section, Stack, StackGap};
use crate::components::typography::{Body, BodyTone, Display, DisplaySize, Eyebrow};
use crate::data::teams::TeamMember;
use leptos::prelude::*;

#[component]
pub fn TeamPage(
    eyebrow: &'static str,
    title: &'static str,
    description: &'static str,
    members: Vec<TeamMember>,
) -> impl IntoView {
    view! {
        <Section class="pt-16 md:pt-24">
            <Container>
                <Stack gap=StackGap::Md>
                    <Eyebrow>{eyebrow}</Eyebrow>
                    <Display size=DisplaySize::Lg>{title}</Display>
                    <Body tone=BodyTone::Secondary class="max-w-[60ch]">
                        {description}
                    </Body>
                    <ArrowLink href="/community".to_string()>"All working groups"</ArrowLink>
                </Stack>
            </Container>
        </Section>

        <Section>
            <Container>
                <Grid min=GridMin::Sm gap=StackGap::Md>
                    {members.into_iter().map(|m| view! { <TeamCard member=m /> }).collect_view()}
                </Grid>
            </Container>
        </Section>
    }
}
