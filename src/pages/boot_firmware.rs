use crate::components::documentation_training::{DocLink, DocumentationTraining};
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::project_introduction::ProjectIntroduction;
use crate::components::repo_view::RepositoryGraph;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn BootFirmware() -> impl IntoView {
    let links = vec![
        DocLink {
            href: "https://opendevicepartnership.github.io/documentation/guide/why/why.html",
            title: "Why ODP?",
            external: true,
        },
        DocLink {
            href: "https://opendevicepartnership.github.io/documentation/guide/intro/getting_started.html",
            title: "Getting Started with ODP",
            external: true,
        },
        DocLink {
            href: "https://opendevicepartnership.github.io/documentation/guide/intro/welcome.html",
            title: "Tutorials",
            external: true,
        },
        DocLink {
            href: "https://opendevicepartnership.github.io/documentation/guide/specs/specifications.html",
            title: "Specifications",
            external: true,
        },
        DocLink {
            href: "/community",
            title: "Contributing to ODP",
            external: false,
        },
    ];

    let project_title = "Boot Firmware (Patina)";
    let project_summary = "A secure and efficient boot firmware for Windows devices";

    let project_what = "Patina is a Rust-based boot firmware platform that rethinks the full UEFI boot flow as a modular system.
It replaces the legacy UEFI dispatcher with a clear, dependency-injected structure that defines policy and behavior explicitly.
Patina supports the same boot phases as UEFI, but with modern Rust interfaces, testable components, and reproducible builds making it ideal for secure and maintainable firmware.";

    let project_why = "The world has changed - UEFI needs a reboot.
Patina replaces brittle C-based firmware with a secure, modular Rust implementation designed for today’s threats.
Embracing modern patterns and dependency injection, Patina simplifies audits, shrinks attack surfaces, and enables predictable firmware behavior.
It’s time for firmware to act like real software — and Patina makes that possible.";

    let nodes_data = r#"[{"id": 0, "name": "patina-readiness-tool", "url": "https://github.com/OpenDevicePartnership/patina-readiness-tool", "classification": "tooling", "order": 1}, {"id": 1, "name": "patina-fw-patcher", "url": "https://github.com/OpenDevicePartnership/patina-fw-patcher", "classification": "tooling", "order": 1}, {"id": 2, "name": "patina-qemu", "url": "https://github.com/OpenDevicePartnership/patina-qemu", "classification": "platform firmware (uefi rom)", "order": 2}, {"id": 3, "name": "patina-dxe-core-qemu", "url": "https://github.com/OpenDevicePartnership/patina-dxe-core-qemu", "classification": "qemu patina dxe core (binary)", "order": 3}, {"id": 4, "name": "patina (patina_dxe_core)", "url": "https://github.com/OpenDevicePartnership/patina", "classification": "patina dxe core (library)", "order": 4}, {"id": 5, "name": "patina (core)", "url": "https://github.com/OpenDevicePartnership/patina", "classification": "core (library)", "order": 5}, {"id": 6, "name": "patina (components)", "url": "https://github.com/OpenDevicePartnership/patina", "classification": "components (library)", "order": 6}, {"id": 7, "name": "patina (sdk)", "url": "https://github.com/OpenDevicePartnership/patina", "classification": "sdk (library)", "order": 7}, {"id": 8, "name": "patina-mtrr", "url": "https://github.com/OpenDevicePartnership/patina-mtrr", "classification": "general purpose (library)", "order": 8}, {"id": 9, "name": "patina-paging", "url": "https://github.com/OpenDevicePartnership/patina-paging", "classification": "general purpose (library)", "order": 8}]"#;
    let links_data = r#"[{"source": 0, "target": 5}, {"source": 0, "target": 7}, {"source": 3, "target": 4}, {"source": 3, "target": 5}, {"source": 3, "target": 6}, {"source": 3, "target": 7}, {"source": 4, "target": 5}, {"source": 4, "target": 6}, {"source": 4, "target": 7}, {"source": 4, "target": 8}, {"source": 4, "target": 9}, {"source": 5, "target": 6}, {"source": 5, "target": 7}, {"source": 5, "target": 9}, {"source": 6, "target": 7}]"#;

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>
            }
        }>

            <div class="w-full min-h-screen" style="overflow-x: auto;">
                <Header />
                <ProjectIntroduction
                    project_title=project_title
                    project_summary=project_summary
                    project_what=project_what
                    project_why=project_why
                    big_image_url="/images/PatinaBackground.png"
                    small_image_url="/images/dark/ProjectIcon_P_Patina_DarkMode.svg"
                />
                <RepositoryGraph nodes=nodes_data links=links_data/>
                <DocumentationTraining links=links />
                <Footer />
            </div>
        </ErrorBoundary>
    }
}
