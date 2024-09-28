use std::sync::Mutex;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct History {
    quiz_number : usize,
    users_choice : String,
    is_correct : bool,
}

pub struct Answer {
    histories : Mutex<Vec<History>>,
}

impl Answer {
    pub fn add(&self, history : History) {
        let mut histories = self.histories.lock().unwrap();
        histories.push(history);
    }

    pub fn get_quiz_numbers(&self) -> Vec<usize> {
        let mut histories = self.histories.lock().unwrap();
        let quiz_numbers : Vec<usize> = histories.iter()
        .map(|history| history.quiz_number)
        .collect();

        return quiz_numbers;
    }
}