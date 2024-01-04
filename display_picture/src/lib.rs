use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {

    let document = web_sys::window().unwrap().document().unwrap();
    
    let parent = document.get_element_by_id("parent").unwrap();
    let child = document.create_element("img").unwrap();

    child.set_attribute("src", "dog.png");
    parent.append_child(&child).unwrap();

    Ok(())
}