mod quiz_provider;
mod html_manipulator;
mod answer;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::html_manipulator::HtmlManipulator;

static HTML_MANIPULATOR: html_manipulator::HtmlManipulator = HtmlManipulator{};

#[wasm_bindgen(start)]
pub fn run() {
    

}

#[wasm_bindgen]
pub fn load_quizzes_from_json(data: JsValue) {
    HTML_MANIPULATOR.init_quizzes(data);
}