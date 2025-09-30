use leptos::prelude::Effect;
use leptos_router::hooks::{use_location, use_navigate};
use leptos::prelude::*;
use crate::components::header::Header;
use crate::components::footer::Footer;

// Welcome Patina Press Release
fn patina_press_release() -> impl IntoView {
    view! {
        <div>
            <p class="mb-4">
                <strong>"October 7, 2025 – Sunnyvale, CA"</strong> " – The " <strong>"Open Device Partnership (ODP)"</strong> " today announced that " <strong>"Patina"</strong>", a new open-source firmware project inspired by and compatible with UEFI, will be made public at the upcoming " 
                <a href="https://uefi.org/events/uefi-2025-developers-conference-and-plugfest" class="underline hover:text-blue-600 transition-colors duration-150">
                    "UEFI 2025 Developer Conference & Plugfest"
                </a>
                ", October 7–10 in Sunnyvale, California. Patina joins a growing portfolio of ODP projects aimed at building a secure, modern foundation for device enablement. To learn more about Patina, please visit the project page and documentation here: " 
                <span class="bg-yellow-200">"TBD"</span>"."
            </p>

            <p class="mb-4">
                "ODP is an industry-wide, open-source initiative focused on advancing " <strong>"security, fundamentals, and standardization"</strong> " in device software. The partnership's work addresses long-standing challenges in firmware and system design by leveraging " <strong>"memory-safe programming languages like Rust"</strong> " and " <strong>"hardware-rooted security features"</strong> " and doing so based on standards that will work across a partner's entire device product line. This approach reduces exposure to common vulnerabilities while providing manufacturers with a sustainable, consistent foundation that lowers engineering costs across product lines."
            </p>

            <p class="mb-4">
                "In addition to Patina, ODP is currently focused on three other major projects:"
            </p>

            <p class="mb-4">
                "Together, these efforts give hardware makers the ability to standardize firmware and device software across their entire portfolios—improving reliability, accelerating time-to-market, and reducing redundant engineering work."
            </p>

            <p class="mb-4">
                "The Open Device Partnership is about building trust at the foundation. By focusing on memory safety, hardware-rooted security, and clear standards, we're making it easier for partners to deliver secure, consistent solutions while also reducing long-term development costs."
            </p>

            <p class="mb-4">
                "With Patina's public launch and the ongoing progress of ODP's other projects, the initiative is now seeking broader industry participation. ODP invites partners, contributors, and stakeholders to join in shaping the future of secure, open device enablement."
            </p>

            <p class="mb-4">
                "👉 Learn more and get involved at " 
                <a href="https://opendevicepartnership.org/" class="underline hover:text-blue-600 transition-colors duration-150">
                    "opendevicepartnership.org"
                </a>"."
            </p>
        </div>
    }
}

#[component]
pub fn AnnouncementsPage() -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();

    // List of announcement links and their content (display_text, title, slug)
    let announcements = vec![
        ("Oct-7-2025 Welcome Patina!", "Patina Project to Launch at UEFI 2025 Developer Conference & Plugfest", "welcome-patina-announcement"),
    ];

    let (selected, set_selected) = signal(0);

    // Set selected from query param if present
    {
        let location = location.clone();
        let set_selected = set_selected.clone();
        let announcements_clone = announcements.clone();
        Effect::new(move |_| {
            let search = location.search.get();
            
            // Check for new slug-based format: ?id=slug
            if search.contains("id=") {
                // Try to extract the id value more flexibly
                if let Some(id_start) = search.find("id=") {
                    let slug = &search[id_start + 3..];
                    // Remove any trailing parameters
                    let slug = if let Some(amp_pos) = slug.find('&') {
                        &slug[..amp_pos]
                    } else {
                        slug
                    };
                    if let Some(idx) = announcements_clone.iter().position(|(_, _, s)| *s == slug) {
                        set_selected.set(idx);
                    }
                }
            }
        });
    }

    view! {
        <div class="flex flex-col w-full min-h-screen background_quaternary">
            <Header background_class="background_quaternary" />
            <h1 class="h1 px-10 pt-20 pb-20">Announcements</h1>
            <div class="flex flex-row w-full flex-1 relative">
                <div class="w-[450px] h-[700px] overflow-y-auto background_tertiary z-10 p-6">
                    <ul class="space-y-4">
                        {announcements.iter().enumerate().map(|(i, (link, _, slug))| {
                            let navigate = navigate.clone();
                            let slug = *slug;
                            view! {
                                <li>
                                    <button
                                        class="link w-full text-left p-3"
                                        on:click=move |_| {
                                            set_selected.set(i);
                                            navigate(&format!("/announcements?id={}", slug), Default::default());
                                        }
                                    >
                                        {*link}
                                    </button>
                                </li>
                            }
                        }).collect::<Vec<_>>()}
                    </ul>
                </div>
                <div class="flex-1 h-[700px] background_primary rounded-tl-[50px] -ml-16 z-20 overflow-y-auto p-10">
                    {move || {
                        let (title, content): (String, AnyView) = if let Some((_, title, slug)) = announcements.get(selected.get()) {
                            let content = match *slug {
                                "welcome-patina-announcement" => patina_press_release().into_any(),
                                _ => view! { <p>"Content not found"</p> }.into_any(),
                            };
                            (title.to_string(), content)
                        } else {
                            ("No announcement selected".to_string(), view! { <p>{""}</p> }.into_any())
                        };
                        view! {
                            <div class="p">
                                <h1 class="h2 pb-6">{title}</h1>
                                <div class="leading-relaxed">
                                    {content}
                                </div>
                            </div>
                        }
                    }}
                </div>
            </div>
            <Footer />
        </div>
    }
}
