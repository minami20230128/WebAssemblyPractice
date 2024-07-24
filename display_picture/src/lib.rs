use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;
use wasm_bindgen::JsCast;
use web_sys::{console, Event, HtmlImageElement};

#[wasm_bindgen(start)]
pub fn embed_picture() -> Result<(), JsValue> {

    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let document = web_sys::window().unwrap().document().unwrap();

    let div = document.get_element_by_id("parent").unwrap();
    //let val = document.create_element("p").unwrap();
    //val.set_inner_html("Hello World from WebAssemblyMan!");
    let a = document.create_element("a").unwrap();
    a.set_attribute("href", "https://ja.wikipedia.org/wiki/甲斐犬");

    let img = document.create_element("img").unwrap();
    img.set_attribute("src", "dog.png");

    let img_clone = img.clone(); // Clone img for closure
    img.add_event_listener_with_callback(
        "click",
        // Closure handles the event
        Closure::wrap(Box::new(move |_event: Event| {
            // Example action on click, like logging
            console::log_1(&"Image clicked!".into());
        })),
    )?;

    a.append_child(&img).unwrap();
    div.append_child(&a).unwrap();

    Ok(())
}

pub fn clear_picture(){
    let document = web_sys::window().unwrap().document().unwrap();
    let img = document.get_element_by_id("dog.png").unwrap();
    if let Some(parent_node) = img.parent_node() {
        parent_node.remove_child(&img).unwrap();
    }
}
