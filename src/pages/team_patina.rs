use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::team_grid::{TeamGrid, TeamMember};

use leptos::prelude::*;

fn create_team() -> Vec<TeamMember> {
    vec![
        TeamMember {
            first_name: "Michael",
            last_name: "Kubacki",
            role: "Project Lead and System Management Mode (MM)",
            github_username: "makubacki",
            github_url: "https://github.com/makubacki",
            image_url: "https://github.com/makubacki.png?size=200",
        },
        TeamMember {
            first_name: "Ron",
            last_name: "Gurr",
            role: "Partner Engagement Lead",
            github_username: "rogurr",
            github_url: "https://github.com/rogurr",
            image_url: "https://github.com/rogurr.png?size=200",
        },
        TeamMember {
            first_name: "Oliver",
            last_name: "Smith-Denny",
            role: "Memory Protections and Paging",
            github_username: "os-d",
            github_url: "https://github.com/os-d",
            image_url: "https://github.com/os-d.png?size=200",
        },
        TeamMember {
            first_name: "Vineel",
            last_name: "Kovvuri",
            role: "CPU",
            github_username: "vineelko",
            github_url: "https://github.com/vineelko",
            image_url: "https://github.com/vineelko.png?size=200",
        },
        TeamMember {
            first_name: "Sherry",
            last_name: "Fan",
            role: "Patina Readiness Tool",
            github_username: "berlin-with0ut-return",
            github_url: "https://github.com/berlin-with0ut-return",
            image_url: "https://github.com/berlin-with0ut-return.png?size=200",
        },
        TeamMember {
            first_name: "Chris",
            last_name: "Fernald",
            role: "Debugger",
            github_username: "cfernald",
            github_url: "https://github.com/cfernald",
            image_url: "https://github.com/cfernald.png?size=200",
        },
        TeamMember {
            first_name: "John",
            last_name: "Schock",
            role: "Memory Allocator",
            github_username: "joschock",
            github_url: "https://github.com/joschock",
            image_url: "https://github.com/joschock.png?size=200",
        },
        TeamMember {
            first_name: "Joey",
            last_name: "Vagedes",
            role: "Component Infrastructure",
            github_username: "Javagedes",
            github_url: "https://github.com/Javagedes",
            image_url: "https://github.com/Javagedes.png?size=200",
        },
        TeamMember {
            first_name: "Mathieu",
            last_name: "Gravel",
            role: "Performance",
            github_username: "magravel",
            github_url: "https://github.com/magravel",
            image_url: "https://github.com/magravel.png?size=200",
        },
    ]
}

#[component]
pub fn TeamPatina() -> impl IntoView {
    let team = create_team();

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

            <div class="w-full min-h-screen overflow-x-hidden">
                <Header />
                <div
                    class="background_primary px-2 md:px-[120px] py-4 md:py-[120px]"
                    style="position: relative;"
                >
                    {/* Back Button: above content on mobile, absolutely positioned on desktop */}
                    <div class="block md:hidden mb-4">
                        <a
                            href="javascript:history.back()"
                            class="block"
                            style="margin: 0; padding: 0;"
                        >
                            <picture>
                                <source srcset="/images/dark/backbutton.svg" media="(prefers-color-scheme: dark)" />
                                <img
                                    src="/images/light/backbutton.svg"
                                    alt="Back"
                                    style="margin: 0; padding: 0; display: block;"
                                />
                            </picture>
                        </a>
                    </div>
                    <div class="hidden md:block" style="position: absolute; left: 0; top: 0;">
                        <a
                            href="javascript:history.back()"
                            class="block"
                            style="margin: 0; padding: 0;"
                        >
                            <picture>
                                <source srcset="/images/dark/backbutton.svg" media="(prefers-color-scheme: dark)" />
                                <img
                                    src="/images/light/backbutton.svg"
                                    alt="Back"
                                    style="margin: 0; padding: 0; display: block;"
                                />
                            </picture>
                        </a>
                    </div>
                    <div class="flex flex-col md:flex-row gap-[80px] items-start">
                        {/* Left Column */}
                        <div class="flex flex-col items-start w-full md:w-[700px]">
                            <span
                                class="h1_mobile md:h1"
                                style="
                                    display: block;
                                    text-align: left;
                                "
                            >
                                {"Meet the team"}
                            </span>
                        </div>
                        {/* Right Column */}
                        <div class="flex flex-col items-start w-full md:w-[600px]">
                            {/* Team Introduction */}
                            <span
                                class="mono"
                                style="
                                    display: block;
                                    text-align: left;
                                "
                            >
                                {"Patina team"}
                            </span>
                            <span
                                class="p1_mobile md:p1"
                                style="
                                    display: block;
                                    text-align: left;
                                "
                            >
                                {"Developing and managing development of a new modern UEFI"}
                            </span>
                        </div>
                    </div>
                </div>
                <TeamGrid members=team />
                <Footer />
            </div>

        </ErrorBoundary>
    }
}
