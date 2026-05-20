//! Layout primitives -- `Container`, `Section`, `Stack`, `Cluster`,
//! `Grid`, and `Surface`. Each one is a thin wrapper around a
//! semantic element with token-driven spacing classes.
//!
//! The whole point: pages compose these primitives instead of
//! hand-rolling `flex`/`grid`/`px-`/`py-` strings. Spacing scale and
//! max-content-width caps are decided in *one* place and the rest of
//! the site inherits them.

use leptos::prelude::*;
use unocss_classes::uno;

/// Centred max-width content wrapper.
///
/// Caps content to 1280px on extreme widths and pads with the fluid
/// `px-section-x` gutter so it never touches the viewport edge.
#[component]
pub fn Container(
    /// Max-width preset. `Wide` caps at 1280px (default), `Narrow`
    /// caps at 760px for prose-heavy blocks (announcement detail,
    /// long form copy).
    #[prop(optional)]
    width: ContainerWidth,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let cap = match width {
        ContainerWidth::Wide => uno!("max-w-[1280px]"),
        ContainerWidth::Narrow => uno!("max-w-[760px]"),
    };
    let final_class = format!("{cap} mx-auto w-full px-section-x {class}");
    view! { <div class=final_class>{children()}</div> }
}

#[derive(Clone, Copy, Default)]
pub enum ContainerWidth {
    #[default]
    Wide,
    Narrow,
}

/// Vertical-rhythm section. Renders a `<section>` with the fluid
/// `py-section-y` token and an optional surface tone.
#[component]
pub fn Section(
    #[prop(optional)] surface: SectionSurface,
    /// `id` on the underlying `<section>`, used for in-page anchors.
    #[prop(into, optional)]
    id: Option<String>,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let surface_class = match surface {
        SectionSurface::Page => uno!("bg-surface-page text-ink-primary"),
        SectionSurface::Sunken => uno!("bg-surface-sunken text-ink-primary"),
        SectionSurface::Inverse => uno!("bg-surface-inverse text-ink-inverse"),
    };
    let final_class = format!("w-full py-section-y {surface_class} {class}");
    view! {
        <section id=id class=final_class>
            {children()}
        </section>
    }
}

#[derive(Clone, Copy, Default)]
pub enum SectionSurface {
    #[default]
    Page,
    Sunken,
    Inverse,
}

/// Vertical stack with token-driven gap. Always lays children out
/// in column direction; for horizontal grouping use [`Cluster`].
#[component]
pub fn Stack(
    #[prop(optional, default = StackGap::Md)] gap: StackGap,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let gap_class = match gap {
        StackGap::Xs => uno!("gap-2"),
        StackGap::Sm => uno!("gap-4"),
        StackGap::Md => uno!("gap-6"),
        StackGap::Lg => uno!("gap-10"),
        StackGap::Xl => uno!("gap-16"),
    };
    let final_class = format!("flex flex-col {gap_class} {class}");
    view! { <div class=final_class>{children()}</div> }
}

#[derive(Clone, Copy, Default)]
pub enum StackGap {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

/// Horizontal flex-wrap grouping. Used for chips, navigation
/// clusters, and footer link rows.
#[component]
pub fn Cluster(
    #[prop(optional, default = StackGap::Md)] gap: StackGap,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let gap_class = match gap {
        StackGap::Xs => uno!("gap-2"),
        StackGap::Sm => uno!("gap-3"),
        StackGap::Md => uno!("gap-5"),
        StackGap::Lg => uno!("gap-8"),
        StackGap::Xl => uno!("gap-12"),
    };
    let final_class = format!("flex flex-wrap items-center {gap_class} {class}");
    view! { <div class=final_class>{children()}</div> }
}

/// Auto-fit responsive grid. `min` is the minimum column width; the
/// grid wraps to as many columns as fit.
#[component]
pub fn Grid(
    /// Minimum column width. Pre-set buckets keep call sites
    /// declarative.
    #[prop(optional, default = GridMin::Md)]
    min: GridMin,
    #[prop(optional, default = StackGap::Lg)] gap: StackGap,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let style = match min {
        GridMin::Sm => "grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));",
        GridMin::Md => "grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));",
    };
    let gap_class = match gap {
        StackGap::Xs => uno!("gap-3"),
        StackGap::Sm => uno!("gap-5"),
        StackGap::Md => uno!("gap-6"),
        StackGap::Lg => uno!("gap-10"),
        StackGap::Xl => uno!("gap-16"),
    };
    let final_class = format!("grid {gap_class} {class}");
    view! {
        <div class=final_class style=style>
            {children()}
        </div>
    }
}

#[derive(Clone, Copy, Default)]
pub enum GridMin {
    Sm,
    #[default]
    Md,
}

/// A tactile card-shaped surface with elevation, border, and radius.
#[component]
pub fn Surface(
    #[prop(optional)] tone: SurfaceTone,
    #[prop(optional, default = SurfaceElevation::E1)] elevation: SurfaceElevation,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let tone_class = match tone {
        SurfaceTone::Raised => uno!("bg-surface-raised border border-border-subtle"),
        SurfaceTone::Outline => uno!("bg-transparent border border-border-strong"),
    };
    let elev_class = match elevation {
        SurfaceElevation::Flat => "",
        SurfaceElevation::E1 => "shadow-elev-1",
    };
    let final_class = format!("rounded-lg p-6 md:p-8 {tone_class} {elev_class} {class}");
    view! { <div class=final_class>{children()}</div> }
}

#[derive(Clone, Copy, Default)]
pub enum SurfaceTone {
    #[default]
    Raised,
    Outline,
}

#[derive(Clone, Copy, Default)]
pub enum SurfaceElevation {
    Flat,
    #[default]
    E1,
}
