// UnoCSS configuration for the ODP website redesign.
//
// What this does
// --------------
// * Scans Rust source for `uno![...]` macro invocations and class
//   string literals. The Rust `unocss-classes` crate transforms the
//   variant-group syntax (e.g. `hover:(bg-x font-y)`) at compile
//   time, but the *resulting* utility names still need to be in
//   the source for the CLI to pick them up. The crate documents
//   that `uno!` literals are statically extractable.
// * Emits `style/uno.generated.css`, which is loaded as a regular
//   stylesheet from `index.html` via Trunk's `data-trunk rel="css"`.
// * Uses preset-wind3 for the Tailwind-compatible utility set,
//   transformer-variant-group to mirror the macro's grouping, and
//   preset-icons for inline SVG icons (lucide collection).
//
// Theme
// -----
// All token values live in CSS custom properties declared in
// `style/base.css` and surfaced here as `var(--...)`. UnoCSS
// utilities like `bg-surface-page` resolve to
// `background-color: var(--color-surface-page)`. This means swapping
// the palette only requires editing base.css; the generated CSS
// changes automatically because variables are resolved at runtime.

import { defineConfig, presetIcons, presetWind3, transformerVariantGroup, transformerDirectives } from "unocss";

export default defineConfig({
    content: {
        filesystem: ["src/**/*.rs", "index.html"],
    },
    presets: [
        presetWind3({
            preflight: false, // we ship our own reset in style/base.css
        }),
        presetIcons({
            scale: 1.1,
            extraProperties: {
                display: "inline-block",
                "vertical-align": "middle",
            },
        }),
    ],
    transformers: [
        transformerVariantGroup(),
        transformerDirectives(),
    ],
    theme: {
        colors: {
            // Surfaces -- the four levels of background depth.
            surface: {
                page: "var(--color-surface-page)",
                raised: "var(--color-surface-raised)",
                sunken: "var(--color-surface-sunken)",
                inverse: "var(--color-surface-inverse)",
            },
            // Text -- monochrome ramp on top of surfaces.
            ink: {
                primary: "var(--color-ink-primary)",
                secondary: "var(--color-ink-secondary)",
                muted: "var(--color-ink-muted)",
                inverse: "var(--color-ink-inverse)",
                accent: "var(--color-ink-accent)",
            },
            // Borders / hairlines.
            border: {
                subtle: "var(--color-border-subtle)",
                strong: "var(--color-border-strong)",
                accent: "var(--color-border-accent)",
            },
            // Brand accents.
            accent: {
                DEFAULT: "var(--color-accent)",
                soft: "var(--color-accent-soft)",
                strong: "var(--color-accent-strong)",
                ink: "var(--color-accent-ink)",
            },
            trust: {
                DEFAULT: "var(--color-trust)",
                soft: "var(--color-trust-soft)",
            },
            // Per-project tag colours.
            project: {
                patina: "var(--color-project-patina)",
                ec: "var(--color-project-ec)",
                services: "var(--color-project-services)",
                "patina-ink": "var(--color-project-patina-ink)",
                "ec-ink": "var(--color-project-ec-ink)",
                "services-ink": "var(--color-project-services-ink)",
            },
            // Semantic.
            success: "var(--color-success)",
            warning: "var(--color-warning)",
            danger: "var(--color-danger)",
            info: "var(--color-info)",
        },
        fontFamily: {
            sans: "var(--font-sans)",
            display: "var(--font-display)",
            mono: "var(--font-mono)",
        },
        fontSize: {
            // Map the design tokens to UnoCSS sizes. Each entry is
            // [font-size, line-height].
            "display-xl": ["var(--font-size-display-xl)", "var(--leading-display)"],
            display: ["var(--font-size-display)", "var(--leading-display)"],
            h1: ["var(--font-size-h1)", "var(--leading-heading)"],
            h2: ["var(--font-size-h2)", "var(--leading-heading)"],
            h3: ["var(--font-size-h3)", "var(--leading-heading)"],
            lead: ["var(--font-size-lead)", "var(--leading-lead)"],
            body: ["var(--font-size-body)", "var(--leading-body)"],
            small: ["var(--font-size-small)", "var(--leading-body)"],
            caption: ["var(--font-size-caption)", "var(--leading-body)"],
        },
        borderRadius: {
            sm: "var(--radius-sm)",
            md: "var(--radius-md)",
            lg: "var(--radius-lg)",
            xl: "var(--radius-xl)",
            pill: "var(--radius-pill)",
        },
        boxShadow: {
            "elev-1": "var(--shadow-1)",
            "elev-2": "var(--shadow-2)",
            "elev-3": "var(--shadow-3)",
            "elev-4": "var(--shadow-4)",
            "focus-ring": "var(--shadow-focus)",
        },
        spacing: {
            // Token-driven scale on top of UnoCSS's default rem-based
            // utilities. We keep the default scale too (UnoCSS merges)
            // so utilities like `p-4`, `gap-6` keep working.
            "section-y": "var(--space-section-y)",
            "section-x": "var(--space-section-x)",
            gutter: "var(--space-gutter)",
        },
        breakpoints: {
            sm: "640px",
            md: "768px",
            lg: "1024px",
            xl: "1280px",
            "2xl": "1536px",
        },
    },
    safelist: [
        // Theme-toggle switches data-theme on <html>; these utilities
        // appear via attribute selectors that the static scan misses.
        "data-[theme=dark]:hidden",
        "data-[theme=light]:hidden",
    ],
});
