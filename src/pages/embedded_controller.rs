use crate::components::controls::TagTone;
use crate::components::project_layout::ProjectLayout;
use crate::data::projects::EMBEDDED_CONTROLLER;
use leptos::prelude::*;

#[component]
pub fn EmbeddedController() -> impl IntoView {
    view! { <ProjectLayout project=&EMBEDDED_CONTROLLER tone=TagTone::Ec /> }
}
