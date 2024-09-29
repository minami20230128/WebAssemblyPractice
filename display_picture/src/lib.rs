mod quiz_provider;
mod html_manipulator;
mod answer;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::quiz_provider::QuizProvider;
use crate::answer::History;
use crate::html_manipulator::HtmlManipulator;
use std::sync::Mutex;
use serde_json::Value;

static QUIZ_PROVIDER: quiz_provider::QuizProvider = QuizProvider {
    quizzes: Mutex::new(Vec::new()),
};

static HISTORY: History = History {
    records: Mutex::new(Vec::new()),
};

static HTML_MANIPULATOR: html_manipulator::HtmlManipulator = HtmlManipulator{
    QUIZ_PROVIDER: &QUIZ_PROVIDER,
    HISTORY: &HISTORY,
};

#[wasm_bindgen(start)]
pub fn run() {
    

}

pub fn load_quizzes(data: Value) {
    QUIZ_PROVIDER.load_quizzes(data);
}

pub fn init_quiz() {
    let index = QUIZ_PROVIDER.get_random_index(&HISTORY);

    match QUIZ_PROVIDER.select_random_quiz(index) {
        Some(quiz) => {
            HTML_MANIPULATOR.put_quiz(&quiz, index);
        }
        None => HTML_MANIPULATOR.show_score_screen().unwrap(),
    }
}