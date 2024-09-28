mod quiz_provider;
mod html_manipulator;
mod answer;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::quiz_provider::QuizProvider;
use crate::answer::Answer;
use crate::html_manipulator::HtmlManipulator;
use wasm_bindgen::JsValue;
use std::sync::Mutex;
use std::result;

static QUIZ_PROVIDER: quiz_provider::QuizProvider = QuizProvider {
    quizzes: Mutex::new(Vec::new()),
};

static ANSWER: Answer = Answer {
    histories: Mutex::new(Vec::new()),
};

static HTML_MANIPULATOR: html_manipulator::HtmlManipulator = HtmlManipulator{};

#[wasm_bindgen(start)]
pub fn run() {
    

}

#[wasm_bindgen]
pub fn load_quizzes(data: JsValue) {
    QUIZ_PROVIDER.load_quizzes(data);
}

pub fn init_quiz() {
    let index = QUIZ_PROVIDER.get_random_index(&ANSWER);

    match QUIZ_PROVIDER.select_random_quiz(index) {
        Some(quiz) => {
            HTML_MANIPULATOR.put_quiz(quiz, index);
        }
        None => HTML_MANIPULATOR.show_score_screen().unwrap(),
    }
}