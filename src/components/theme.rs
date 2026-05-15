//! Theme: provider, toggle, and storage glue.
//!
//! Two themes are supported: `light` (default) and `dark`. The
//! resolved theme is set as the `data-theme` attribute on the
//! `<html>` element, which `style/base.css` keys all of its dark-
//! mode design tokens off.
//!
//! Resolution order on first paint:
//!   1. `localStorage["odp-theme"]` if present (user override),
//!   2. `prefers-color-scheme: dark` from the OS,
//!   3. fallback to `light`.
//!
//! The toggle button cycles `light <-> dark`, persists the choice in
//! `localStorage`, and updates `<html data-theme=...>` so CSS picks
//! up the change without a re-render.

use leptos::ev;
use leptos::prelude::*;
use unocss_classes::uno;
use web_sys::window;

const STORAGE_KEY: &str = "odp-theme";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    fn as_attr(self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    fn opposite(self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}

/// Resolve the initial theme from localStorage / OS preference.
fn read_initial_theme() -> Theme {
    let Some(window) = window() else {
        return Theme::Light;
    };

    if let Ok(Some(storage)) = window.local_storage() {
        if let Ok(Some(value)) = storage.get_item(STORAGE_KEY) {
            return match value.as_str() {
                "dark" => Theme::Dark,
                _ => Theme::Light,
            };
        }
    }

    if let Ok(Some(media)) = window.match_media("(prefers-color-scheme: dark)") {
        if media.matches() {
            return Theme::Dark;
        }
    }

    Theme::Light
}

fn apply_theme(theme: Theme) {
    let Some(window) = window() else { return };
    let Some(document) = window.document() else {
        return;
    };
    let Some(html) = document.document_element() else {
        return;
    };
    let _ = html.set_attribute("data-theme", theme.as_attr());

    if let Ok(Some(storage)) = window.local_storage() {
        let _ = storage.set_item(STORAGE_KEY, theme.as_attr());
    }
}

/// Reactive theme state shared via Leptos context. Components that
/// need to react to theme changes (e.g. theme-aware logos) can call
/// `expect_context::<ThemeState>()`.
#[derive(Clone, Copy)]
pub struct ThemeState(pub RwSignal<Theme>);

/// Provides the [`ThemeState`] context and applies the resolved
/// theme on first mount.
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let theme = RwSignal::new(read_initial_theme());
    provide_context(ThemeState(theme));

    Effect::new(move |_| {
        apply_theme(theme.get());
    });

    children()
}

/// Sun/moon icon button that toggles the theme.
#[component]
pub fn ThemeToggle() -> impl IntoView {
    let ThemeState(theme) = expect_context::<ThemeState>();

    let label = move || match theme.get() {
        Theme::Light => "Switch to dark theme",
        Theme::Dark => "Switch to light theme",
    };

    let icon_class = move || match theme.get() {
        Theme::Light => uno!("i-lucide-moon block w-5 h-5"),
        Theme::Dark => uno!("i-lucide-sun block w-5 h-5"),
    };

    view! {
        <button
            type="button"
            class=uno![
                "inline-flex items-center justify-center w-10 h-10 rounded-md",
                "text-ink-secondary hover:(text-ink-primary bg-surface-sunken)",
                "transition-colors duration-200"
            ]
            aria-label=label
            on:click=move |_: ev::MouseEvent| {
                let next = theme.get_untracked().opposite();
                theme.set(next);
            }
        >
            <span class=icon_class aria-hidden="true"></span>
        </button>
    }
}

/// Direct DOM helper used by `App` to apply the *initial* theme as
/// early as possible -- before Leptos hydration -- to avoid a
/// "theme flash" on first paint. Safe to call from any context.
pub fn paint_initial_theme() {
    let theme = read_initial_theme();
    apply_theme(theme);
}
