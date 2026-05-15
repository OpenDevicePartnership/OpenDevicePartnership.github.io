use crate::data::teams::ec_team;
use crate::pages::team_page::TeamPage;
use leptos::prelude::*;

#[component]
pub fn TeamEC() -> impl IntoView {
    view! {
        <TeamPage
            eyebrow="Secure EC working group"
            title="Embedded controllers, hardened."
            description="Developing and managing secure EC internals."
            members=ec_team()
        />
    }
}
