use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TranslatorError {
    #[error("API error: {0}")]
    ApiError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

#[async_trait]
pub trait Translator: Send + Sync {
    /// 執行翻譯作業
    /// # Arguments
    /// * `text` - 要翻譯的原始文本
    /// * `source_lang` - 原始語言代碼 (例如 "en")
    /// * `target_lang` - 目標語言代碼 (例如 "zh")
    async fn translate(&self, text: &str, source_lang: &str, target_lang: &str) -> Result<String, TranslatorError>;
}

/// 用於測試或開發階段的虛擬翻譯器
pub struct DummyTranslator;

#[async_trait]
impl Translator for DummyTranslator {
    async fn translate(&self, text: &str, _source_lang: &str, _target_lang: &str) -> Result<String, TranslatorError> {
        Ok(format!("[Dummy Translation] {}", text))
    }
}
