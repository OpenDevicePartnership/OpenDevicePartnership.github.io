use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::landing_page::LandingPage;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn GettingStarted() -> impl IntoView {
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
                <LandingPage />
                <Footer />
            </div>
        </ErrorBoundary>
    }
}
