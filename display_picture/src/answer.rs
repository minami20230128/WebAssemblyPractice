use std::sync::Mutex;
use crate::quiz_provider::Quiz;

pub struct Record {
    pub quiz_number : usize,
    pub users_choice : String,
    pub is_correct : bool,
}

impl Record {
    pub fn set_is_correct(&mut self, is_correct: bool){
        self.is_correct = is_correct;
    }
}

pub struct History {
    pub records : Mutex<Vec<Record>>,
}

impl History {
    pub fn add(&self, record : Record) {
        let mut records = self.records.lock().unwrap();
        records.push(record);
    }

    pub fn get_quiz_numbers(&self) -> Vec<usize> {
        let mut records = self.records.lock().unwrap();
        let quiz_numbers : Vec<usize> = records.iter()
        .map(|record| record.quiz_number)
        .collect();

        return quiz_numbers;
    }

    pub fn calc_score(&self, quizzes: Vec<Quiz>) ->i32 {
        let mut score = 0;
        let mut records = self.records.lock().unwrap();
        for record in records.iter_mut(){
            let quiz = quizzes.get(record.quiz_number).unwrap();
            let is_correct = record.users_choice == quiz.correct_answer;
            record.set_is_correct(is_correct);

            if is_correct {
                score += quiz.score;
            }
        }
        
        return score;
    }
}