use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;
use web_sys::{HtmlElement, NodeList};


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

    put_buttons(4);

    Ok(())
}

pub fn clear_picture(){
    let document = web_sys::window().unwrap().document().unwrap();
    let img = document.get_element_by_id("dog.png").unwrap();
    if let Some(parent_node) = img.parent_node() {
        parent_node.remove_child(&img).unwrap();
    }
}

// グローバル変数としてカウントを保持する
static mut COUNT: u32 = 0;

// ボタンを作成する関数
#[wasm_bindgen]
pub fn put_button() -> Result<(), JsValue> {
    unsafe {
        let document = web_sys::window().unwrap().document().unwrap();

        // 新しいボタンを作成
        let button = document.create_element("button")?.dyn_into::<HtmlElement>()?;
        button.set_id("add_soft_button");
        button.set_inner_html(&COUNT.to_string());

        // ボタンにクリックイベントを設定
        let button_clone = button.clone();
        let closure = Closure::wrap(Box::new(move || {
            event();
        }) as Box<dyn Fn()>);
        button.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget(); // ClosureをJavaScriptで保持させる

        // <body>にボタンを追加
        document.body().unwrap().append_child(&button)?;

        // カウントを増加
        COUNT += 1;
    }
    Ok(())
}

// イベントを処理する関数
#[wasm_bindgen]
pub fn event() -> Result<(), JsValue> {
    remove_all_buttons()?;
    put_buttons(4)
}

// ページ内のすべてのボタンを削除する関数
#[wasm_bindgen]
pub fn remove_all_buttons() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    // ページ内のすべてのボタン要素を取得
    let buttons:NodeList = document.query_selector_all("button").unwrap();

    // ボタン要素を削除する
    for i in (0..buttons.length()).rev() {
        if let Some(button) = buttons.get(i) {
            if let Some(button_element) = button.dyn_ref::<HtmlElement>() {
                button_element.remove();
            }
        }
    }

    Ok(())
}

// 指定された数のボタンを追加する関数
#[wasm_bindgen]
pub fn put_buttons(num: u32) -> Result<(), JsValue> {
    for _ in 0..num {
        put_button()?;
    }
    Ok(())
}

//流れ
//ボタンを4つ表示
//ボタンを押す
//すべてのボタンを消去
//次のボタンの表示・イベントを設定
//次のボタンを表示

//init()で最初のボタンの表示・処理を設定・設置
//
