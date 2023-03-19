use std::error::Error;

use async_trait::async_trait;
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct HttpClient {
    client: reqwest::Client,
}

#[async_trait]
pub trait HttpClientTrait {
    fn new() -> Self;

    async fn post<S: Serialize + ?Sized + Sync, T: DeserializeOwned>(
        &self,
        url: &str,
        headers: HeaderMap,
        json: &S,
    ) -> Result<T, Box<dyn Error>>;
}

#[async_trait]
impl HttpClientTrait for HttpClient {
    fn new() -> Self {
        HttpClient {
            client: reqwest::Client::new(),
        }
    }

    async fn post<S: Serialize + ?Sized + Sync, T: DeserializeOwned>(
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
