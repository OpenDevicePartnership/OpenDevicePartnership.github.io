use leptos::prelude::*;
use leptos_router::components::A;

#[derive(Clone)]
pub struct DocLink {
    pub href: &'static str,
    pub title: &'static str,
    pub external: bool,
}

#[component]
pub fn DocumentationTraining(#[prop(default = vec![])] links: Vec<DocLink>) -> impl IntoView {
    view! {
        <section
            class="flex flex-col md:flex-row items-start background_primary w-full overflow-x-hidden px-6 py-8 md:py-[60px] md:px-[120px]"
        >
            {/* Left: Image and text box */}
            <div class="flex flex-col items-start w-full" style="align-items: flex-start;">
                <picture>
                    <source srcset="/images/dark/documentation.svg" media="(prefers-color-scheme: dark)" />
                    <img
                        src="/images/light/documentation.svg"
                        alt="Documentation Icon"
                        class="w-[80px] h-[80px] md:w-[150px] md:h-[150px] object-contain mb-4"
                        style="display: block; margin-bottom: 16px;"
                    />
                </picture>
                <span
                    class="h2_mobile md:h2 text-left break-words"
                    style="word-break:break-word; display: block; text-align: left;"
                >
                    "Documentation"
                </span>
                <div style="height: 10px;"></div>
                <span
                    class="p1_mobile md:p1 text-left break-words"
                    style="word-break:break-word; display: block; text-align: left;"
                >
                    "Start developing with ODP"
                </span>
            </div>

            {/* Spacer between left and right */}
            <div class="hidden md:block" style="width: 200px;"></div>

            {/* Right: List of hyperlinks */}
            <ul class="flex flex-col pt-4 w-full max-w-full break-words md:pt-4 md:w-[760px] md:max-w-[760px]">
                {links.into_iter().map(|link| view! {
                    <li>
                        <Show
                            when=move || link.external
                            fallback= move || view! {
                            <div class="link_large_mobile md:link_large internal-link" style="text-decoration: none;">
                                <A href=link.href>
                                    <span style="text-decoration: none;">{"→ "}</span>
                                    <span style="text-decoration: underline;">{link.title}</span>
                                </A>
                            </div>
                        }
                        >
                            <div class="link_large_mobile md:link_large external-link" style="text-decoration: none;">
                                <a href=link.href target="_blank" style="text-decoration: none;">
                                    <span style="text-decoration: none;">{"→ "}</span>
                                    <span style="text-decoration: underline;">{link.title}</span>
                                </a>
                            </div>
                        </Show>
                    </li>
                }).collect_view()}
            </ul>
        </section>
    }
}
