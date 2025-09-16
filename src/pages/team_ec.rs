use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::team_grid::{TeamGrid, TeamMember};

use leptos::prelude::*;

fn create_team() -> Vec<TeamMember> {
    vec![
        TeamMember {
            first_name: "Jerry",
            last_name: "Xie",
            role: "Team leader",
            github_username: "jerrysxie",
            github_url: "https://github.com/jerrysxie",
            image_url: "https://github.com/jerrysxie.png?size=200",
        },
        TeamMember {
            first_name: "Felipe",
            last_name: "Balbi",
            role: "",
            github_username: "felipebalbi",
            github_url: "https://github.com/felipebalbi",
            image_url: "https://github.com/felipebalbi.png?size=200",
        },
        TeamMember {
            first_name: "Robert",
            last_name: "Zieba",
            role: "",
            github_username: "RobertZ2011",
            github_url: "https://github.com/RobertZ2011",
            image_url: "https://github.com/RobertZ2011.png?size=200",
        },
        TeamMember {
            first_name: "Matteo",
            last_name: "Tullo",
            role: "",
            github_username: "tullom",
            github_url: "https://github.com/tullom",
            image_url: "https://github.com/tullom.png?size=200",
        },
        TeamMember {
            first_name: "Kurtis",
            last_name: "Dinelle",
            role: "",
            github_username: "kurtjd",
            github_url: "https://github.com/kurtjd",
            image_url: "https://github.com/kurtjd.png?size=200",
        },
        TeamMember {
            first_name: "Jimi",
            last_name: "Huard",
            role: "",
            github_username: "JamesHuard",
            github_url: "https://github.com/JamesHuard",
            image_url: "https://github.com/JamesHuard.png?size=200",
        },
        TeamMember {
            first_name: "Adam",
            last_name: "Sasine",
            role: "",
            github_username: "asasine",
            github_url: "https://github.com/asasine",
            image_url: "https://github.com/asasine.png?size=200",
        },
        TeamMember {
            first_name: "Billy",
            last_name: "Price",
            role: "",
            github_username: "williampMSFT",
            github_url: "https://github.com/williampMSFT",
            image_url: "https://github.com/williampMSFT.png?size=200",
        },
    ]
}

#[component]
pub fn TeamEC() -> impl IntoView {
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
                    class="background_primary"
                    style="padding: 120px; position: relative;"
                >
                    <div style="position: absolute; left: 0; top: 0;">
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
                                {"Meet the team"}
                            </span>
                        </div>
                        {/* Right Column */}
                        <div class="flex flex-col items-start" style="width: 600px;">
                            {/* Team Introduction */}
                            <span
                                class="mono"
                                style="
                                    display: block;
                                    text-align: left;
                                "
                            >
                                {"Secure EC team"}
                            </span>
                            <span
                                class="p1"
                                style="
                                    display: block;
                                    text-align: left;
                                "
                            >
                                {"Developing and managing secure EC internals"}
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
