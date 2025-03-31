use std::{
    pin::Pin,
    task::{Context, Poll},
};

use anyhow::Result;
use futures::Stream;
use http_client::{
    http::request::Builder as RequestBuilder, RequestBuilderExt, ResponseAsyncBodyExt,
};

use crate::Ollama;

impl Ollama {
    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let response = self
            .http_client
            .send(
                RequestBuilder::new()
                    .method("POST")
                    .uri(format!("{}api/chat", self.uri))
                    .json(request)?,
            )
            .await?;

        Ok(ChatResponse(response.stream_json()))
    }
}

#[derive(serde::Serialize)]
pub struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<serde_json::Value>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    keep_alive: Option<String>,
}

pub struct ChatResponse(Pin<Box<dyn Stream<Item = Result<ChatEvent>> + Send>>);

impl Stream for ChatResponse {
    type Item = Result<ChatEvent>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.as_mut().poll_next(cx)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ChatEvent {
    pub model: String,
    pub created_at: String,
    pub message: ChatMessage,
    pub done: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_eval_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_eval_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_duration: Option<u64>,
}
