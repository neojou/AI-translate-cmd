use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum TranslateError {
    #[error("Language '{0}' is not in the allowed list: {1:?}")]
    NotAllowedLanguage(String, Vec<String>),
}

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

    /// 設定輸入語言。如果語言不在 `allowed_langs` 中，則回傳錯誤。
    pub fn set_input_lang(&mut self, lang: String) -> Result<(), TranslateError> {
        if self.allowed_langs.contains(&lang) {
            self.input_lang = lang;
            Ok(())
        } else {
            Err(TranslateError::NotAllowedLanguage(lang, self.allowed_langs.clone()))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_initialization() {
        let translator = AITranslate::new();
        assert_eq!(translator.get_input_lang(), "en");
        assert_eq!(translator.get_allowed_langs(), &vec!["en".to_string(), "ja".to_string()]);
    }

    #[test]
    fn test_set_input_lang_success() {
        let mut translator = AITranslate::new();
        assert!(translator.set_input_lang("ja".to_string()).is_ok());
        assert_eq!(translator.get_input_lang(), "ja");
    }

    #[test]
    fn test_set_input_lang_failure() {
        let mut translator = AITranslate::new();
        let result = translator.set_input_lang("zh".to_string());
        assert!(result.is_err());
        if let Err(TranslateError::NotAllowedLanguage(lang, allowed)) = result {
            assert_eq!(lang, "zh");
            assert!(allowed.contains(&"en".to_string()));
            assert!(allowed.contains(&"ja".to_string()));
        } else {
            panic!("Expected NotAllowedLanguage error");
        }
    }

    #[test]
    fn test_add_allowed_lang() {
        let mut translator = AITranslate::new();
        translator.add_allowed_lang("zh".to_string());
        assert!(translator.get_allowed_langs().contains(&"zh".to_string()));
        // Ensure no duplicates
        translator.add_allowed_lang("zh".to_string());
        assert_eq!(translator.get_allowed_langs().len(), 3);
    }
}
