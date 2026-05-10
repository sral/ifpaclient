use serde::de::DeserializeOwned;

use crate::IfpaError;

const BASE_URL: &str = "https://api.ifpapinball.com";

pub struct IfpaClient {
    pub(crate) http: reqwest::Client,
    pub(crate) api_key: String,
    pub(crate) base_url: String,
}

impl IfpaClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            api_key: api_key.into(),
            base_url: BASE_URL.to_string(),
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub(crate) fn request(&self, path: &str) -> reqwest::RequestBuilder {
        self.http
            .get(format!("{}{}", self.base_url, path))
            .query(&[("api_key", &self.api_key)])
    }

    pub(crate) async fn send<T: DeserializeOwned>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<T, IfpaError> {
        let response = request.send().await?;
        let status = response.status();
        if !status.is_success() {
            let message = response.text().await.unwrap_or_default();
            return Err(IfpaError::Api {
                status: status.as_u16(),
                message,
            });
        }
        Ok(response.json().await?)
    }
}
