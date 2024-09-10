pub struct History {
    quiz_number : usize,
    users_answer : i32,
    is_correct : bool,
}

pub struct Result {
    histories : Mutex<Vec<History>>,
}

static INSTANCE: Result = Result {
    histories: Mutex::new(Vec::new()),
};

impl Result {
    pub fn add(&self, history : Histiry) {
        let mut histories = self.histories.lock().unwrap();
        histories.push(history);
    }

    pub fn quiz_numbers() -> Vec<usize> {
        let mut histories = self.histories.lock().unwrap();
        let quiz_numbers : Vec<usize> = histories.iter()
        .map(|&history| history.quiz_number)
        .collect();

        return quiz_numbers;
    }
}