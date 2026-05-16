//! Media primitives: theme-aware logo and icon helpers.

use leptos::prelude::*;
use unocss_classes::uno;

/// Theme-aware ODP logo. Uses `<picture>` so the browser swaps
/// between the light and dark SVG without a JS round trip.
#[component]
pub fn Logo(#[prop(into, optional)] class: String) -> impl IntoView {
    let class = if class.is_empty() {
        uno!("h-8 md:h-10 w-auto").to_string()
    } else {
        class
    };
    view! {
        <>
            <img
                src="/images/light/odplogo.svg"
                alt="Open Device Partnership"
                class={
                    let class = class.clone();
                    format!("{class} logo-light")
                }
            />

            <img
                src="/images/dark/odplogo.svg"
                alt="Open Device Partnership"
                class=move || format!("{class} logo-dark")
            />
        </>
    }
}

/// UnoCSS preset-icons handle. Pass any lucide icon name (without
/// the `i-lucide-` prefix), e.g. `name="shield"` to render the
/// `i-lucide-shield` icon.
#[component]
pub fn Icon(#[prop(into)] name: String, #[prop(into, optional)] class: String) -> impl IntoView {
    let class = format!("i-lucide-{name} {class}");
    view! { <span class=class aria-hidden="true"></span> }
}

/// External-link / GitHub / Zulip / Discord brand iconography. We
/// keep these as inline SVGs because lucide's brand glyphs aren't
/// quite right and the assets are cheap.
#[component]
pub fn BrandIcon(#[prop(into)] name: String, #[prop(into, optional)] class: String) -> impl IntoView {
    let class = if class.is_empty() {
        uno!("w-5 h-5 text-current").to_string()
    } else {
        class
    };
    let path: &'static str = match name.as_str() {
        // Octocat-style GitHub mark.
        "github" => {
            "M12 .5C5.65.5.5 5.65.5 12c0 5.08 3.29 9.39 7.86 10.91.58.11.79-.25.79-.55 0-.27-.01-1.16-.02-2.1-3.2.7-3.88-1.36-3.88-1.36-.52-1.34-1.28-1.7-1.28-1.7-1.05-.72.08-.71.08-.71 1.16.08 1.77 1.19 1.77 1.19 1.03 1.77 2.71 1.26 3.37.96.1-.75.4-1.26.73-1.55-2.55-.29-5.23-1.28-5.23-5.69 0-1.26.45-2.28 1.19-3.08-.12-.29-.52-1.46.11-3.05 0 0 .97-.31 3.18 1.18a11.04 11.04 0 0 1 5.79 0c2.21-1.49 3.18-1.18 3.18-1.18.63 1.59.23 2.76.11 3.05.74.8 1.19 1.82 1.19 3.08 0 4.42-2.69 5.4-5.25 5.68.41.35.78 1.04.78 2.1 0 1.52-.01 2.74-.01 3.11 0 .3.21.66.8.55C20.21 21.39 23.5 17.08 23.5 12 23.5 5.65 18.35.5 12 .5z"
        }
        // Zulip Z mark.
        "zulip" => {
            "M3 5.5C3 4.12 4.12 3 5.5 3h13A2.5 2.5 0 0 1 21 5.5c0 .68-.27 1.31-.73 1.78l-9.5 9.72H18.5a2.5 2.5 0 1 1 0 5h-13A2.5 2.5 0 0 1 3 19.5c0-.68.27-1.31.73-1.78l9.5-9.72H5.5A2.5 2.5 0 0 1 3 5.5z"
        }
        // Discord controller.
        "discord" => {
            "M19.62 5.34a17.4 17.4 0 0 0-4.32-1.34c-.04 0-.08.02-.1.06-.18.32-.39.74-.53 1.07a16.07 16.07 0 0 0-4.92 0c-.15-.34-.36-.75-.55-1.07a.1.1 0 0 0-.1-.06 17.32 17.32 0 0 0-4.32 1.34.09.09 0 0 0-.04.04C2 9.46 1.32 13.46 1.66 17.42c0 .03.02.06.04.08a17.5 17.5 0 0 0 5.27 2.66.1.1 0 0 0 .11-.04c.4-.55.77-1.13 1.08-1.74a.1.1 0 0 0-.06-.14 11.5 11.5 0 0 1-1.65-.79.1.1 0 0 1 0-.16c.11-.08.22-.17.33-.26a.1.1 0 0 1 .1-.01c3.46 1.58 7.21 1.58 10.63 0a.1.1 0 0 1 .1.01c.11.09.22.18.33.26a.1.1 0 0 1 0 .16c-.53.31-1.08.57-1.65.79a.1.1 0 0 0-.05.14c.32.61.69 1.19 1.08 1.74a.1.1 0 0 0 .11.04 17.45 17.45 0 0 0 5.28-2.66.1.1 0 0 0 .04-.08c.4-4.57-.68-8.54-2.88-12.04a.07.07 0 0 0-.04-.04zM8.52 14.99c-1.04 0-1.9-.96-1.9-2.13s.84-2.13 1.9-2.13c1.07 0 1.92.97 1.9 2.13 0 1.18-.84 2.13-1.9 2.13zm7.05 0c-1.04 0-1.9-.96-1.9-2.13s.84-2.13 1.9-2.13c1.07 0 1.92.97 1.9 2.13 0 1.18-.84 2.13-1.9 2.13z"
        }
        _ => "",
    };
    view! {
        <svg
            class=class
            viewBox="0 0 24 24"
            fill="currentColor"
            xmlns="http://www.w3.org/2000/svg"
            aria-hidden="true"
        >
            <path d=path />
        </svg>
    }
}
