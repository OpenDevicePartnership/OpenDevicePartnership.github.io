use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Main() -> impl IntoView {
    view! {
        <main class="background_primary">
            <div
                class="mx-auto flex items-start justify-between w-full"
            >
                <div class="pl-[20px] md:pl-[120px] flex flex-col gap-6">
                    <h1
                        class="h1 py-4"
                        style="text-align: left;"
                    >
                        "Building the Future of Trusted System Software Together"
                    </h1>
                    <p
                        class="p1"
                        style="width: 900px; height: auto;"
                    >
                        "Leading technology partners creating secure, reusable, and reliable firmware for modern client devices."
                    </p>
                </div>

                <div class="flex flex-col">
                    <div style="border: none; text-decoration: none;" class="flex background_secondary w-[478px] h-[176px] items-center justify-center px-[60px]">
                        <A
                            href="/getting-started"
                        >
                            <div class="flex flex-row items-center justify-center gap-4">
                                <span class="h3">"Getting started"</span>
                                <span class="h3">"→"</span>
                            </div>
                        </A>
                    </div>

                    <div style="border: none; text-decoration: none;" class="flex background_tertiary w-[478px] h-[176px] items-center justify-center px-[60px]">
                        <A
                            href="/projects"
                        >
                            <div class="flex flex-row items-center justify-center gap-4">
                                <span class="h3">"Projects"</span>
                                <span class="h3">"→"</span>
                            </div>
                        </A>
                    </div>
                </div>
            </div>

            {/* Video Section */}
            <div class="flex flex-col pt-10" style="padding-left: 117px;">
                {/* Top row: icon/text + main iframe */}
                <div class="flex flex-row items-start w-full">
                    <div class="flex flex-col items-start w-[423px] mr-[60px]">
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
                        <span class="h2"
                              style="
                                  display: block;
                                  text-align: left;
                              ">
                            "Welcome"
                        </span>
                        <div style="height: 10px;"></div>
                        <span class="p1"
                              style="
                                  display: block;
                                  text-align: left;
                              ">
                            "Learn how ODP projects help build secure, modern devices"
                        </span>
                    </div>
                    <iframe
                        width="1200"
                        height="690"
                        style="border-radius: 10px; display: block;"
                        src="https://www.youtube.com/embed/FMlPxYSY1LM?rel=0"
                        title="YouTube Video of the Open Device Partnership"
                        frameborder="0"
                        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                        allowfullscreen>
                    </iframe>
                </div>
            </div>
        </main>
    }
}
