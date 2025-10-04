use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::team_grid::{TeamGrid, TeamMember};

use leptos::prelude::*;

fn create_team() -> Vec<TeamMember> {
    vec![
        TeamMember {
            first_name: "Phil",
            last_name: "Weber",
            role: "Team leader",
            github_username: "philgweber",
            github_url: "https://github.com/philgweber",
            image_url: "https://github.com/philgweber.png?size=200",
        },
        TeamMember {
            first_name: "Dylan",
            last_name: "Knutson",
            role: "Team leader",
            github_username: "dymk",
            github_url: "https://github.com/dymk",
            image_url: "https://github.com/dymk.png?size=200",
        },
    ]
}

#[component]
pub fn TeamECServices() -> impl IntoView {
    let team = create_team();

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
                            <span
                                class="mono"
                                style="
                                    display: block;
                                    text-align: left;
                                "
                            >
                                {"Unified EC services team"}
                            </span>
                            <span
                                class="p1_mobile md:p1"
                                style="
                                    display: block;
                                    text-align: left;
                                "
                            >
                                {"Designing and managing implementation of a unified EC Services interface"}
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
