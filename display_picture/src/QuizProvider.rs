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

static INSTANCE: QuizProvider = QuizProvider {
    quizzes: Mutex::new(Vec::new()),
};

impl QuizProvider {
    pub fn load_quizzes(&self, data: JsValue){
        let vec: Vec<Quiz> = from_value(data).expect("Failed to parse JSON");
        let mut quizzes = self.quizzes.lock.unwrap();
        *quizzes = vec;
    }
}