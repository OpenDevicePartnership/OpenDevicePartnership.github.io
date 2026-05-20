use crate::components::controls::TagTone;
use crate::components::project_layout::ProjectLayout;
use crate::data::projects::PATINA;
use leptos::prelude::*;

#[component]
pub fn BootFirmware() -> impl IntoView {
    view! { <ProjectLayout project=&PATINA tone=TagTone::Patina /> }
}
