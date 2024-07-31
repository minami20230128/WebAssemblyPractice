use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;
use web_sys::{HtmlElement, NodeList};
use serde::{Serialize, Deserialize};
use serde_json::Result as SerdeResult;
use std::fs::File;
use std::io::Read;
use std::sync::Mutex;
use std::sync::Arc;

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

    //load_quizzes();
    put_buttons();

    Ok(())
}

pub fn clear_picture(){
    let document = web_sys::window().unwrap().document().unwrap();
    let img = document.get_element_by_id("dog.png").unwrap();
    if let Some(parent_node) = img.parent_node() {
        parent_node.remove_child(&img).unwrap();
    }
}

// ボタンを作成する関数
#[wasm_bindgen]
pub fn put_button(answer : &str) -> Result<(), JsValue> {
    unsafe {
        let document = web_sys::window().unwrap().document().unwrap();

        // 新しいボタンを作成
        let button = document.create_element("button")?.dyn_into::<HtmlElement>()?;
        button.set_id("add_soft_button");
        button.set_inner_html(answer);

        // ボタンにクリックイベントを設定
        let button_clone = button.clone();
        let closure = Closure::wrap(Box::new(move || {
            event();
        }) as Box<dyn Fn()>);
        button.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget(); // ClosureをJavaScriptで保持させる

        // <body>にボタンを追加
        document.body().unwrap().append_child(&button)?;
    }
    Ok(())
}

lazy_static::lazy_static! {
    static ref QUIZZES: Arc<Mutex<Vec<Quiz>>> = Arc::new(Mutex::new(Vec::new()));
    static ref INDEX: Mutex<usize> = Mutex::new(0);
}

// イベントを処理する関数
#[wasm_bindgen]
pub fn event() -> Result<(), JsValue> {
    remove_all_buttons()?;
    put_buttons()
}

// 指定された数のボタンを追加する関数
#[wasm_bindgen]
pub fn put_buttons() -> Result<(), JsValue> {
    //let quizzes = QUIZZES.lock().unwrap();
    //let index = *INDEX.lock().unwrap();
    //
    //if index >= quizzes.len() {
    //    return Err(JsValue::from_str("Index out of bounds"));
    //}
    //let answers = &quizzes[index].options;

    let answers = vec!["朝", "昼", "夕", "夜"];
    for answer in answers.iter() {
        // JavaScript 側にボタンを作成するための関数を呼び出す
        put_button(answer)?;
    }

    // インデックスを更新
    let mut index_lock = INDEX.lock().unwrap();
    *index_lock += 1;
    
    Ok(())
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Quiz {
    pub question: String,
    pub options: Vec<String>,
    pub correct_answer: String,
}

#[wasm_bindgen]
pub fn initialize_quizzes() -> Result<(), JsValue> {
    let quizzes = load_quizzes().map_err(|e| JsValue::from_str(&e.to_string()))?;
    let mut quiz_lock = QUIZZES.lock().unwrap();
    *quiz_lock = quizzes;
    Ok(())
}

fn load_quizzes() -> Result<Vec<Quiz>, Box<dyn std::error::Error>> {
    let file_path = "questions.json";
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let quizzes: Vec<Quiz> = serde_json::from_str(&contents)?;
    Ok(quizzes)
}

//流れ
//スタートボタンを表示
//（クリックする）
//スタート画面の背景とボタンを消す
//問題画面の背景、問題文、ボタン4つを表示する

//(いずれかのボタンをクリックする)
//すべてのボタンを消去
//次の問題・ボタンを表示

//(5問目に回答する)
//現在の背景・問題文・ボタン消す
//得点・今までの問題の正誤・正しい答えを表示
