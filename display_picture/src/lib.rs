use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn embed_picture() -> Result<(), JsValue> {

    let document = web_sys::window().unwrap().document().unwrap();

    let div = document.get_element_by_id("div").unwrap();
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