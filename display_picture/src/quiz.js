class Quiz {
    constructor(question, options, answer) {
        this.question = question;
        this.options = options;
        this.answer = answer;
    }

    toJson() {
        return JSON.stringify({
            question: this.question,
            options: this.options,
            correct_answer: this.answer
        });
    }
}

export async function sendRandomQuizToRust() {
    if (quizzes.length === 0) {
        console.error("No quizzes available.");
        return;
    }
    const randomIndex = Math.floor(Math.random() * quizzes.length);
    const randomQuiz = quizzes[randomIndex];
    const wasm = await import('../src/index.js');
    wasm.receiveQuizData(randomQuiz.toJson());
}

// グローバル変数として quizzes を保持
let quizzes = [];

// JSON ファイルを読み込んで quizzes を初期化
export async function initializeQuizzes(url) {
    try {
        const response = await fetch(url);
        const data = await response.json();
        quizzes = data.map(item => new Quiz(item.question, item.answer));
    } catch (error) {
        console.error("Error initializing quizzes:", error);
    }
}