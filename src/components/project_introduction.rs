use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn ProjectIntroduction(
    #[prop(into)] project_title: String,
    #[prop(into)] project_summary: String,
    #[prop(into)] project_what: String,
    #[prop(into)] project_why: String,
    #[prop(into)] project_who: String,
    #[prop(into, optional)] small_image_url: String,
    #[prop(into, optional)] big_image_url: String,
) -> impl IntoView {
    view! {
        <section class="background_primary">
            <div class="flex flex-col md:flex-row gap-[80px]">
                {/* Left Column: Big Picture with Overlayed Text and Small Image */}
                <div
                    class="relative w-full md:w-[700px] h-[400px] md:h-[630px]"
                    style="margin-left: 0; padding-left: 0; flex-shrink: 0; position: relative; display: flex; align-items: center; justify-content: flex-start;"
                >
                    {/* Big Picture */}
                    <img
                        src={big_image_url}
                        alt="Project Main"
                        class="w-full h-[400px] md:w-[700px] md:h-[630px]"
                        style="object-fit: cover; display: block; border-top-right-radius: 40px; border-bottom-right-radius: 40px;"
                    />
                    {/* Overlayed Text and Small Picture */}
                    <div
                        class="absolute top-1/2 left-0 w-[90%] pl-2 md:pl-[60px] flex flex-col items-start"
                        style="transform: translateY(-50%); z-index: 2; text-align: left;"
                    >
                        {/* Small Picture aligned with project title and 60px above */}
                        <img
                            src={small_image_url}
                            alt="Project Logo"
                            class="w-[48px] h-[48px] md:w-[102px] md:h-[102px] mb-4 md:mb-[60px] ml-0"
                            style="object-fit: contain;"
                        />
                        <span
                            class="h2_mobile md:h1"
                            style="display: block; color: white; margin-bottom: 10px; word-break: break-word; text-align: left;"
                        >
                            {project_title}
                        </span>
                        <span
                            class="p3_mobile md:p1"
                            style="display: block; color: white; word-break: break-word; text-align: left;"
                        >
                            {project_summary}
                        </span>
                    </div>
                </div>
                {/* Right Column */}
                <div class="flex flex-col items-start px-2 md:px-0 w-full">
                    {/* WHAT label */}
                    <span
                        class="mono"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"WHAT"}
                    </span>
                    {/* WHAT description */}
                    <span
                        class="p2_mobile md:p2"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {project_what}
                    </span>
                    {/* WHY label */}
                    <span
                        class="mono"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"WHY"}
                    </span>
                    {/* WHY description */}
                    <span
                        class="p2_mobile md:p2"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {project_why}
                    </span>
                    {/* WHO label */}
                    <span
                        class="mono"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"WHO"}
                    </span>
                    {/* WHO description */}
                    <div class="link_mobile md:link block text-left">
                        <A href={project_who}>
                            {"Learn more about the team →"}
                        </A>
                    </div>
                </div>
            </div>
        </section>
        <div class="background_primary px-2 md:px-0">
            <span
                class="p1_mobile md:p1"
                style="
                display: block;
                text-align: left;
                padding-left: 20px;
                padding-top: 100px;
                padding-bottom: 20px;
                "
            >
                Repository Diagram
            </span>
        </div>
    }
}
