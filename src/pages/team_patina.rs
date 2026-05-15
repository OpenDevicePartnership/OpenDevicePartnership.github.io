use crate::data::teams::patina_team;
use crate::pages::team_page::TeamPage;
use leptos::prelude::*;

#[component]
pub fn TeamPatina() -> impl IntoView {
    view! {
        <TeamPage
            eyebrow="Patina working group"
            title="Boot firmware, in the open."
            description="Developing and managing a new modern UEFI."
            members=patina_team()
        />
    }
}
