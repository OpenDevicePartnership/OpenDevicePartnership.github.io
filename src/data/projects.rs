//! Per-project marketing copy (title, summary, "what", "why") and the
//! associated repository graph data.
//!
//! Project copy stays in Rust so we can reference it as `&'static str`
//! in components. Graph data lives in `data/graphs/*.json` and is
//! pulled in via `include_str!` -- no JSON parsing in the wasm
//! bundle, the strings are passed straight to the D3 renderer.

pub struct ProjectCopy {
    pub title: &'static str,
    /// Short label for the cross-project navigation tabs (e.g. "Patina").
    pub short_label: &'static str,
    /// Site-relative route for this project's page (e.g. "/boot-firmware").
    pub route: &'static str,
    pub summary: &'static str,
    pub what: &'static str,
    pub why: &'static str,
    pub team_route: &'static str,
    pub big_image_url: &'static str,
    pub small_image_url: &'static str,
    pub nodes_json: &'static str,
    pub links_json: &'static str,
}

pub const PATINA: ProjectCopy = ProjectCopy {
    title: "Patina",
    short_label: "Patina",
    route: "/boot-firmware",
    summary: "Modern Boot Firmware",
    what: r#"Patina is a UEFI compatible firmware interface written in the Rust language with a focus on memory safety and composition. For Patina, we re-evaluated the good and the bad from today's UEFI boot firmware and used this opportunity to embrace new language capabilities, software architecture, programming paradigms, and industry supported tooling. Patina isn't designed to replace everything necessary for system boot but instead to provide a sustainable path forward with high return on investment. Learn more about Patina on our <a href="https://github.com/OpenDevicePartnership/patina" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">GitHub repository</a> and check out the <a href="https://opendevicepartnership.github.io/patina/" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">Documentation</a> page."#,
    why: "A lot has changed in the last quarter century. UEFI boot firmware has scaled remarkably well, seamlessly ushering in new generations of hardware for PCs, but as active maintainers of UEFI we know the systemic problems that can’t be addressed without significant change. We understand the challenges of supporting a vast, diverse ecosystem of hardware devices as unique as each user.  We see the nuance in our industry's partnerships and the supply chains critical for their success.  For this reason, we started the Patina project to build a future and a coalition ready for the next set of challenges.",
    team_route: "/team-patina",
    big_image_url: "/images/PatinaBackground.webp",
    small_image_url: "/images/dark/ProjectIcon_P_Patina_DarkMode.webp",
    nodes_json: include_str!("../../data/graphs/patina_nodes.json"),
    links_json: include_str!("../../data/graphs/patina_links.json"),
};

pub const EMBEDDED_CONTROLLER: ProjectCopy = ProjectCopy {
    title: "Secure Embedded Controller",
    short_label: "Secure EC",
    route: "/embedded-controller",
    summary: "A hardened firmware platform for modern embedded controllers",
    what: r#"The ODP Secure EC stack is a Rust-based firmware platform for modern embedded controllers, supporting both discrete and integrated ECs.

It provides modular subsystems for power sequencing, thermal policy, event routing, and more. 
Components are defined by traits, composed into devices, and managed by a shared runtime that drives platform behavior.
Built for portability and testability, it supports both std and no-std builds and integrates cleanly with real-time runtimes like Embassy. Learn more about Secure EC on our <a href="https://github.com/OpenDevicePartnership/embedded-services" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">GitHub repository</a> and check out the <a href="https://opendevicepartnership.github.io/odp-embedded-controller/secure-ec-overview.html" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">Documentation</a> page."#,
    why: "Embedded Controllers do more than ever — yet many EC stacks are stuck in the past.
The ODP EC firmware rethinks the EC as a secure, modular orchestrator for power, telemetry, and system policy. With clearly scoped components and Rust’s safety guarantees, it helps you move faster, catch bugs earlier, and support diverse platforms with confidence.
It’s a modern foundation for building reliable, adaptable EC firmware — not just patching legacy code.",
    team_route: "/team-ec",
    big_image_url: "/images/ECBackground.webp",
    small_image_url: "/images/dark/ProjectIcon_EC_Patina_DarkMode.webp",
    nodes_json: include_str!("../../data/graphs/ec_nodes.json"),
    links_json: include_str!("../../data/graphs/ec_links.json"),
};

