use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::main::Main;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
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
                <Footer />
            </div>
        </ErrorBoundary>
    }
}
