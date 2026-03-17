use leptos::prelude::*;

struct Partner {
    name: &'static str,
    logo: &'static str,
}

const PARTNERS: &[Partner] = &[
    Partner {
        name: "Microsoft Surface",
        logo: "/images/partners/microsoft_surface.svg",
    },
    Partner {
        name: "NXP",
        logo: "/images/partners/nxp.svg",
    },
    Partner {
        name: "tweede golf",
        logo: "/images/partners/tweede_golf.svg",
    },
    Partner {
        name: "CIX",
        logo: "/images/partners/cix.svg"
    }
];

#[component]
pub fn PartnersGrid() -> impl IntoView {
    view! {
        <section class="background_primary px-4 md:px-[120px] py-[80px]">
            <div class="mb-[60px]">
                <span class="h1_mobile md:h1 break-words w-full" style="display: block; text-align: left;">
                    {"Our Partners"}
                </span>
            </div>
            <div class="grid grid-cols-2 md:grid-cols-4 gap-[40px] md:gap-[60px] items-center justify-items-center">
                {PARTNERS.iter().map(|partner| {
                    view! {
                        <div class="flex items-center justify-center w-full h-[80px] md:h-[100px]">
                            <img
                                src=partner.logo
                                alt=partner.name
                                class="max-w-[180px] max-h-[70px] md:max-w-[200px] md:max-h-[80px] object-contain"
                            />
                        </div>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}
