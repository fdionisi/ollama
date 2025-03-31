use std::{
    pin::Pin,
    task::{Context, Poll},
};

use anyhow::Result;
use futures::Stream;
use http_client::{Request, RequestBuilderExt, ResponseAsyncBodyExt};

use crate::Ollama;

impl Ollama {
    pub async fn completion(&self, request: GenerateRequest) -> Result<GenerateResponse> {
        let response = self
            .http_client
            .send(
                Request::builder()
                    .method("POST")
                    .uri(format!("{}api/generate", self.uri))
                    .json(request)?,
            )
            .await?;

        Ok(GenerateResponse(response.stream_json()))
    }
}

#[derive(serde::Serialize)]
pub struct GenerateRequest {
    model: String,
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<serde_json::Value>,
    stream: bool,
}

pub struct GenerateResponse(Pin<Box<dyn Stream<Item = Result<GenerateEvent>> + Send>>);

impl Stream for GenerateResponse {
    type Item = Result<GenerateEvent>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.as_mut().poll_next(cx)
    }
}

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct GenerateEvent {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
}
