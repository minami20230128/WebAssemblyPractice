mod quiz_provider;
mod html_manipulator;
mod result;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::quiz_provider::QuizProvider;
use crate::result::Result;
use crate::html_manipulator::HtmlManipulator;
use wasm_bindgen::JsValue;
use std::sync::Mutex;

static QUIZ_PROVIDER: quiz_provider::QuizProvider = QuizProvider {
    quizzes: Mutex::new(Vec::new()),
};

static RESULT: result::Result = Result {
    histories: Mutex::new(Vec::new()),
};

static HTML_MANIPULATOR: html_manipulator::HtmlManipulator = HtmlManipulator{};

#[wasm_bindgen(start)]
pub fn run() {
    

}

#[wasm_bindgen]
pub fn load_quizzes(data: JSvalue) {
    QUIZ_PROVIDER.load_quizzes(data);
}

pub fn init_quiz() -> result::Reesult<(), JsValue> {
    let index = QUIZ_PROVIDER.get_random_index(RESULT);
    QUIZ_PROVIDER.select_random_quiz(index)
}