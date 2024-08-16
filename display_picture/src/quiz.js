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
    printQuizzes();
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
export function initializeQuizzes() {
    const fs = require('fs/promises');
    const url = "question.json";
    const data = fs.readFileSync(url, 'utf8');
    
    // JSONデータをパースしてオブジェクトの配列に変換する
    const quizzesData = JSON.parse(data);
    
    // Quizクラスの配列を作成する
    const quizzes = quizzesData.map(item => new Quiz(item.question, item.options, item.answer));
    
    return quizzes;
}