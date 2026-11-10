pub struct AITranslate {
    input_lang: String,
    allowed_langs: Vec<String>,
}

impl AITranslate {
    /// 建構子：初始化一個預設語言為 "en"，且允許語言為 ["en", "ja"] 的物件
    pub fn new() -> Self {
        Self {
            input_lang: "en".to_string(),
            allowed_langs: vec!["en".to_string(), "ja".to_string()],
        }
    }

    /// 設定輸入語言。會檢查輸入的語言是否在 `allowed_langs` 陣列中。
    pub fn set_input_lang(&mut self, lang: String) -> bool {
        if self.allowed_langs.contains(&lang) {
            self.input_lang = lang;
            true
        } else {
            false
        }
    }

    /// 取得目前的語言
    pub fn get_input_lang(&self) -> &str {
        &self.input_lang
    }

    /// 讓使用者可以動態增加允許的語言
    pub fn add_allowed_lang(&mut self, lang: String) {
        if !self.allowed_langs.contains(&lang) {
            self.allowed_langs.push(lang);
        }
    }

    /// 取得目前所有允許的語言清單
    pub fn get_allowed_langs(&self) -> &[String] {
        &self.allowed_langs
    }
}
