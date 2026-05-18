//! Interactive controls: `Button`, `LinkButton`, `Tag`, `Badge`,
//! and `ArrowLink`.

use leptos::ev;
use leptos::prelude::*;
use unocss_classes::uno;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Primary,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonSize {
    #[default]
    Md,
}

fn button_classes(variant: ButtonVariant, size: ButtonSize) -> String {
    let variant_class = match variant {
        ButtonVariant::Primary => {
            uno!(
                "bg-accent text-accent-ink border border-accent",
                "hover:(bg-accent-strong border-accent-strong)",
                "shadow-elev-1"
            )
        }
    };
    let size_class = match size {
        ButtonSize::Md => uno!("px-5 py-3 text-body"),
    };
    format!(
        "{} {} {} {}",
        uno!("inline-flex items-center justify-center gap-2 rounded-md font-medium"),
        uno!("transition-colors duration-200 cursor-pointer"),
        variant_class,
        size_class
    )
}

/// Imperative `<button>`.
#[component]
pub fn Button(
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let final_class = format!("{} {class}", button_classes(variant, size));
    view! {
        <button
            type="button"
            class=final_class
            on:click=move |e| {
                if let Some(cb) = on_click {
                    cb.run(e);
                }
            }
        >
            {children()}
        </button>
    }
}

/// `<a>` styled like a `Button`. Used for CTAs that navigate.
#[component]
pub fn LinkButton(
    #[prop(into)] href: String,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    #[prop(optional, default = false)] external: bool,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let final_class = format!("{} {class}", button_classes(variant, size));
    let target = if external { Some("_blank") } else { None };
    let rel = if external { Some("noopener noreferrer") } else { None };
    view! {
        <a href=href class=final_class target=target rel=rel>
            {children()}
        </a>
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum TagTone {
    #[default]
    Neutral,
    Accent,
    Trust,
    Patina,
    Ec,
    Services,
}

/// Pill-shaped label. Used as project tags and section eyebrows.
#[component]
pub fn Tag(
    #[prop(optional)] tone: TagTone,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let tone_class = match tone {
        TagTone::Neutral => uno!("bg-surface-sunken text-ink-secondary"),
        TagTone::Accent => uno!("bg-accent-soft text-ink-accent"),
        TagTone::Trust => uno!("bg-trust-soft text-trust"),
        TagTone::Patina => "bg-[var(--color-project-patina)]/15 text-[var(--color-project-patina-ink)]".to_string(),
        TagTone::Ec => "bg-[var(--color-project-ec)]/15 text-[var(--color-project-ec-ink)]".to_string(),
        TagTone::Services => {
            "bg-[var(--color-project-services)]/15 text-[var(--color-project-services-ink)]".to_string()
        }
    };
    let final_class = format!(
        "{} {tone_class} {class}",
        uno!("inline-flex items-center px-3 py-1 rounded-pill text-caption font-mono uppercase tracking-wider")
    );
    view! { <span class=final_class>{children()}</span> }
}

/// Inline link with an arrow affordance and underline-on-hover.
#[component]
pub fn ArrowLink(
    #[prop(into)] href: String,
    #[prop(optional, default = false)] external: bool,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let target = if external { Some("_blank") } else { None };
    let rel = if external { Some("noopener noreferrer") } else { None };
    let final_class = format!(
        "{} {class}",
        uno!(
            "group inline-flex items-baseline gap-2 text-ink-primary",
            "border-b border-transparent hover:border-current",
            "transition-colors duration-200"
        )
    );
    view! {
        <a href=href class=final_class target=target rel=rel>
            <span>{children()}</span>
            <span
                class=uno![
                    "i-lucide-arrow-up-right w-4 h-4 text-ink-muted",
                    "group-hover:(text-accent translate-x-0.5 translate-y--0.5)",
                    "transition-transform duration-200"
                ]
                aria-hidden="true"
            ></span>
        </a>
    }
}

/// Plain inline link (underline on hover).
#[component]
pub fn InlineLink(
    #[prop(into)] href: String,
    #[prop(optional, default = false)] external: bool,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let target = if external { Some("_blank") } else { None };
    let rel = if external { Some("noopener noreferrer") } else { None };
    let final_class = format!(
        "{} {class}",
        uno!("text-ink-accent underline decoration-from-font underline-offset-4 hover:decoration-2")
    );
    view! {
        <a href=href class=final_class target=target rel=rel>
            {children()}
        </a>
    }
}
