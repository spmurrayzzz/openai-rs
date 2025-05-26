use reqwest::Client as HttpClient;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use std::sync::Arc;

use crate::error::Error;
use crate::models::chat::{ChatCompletionRequest, ChatCompletionResponse};

#[derive(Clone)]
pub struct OpenAIClient {
    base_url: Arc<String>,
    http_client: HttpClient,
}

impl OpenAIClient {
    pub fn new<S: Into<String>>(api_key: S) -> Self {
        let api_key = api_key.into();
        let base_url = "https://api.openai.com/v1".to_string();

        let mut headers = HeaderMap::new();
        let bearer = format!("Bearer {}", api_key);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&bearer).expect("Invalid API key"),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let http_client = HttpClient::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client");

        OpenAIClient {
            base_url: Arc::new(base_url),
            http_client,
        }
    }

    pub async fn create_chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, Error> {
        let url = format!("{}/chat/completions", self.base_url);

        let response = self
            .http_client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(Error::Http)?;

        let status = response.status();
        let text = response.text().await.map_err(Error::Http)?;

        if !status.is_success() {
            let api_error = serde_json::from_str::<serde_json::Value>(&text)
                .ok()
                .and_then(|v| {
                    v.get("error")
                        .and_then(|e| e.get("message"))
                        .and_then(|m| m.as_str())
                        .map(|s| s.to_string())
                });
            let msg = api_error.unwrap_or_else(|| text.clone());
            return Err(Error::Api(msg));
        }

        let completion: ChatCompletionResponse =
            serde_json::from_str(&text).map_err(Error::Serialization)?;

        Ok(completion)
    }
}
