use crate::components::documentation_training::{DocLink, DocumentationTraining};
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::project_introduction::ProjectIntroduction;
use crate::components::repo_view::RepositoryGraph;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn EmbeddedController() -> impl IntoView {
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

    let project_title = "Secure Embedded Controller";
    let project_summary = "A hardened firmware platform for modern embedded controllers";
    let project_what = "The ODP Secure EC stack is a Rust-based firmware platform for modern embedded controllers, supporting both discrete and integrated ECs.

It provides modular subsystems for power sequencing, thermal policy, event routing, and more. 
Components are defined by traits, composed into devices, and managed by a shared runtime that drives platform behavior.
Built for portability and testability, it supports both std and no-std builds and integrates cleanly with real-time runtimes like Embassy.";

    let project_why = "Embedded Controllers do more than ever — yet many EC stacks are stuck in the past.
The ODP EC firmware rethinks the EC as a secure, modular orchestrator for power, telemetry, and system policy. With clearly scoped components and Rust’s safety guarantees, it helps you move faster, catch bugs earlier, and support diverse platforms with confidence.
It’s a modern foundation for building reliable, adaptable EC firmware — not just patching legacy code.";

    let nodes_data = r#"[{"id": 0, "name": "embedded-services", "url": "https://github.com/OpenDevicePartnership/embedded-services", "classification": "EC Services", "order": 2}, {"id": 1, "name": "pcal6416a", "url": "https://github.com/OpenDevicePartnership/pcal6416a", "classification": "Driver", "order": 4}, {"id": 2, "name": "embedded-cfu", "url": "https://github.com/OpenDevicePartnership/embedded-cfu", "classification": "EC Subsystem", "order": 3}, {"id": 3, "name": "embassy-imxrt", "url": "https://github.com/OpenDevicePartnership/embassy-imxrt", "classification": "MCU HAL", "order": 6}, {"id": 4, "name": "embedded-hal", "url": "https://github.com/rust-embedded/embedded-hal", "classification": "HAL Abstractions", "order": 5}, {"id": 5, "name": "embedded-mcu", "url": "https://github.com/OpenDevicePartnership/embedded-mcu", "classification": "HAL Abstractions", "order": 5}, {"id": 6, "name": "embedded-fans", "url": "https://github.com/OpenDevicePartnership/embedded-fans", "classification": "EC Subsystem", "order": 3}, {"id": 7, "name": "lis2dw12-i2c", "url": "https://github.com/OpenDevicePartnership/lis2dw12-i2c", "classification": "Driver", "order": 4}, {"id": 8, "name": "tps6699x", "url": "https://github.com/OpenDevicePartnership/tps6699x", "classification": "Driver", "order": 4}, {"id": 9, "name": "tmp108", "url": "https://github.com/OpenDevicePartnership/tmp108", "classification": "Driver", "order": 4}, {"id": 10, "name": "npcx490m-pac", "url": "https://github.com/OpenDevicePartnership/npcx490m-pac", "classification": "MCU PAC", "order": 7}, {"id": 11, "name": "embedded-usb-pd", "url": "https://github.com/OpenDevicePartnership/embedded-usb-pd", "classification": "EC Subsystem", "order": 3}, {"id": 12, "name": "embedded-power-sequence", "url": "https://github.com/OpenDevicePartnership/embedded-power-sequence", "classification": "EC Subsystem", "order": 3}, {"id": 13, "name": "embedded-batteries", "url": "https://github.com/OpenDevicePartnership/embedded-batteries", "classification": "EC Subsystem", "order": 3}, {"id": 14, "name": "bq25773", "url": "https://github.com/OpenDevicePartnership/bq25773", "classification": "Driver", "order": 4}, {"id": 15, "name": "bq40z50", "url": "https://github.com/OpenDevicePartnership/bq40z50", "classification": "Driver", "order": 4}, {"id": 16, "name": "ec-slimloader", "url": "https://github.com/OpenDevicePartnership/ec-slimloader", "classification": "Bootloader", "order": 8}, {"id": 17, "name": "mimxrt633s-pac", "url": "https://github.com/OpenDevicePartnership/mimxrt633s-pac", "classification": "MCU PAC", "order": 7}, {"id": 18, "name": "mimxrt685s-pac", "url": "https://github.com/OpenDevicePartnership/mimxrt685s-pac", "classification": "MCU PAC", "order": 7}, {"id": 19, "name": "embedded-regulator", "url": "https://github.com/OpenDevicePartnership/embedded-regulator", "classification": "EC Subsystem", "order": 3}, {"id": 20, "name": "embedded-keyboard-rs", "url": "https://github.com/OpenDevicePartnership/embedded-keyboard-rs", "classification": "EC Subsystem", "order": 3}, {"id": 21, "name": "bq25770g", "url": "https://github.com/OpenDevicePartnership/bq25770g", "classification": "Driver", "order": 4}, {"id": 22, "name": "ec-slimloader-descriptors", "url": "https://github.com/OpenDevicePartnership/ec-slimloader-descriptors", "classification": "Bootloader", "order": 8}, {"id": 23, "name": "embedded-sensors", "url": "https://github.com/OpenDevicePartnership/embedded-sensors", "classification": "EC Subsystem", "order": 3}, {"id": 24, "name": "mimxrt600-fcb", "url": "https://github.com/OpenDevicePartnership/mimxrt600-fcb", "classification": "Tools", "order": 9}, {"id": 25, "name": "soc-embedded-controller", "url": "https://github.com/OpenDevicePartnership/soc-embedded-controller", "classification": "EC Platform", "order": 1}, {"id": 26, "name": "embassy-npcx", "url": "https://github.com/OpenDevicePartnership/embassy-npcx", "classification": "MCU HAL", "order": 6}, {"id": 27, "name": "systemview-tracing", "url": "https://github.com/OpenDevicePartnership/systemview-tracing", "classification": "Tools", "order": 9}, {"id": 28, "name": "bq25723", "url": "https://github.com/OpenDevicePartnership/bq25723", "classification": "Driver", "order": 4}, {"id": 29, "name": "bq25713", "url": "https://github.com/OpenDevicePartnership/bq25713", "classification": "Driver", "order": 4}, {"id": 30, "name": "MX25U1632FZUI02", "url": "https://github.com/OpenDevicePartnership/MX25U1632FZUI02", "classification": "Driver", "order": 4}, {"id": 31, "name": "W25Q16JWXHIM", "url": "https://github.com/OpenDevicePartnership/W25Q16JWXHIM", "classification": "Driver", "order": 4}, {"id": 32, "name": "mec17xx-pac", "url": "https://github.com/OpenDevicePartnership/mec17xx-pac", "classification": "MCU PAC", "order": 7}, {"id": 33, "name": "is31fl3743b", "url": "https://github.com/OpenDevicePartnership/is31fl3743b", "classification": "Driver", "order": 4}, {"id": 34, "name": "npcx490m-examples", "url": "https://github.com/OpenDevicePartnership/npcx490m-examples", "classification": "Tools", "order": 9}, {"id": 35, "name": "mimxrt685s-examples", "url": "https://github.com/OpenDevicePartnership/mimxrt685s-examples", "classification": "Tools", "order": 9}, {"id": 36, "name": "embassy-microchip", "url": "https://github.com/OpenDevicePartnership/embassy-microchip", "classification": "MCU HAL", "order": 6}, {"id": 37, "name": "cec17-data", "url": "https://github.com/OpenDevicePartnership/cec17-data", "classification": "MCU PAC", "order": 7}, {"id": 38, "name": "nxp-header", "url": "https://github.com/OpenDevicePartnership/nxp-header", "classification": "Tools", "order": 9}, {"id": 39, "name": "tps65994ae", "url": "https://github.com/OpenDevicePartnership/tps65994ae", "classification": "Driver", "order": 4}, {"id": 40, "name": "bq24773", "url": "https://github.com/OpenDevicePartnership/bq24773", "classification": "Driver", "order": 4}, {"id": 41, "name": "rt4531", "url": "https://github.com/OpenDevicePartnership/rt4531", "classification": "Driver", "order": 4}, {"id": 42, "name": "bq25730", "url": "https://github.com/OpenDevicePartnership/bq25730", "classification": "Driver", "order": 4}, {"id": 43, "name": "embedded-storage", "url": "https://github.com/rust-embedded-community/embedded-storage", "classification": " EC Subsystem", "order": 3}]"#;
    let links_data = r#"[{"source": 0, "target": 2}, {"source": 0, "target": 4}, {"source": 0, "target": 5}, {"source": 0, "target": 6}, {"source": 0, "target": 11}, {"source": 0, "target": 12}, {"source": 0, "target": 13}, {"source": 0, "target": 19}, {"source": 0, "target": 20}, {"source": 0, "target": 23}, {"source": 1, "target": 4}, {"source": 2, "target": 43}, {"source": 3, "target": 17}, {"source": 3, "target": 18}, {"source": 4, "target": 3}, {"source": 4, "target": 26}, {"source": 4, "target": 36}, {"source": 5, "target": 3}, {"source": 5, "target": 26}, {"source": 5, "target": 36}, {"source": 6, "target": 4}, {"source": 7, "target": 4}, {"source": 8, "target": 4}, {"source": 9, "target": 4}, {"source": 11, "target": 8}, {"source": 11, "target": 39}, {"source": 12, "target": 4}, {"source": 12, "target": 19}, {"source": 13, "target": 14}, {"source": 13, "target": 15}, {"source": 13, "target": 21}, {"source": 13, "target": 28}, {"source": 13, "target": 29}, {"source": 13, "target": 40}, {"source": 13, "target": 42}, {"source": 14, "target": 4}, {"source": 15, "target": 4}, {"source": 16, "target": 22}, {"source": 20, "target": 4}, {"source": 21, "target": 4}, {"source": 23, "target": 9}, {"source": 25, "target": 0}, {"source": 26, "target": 10}, {"source": 28, "target": 4}, {"source": 29, "target": 4}, {"source": 30, "target": 5}, {"source": 31, "target": 5}, {"source": 33, "target": 4}, {"source": 36, "target": 32}, {"source": 36, "target": 37}, {"source": 39, "target": 4}, {"source": 40, "target": 4}, {"source": 41, "target": 4}, {"source": 42, "target": 4}, {"source": 43, "target": 30}, {"source": 43, "target": 31}]"#;

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
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
                <ProjectIntroduction project_title=project_title project_summary=project_summary project_what=project_what project_why=project_why
                    big_image_url="/images/ECBackground.png"
                    small_image_url="/images/dark/ProjectIcon_EC_Patina_DarkMode.svg"/>
                <RepositoryGraph nodes=nodes_data links=links_data/>
                <DocumentationTraining links=links />
                <Footer />
            </div>
        </ErrorBoundary>
    }
}
