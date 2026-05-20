//! 404.

use crate::components::controls::{ArrowLink, LinkButton};
use crate::components::layout::{Cluster, Container, ContainerWidth, Section, Stack, StackGap};
use crate::components::typography::{Body, BodyTone, Display, DisplaySize, Eyebrow};
use leptos::prelude::*;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <Section class="pt-24 md:pt-32">
            <Container width=ContainerWidth::Narrow>
                <Stack gap=StackGap::Lg>
                    <Eyebrow>"404"</Eyebrow>
                    <Display size=DisplaySize::Lg>"We couldn't find that page."</Display>
                    <Body tone=BodyTone::Secondary>
                        "It may have moved, or never existed. Try one of these instead."
                    </Body>
                    <Cluster gap=StackGap::Sm>
                        <LinkButton href="/".to_string()>"Back to home"</LinkButton>
                        <ArrowLink href="/projects".to_string()>"See the projects"</ArrowLink>
                        <ArrowLink href="/community".to_string()>"Visit the community"</ArrowLink>
                    </Cluster>
                </Stack>
            </Container>
        </Section>
    }
}
