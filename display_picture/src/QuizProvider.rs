pub struct QuizProvider {
    pub quizzes : Mutex<Vec<Quiz>>,
}

static INSTANCE: QuizProvider = QuizProvider {
    pub quizzes: Mutex::new(Vec::new()),
};

impl QuizProvider {
    load_quizzes(&self, data: JsValue){
        let vec: Vec<Quiz> = from_value(data).expect("Failed to parse JSON");
        let mut quizzes = self.quizzes.lock.unwrap();
        *quizzes = vec;
    }
}