use std::sync::Mutex;

pub struct Record {
    pub quiz_number : usize,
    pub users_choice : String,
    pub is_correct : bool,
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

    pub fn calc_score(&self) ->usize {
        let records = self.records.lock().unwrap();
        let score = records.iter()
        .filter(|record| record.is_correct).count();

        return score;
    }
}