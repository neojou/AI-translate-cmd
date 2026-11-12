use crate::translator::{Translator, TranslatorError};
use thiserror::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum TranslateError {
    #[error("Language '{0}' is not in the allowed list: {1:?}")]
    NotAllowedLanguage(String, Vec<String>),
    #[error("Translator error: {0}")]
    Translator(#[from] TranslatorError),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub struct AITranslate {
    input_lang: String,
    allowed_langs: Vec<String>,
    lang_map: HashMap<String, String>,
    translator: Box<dyn Translator>,
}

#[allow(dead_code)]
impl AITranslate {
    /// 建構子：初始化一個預設語言為 "英文"，且允許語言為 ["en", "ja"] 的物件，並注入翻譯器
    pub fn new(translator: Box<dyn Translator>) -> Self {
        let mut lang_map = HashMap::new();
        lang_map.insert("en".to_string(), "English".to_string());
        lang_map.insert("ja".to_string(), "Japanese".to_string());

        Self {
            input_lang: "English".to_string(),
            allowed_langs: vec!["en".to_string(), "ja".to_string()],
            lang_map,
            translator,
        }
    }

    /// 設定輸入語言。如果語言不在 `allowed_langs` 中，則回傳錯誤。
    /// 若語言在 `lang_map` 中有對應名稱，則會儲存名稱而非代碼。
    pub fn set_input_lang(&mut self, lang: String) -> Result<(), TranslateError> {
        if self.allowed_langs.contains(&lang) {
            let display_name = self.lang_map.get(&lang).cloned().unwrap_or(lang.clone());
            self.input_lang = display_name;
            Ok(())
        } else {
            Err(TranslateError::NotAllowedLanguage(lang, self.allowed_langs.clone()))
        }
    }

    pub fn get_input_lang(&mut self) -> &str {
        &self.input_lang
    }
    
    /// 單一文本翻譯 (內部使用)
    pub async fn translate(&self, text: &str) -> Result<String, TranslateError> {
        let target_lang = "zh-TW";
        let translated_text = self.translator.translate(text, &self.input_lang, target_lang).await?;
        Ok(translated_text)
    }

    /// 處理大型檔案翻譯
    pub async fn translate_file(&self, input_path: &str, output_path: &str) -> Result<(), TranslateError> {
        let input_file = File::open(input_path)
            .map_err(|e| TranslateError::IoError(e))?;
        
        let total_size = input_file.metadata()
            .map_err(|e| TranslateError::IoError(e))?.len();
        
        let mut reader = BufReader::new(input_file);
        let mut output_file = File::create(output_path)
            .map_err(|e| TranslateError::IoError(e))?;

        let mut current_segment = String::new();
        let mut processed_bytes = 0u64;
        let max_chunk_size = 2000; // 最大字元數限制

        let mut line = String::new();
        
        while reader.read_line(&mut line)? > 0 {
            let line_len = line.len() as u64;
            processed_bytes += line_len;
            
            // 1. 檢查是否為空行 (段落結束)
            if line.trim().is_empty() {
                if !current_segment.is_empty() {
                    self.process_segment(&mut current_segment, &mut output_file).await?;
                }
            } else {
                current_segment.push_str(&line);

                // 2. 檢查是否達到最大長度限制
                if current_segment.len() >= max_chunk_size {
                    // 嘗試尋找最近的標點符號來切斷，避免切斷句子
                    if let Some(split_idx) = self.find_last_sentence_end(&current_segment) {
                        let (to_translate, remaining) = current_segment.split_at(split_idx + 1);
                        self.process_segment(&mut to_translate.to_string(), &mut output_file).await?;
                        current_segment = remaining.to_string();
                    } else {
                        // 如果找不到句末標點，強制切斷以防記憶體溢出
                        self.process_segment(&mut current_segment, &mut output_file).await?;
                        current_segment.clear();
                    }
                }
            }

            // 3. 更新進度
            let progress = (processed_bytes as f64 / total_size as f64) * 100.0;
            print!("\r正在處理中... 進度: {:.2}%", progress);
            use std::io::Write;
            io::stdout().flush().unwrap();

            line.clear();
        }

        // 最後一段處理
        if !current_segment.is_empty() {
            self.process_segment(&mut current_segment, &mut output_file).await?;
        }

        println!("\n✅ 翻譯完成！");
        Ok(())
    }

    async fn process_segment(&self, segment: &mut String, output: &mut File) -> Result<(), TranslateError> {
        let trimmed = segment.trim();
        if !trimmed.is_empty() {
            let translated = self.translate(trimmed).await?;
            output.write_all(translated.as_bytes()).map_err(|e| TranslateError::IoError(e))?;
            output.write_all(b"\n\n").map_err(|e| TranslateError::IoError(e))?;
        }
        segment.clear();
        Ok(())
    }

    /// 尋找最近的句末標點符號
    fn find_last_sentence_end(&self, text: &str) -> Option<usize> {
        let punctuations = ['.', '!', '?', '。', '！', '？'];
        // 從後往前找
        for (i, c) in text.char_indices().rev() {
            if punctuations.contains(&c) {
                return Some(i);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockTranslator;

    #[async_trait::async_trait]
    impl Translator for MockTranslator {
        async fn translate(&self, text: &str, _source_lang: &str, _target_lang: &str) -> Result<String, TranslatorError> {
            Ok(format!("translated: {}", text))
        }
    }

}
