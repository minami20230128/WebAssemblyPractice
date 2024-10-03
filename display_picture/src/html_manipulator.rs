use crate::quiz_provider::Quiz;
use crate::quiz_provider::QuizProvider;
use crate::answer::Record;
use crate::answer::History;
use js_sys::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlElement;
use wasm_bindgen::closure::Closure;
use web_sys::MouseEvent;
use web_sys::NodeList;
use wasm_bindgen::JsCast;
use std::sync::Mutex;

extern "C" {
    fn logInfo(message: &str);
    fn logError(message: &str);
}

static QUIZ_PROVIDER: QuizProvider = QuizProvider {
    quizzes: Mutex::new(Vec::new()),
};

static HISTORY: History = History {
    records: Mutex::new(Vec::new()),
};

pub struct HtmlManipulator;

impl HtmlManipulator  {
    fn get_document(&self) -> Option<Document> {
        return web_sys::window().unwrap().document();
    }

    fn get_parent_div(&self, document : &Document) -> Option<Element>{
        return document.get_element_by_id("parent");
    }

    fn put_p(&self, inner_html : &str, is_hidden : bool, id : &str) -> Result<(), JsValue> {
        let document = self.get_document().unwrap();
        let div = self.get_parent_div(&document).unwrap();
        let p = document.create_element("p").unwrap();
        p.set_inner_html(inner_html);
        if is_hidden {
            p.set_attribute("hidden", "");
        }
        
        if !id.is_empty() {
            p.set_id(id);
        }

        div.append_child(&p).unwrap();

        Ok(())
    }

    fn get_p(&self, id : &str) -> Option<Element>
    {
        let document = web_sys::window().unwrap().document().unwrap();
        document.get_element_by_id(&id)
    }

    pub fn init_quizzes(&self, data : JsValue) {
        QUIZ_PROVIDER.load_quizzes(data);

        let index = QUIZ_PROVIDER.get_random_index(&HISTORY);

        match QUIZ_PROVIDER.select_random_quiz(index) {
            Some(quiz) => {
                self.put_quiz(&quiz, index);
            }
            None => self.show_score_screen().unwrap(),
        }
    }


    pub fn put_quiz(&self, quiz : &Quiz, index : usize){
        self.put_question(quiz);
        self.put_options(quiz);
        self.put_answer(quiz);
        self.put_index(index);
    }

    fn put_button(inner_html : &str) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let button = document.create_element("button")?.dyn_into::<HtmlElement>()?;
        button.set_inner_html(inner_html);

        let closure = Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
            let target = mouse_event.target().unwrap();
            let button = target.dyn_into::<HtmlElement>().unwrap();
            let inner_html = button.inner_html();
            Self::event(inner_html);
        })  as Box<dyn Fn(MouseEvent)>);

        button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget(); // ClosureをJavaScriptで保持させる
    
        // <body>にボタンを追加
        document.body().unwrap().append_child(&button)?;
    
        Ok(())
    }

    fn put_question(&self, quiz : &Quiz) -> Result<(), JsValue> {
        self.put_p(&quiz.question, false, "")?;

        Ok(())
    }

    fn put_options(&self, quiz : &Quiz) -> Result<(), JsValue> {
        let answers = &quiz.options;

        for answer in answers.iter() {
            // JavaScript 側にボタンを作成するための関数を呼び出す
            let value = Self::put_button(answer);
            value?;
        }

        Ok(())
    }
    
    fn put_answer(&self, quiz : &Quiz) -> Result<(), JsValue> {
        self.put_p(&quiz.correct_answer, true, "answer")?;

        Ok(())
    }

    fn put_index(&self, index : usize) -> Result<(), JsValue> {
        self.put_p(&index.to_string(), true, "index")?;
        
        Ok(())
    }

    pub fn get_prev_quiz_index(&self) -> usize {
        let index_elem = self.get_p("index").unwrap();
        let index_str = index_elem.inner_html();
        return index_str.parse().expect("変換に失敗しました");
    }

    pub fn event(response : String) -> Result<(), JsValue> {

        let record = Record {
            quiz_number : Self.get_prev_quiz_index(),
            users_choice : response.clone(),
            is_correct : Self.check_answer(response),
        };

        HISTORY.add(record);

        match Self.remove_question() {
            Ok(_) => {},
            Err(e) => return Err(JsValue::from_str(&format!("Error removing buttons: {:?}", e))),
        }

        match Self.remove_all_buttons() {
            Ok(_) => {},
            Err(e) => return Err(JsValue::from_str(&format!("Error removing buttons: {:?}", e))),
        }

        let index = QUIZ_PROVIDER.get_random_index(&HISTORY);

        match QUIZ_PROVIDER.select_random_quiz(index) {
            Some(quiz) => {
                Self.put_quiz(&quiz, index);
            }
            None => Self.show_score_screen().unwrap(),
        }

        Ok(())
    }

    pub fn show_score_screen(&self) -> Result<(), JsValue> {
        let score = HISTORY.calc_score();
        let format = format!("{}問正解", &score.to_string());
        self.put_p(&format, false, "")?;

        Ok(())
    }

    pub fn check_answer(&self, response : String) -> bool {
        let answer_elem = self.get_p("answer").unwrap();
        let answer = answer_elem.inner_html();

        return response == answer;
    }

    pub fn remove_question(&self)-> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();

        // ページ内のすべてのボタン要素を取得
        let buttons:NodeList = document.query_selector_all("p").unwrap();

        // ボタン要素を削除する
        for i in (0..buttons.length()).rev() {
            if let Some(button) = buttons.get(i) {
                if let Some(button_element) = button.dyn_ref::<HtmlElement>() {
                    button_element.remove();
                }
            }
        }

        Ok(())
    }

    // ページ内のすべてのボタンを削除する関数
    pub fn remove_all_buttons(&self) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();

        // ページ内のすべてのボタン要素を取得
        let buttons:NodeList = document.query_selector_all("button").unwrap();

        // ボタン要素を削除する
        for i in (0..buttons.length()).rev() {
            if let Some(button) = buttons.get(i) {
                if let Some(button_element) = button.dyn_ref::<HtmlElement>() {
                    button_element.remove();
                }
            }
        }

        Ok(())
    }
}