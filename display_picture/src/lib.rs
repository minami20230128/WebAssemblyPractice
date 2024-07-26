use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;

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

#[wasm_bindgen]
pub fn hide_button() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let button = document.get_element_by_id("buttonToDelete");

    if let Some(button) = button {
        button.remove();
    }

    Ok(())
}