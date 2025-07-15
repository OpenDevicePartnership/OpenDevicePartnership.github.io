use leptos::prelude::*;

#[derive(Clone)]
pub struct DocLink {
    pub href: &'static str,
    pub title: &'static str,
}

#[component]
pub fn DocumentationTraining(
    #[prop(default = vec![])] links: Vec<DocLink>
) -> impl IntoView {
    view! {
        <section
            class="flex flex-row items-start background_primary"
            style="padding: 120px;"
        >
            {/* Left: Image and text box */}
            <div class="flex flex-col items-start" style="min-width: 663px; align-items: flex-start;">
                <picture>
                    <source srcset="/images/dark/documentation.svg" media="(prefers-color-scheme: dark)" />
                    <img
                        src="/images/light/documentation.svg"
                        alt="Documentation Icon"
                        style="
                            width: 150px;
                            height: 150px;
                            object-fit: contain;
                            display: block;
                            margin-bottom: 16px;
                        "
                    />
                </picture>
                <span
                    class="h2"
                    style="
                        text-align: left;
                        display: block;
                    "
                >
                    "Documentation"
                </span>
                <div style="height: 10px;"></div>
                <span
                    class="p1"
                    style="
                        text-align: left;
                        display: block;
                    "
                >
                    "Start developing with ODP"
                </span>
            </div>

            {/* Spacer between left and right */}
            <div style="width: 200px;"></div>

            {/* Right: List of hyperlinks */}
            <ul class="flex flex-col pt-4" style="width: 760px;">
                {links.into_iter().map(|link| view! {
                    <li>
                        <a href=link.href class="link_large" style="text-decoration: none;">
                            <span style="text-decoration: none;">{"→ "}</span>
                            <span style="text-decoration: underline;">{link.title}</span>
                        </a>
                    </li>
                }).collect_view()}
            </ul>
        </section>
    }
}