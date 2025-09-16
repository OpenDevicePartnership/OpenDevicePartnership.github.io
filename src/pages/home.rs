use crate::components::documentation_training::{DocLink, DocumentationTraining};
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::main::Main;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    // Documentation links for the DocumentationTraining section
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
                <Main />
                <DocumentationTraining links=links />
                <Footer />
            </div>
        </ErrorBoundary>
    }
}
