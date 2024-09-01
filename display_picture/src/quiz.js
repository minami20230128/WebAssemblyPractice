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

    toString(){
        console.log(this.question);
        console.log(this.options);
        console.log(this.answer);
    }
}

class QuizProvider {
    constructor(quizzes){
        this.quizzes = quizzes
    }

    sendRandomQuizToRust() {

        console.log("sendRandomQuizToRust start");
        //printQuizzes();
        if (this.quizzes.length === 0) {
            console.error("No quizzes available.");
            return;
        }
        const randomIndex = Math.floor(Math.random() * this.quizzes.length);
        console.log(randomIndex);
        const randomQuiz = this.quizzes[randomIndex];
        console.log(randomQuiz);
        randomQuiz.toString();

        return JSON.stringify({
            question: randomQuiz.question,
            options: randomQuiz.options,
            correct_answer: randomQuiz.answer
        });
    }
}

window.handleQuizResultSync = function() {
    // Call the async function and handle the promise
    return window.quizProvider.sendRandomQuizToRust();
};

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

window.quizProvider = null;

const embeddedQuizData = [
    { question: "What is 2+2?", options: ["3", "4"], correct_answer: "4" },
    { question: "What is 3+5?", options: ["7", "8"], correct_answer: "8" }
];


export function initializeQuizzes() {
    // Initialize quizzes from embedded data
    const quizzes = embeddedQuizData.map(item => new Quiz(item.question, item.options, item.correct_answer));
    window.quizProvider = new QuizProvider(quizzes);
    console.log("Quizzes initialized:", quizzes);
}

function logInfo(message) {
    console.info(message);
}

function logError(message) {
    console.error(message);
}

// Expose these functions globally
window.logInfo = logInfo;
window.logError = logError;