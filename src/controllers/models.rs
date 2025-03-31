use std::{
    pin::Pin,
    task::{Context, Poll},
};

use anyhow::{Error, Result};
use futures::Stream;
use http_client::{
    http::request::Builder as RequestBuilder, RequestBuilderExt, ResponseAsyncBodyExt,
};

use crate::Ollama;

impl Ollama {
    pub async fn pull_model(
        &self,
        model_name: String,
        allow_insecure: bool,
    ) -> Result<PullModelResponse> {
        let response = self
            .http_client
            .send(
                RequestBuilder::new()
                    .method("POST")
                    .uri(format!("{}api/pull", self.uri))
                    .json(PullModelRequest {
                        name: model_name,
                        insecure: allow_insecure,
                        stream: true,
                    })?,
            )
            .await?;

        Ok(PullModelResponse(response.stream_json()))
    }

    pub async fn list_local_models(&self) -> Result<ListLocalModelsResponse> {
        Ok(self
            .http_client
            .send(
                RequestBuilder::new()
                    .method("GET")
                    .uri(format!("{}api/tags", self.uri))
                    .end()?,
            )
            .await?
            .json::<ListLocalModelsResponse>()
            .await?)
    }

    pub async fn delete_model(&self, model_name: String) -> Result<()> {
        Ok(self
            .http_client
            .send(
                RequestBuilder::new()
                    .method("DELETE")
                    .uri(format!("{}api/delete", self.uri))
                    .json(DeleteModelRequest { name: model_name })?,
            )
            .await?
            .json::<()>()
            .await?)
    }

    pub async fn show_model_info(&self, model_name: String) -> Result<ModelInfo> {
        Ok(self
            .http_client
            .send(
                RequestBuilder::new()
                    .method("POST")
                    .uri(format!("{}api/show", self.uri))
                    .json(ModelInfoRequest { name: model_name })?,
            )
            .await?
            .json::<ModelInfo>()
            .await?)
    }

    pub async fn copy_model(&self, source: String, destination: String) -> Result<()> {
        Ok(self
            .http_client
            .send(
                RequestBuilder::new()
                    .method("POST")
                    .uri(format!("{}api/show", self.uri))
                    .json(CopyModelRequest {
                        source,
                        destination,
                    })?,
            )
            .await?
            .json::<()>()
            .await?)
    }
}

#[derive(serde::Serialize)]
struct CopyModelRequest {
    source: String,
    destination: String,
}

#[derive(serde::Serialize)]
struct ModelInfoRequest {
    name: String,
}

#[derive(serde::Serialize)]
struct DeleteModelRequest {
    name: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ListLocalModelsResponse {
    pub models: Vec<LocalModel>,
}

pub struct PullModelResponse(Pin<Box<dyn Stream<Item = Result<PullModelEvent, Error>> + Send>>);

impl Stream for PullModelResponse {
    type Item = Result<PullModelEvent>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.as_mut().poll_next(cx)
    }
}

impl Extend<PullModelEvent> for PullModelEvent {
    fn extend<T: IntoIterator<Item = PullModelEvent>>(&mut self, iter: T) {
        for event in iter {
            self.status = event.status;
            self.digest = event.digest;
            self.total = event.total.or(self.total);
            self.completed = event.completed.or(self.completed);
        }
    }
}

#[derive(Clone, Debug, serde::Serialize)]
struct PullModelRequest {
    name: String,
    insecure: bool,
    stream: bool,
}

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct PullModelEvent {
    pub status: String,
    pub digest: Option<String>,
    pub total: Option<u64>,
    pub completed: Option<u64>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct LocalModel {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ModelInfo {
    pub license: Option<String>,
    pub modelfile: Option<String>,
    pub parameters: Option<String>,
    pub template: Option<String>,
}
