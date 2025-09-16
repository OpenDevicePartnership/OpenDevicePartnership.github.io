use leptos::prelude::*;
use odp::App;

fn main() {
    // GitHub Pages SPA redirect support: handle ?p= or ?redirect= route param
    if let Some(window) = web_sys::window() {
        if let Ok(search) = window.location().search() {
            let (param, offset) = if let Some(idx) = search.find("?p=") {
                ("?p=", idx + 3)
            } else if let Some(idx) = search.find("?redirect=") {
                ("?redirect=", idx + 10)
            } else {
                ("", 0)
            };
            if !param.is_empty() && offset < search.len() {
                let encoded = &search[offset..];
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
