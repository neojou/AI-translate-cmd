use crate::translator::{Translator, TranslatorError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChatMessageResponse,
}

#[derive(Deserialize)]
struct ChatMessageResponse {
    content: String,
}

pub struct OllamaTranslator {
    base_url: String,
    model: String,
    client: Client,
    system_prompt_template: &'static str,
}

impl OllamaTranslator {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            base_url,
            model,
            client: Client::new(),
            system_prompt_template: include_str!("AI-commands/translate.md"),
        }
    }
}

#[async_trait]
impl Translator for OllamaTranslator {
    async fn translate(&self, text: &str, source_lang: &str, _target_lang: &str) -> Result<String, TranslatorError> {
        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));

        let system_prompt = self
            .system_prompt_template
            .replace("{{source_language}}", source_lang)
            .replace("{{passage}}", text);

        let request_body = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: "Please perform the translation as instructed.".to_string(),
                },
            ],
        };

        let response = self.client
            .post(url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| TranslatorError::NetworkError(e.to_string()))?;

        if response.status().is_success() {
            let res_body: ChatCompletionResponse = response
                .json()
                .await
                .map_err(|e| TranslatorError::SerializationError(e.to_string()))?;

            if let Some(choice) = res_body.choices.first() {
                Ok(choice.message.content.trim().to_string())
            } else {
                Err(TranslatorError::ApiError("No choices returned from Ollama".to_string()))
            }
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(TranslatorError::ApiError(format!("Ollama API error (status {}): {}", status, error_text)))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn prompt_is_embedded() {
        let raw = include_str!("AI-commands/translate.md");
        assert!(!raw.trim().is_empty());
    }
}
