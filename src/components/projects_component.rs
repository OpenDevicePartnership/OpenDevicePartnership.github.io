use crate::components::image_button::ImageButton;
use crate::components::ui::{
    ArrowLink, Heading, HeadingLevel, LabeledSection, Section, Text, TextSize, TwoColumnIntro,
};
use leptos::prelude::*;

#[component]
pub fn ProjectsComponent() -> impl IntoView {
    view! {
        <TwoColumnIntro
            section_class="py-8 md:py-20 lg:py-32"
            left=|| {
                view! {
                    <Heading level=HeadingLevel::H1 class="text-left">
                        {"System Firmware Domains"}
                    </Heading>
                    <Heading level=HeadingLevel::H2 class="text-left">
                        {"Reusable foundations for secure, high-quality device platforms"}
                    </Heading>
                }
            }
            right=|| {
                view! {
                    <LabeledSection label="WHAT">
                        <Text size=TextSize::Lead class="text-left">
                            {"ODP supports development across three core areas of system firmware. Each domain is designed for modularity, security, and long-term reuse across hardware platforms."}
                        </Text>
                    </LabeledSection>
                    <LabeledSection label="WHY">
                        <Text size=TextSize::Lead class="text-left">
                            {"Modern computing devices need modern solutions that prioritize memory safety and security from the language on up."}
                        </Text>
                    </LabeledSection>
                }
            }
        />

        // Projects Details Section
        <Section class="py-8 md:py-20 lg:py-32">
            <div class="flex flex-col gap-16">

                <ProjectRow
                    href="/boot-firmware"
                    img_src="/images/patina.webp"
                    alt="Boot Firmware"
                    title="Patina (Boot Firmware)"
                    tagline="Rethink your boot firmware"
                    description="Patina provides a UEFI compatible firmware interface written in the Rust language with a focus on memory safety and composition.  Compatible with current UEFI device drivers and loaders but with a focus on the future."
                    docs_href="https://opendevicepartnership.github.io/patina"
                    docs_title="Read the Patina Documentation"
                    source_href="https://github.com/opendevicepartnership/patina"
                    source_title="View Patina Source Code on GitHub"
                />
                <ProjectRow
                    href="/embedded-controller"
                    img_src="/images/ec.webp"
                    alt="Embedded Controller"
                    title="Secure Embedded Controller"
                    tagline="A Secure end-to-end Rust-based EC implementation"
                    description="ODP EC provides a modern embedded controller firmware written in Rust. Designed for safety and composability, it serves as a foundational layer for secure device management."
                    docs_href="https://opendevicepartnership.github.io/odp-embedded-controller/secure-ec-overview.html"
                    docs_title="Read the Secure EC Documentation"
                    source_href="https://github.com/OpenDevicePartnership/embedded-services"
                    source_title="View Secure EC Source Code on GitHub"
                />
                <ProjectRow
                    href="/windows-ec-services"
                    img_src="/images/ec_services.webp"
                    alt="EC Services"
                    title="Unified Embedded Controller Services"
                    tagline="A standard and secure cross-architecture EC services implementation"
                    description="ODP EC Services provides a modern EC services firmware written in Rust. Designed for safety and composability, it serves as a foundational layer for secure EC services on Windows platforms."
                    docs_href="https://opendevicepartnership.github.io/odp-embedded-controller/specs/ec_interface/ec_interface.html"
                    docs_title="Read the EC Services Documentation"
                    source_href="https://github.com/OpenDevicePartnership/haf-ec-service"
                    source_title="View EC Services Source Code on GitHub"
                />
            </div>
        </Section>
    }
}

/// One "image | title + tagline + description + two doc links" row
/// from the projects page. Three nearly-identical instances were
/// inlined before this extraction.
#[component]
fn ProjectRow(
    href: &'static str,
    img_src: &'static str,
    alt: &'static str,
    title: &'static str,
    tagline: &'static str,
    description: &'static str,
    docs_href: &'static str,
    docs_title: &'static str,
    source_href: &'static str,
    source_title: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex flex-col md:flex-row gap-8 md:gap-16 items-center">
            <ImageButton href=href img_src=img_src alt=alt class="aspect-square max-w-[600px]" />
            <div class="flex flex-col items-start w-full md:w-auto mt-8 md:mt-0 flex-1">
                <Heading level=HeadingLevel::H2 class="text-left">
                    {title}
                </Heading>
                <Text size=TextSize::Lead class="text-left">
                    {tagline}
                </Text>
                <Text size=TextSize::Large class="text-left">
                    {description}
                </Text>
                <div class="flex flex-col gap-2">
                    <ProjectLink href=docs_href title=docs_title />
                    <ProjectLink href=source_href title=source_title />
                </div>
            </div>
        </div>
    }
}

#[component]
fn ProjectLink(href: &'static str, title: &'static str) -> impl IntoView {
    view! { <ArrowLink href=href title=title /> }
}