pub const EC_SERVICES: ProjectCopy = ProjectCopy {
    title: "Unified Embedded Controller Interface",
    short_label: "EC Services",
    route: "/windows-ec-services",
    summary: "",
    what: r#"The Unified Windows EC Service interface defines runtime coordination between firmware components using async message-passing. 
    Each service manages a domain — like power, battery, or host communication — and exposes a structured protocol. 
    Components register with services and receive commands for events, capabilities, and state changes. 
    This model enables loosely coupled subsystems, observability, and test injection — without sacrificing platform coherence. Learn more about EC Services on our <a href="https://github.com/OpenDevicePartnership/haf-ec-service" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">GitHub repository</a> and check out the <a href="https://opendevicepartnership.github.io/odp-embedded-controller/specs/ec_interface/ec_interface.html" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">Documentation</a> page."#,
    why: "Without a common interface, EC firmware becomes tangled and brittle. 
    Unified EC Services -- designed for Windows Platforms -- bring structure and predictability by defining how components interact at runtime. 
    With async protocols and policy-aware life cycles, they support clean separation of concerns and cross-subsystem coordination. 
    Whether debugging power flows or integrating a new device, these services provide the glue, guardrails, and visibility you need.",
    team_route: "/team-ec-services",
    big_image_url: "/images/ECServicesBackground.webp",
    small_image_url: "/images/dark/ProjectIcon_ES_Patina_DarkMode.webp",
    nodes_json: include_str!("../../data/graphs/ec_services_nodes.json"),
    links_json: include_str!("../../data/graphs/ec_services_links.json"),
};

#[cfg(test)]
mod tests {
    //! These tests guard the data the D3 renderer consumes. The
    //! renderer ([`crate::components::repo_view`]) ships JSON strings
    //! straight to JavaScript without parsing them in Rust, so a
    //! malformed JSON file would only blow up at runtime in the
    //! browser. The tests below catch:
    //!
    //!  * malformed JSON (typo, trailing comma, …),
    //!  * empty graphs (an oversight that would render a blank SVG),
    //!  * nodes without an `id` field,
    //!  * links whose `source`/`target` reference a missing node id.
    //!
    //! The graph script-load race (the bug fixed alongside t21) is a
    //! browser-runtime concern and is not exercised here -- the data
    //! invariants below at least guarantee that, once the renderer
    //! does run, the payload is shaped correctly.
    use super::*;
    use serde_json::Value;

    fn parse_array(json: &str) -> Vec<Value> {
        let value: Value = serde_json::from_str(json).expect("graph json must parse");
        value.as_array().expect("graph json must be a top-level array").clone()
    }

    fn assert_graph_well_formed(name: &str, nodes_json: &str, links_json: &str) {
        let nodes = parse_array(nodes_json);
        let links = parse_array(links_json);

        assert!(
            !nodes.is_empty(),
            "{name}: nodes must not be empty (would render a blank graph)"
        );
        assert!(!links.is_empty(), "{name}: links must not be empty");

        // Node ids may be either numbers or strings -- d3-force keys
        // by `===` equality, so we compare the JSON-encoded form to
        // catch duplicates regardless of representation.
        let mut ids: std::collections::HashSet<String> = std::collections::HashSet::new();
        for node in &nodes {
            let id = node
                .get("id")
                .unwrap_or_else(|| panic!("{name}: every node must have an `id` field"));
            assert!(
                id.is_number() || id.is_string(),
                "{name}: node id must be a number or a string, got {id}"
            );
            let key = id.to_string();
            assert!(
                ids.insert(key.clone()),
                "{name}: duplicate node id {key} -- d3-force will collapse the duplicates"
            );
        }

        for (i, link) in links.iter().enumerate() {
            let source = link
                .get("source")
                .unwrap_or_else(|| panic!("{name}: link[{i}] missing `source`"))
                .to_string();
            let target = link
                .get("target")
                .unwrap_or_else(|| panic!("{name}: link[{i}] missing `target`"))
                .to_string();
            assert!(
                ids.contains(&source),
                "{name}: link[{i}].source {source} does not reference any node id"
            );
            assert!(
                ids.contains(&target),
                "{name}: link[{i}].target {target} does not reference any node id"
            );
        }
    }

    #[test]
    fn patina_graph_is_well_formed() {
        assert_graph_well_formed("PATINA", PATINA.nodes_json, PATINA.links_json);
    }

    #[test]
    fn embedded_controller_graph_is_well_formed() {
        assert_graph_well_formed(
            "EMBEDDED_CONTROLLER",
            EMBEDDED_CONTROLLER.nodes_json,
            EMBEDDED_CONTROLLER.links_json,
        );
    }

    #[test]
    fn ec_services_graph_is_well_formed() {
        assert_graph_well_formed("EC_SERVICES", EC_SERVICES.nodes_json, EC_SERVICES.links_json);
    }

    #[test]
    fn project_copy_has_required_fields() {
        for project in [&PATINA, &EMBEDDED_CONTROLLER, &EC_SERVICES] {
            assert!(!project.title.is_empty(), "title must not be empty");
            assert!(!project.what.is_empty(), "what must not be empty");
            assert!(!project.why.is_empty(), "why must not be empty");
            assert!(
                project.team_route.starts_with('/'),
                "team_route must be a router path: {:?}",
                project.team_route
            );
            assert!(
                project.big_image_url.starts_with('/'),
                "big_image_url must be site-relative: {:?}",
                project.big_image_url
            );
            assert!(
                project.small_image_url.starts_with('/'),
                "small_image_url must be site-relative: {:?}",
                project.small_image_url
            );
        }
    }
}
