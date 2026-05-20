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

/// Click-to-play YouTube video facade.
///
/// On first paint we render *zero* third-party assets: just a
/// styled 16:9 button with our own play affordance and caption.
/// When the user clicks (or presses Enter / Space on the button)
/// we swap in a `youtube-nocookie.com` iframe with `autoplay=1`,
/// so playback starts immediately on their explicit opt-in.
///
/// This protects first-paint perf (the live YouTube player pulls
/// ~hundreds of KB of JS) and avoids any request to Google domains
/// for visitors who never engage with the video.
#[component]
pub fn VideoFacade(
    /// YouTube video id (the `v=...` portion of the watch URL).
    #[prop(into)]
    youtube_id: String,
    /// Accessible title for the video; used as the visible caption,
    /// the button's `aria-label`, and the iframe's `title`.
    #[prop(into)]
    title: String,
) -> impl IntoView {
    let playing = RwSignal::new(false);
    let title_for_button = title.clone();
    let aria_label = format!("Play video: {title}");

    let iframe_html = {
        let id = escape_attr(&youtube_id);
        let title_esc = escape_attr(&title);
        format!(
            "<iframe \
                src=\"https://www.youtube-nocookie.com/embed/{id}?rel=0&autoplay=1\" \
                title=\"{title_esc}\" \
                loading=\"lazy\" \
                referrerpolicy=\"strict-origin-when-cross-origin\" \
                allow=\"autoplay; encrypted-media; fullscreen; picture-in-picture\" \
                allowfullscreen \
                class=\"w-full h-full block border-0\"></iframe>"
        )
    };

    view! {
        <div class=uno!(
            "aspect-video w-full overflow-hidden rounded-lg border border-border-subtle bg-surface-sunken shadow-elev-1"
        )>
            <Show
                when=move || playing.get()
                fallback=move || {
                    let label = aria_label.clone();
                    let caption = title_for_button.clone();
                    view! {
                        <button
                            type="button"
                            aria-label=label
                            on:click=move |_| playing.set(true)
                            class=uno!(
                                "group relative w-full h-full flex flex-col items-center justify-center gap-4 bg-gradient-to-br from-surface-sunken to-surface-raised text-ink-primary cursor-pointer focus-visible:(outline-2 outline-offset-2 outline-ink-accent) transition-colors hover:(bg-gradient-to-br from-surface-raised to-surface-sunken)"
                            )
                        >
                            <span class=uno!(
                                "flex items-center justify-center w-16 h-16 md:w-20 md:h-20 rounded-full bg-ink-accent text-ink-inverse shadow-elev-2 transition-transform group-hover:scale-110"
                            )>
                                <span
                                    class="i-lucide-play w-7 h-7 md:w-9 md:h-9 translate-x-0.5"
                                    aria-hidden="true"
                                ></span>
                            </span>
                            <span class=uno!(
                                "text-caption font-mono uppercase tracking-[0.18em] text-ink-secondary"
                            )>"Watch the intro"</span>
                            <span class=uno!(
                                "text-h3 font-semibold max-w-[24ch] text-center px-6"
                            )>{caption}</span>
                        </button>
                    }
                }
            >
                {
                    let html = iframe_html.clone();
                    view! { <div class=uno!("w-full h-full") inner_html=html></div> }
                }
            </Show>
        </div>
    }
}

/// Minimal HTML attribute escaper for values we render via
/// `inner_html` (the YouTube id and accessible title on
/// [`VideoFacade`]). Both are caller-controlled strings, so we
/// escape the four characters that have meaning inside a
/// double-quoted attribute value.
fn escape_attr(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
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
        // YouTube rounded-rect with play triangle.
        "youtube" => {
            "M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12z"
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
