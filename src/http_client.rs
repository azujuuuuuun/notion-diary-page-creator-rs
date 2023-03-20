use std::error::Error;

use async_trait::async_trait;
use mockall::automock;
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new(client: reqwest::Client) -> Self {
        HttpClient { client }
    }
}

#[automock]
#[async_trait]
pub trait HttpClientTrait {
    async fn post<S: Serialize + Sync + 'static, T: DeserializeOwned + 'static>(
        &self,
        url: &str,
        headers: HeaderMap,
        json: &S,
    ) -> Result<T, Box<dyn Error>>;
}

#[async_trait]
impl HttpClientTrait for HttpClient {
    async fn post<S: Serialize + Sync + 'static, T: DeserializeOwned + 'static>(
        &self,
        url: &str,
        headers: HeaderMap,
        json: &S,
    ) -> Result<T, Box<dyn Error>> {
        let resp = self
            .client
            .post(url)
            .headers(headers)
            .json(json)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(resp)
    }
}
