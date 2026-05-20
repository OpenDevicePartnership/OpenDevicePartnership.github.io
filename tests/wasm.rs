#![cfg(target_arch = "wasm32")]

//! Headless wasm-bindgen tests for the redesigned site.
//!
//! Smoke-level tests that render a few key components into the DOM
//! and assert on the resulting markup. Run with
//! `wasm-pack test --headless --chrome` (or any other browser
//! driver).

use leptos::prelude::*;
use odp::components::controls::{Button, Tag, TagTone};
use odp::components::typography::{Body, Display, Heading};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn render<F, V>(view_fn: F) -> web_sys::Element
where
    F: FnOnce() -> V + 'static,
    V: IntoView + 'static,
{
    let document = web_sys::window().unwrap().document().unwrap();
    let host = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&host).unwrap();
    let host_el: web_sys::HtmlElement = host.clone().unchecked_into();
    leptos::mount::mount_to(host_el, view_fn).forget();
    host
}

#[wasm_bindgen_test]
fn typography_renders_text() {
    let host = render(|| {
        view! {
            <div>
                <Display>"Display text"</Display>
                <Heading>"Heading text"</Heading>
                <Body>"Body text"</Body>
            </div>
        }
    });
    let html = host.inner_html();
    assert!(html.contains("Display text"));
    assert!(html.contains("Heading text"));
    assert!(html.contains("Body text"));
}

#[wasm_bindgen_test]
fn button_emits_label() {
    let host = render(|| {
        view! { <Button>"Click me"</Button> }
    });
    assert!(host.inner_html().contains("Click me"));
}

#[wasm_bindgen_test]
fn tag_renders_with_tone_class() {
    let host = render(|| {
        view! { <Tag tone=TagTone::Accent>"Beta"</Tag> }
    });
    let html = host.inner_html();
    assert!(html.contains("Beta"));
    assert!(html.contains("text-ink-accent"));
}
