use leptos::prelude::*;
use std::sync::Arc;
use crate::components::header::Header;
use crate::components::footer::Footer;

#[component]
pub fn AnnouncementsPage() -> impl IntoView {
    // List of announcement links and their content
    let announcements = Arc::new(vec![
        ("Announcement 1", "Welcome Patina!"),
        ("Announcement 2", "Empty Text!"),
    ]);

    let (selected, set_selected) = signal(0);

    view! {
        <div class="flex flex-col w-full min-h-screen background_quaternary">
            <Header background_class="background_quaternary" />
            <h1 class="h1 px-10 pt-20 pb-20">Announcements</h1>
            <div class="flex flex-row w-full flex-1 relative">
                <div class="w-[450px] h-[700px] overflow-y-auto background_tertiary z-10 p-6">
                    <ul class="space-y-4">
                        {let announcements = announcements.clone();
                         announcements.iter().enumerate().map(|(i, (title, _))| {
                            view! {
                                <li>
                                    <button
                                        class="link w-full text-left p-3"
                                        on:click=move |_| set_selected.set(i)
                                    >
                                        {*title}
                                    </button>
                                </li>
                            }
                        }).collect::<Vec<_>>()}
                    </ul>
                </div>
                <div class="flex-1 h-[700px] background_primary rounded-tl-[50px] -ml-16 z-20 overflow-y-auto p-10">
                    {move || {
                        let announcements_content = announcements.clone();
                        if let Some((title, content)) = announcements_content.get(selected.get()) {
                            view! {
                                <div class="p">
                                    <h1 class="h2 pb-6">{*title}</h1>
                                    <p>{*content}</p>
                                </div>
                            }
                        } else {
                            view! {
                                <div class="p">
                                    <h1 class="h2 pb-6">{"No announcement selected"}</h1>
                                    <p>{""}</p>
                                </div>
                            }
                        }
                    }}
                </div>
            </div>
            <Footer />
        </div>
    }
}
