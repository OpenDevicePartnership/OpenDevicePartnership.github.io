use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Main() -> impl IntoView {
    view! {
        <main class="background_primary">
            <div
                class="mx-auto flex flex-col md:flex-row items-start justify-between w-full px-2 sm:px-4"
            >
                <div class="pl-0 md:pl-[120px] flex flex-col gap-6 w-full md:w-auto">
                    <h1
                        class="h1_mobile md:h1 py-4 w-full max-w-full text-left break-words"
                        style="word-break:break-word;"
                    >
                        "Building the Future of Trusted System Software Together"
                    </h1>
                    <p
                        class="p1_mobile md:p1 w-full max-w-full text-left"
                        style="word-break:break-word;"
                    >
                        "Leading technology partners creating secure, reusable, and reliable firmware for modern client devices."
                    </p>
                </div>

                <div class="flex flex-col w-full md:w-auto mt-4 md:mt-0">
                    <div style="border: none; text-decoration: none;" class="flex background_secondary w-full md:w-[478px] h-[120px] md:h-[176px] items-center justify-center px-4 md:px-[60px]">
                        <A
                            href="/getting-started"
                        >
                            <div class="flex flex-row items-center justify-center gap-4 w-full max-w-full">
                                <span class="h3_mobile md:h3">"Getting started"</span>
                                <span class="h3_mobile md:h3">"→"</span>
                            </div>
                        </A>
                    </div>

                    <div style="border: none; text-decoration: none;" class="flex background_tertiary w-full md:w-[478px] h-[120px] md:h-[176px] items-center justify-center px-4 md:px-[60px]">
                        <A
                            href="/projects"
                        >
                            <div class="flex flex-row items-center justify-center gap-4 w-full max-w-full">
                                <span class="h3_mobile md:h3">"Projects"</span>
                                <span class="h3_mobile md:h3">"→"</span>
                            </div>
                        </A>
                    </div>
                </div>
            </div>

            {/* Video Section */}
            <div class="flex flex-col pt-10 px-2 sm:px-4 md:pl-[117px] w-full">
                {/* Top row: icon/text + main iframe */}
                <div class="flex flex-col lg:flex-row items-start w-full gap-4">
                    <div class="flex flex-col items-start w-full lg:w-[423px] mr-0 lg:mr-[60px] mb-6 lg:mb-0">
                        <picture>
                            <source srcset="/images/dark/video.svg" media="(prefers-color-scheme: dark)" />
                            <img
                                src="/images/light/video.svg"
                                alt="Video Icon"
                                style="
                                    width: 150px;
                                    height: 150px;
                                    padding: 0;
                                    object-fit: contain;
                                    display: block;
                                    margin-bottom: 16px;
                                "
                            />
                        </picture>
                        <span class="h2_mobile md:h2 block w-full max-w-full text-left break-words" style="word-break:break-word;">
                            "Welcome"
                        </span>
                        <div style="height: 10px;"></div>
                        <span class="p1_mobile md:p1 block w-full max-w-full text-left break-words" style="word-break:break-word;">
                            "Learn how ODP projects help build secure, modern devices"
                        </span>
                    </div>
                    <div class="w-full lg:w-[1200px] aspect-video rounded-lg overflow-hidden" style="max-width:100vw;">
                        <iframe
                            class="w-full h-full"
                            style="border-radius: 10px; display: block;"
                            src="https://www.youtube.com/embed/FMlPxYSY1LM?rel=0"
                            title="YouTube Video of the Open Device Partnership"
                            frameborder="0"
                            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                            allowfullscreen>
                        </iframe>
                    </div>
                </div>
            </div>
        </main>
    }
}
