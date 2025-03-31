mod controllers;

pub use crate::controllers::*;

use std::sync::Arc;

use http_client::{http::Uri, HttpClient};

pub struct Ollama {
    uri: Uri,
    http_client: Arc<dyn HttpClient>,
}

pub struct OllamaBuilder {
    uri: Option<Uri>,
    http_client: Option<Arc<dyn HttpClient>>,
}

impl Ollama {
    pub fn builder() -> OllamaBuilder {
        OllamaBuilder {
            uri: None,
            http_client: None,
        }
    }
}

impl OllamaBuilder {
    pub fn with_http_client(&mut self, http_client: Arc<dyn HttpClient>) -> &mut Self {
        self.http_client = Some(http_client);
        self
    }

    pub fn with_uri<U: Into<Uri>>(&mut self, uri: U) -> &mut Self {
        self.uri = Some(uri.into());
        self
    }

    pub fn build(&self) -> Ollama {
        Ollama {
            uri: self
                .uri
                .to_owned()
                .unwrap_or_else(|| "http://localhost:11434/".parse().unwrap()),
            http_client: self.http_client.to_owned().unwrap(),
        }
    }
}
