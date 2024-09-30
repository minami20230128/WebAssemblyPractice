use crate::answer::History;
use std::sync::Mutex;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::JsValue;
use std::collections::HashSet;
use rand::Rng;
use serde::{Deserialize};

#[derive(Deserialize, Clone)]
pub struct Quiz {
    pub question: String,
    pub options: Vec<String>,
    pub correct_answer: String,
}

impl Quiz {
    pub fn new() -> Self {
        Quiz {
            question: String::new(),
            options: Vec::new(),
            correct_answer: String::new(),
        }
    }
}

pub struct QuizProvider {
    pub quizzes : Mutex<Vec<Quiz>>,
}

impl QuizProvider {
    pub fn load_quizzes(&self, data: JsValue){
        let vec: Vec<Quiz> = from_value(data).expect("Failed to parse JSON");
        let mut quizzes = self.quizzes.lock().unwrap();
        *quizzes = vec;
    }

    pub fn select_random_quiz(&self, index : usize) -> Option<Quiz> {
        let quizzes = self.quizzes.lock().unwrap();
        quizzes.get(index).cloned()
    }
    
    pub fn get_random_index(&self, history : &History) -> usize {
        let quizzes = self.quizzes.lock().unwrap();

        let displayed = history.get_quiz_numbers();
        let hash_displayed : HashSet<usize> = HashSet::from_iter(displayed.iter().cloned());
    
        let length = quizzes.len();
    
        let range: HashSet<usize> = (0..=quizzes.len() - 1).collect();
    
        let available_numbers: Vec<usize> = 
            range.difference(&hash_displayed)
            .copied()
            .collect();
    
        if available_numbers.is_empty() {
            return length;
        }
    
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..available_numbers.len()); 
        let random_index = available_numbers[idx];
    
        random_index   
    }
}