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

    //const require = createRequire(import.meta.url);
    //const fs = require('fs');

    console.log("initialize quizzes");
    const url = "../src/questions.json";
    //const data = fs.readFileSync(url, 'utf8');

    let data;

    fetch(url)
    .then(response => response.json())
    .then(data => {
        console.log('JSONファイルの内容:');
        console.log(data);
    })
    .catch(error => {
        console.error('エラー:', error);
    });
    
    // Quizクラスの配列を作成する
    const quizzes = data.map(item => new Quiz(item.question, item.options, item.correct_answer));
    
    return quizzes;
}