use std::sync::Mutex;
use wasm_bindgen::JsValue;
use serde_json::from_value;
use std::collections::HashSet;
use wasm_bindgen::JsCast;

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
    quizzes : Mutex<Vec<Quiz>>,
}

impl QuizProvider {
    pub fn load_quizzes(&self, data: JsValue){
        let vec: Vec<Quiz> = from_value(data).expect("Failed to parse JSON");
        let mut quizzes = self.quizzes.lock.unwrap();
        *quizzes = vec;
    }

    pub fn select_random_quiz(&self, index : usize) -> Option<Quiz> {
        self.quizzes.get(index).cloned()
    }
    
    pub fn get_random_index(&self, result : Result) -> usize {
        let quizzes = self.quizzes.lock().unwrap();

        let mut displayed = result.get_quiz_numbers();
        logInfo("get_random_index started");
        let hash_displayed : HashSet<usize> = HashSet::from_iter(displayed.iter().cloned());
    
        let length = quizzes.len();
        logInfo(&length.to_string());
    
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