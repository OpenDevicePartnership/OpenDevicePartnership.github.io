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

    let project_title = "Patina";
    let project_summary = "Modern Boot Firmware";

    let project_what = r#"Patina is a UEFI compatible firmware interface written in the Rust language with a focus on memory safety and composition. For Patina, we re-evaluated the good and the bad from today's UEFI boot firmware and used this opportunity to embrace new language capabilities, software architecture, programming paradigms, and industry supported tooling. Patina isn't designed to replace everything necessary for system boot but instead to provide a sustainable path forward with high return on investment. Learn more about Patina on our <a href="https://github.com/OpenDevicePartnership/patina" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">GitHub repository</a> and check out the <a href="https://opendevicepartnership.github.io/patina/" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">Documentation</a> page."#;
    let project_why = "A lot has changed in the last quarter century. UEFI boot firmware has scaled remarkably well, seamlessly ushering in new generations of hardware for PCs, but as active maintainers of UEFI we know the systemic problems that can’t be addressed without significant change. We understand the challenges of supporting a vast, diverse ecosystem of hardware devices as unique as each user.  We see the nuance in our industry's partnerships and the supply chains critical for their success.  For this reason, we started the Patina project to build a future and a coalition ready for the next set of challenges.";
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
                    project_who="/team-patina"
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
