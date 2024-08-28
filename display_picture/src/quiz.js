//import { createRequire } from 'module';

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

class QuizProvider {
    constructor(quizzes){
        this.quizzes = quizzes
    }

    async sendRandomQuizToRust() {
        printQuizzes();
        if (this.quizzes.length === 0) {
            console.error("No quizzes available.");
            return;
        }
        const randomIndex = Math.floor(Math.random() * this.quizzes.length);
        const randomQuiz = this.quizzes[randomIndex];
        const wasm = await import('../src/index.js');
        wasm.receiveQuizData(randomQuiz.toJson());
    }
}

// グローバル変数として quizzes を保持
let quizzes = [];

export function printQuizzes() {
    quizzes.forEach((quiz, index) => {
        console.log(`Quiz ${index + 1}:`);
        console.log(`Question: ${quiz.question}`);
        console.log(`Options: ${quiz.options.join(",")}`);
        console.log(`Correct Answer: ${quiz.answer}`);
        console.log('---');
    });
}

// JSON ファイルを読み込んで quizzes を初期化

export async function initializeQuizzes() {
    const url = "../src/questions.json";

    try {
        // fetch を使ってローカルの JSON ファイルを取得
        const response = await fetch(url);

        // レスポンスが OK かどうかを確認
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }

        // レスポンスを JSON 形式に変換
        const data = await response.json();
        quizzes = data.map(item => new Quiz(item.question, item.options, item.correct_answer));
        quizProvider = new QuizProvider(quizzes);

        window.QuizProvider = quizProvider;

        console.log(quizzes);

    } catch (error) {
        // エラー処理
        console.error('There has been a problem with your fetch operation:', error);
    }
}