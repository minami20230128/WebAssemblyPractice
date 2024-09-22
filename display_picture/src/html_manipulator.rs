pub fn HtmlManipulator {

}

impl HtmlManipulator  {
    pub fn put_quiz(&self, quiz : Quiz, index : i32){
        self.put_question(quiz.question);
        self.put_options(quiz.options);
        self.put_answer(quiz.answer);
        self.put_index(index);
    }

    fn put_button(&self, inner_html : &str, quiz_provider : QuizProvider, result : Result) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
    
        // 新しいボタンを作成
        let button = document.create_element("button")?.dyn_into::<HtmlElement>()?;
        button.set_id("add_soft_button");
        button.set_inner_html(inner_html);
    
        // ボタンにクリックイベントを設定
        let button_clone = button.clone();
        let closure = Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
            let target = mouse_event.target().unwrap();
            let button = target.dyn_into::<HtmlElement>().unwrap();
            let inner_html = button.inner_html();
            self.event(inner_html, quiz_provider, result);
        })  as Box<dyn Fn(MouseEvent)>);
        button.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget(); // ClosureをJavaScriptで保持させる
    
        // <body>にボタンを追加
        document.body().unwrap().append_child(&button)?;
    
        Ok(())
    }

    fn put_question(&self, quiz : Quiz) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let div = document.get_element_by_id("parent").unwrap();
        let p = document.create_element("p").unwrap();
        p.set_inner_html(&quiz.question);
        div.append_child(&p).unwrap();

        Ok(())
    }

    fn put_options(&self, quiz : Quiz) -> Result<(), JsValue> {
        let answers = quiz.options;

        for answer in answers.iter() {
            // JavaScript 側にボタンを作成するための関数を呼び出す
            self.put_button(answer)?;
        }

        Ok(())
    }
    
    fn put_answer(&self, quiz : Quiz) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let div = document.get_element_by_id("parent").unwrap();
        let p = document.create_element("p").unwrap();
        p.set_inner_html(&quiz.correct_answer);
        p.set_attribute("hidden", "")?;
        p.set_id("answer");
        div.append_child(&p).unwrap();
        
        Ok(())
    }

    fn put_index(&self, index : usize) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let div = document.get_element_by_id("parent").unwrap();
        let p = document.create_element("p").unwrap();
        p.set_inner_html(&index.to_string());
        p.set_id("index");
        div.append_child(&p).unwrap();
        
        Ok(())
    }

    #[wasm_bindgen]
    pub fn get_prev_quiz_index() -> usize {
        let document = web_sys::window().unwrap().document().unwrap();
        let div = document.get_element_by_id("parent").unwrap();
        let index_elem = document.get_element_by_id("index").unwrap();
        let index_str = index_elem.inner_html();
        return index_str.parse().expect("変換に失敗しました");
    }


    #[wasm_bindgen]
    pub fn event(response : String, quiz_provider : QuizProvider, result : Result) -> Result<(), JsValue> {

        logInfo("event started");

        let history = History {
            quiz_number : self.get_prev_quiz_index(),
            users_choice : response,
            is_correct : self.check_answer(response),
        }

        result.add(history);

        match self.remove_question() {
            Ok(_) => {},
            Err(e) => return Err(JsValue::from_str(&format!("Error removing buttons: {:?}", e))),
        }

        match self.remove_all_buttons() {
            Ok(_) => {},
            Err(e) => return Err(JsValue::from_str(&format!("Error removing buttons: {:?}", e))),
        }

        let index = quiz_provider.get_random_index(result);

        match quiz_provider.select_random_quiz(index) {
            Some(quiz) => {
                logInfo(&quiz.question);
                self.put_quiz(quiz, index);
            }
            None => show_score_screen().unwrap(),
        }

        Ok(())
    }

    #[wasm_bindgen]
    pub fn show_score_screen() -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let div = document.get_element_by_id("parent").unwrap();
        let p = document.create_element("p").unwrap();
        p.set_inner_html("end");
    
        //let score_elem = document.create_element("p").unwrap();
        //let mut score = SCORE.lock().unwrap();
        //let result = format!("{}問正解", &score.to_string());
        //score_elem.set_inner_html(&result);
        div.append_child(&p).unwrap();
        Ok(())
    }

    #[wasm_bindgen]
    pub fn check_answer(&self, response : String) -> bool {
        let document = web_sys::window().unwrap().document().unwrap();
        let div = document.get_element_by_id("parent").unwrap();
        let answer_elem = document.get_element_by_id("answer").unwrap();
        let answer = answer_elem.inner_html();

        return response == answer;
    }

    #[wasm_bindgen]
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
    #[wasm_bindgen]
    pub fn remove_all_buttons() -> Result<(), JsValue> {
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