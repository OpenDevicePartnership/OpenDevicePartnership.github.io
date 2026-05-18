//! ODP website root.
//!
//! The page chrome (sticky `NavBar`, `Footer`, theme provider) is
//! rendered once around the route tree. Individual pages render
//! only their content; vertical rhythm is handled by the
//! `<Section>` primitives inside each page.

use crate::components::nav::{Footer, NavBar};
use crate::components::theme::ThemeProvider;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

pub mod components;
mod data;
mod pages;

use crate::pages::announcements::{AnnouncementDetailPage, AnnouncementsPage};
use crate::pages::boot_firmware::BootFirmware;
use crate::pages::community::Community;
use crate::pages::embedded_controller::EmbeddedController;
use crate::pages::getting_started::GettingStarted;
use crate::pages::home::Home;
use crate::pages::not_found::NotFoundPage;
use crate::pages::projects::Projects;
use crate::pages::team_ec::TeamEC;
use crate::pages::team_ec_services::TeamECServices;
use crate::pages::team_patina::TeamPatina;
use crate::pages::unified_ec_services::WindowsEcServices;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" />
        <Title text="Open Device Partnership" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <Meta
            name="description"
            content="Open Device Partnership: an open collaboration for secure, modern device firmware. Built in the open, by the people who maintain it."
        />

        <ThemeProvider>
            <Router base="/">
                <div class="flex flex-col min-h-screen w-full bg-surface-page text-ink-primary">
                    <NavBar />
                    <main class="flex-1 w-full">
                        <Routes fallback=NotFoundPage>
                            <Route path=path!("/") view=Home />
                            <Route path=path!("/projects") view=Projects />
                            <Route path=path!("/getting-started") view=GettingStarted />
                            <Route path=path!("/community") view=Community />
                            <Route path=path!("/announcements") view=AnnouncementsPage />
                            <Route path=path!("/announcements/:slug") view=AnnouncementDetailPage />
                            <Route path=path!("/boot-firmware") view=BootFirmware />
                            <Route path=path!("/embedded-controller") view=EmbeddedController />
                            <Route path=path!("/windows-ec-services") view=WindowsEcServices />
                            <Route path=path!("/team-patina") view=TeamPatina />
                            <Route path=path!("/team-ec") view=TeamEC />
                            <Route path=path!("/team-ec-services") view=TeamECServices />
                        </Routes>
                    </main>
                    <Footer />
                </div>
            </Router>
        </ThemeProvider>
    }
}
