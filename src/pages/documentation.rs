use crate::components::header::Header;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Documentation() -> impl IntoView {
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

            <main>
                <Header />
                <div class="h-36 bg-white" />
                <div class="bg-gradient-to-tl bg_white text-black font-mono flex flex-col min-h-screen">
                    <div class="flex flex-row-reverse flex-wrap m-auto">
                        <h1 class="text-9xl font-bold font-sans">Coming soon...</h1>
                    </div>
                </div>
            </main>
        </ErrorBoundary>
    }
}
