use crate::data::teams::ec_services_team;
use crate::pages::team_page::TeamPage;
use leptos::prelude::*;

#[component]
pub fn TeamECServices() -> impl IntoView {
    view! {
        <TeamPage
            eyebrow="Unified EC Services working group"
            title="One interface across devices."
            description="Designing and managing implementation of a unified EC services interface."
            members=ec_services_team()
        />
    }
}
