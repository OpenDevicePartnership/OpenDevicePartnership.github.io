use leptos::prelude::*;
use odp::App;

fn main() {
    // GitHub Pages SPA redirect support: handle ?p= route param
    if let Some(window) = web_sys::window() {
        if let Ok(location) = window.location().search() {
            if let Some(idx) = location.find("?p=") {
                let encoded = &location[idx + 3..];
                if let Ok(path) = urlencoding::decode(encoded) {
                    let history = window.history().unwrap();
                    history
                        .replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&path))
                        .ok();
                }
            }
        }
    }

    console_error_panic_hook::set_once();
    mount_to_body(App);
}
