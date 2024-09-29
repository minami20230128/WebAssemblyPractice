use std::sync::Mutex;

pub struct Record {
    quiz_number : usize,
    users_choice : String,
    is_correct : bool,
}

pub struct History {
    records : Mutex<Vec<Record>>,
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
}