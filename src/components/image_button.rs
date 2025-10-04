use leptos::prelude::*;

#[component]
pub fn ImageButton(
    #[prop(into)] href: String,
    #[prop(into)] img_src: String,
    #[prop(into, default = String::from("Button Image"))] alt: String,
    #[prop(default = 350)] width: u32,
    #[prop(default = 320)] height: u32,
    #[prop(default = None)] mobile_width: Option<u32>,
    #[prop(default = None)] mobile_height: Option<u32>,
) -> impl IntoView {
    view! {
        <a
            href=href
            style="
                display: inline-block;
                border: none;
                background: none;
                padding: 0;
                cursor: pointer;
                text-decoration: none;
            "
        >
            <style>{
                if let (Some(mw), Some(mh)) = (mobile_width, mobile_height) {
                    format!(
                        "@media (max-width: 767px) {{ img[alt='{}'] {{ width: {}px !important; height: {}px !important; }} }}",
                        alt, mw, mh
                    )
                } else {
                    String::new()
                }
            }</style>
            <img
                src=img_src
                alt=alt
                style=format!(
                    "
                        width: {}px;
                        height: {}px;
                        border-radius: 45.7px;
                        object-fit: cover;
                        display: block;
                    ",
                    width, height
                )
            />
        </a>
    }
}
