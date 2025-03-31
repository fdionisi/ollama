use anyhow::Result;
use http_client::{
    http::request::Builder as RequestBuilder, RequestBuilderExt, ResponseAsyncBodyExt,
};

use crate::Ollama;

impl Ollama {
    pub async fn embed(&self, request: EmbedRequest) -> Result<EmbedResponse> {
        let response = self
            .http_client
            .send(
                RequestBuilder::new()
                    .method("POST")
                    .uri(format!("{}api/embed", self.uri))
                    .json(request)?,
            )
            .await?;

        Ok(response.json::<EmbedResponse>().await?)
    }
}

#[derive(serde::Serialize)]
pub struct EmbedRequest {
    pub model: String,
    pub input: EmbedInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum EmbedInput {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EmbedResponse {
    pub model: String,
    pub embeddings: Vec<Vec<f32>>,
    pub total_duration: Option<u64>,
    pub load_duration: Option<u64>,
    pub prompt_eval_count: Option<u64>,
    pub prompt_eval_duration: Option<u64>,
    pub eval_count: Option<u64>,
    pub eval_duration: Option<u64>,
}
