use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;
use web_sys::{HtmlElement, NodeList, MouseEvent};
use serde::{Serialize, Deserialize};
use serde_json;
use std::sync::Mutex;

static SCORE: Mutex<i32> = Mutex::new(0);

#[wasm_bindgen(start)]
pub fn run() -> Result<(), wasm_bindgen::JsValue> {
    embed_picture();
    start_quiz_initialization();
    let str = receive_quiz_data();
    let quiz: Quiz = match serde_json::from_str(&str) {
        Ok(q) => q,
        Err(e) => return Err(JsValue::from_str(&format!("JSON parsing error: {:?}", e))),
    };

    quiz.put_question();
    quiz.put_options();
    quiz.put_answer()
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
    let closure = Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
        let target = mouse_event.target().unwrap();
        let button = target.dyn_into::<HtmlElement>().unwrap();
        let inner_html = button.inner_html();
        event(inner_html);
    })  as Box<dyn Fn(MouseEvent)>);
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

impl Quiz {

    pub fn put_question(&self) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let div = document.get_element_by_id("parent").unwrap();
        let p = document.create_element("p").unwrap();
        p.set_inner_html(&self.question);
        div.append_child(&p).unwrap();

        Ok(())
    }

    pub fn put_options(&self) -> Result<(), JsValue> {
        let answers = &self.options;

        for answer in answers.iter() {
            // JavaScript 側にボタンを作成するための関数を呼び出す
            put_button(answer)?;
        }

        Ok(())
    }

    pub fn put_answer(&self) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let div = document.get_element_by_id("parent").unwrap();
        let p = document.create_element("p").unwrap();
        p.set_inner_html(&self.correct_answer);
        p.set_attribute("hidden", "")?;
        p.set_id("answer");
        div.append_child(&p).unwrap();
        
        Ok(())
    }
}

pub fn receive_quiz_data() -> String  {
    return handle_quiz_result_sync();
}

#[wasm_bindgen]
pub fn check_answer(response : String) {
    let document = web_sys::window().unwrap().document().unwrap();
    let div = document.get_element_by_id("parent").unwrap();
    let answer_elem = document.get_element_by_id("answer").unwrap();
    let answer = answer_elem.inner_html();

    let mut score = SCORE.lock().unwrap();

    if response == answer {
        *score += 1;
    }

    logInfo(&score.to_string());
}

// イベントを処理する関数
#[wasm_bindgen]
pub fn event(inner_html : String) -> Result<(), JsValue> {

    check_answer(inner_html);

    match remove_question() {
        Ok(_) => {},
        Err(e) => return Err(JsValue::from_str(&format!("Error removing buttons: {:?}", e))),
    }

    match remove_all_buttons() {
        Ok(_) => {},
        Err(e) => return Err(JsValue::from_str(&format!("Error removing buttons: {:?}", e))),
    }

    let str = receive_quiz_data();
    logInfo(&str);

    if str == "end" {
        show_score_screen();
    } else {
        // JSONのデシリアライズエラーハンドリング
        let quiz: Quiz = match serde_json::from_str(&str) {
            Ok(q) => q,
            Err(e) => return Err(JsValue::from_str(&format!("JSON parsing error: {:?}", e))),
        };
        quiz.put_question();
        quiz.put_options();
        quiz.put_answer();
    }

    Ok(())
}

pub fn show_score_screen() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let div = document.get_element_by_id("parent").unwrap();
    let p = document.create_element("p").unwrap();
    p.set_inner_html("end");

    let score_elem = document.create_element("p").unwrap();
    let mut score = SCORE.lock().unwrap();
    let result = format!("{}問正解", &score.to_string());
    score_elem.set_inner_html(&result);
    div.append_child(&score_elem).unwrap();
    Ok(())
}

#[wasm_bindgen]
pub fn remove_question()-> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    // ページ内のすべてのボタン要素を取得
    let buttons:NodeList = document.query_selector_all("p").unwrap();

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