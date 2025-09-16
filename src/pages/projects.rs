use crate::components::documentation_training::{DocLink, DocumentationTraining};
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::projects_component::ProjectsComponent;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Projects() -> impl IntoView {
    let links = vec![
        DocLink {
            href: "https://opendevicepartnership.github.io/documentation/guide/why/why.html",
            title: "Why ODP?",
            external: true,
        },
        DocLink {
            href: "https://opendevicepartnership.github.io/documentation/guide/intro/getting_started.html",
            title: "Getting Started with ODP",
            external: true,
        },
        DocLink {
            href: "https://opendevicepartnership.github.io/documentation/guide/intro/welcome.html",
            title: "Tutorials",
            external: true,
        },
        DocLink {
            href: "https://opendevicepartnership.github.io/documentation/guide/specs/specifications.html",
            title: "Specifications",
            external: true,
        },
        DocLink {
            href: "/community",
            title: "Contributing to ODP",
            external: false,
        },
    ];

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
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

            <div class="w-full min-h-screen" style="overflow-x: auto;">
                <Header />
                <ProjectsComponent />
                <DocumentationTraining links=links />
                <Footer />
            </div>
        </ErrorBoundary>
    }
}
