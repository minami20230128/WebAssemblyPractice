use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;
use web_sys::{HtmlElement, NodeList};
use serde::{Serialize, Deserialize};
use serde_json::Result as SerdeResult;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), wasm_bindgen::JsValue> {
    embed_picture();
    start_quiz_initialization();
    let quiz = receive_quiz_data();
    put_buttons(&quiz)
}

//Cargo.tomlからのパスを指定する
#[wasm_bindgen(module = "/src/quiz.js")]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    pub type QuizProvider;

    #[wasm_bindgen(js_name = "sendRandomQuizToRust")]
    fn send_random_quiz_to_rust() -> js_sys::Promise;
}

#[wasm_bindgen(module = "/src/quiz.js")]
extern "C" {
    fn initializeQuizzes();
}

#[wasm_bindgen]
pub fn start_quiz_initialization() {
    // JavaScript の initializeQuizzes 関数を呼び出す
    initializeQuizzes();
}

#[wasm_bindgen]
extern "C" {
    // JavaScript function that returns a Promise
    #[wasm_bindgen(js_name = "handleQuizResultSync")]
    fn handle_quiz_result_sync() -> String;
}

#[wasm_bindgen]
extern "C" {
    fn logInfo(message: &str);
    fn logError(message: &str);
}

// ボタンを作成する関数
#[wasm_bindgen]
pub fn put_button(answer : &str) -> Result<(), JsValue> {
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

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Quiz {
    pub question: String,
    pub options: Vec<String>,
    pub correct_answer: String,
}

pub fn receive_quiz_data() -> Quiz  {
    logInfo("receive_quiz_data");
    let json_string = handle_quiz_result_sync();
    logInfo(&json_string);
    
    match serde_json::from_str(&json_string) {
        Ok(quiz) => quiz, // Return the parsed Quiz struct
        Err(err) => {
            // Return a default Quiz instance in case of parsing error
            eprintln!("Failed to parse JSON into Quiz: {:?}", err);
            Quiz {
                question: "Error".to_string(),
                options: vec![],
                correct_answer: "Parsing Error".to_string(),
            }
        }
    }
}

// イベントを処理する関数
#[wasm_bindgen]
pub fn event() -> Result<(), JsValue> {
    remove_all_buttons()?;
    let quiz = receive_quiz_data();
    logInfo(&quiz.question);
    put_buttons(&quiz)
}

// 指定された数のボタンを追加する関数
pub fn put_buttons(quiz: &Quiz) -> Result<(), JsValue> {

    let answers = &quiz.options;

    for answer in answers.iter() {
        // JavaScript 側にボタンを作成するための関数を呼び出す
        put_button(answer)?;
    }

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