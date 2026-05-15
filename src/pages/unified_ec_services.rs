use crate::components::controls::TagTone;
use crate::components::project_layout::ProjectLayout;
use crate::data::projects::EC_SERVICES;
use leptos::prelude::*;

#[component]
pub fn WindowsEcServices() -> impl IntoView {
    view! { <ProjectLayout project=&EC_SERVICES tone=TagTone::Services /> }
}
