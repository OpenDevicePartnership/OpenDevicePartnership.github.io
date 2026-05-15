//! Typography primitives.
//!
//! `Display` is the editorial serif used in hero blocks and section
//! openers. `Heading` covers `h1`/`h2`/`h3` with the sans body
//! face. `Lead` is the tightened intro paragraph; `Body` is regular
//! prose. `Mono` is the monospace eyebrow label (uppercase) used as
//! a section tag.

use leptos::prelude::*;
use unocss_classes::uno;

/// Editorial serif display heading. Renders as `<h1>` by default.
#[component]
pub fn Display(
    #[prop(optional, default = DisplaySize::Lg)] size: DisplaySize,
    /// Element tag. Defaults to `h1` so a hero gets a real page
    /// heading; pass `h2` for sub-hero displays.
    #[prop(into, optional, default = "h1".to_string())]
    tag: String,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let size_class = match size {
        DisplaySize::Xl => uno!("text-display-xl"),
        DisplaySize::Lg => uno!("text-display"),
        DisplaySize::Md => uno!("text-h1"),
    };
    let final_class = format!(
        "{size_class} {} text-ink-primary tracking-tight {class}",
        uno!("font-display font-medium")
    );
    match tag.as_str() {
        "h2" => view! { <h2 class=final_class>{children()}</h2> }.into_any(),
        "h3" => view! { <h3 class=final_class>{children()}</h3> }.into_any(),
        _ => view! { <h1 class=final_class>{children()}</h1> }.into_any(),
    }
}

#[derive(Clone, Copy, Default)]
pub enum DisplaySize {
    Xl,
    #[default]
    Lg,
    Md,
}

/// Sans-serif heading. Maps `level` to both the semantic tag and
/// the design-token type size.
#[component]
pub fn Heading(
    #[prop(optional, default = HeadingLevel::H2)] level: HeadingLevel,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let (size_class, font_class) = match level {
        HeadingLevel::H1 => (uno!("text-h1"), uno!("font-semibold")),
        HeadingLevel::H2 => (uno!("text-h2"), uno!("font-semibold")),
        HeadingLevel::H3 => (uno!("text-h3"), uno!("font-medium")),
    };
    let final_class = format!("{size_class} {font_class} text-ink-primary tracking-tight {class}");
    match level {
        HeadingLevel::H1 => view! { <h1 class=final_class>{children()}</h1> }.into_any(),
        HeadingLevel::H2 => view! { <h2 class=final_class>{children()}</h2> }.into_any(),
        HeadingLevel::H3 => view! { <h3 class=final_class>{children()}</h3> }.into_any(),
    }
}

#[derive(Clone, Copy, Default)]
pub enum HeadingLevel {
    H1,
    #[default]
    H2,
    H3,
}

/// Tightened intro paragraph in the lead size.
#[component]
pub fn Lead(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let final_class = format!(
        "text-lead font-normal text-ink-secondary {} {class}",
        uno!("max-w-[60ch]")
    );
    view! { <p class=final_class>{children()}</p> }
}

/// Regular body paragraph.
#[component]
pub fn Body(
    #[prop(optional, default = BodyTone::Primary)] tone: BodyTone,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let tone_class = match tone {
        BodyTone::Primary => uno!("text-ink-primary"),
        BodyTone::Secondary => uno!("text-ink-secondary"),
        BodyTone::Muted => uno!("text-ink-muted"),
    };
    let final_class = format!("text-body font-normal {tone_class} {class}");
    view! { <p class=final_class>{children()}</p> }
}

#[derive(Clone, Copy, Default)]
pub enum BodyTone {
    #[default]
    Primary,
    Secondary,
    Muted,
}

/// Uppercase monospace eyebrow label. Used as section tags and as
/// the small label above a hero (e.g. "PROJECT").
#[component]
pub fn Eyebrow(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let final_class = format!("text-caption font-mono uppercase tracking-[0.18em] text-ink-muted {class}",);
    view! { <span class=final_class>{children()}</span> }
}

/// Inline monospace span (no transformation, body size).
#[component]
pub fn Mono(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let final_class = format!("font-mono text-small {class}");
    view! { <span class=final_class>{children()}</span> }
}
