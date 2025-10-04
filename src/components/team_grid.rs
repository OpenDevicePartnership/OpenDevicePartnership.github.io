use leptos::html::*;
use leptos::prelude::*;

#[derive(Clone)]
pub struct TeamMember {
    pub first_name: &'static str,
    pub last_name: &'static str,
    pub role: &'static str,
    pub github_username: &'static str,
    pub github_url: &'static str,
    pub image_url: &'static str,
}

#[component]
pub fn TeamGrid(#[prop(into)] members: Vec<TeamMember>) -> impl IntoView {
    view! {
        <div class="background_primary">
            <div class="grid-container px-2 md:px-[120px]" style="padding-bottom: 120px;">
                {members.into_iter().map(|member| {
                    view! {
                        <div>
                            <img class="member-image" src=member.image_url alt="Profile Picture" />
                            <div class="h3_mobile md:h3">{format!("{} {}", member.first_name, member.last_name)}</div>
                            <div class="p2_mobile md:p2">{member.role}</div>
                            <div class="p2_mobile md:p2">
                                {"GitHub: "}
                                <a class="link" href=member.github_url target="_blank">{member.github_username}</a>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
