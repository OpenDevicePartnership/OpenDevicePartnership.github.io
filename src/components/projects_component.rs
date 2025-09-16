use crate::components::image_button::ImageButton;
use leptos::prelude::*;

#[component]
pub fn ProjectsComponent() -> impl IntoView {
    view! {
        <section
            class="background_primary"
            style="
                padding: 120px;
            "
        >
            <div class="flex flex-row gap-[80px]">
                {/* Left Column */}
                <div class="flex flex-col items-start" style="width: 700px;">
                    <span
                        class="h1"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"System Firmware Domains"}
                    </span>
                    <span
                        class="h2"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Reusable foundations for secure, high-quality device platforms"}
                    </span>
                </div>
                {/* Right Column */}
                <div class="flex flex-col items-start" style="width: 600px;">
                    {/* WHAT label */}
                    <span
                        class="mono"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"WHAT"}
                    </span>
                    {/* WHAT description */}
                    <span
                        class="p1"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"ODP supports development across three core areas of system firmware. Each domain is designed for modularity, security, and long-term reuse across hardware platforms."}
                    </span>
                    {/* WHY label */}
                    <span
                        class="mono"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"WHY"}
                    </span>
                    {/* WHY description */}
                    <span
                        class="p1"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Modern computing devices need modern solutions that prioritize memory safety and security from the language on up."}
                    </span>
                </div>
            </div>
        </section>

        // Projects Details Section
        <section
            class="background_primary"
            style="
                padding: 120px;
            "
        >
            <div class="flex flex-col gap-[60px]">
                {/* Row 1 */}
                <div class="flex flex-row gap-[60px] items-center">
                    {/* Image Button */}
                    <div>
                        <ImageButton href="/boot-firmware" img_src="/images/patina.png" alt="Boot Firmware" width=600 height=518 />
                    </div>
                    {/* Text Content */}
                    <div class="flex flex-col items-start" style="flex: 1;">
                        <span
                            class="h2"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Boot Firmware (Patina)"}
                        </span>
                        <span
                            class="p1"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"A secure Rust-based UEFI implementation"}
                        </span>
                        <span
                            class="p2"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Patina provides a modern UEFI firmware written in Rust. Designed for safety and composability, it serves as a foundational layer for secure boot on client platforms."}
                        </span>
                        <div class="flex flex-col gap-[8px]">
                            <a
                                href="https://opendevicepartnership.github.io/documentation/tracks/patina/track_overview.html"
                                class="link"
                                style="text-decoration: none;"
                                target="_blank"
                            >
                                <span style="text-decoration: none;">{"→ "}</span>
                                <span style="text-decoration: underline;">{"Read the Boot Firmware Guide"}</span>
                            </a>
                            <span
                                class="p2"
                                style="
                                    display: block;
                                    text-align: left;
                                    color: #888;
                                "
                            >
                                {"Patina GitHub repo coming soon!"}
                            </span>
                        </div>
                    </div>
                </div>
                {/* Row 2 */}
                <div class="flex flex-row gap-[60px] items-center">
                    <div>
                        <ImageButton href="/embedded-controller" img_src="/images/ec.png" alt="Embedded Controller" width=600 height=518 />
                    </div>
                    <div class="flex flex-col items-start" style="flex: 1;">
                        <span
                            class="h2"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Secure Embedded Controller"}
                        </span>
                        <span
                            class="p1"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"A Secure end-to-end Rust-based EC implementation"}
                        </span>
                        <span
                            class="p2"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"ODP EC provides a modern embedded controller firmware written in Rust. Designed for safety and composability, it serves as a foundational layer for secure device management."}
                        </span>
                        <div class="flex flex-col gap-[8px]">
                            <a
                                href="https://opendevicepartnership.github.io/documentation/tracks/embedded_controller/track_overview.html"
                                class="link"
                                style="text-decoration: none;"
                                target="_blank"
                            >
                                <span style="text-decoration: none;">{"→ "}</span>
                                <span style="text-decoration: underline;">{"Read the Secure EC Guide"}</span>
                            </a>
                            <a
                                href="https://github.com/OpenDevicePartnership/embedded-services"
                                class="link"
                                style="text-decoration: none;"
                            >
                                <span style="text-decoration: none;">{"→ "}</span>
                                <span style="text-decoration: underline;">{"View Secure EC (core) on GitHub"}</span>
                            </a>
                        </div>
                    </div>
                </div>
                {/* Row 3 */}
                <div class="flex flex-row gap-[60px] items-center">
                    <div>
                        <ImageButton href="/windows-ec-services" img_src="/images/ec_services.png" alt="EC Services" width=600 height=518 />
                    </div>
                    <div class="flex flex-col items-start" style="flex: 1;">
                        <span
                            class="h2"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Unified Embedded Controller Services"}
                        </span>
                        <span
                            class="p1"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"A standard and secure cross-architecture EC services implementation"}
                        </span>
                        <span
                            class="p2"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"ODP EC Services provides a modern EC services firmware written in Rust. Designed for safety and composability, it serves as a foundational layer for secure EC services on Windows platforms."}
                        </span>
                        <div class="flex flex-col gap-[8px]">
                            <a
                                href="https://opendevicepartnership.github.io/documentation/guide/intro/concepts/EC_Services.html"
                                class="link"
                                style="text-decoration: none;"
                                target="_blank"
                            >
                                <span style="text-decoration: none;">{"→ "}</span>
                                <span style="text-decoration: underline;">{"Read the EC Services Guide"}</span>
                            </a>
                            <a
                                href="https://github.com/OpenDevicePartnership/haf-ec-service"
                                class="link"
                                style="text-decoration: none;"
                            >
                                <span style="text-decoration: none;">{"→ "}</span>
                                <span style="text-decoration: underline;">{"View EC Services on GitHub"}</span>
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
