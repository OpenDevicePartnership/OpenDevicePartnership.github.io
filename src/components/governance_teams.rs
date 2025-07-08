use leptos::prelude::*;

#[component]
pub fn GovernanceTeams() -> impl IntoView {
    view! {
        <section
            class="background_primary"
            style="
                padding: 120px;
            "
        >
            {/* Row 1: Header */}
            <div style="margin-bottom: 56px;">
                <span
                    class="p_mono"
                    style="
                        display: block;
                    "
                >
                    {"GOVERNANCE"}
                </span>
            </div>

            {/* Row 2: Two Columns */}
            <div class="flex flex-row gap-[60px]" style="margin-bottom: 80px;">
                <div style="width: 950px;">
                    <span class="h1" style="display: block; text-align: left;">
                        {"How ODP is built by its community"}
                    </span>
                </div>
                <div class="flex flex-col justify-start" style="flex: 1; min-width: 600px; max-width: 900px;">
                    <span
                        class="p2"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >

                        {"The Open Device Partnership (ODP) is a collaborative open-source initiative designed to promote cooperative innovation in firmware development through contribution and transparency. "}
                        <br/>
                        <br/>
                        {"To support this vision, ODP has adopted a lightweight yet formal governance model that clarifies how decisions are made, how contributions are recognized, and how the community remains focused on shared goals. "}
                        <br/>
                        <br/>
                        {"Like most open-source communities, the ODP Governance model consists of a group of core maintainers that are responsible for guiding the technical direction of the project known as the \"Technical Steering Committee\" (TSC). Members are typically industry-experienced contributors nominated by the community members or its sponsors."}
                        <br/>
                        {"The model also provides for certain \"Working Groups\", which are task-focused teams that handle the development or specification within a particular area of concern (for example, Patina, EC Services)."}
                        <br/>
                        {"Decisions are made via public discussion followed by majority vote among TSC members. "}
                        <br/>
                        {"All official actions, roadmaps, and meeting notes are transparently published, and all project materials are hosted openly on GitHub. "}
                        <br/>
                        <br/>
                        {"For more information, see the complete Governance policies at the "}
                        <a href="https://github.com/OpenDevicePartnership/governance/README.md" target="_blank" style="text-decoration: underline;">{"ODP Governance Repository"}</a>
                        {", or join the discussion on our public chat on "}
                        <a href="https://opendevicepartnership.zulipchat.com/" target="_blank" style="text-decoration: underline;">{"Zulip"}</a>
                        {"."}                    
                    </span>
                </div>
            </div>

            {/* Row 3: Teams Image and Label */}
            <div class="flex flex-col items-left" style="margin-bottom: 80px;">
                <picture>
                    <source srcset="/images/dark/Teams.svg" media="(prefers-color-scheme: dark)" />
                    <img
                        src="/images/light/Teams.svg"
                        alt="Teams"
                        class="icon"
                    />
                </picture>
                <span
                    class="h2"
                    style="
                        display: flex;
                        justify-content: left;
                        align-items: left;
                    "
                >
                    {"Teams"}
                </span>
            </div>

            {/* Row 4: Teams */}
            <div class="flex flex-row items-stretch" style="gap: 123px;">
                <div class="flex flex-col items-start h-full" style="width: 320px; min-height: 350px; justify-content: flex-start;">
                    <span
                        class="h3"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Boot Firmware (Patina)"}
                    </span>
                    <span
                        class="p2"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Developing and managing development of a new modern UEFI"}
                    </span>
                    <div style="flex: 1 1 auto;"></div>
                    <a
                        href="/team-patina"
                        class="odp-btn odp-btn-text"
                        style="
                            margin-top: auto;
                            text-decoration: none;
                        "
                    >
                        {"Members + Contacts"}
                    </a>
                </div>
                <div class="flex flex-col items-start h-full" style="width: 320px; min-height: 350px; justify-content: flex-start;">
                    <span
                        class="h3"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Secure Embedded Controller"}
                    </span>
                    <span
                        class="p2"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Developing and managing secure EC internals"}
                    </span>
                    <div style="flex: 1 1 auto;"></div>
                    <a
                        href="/team-ec"
                        class="odp-btn odp-btn-text"
                        style="
                            margin-top: auto;
                            text-decoration: none;
                        "
                    >
                        {"Members + Contacts"}
                    </a>
                </div>
                <div class="flex flex-col items-start h-full" style="width: 320px; min-height: 350px; justify-content: flex-start;">
                    <span
                        class="h3"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Unified EC Services"}
                    </span>
                    <span
                        class="p2"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Designing and managing implementation of a unified EC Services interface"}
                    </span>
                    <div style="flex: 1 1 auto;"></div>
                    <a
                        href="team-ec-services"
                        class="odp-btn odp-btn-text"
                        style="
                            margin-top: auto;
                            text-decoration: none;
                        "
                    >
                        {"Members + Contacts"}
                    </a>
                </div>
            </div>
        </section>
    }
}