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

    toString() {
        console.log(this.question);
        console.log(this.options);
        console.log(this.answer);
    }
}

class QuizProvider {
    constructor(quizzes) {
        this.quizzes = quizzes;
        this.displayedIndexes = []; // Array to keep track of displayed quiz indexes in order
    }

    getRemainingQuizzes() {
        // Filter out quizzes that have been displayed
        return this.quizzes.filter((_, index) => !this.displayedIndexes.some(displayed => displayed.index === index));
    }

    sendRandomQuizToRust() {
        console.log("sendRandomQuizToRust start");

        const remainingQuizzes = this.getRemainingQuizzes();
        if (remainingQuizzes.length === 0) {
            console.log("All quizzes have been displayed.");
            return "end"; // Return a signal that all quizzes are completed
        }

        const randomIndex = Math.floor(Math.random() * remainingQuizzes.length);
        const quizIndex = this.quizzes.indexOf(remainingQuizzes[randomIndex]); // Find the original index
        const randomQuiz = this.quizzes[quizIndex];
        
        console.log(randomIndex);
        console.log(randomQuiz);
        randomQuiz.toString();

        // Add to displayedIndexes in the order they are displayed
        this.displayedIndexes.push({ index: quizIndex, quiz: randomQuiz });

        return JSON.stringify({
            question: randomQuiz.question,
            options: randomQuiz.options,
            correct_answer: randomQuiz.answer
        });
    }
}

window.handleQuizResultSync = function() {
    // Call the function and handle the result
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
