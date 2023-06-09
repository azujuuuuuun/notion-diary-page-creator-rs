use std::{collections::HashMap, error::Error};

use async_trait::async_trait;
use http::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use mockall::automock;
use serde::{Deserialize, Serialize};

use crate::http_client::HttpClientTrait;

#[derive(Serialize)]
pub struct QueryDatabaseDateFilter {
    pub equals: String,
}

#[derive(Serialize)]
pub struct QueryDatabaseFilter {
    pub property: String,
    pub date: QueryDatabaseDateFilter,
}

#[derive(Serialize)]
pub struct QueryDatabaseParams {
    pub filter: QueryDatabaseFilter,
}

#[derive(Deserialize, Debug)]
pub struct QueryDatabaseResult {}

#[derive(Deserialize, Debug)]
pub struct QueryDatabaseResponse {
    pub results: Vec<QueryDatabaseResult>,
}

#[derive(Serialize, Debug)]
pub struct CreatePageParent {
    pub database_id: String,
}

#[derive(Serialize, Debug)]
pub struct CreatePageTitleText {
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct CreatePageTitle {
    pub text: CreatePageTitleText,
}

#[derive(Serialize, Debug)]
pub struct CreatePageDate {
    pub start: String,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum CreatePageProperty {
    Title { title: Vec<CreatePageTitle> },
    Date { date: CreatePageDate },
}

#[derive(Serialize, Debug)]
pub struct CreatePageParams {
    pub parent: CreatePageParent,
    pub properties: HashMap<String, CreatePageProperty>,
}

#[derive(Deserialize, Debug)]
pub struct CreatePageResponse {}

pub struct NotionApiClient<'a, C: HttpClientTrait> {
    http_client: &'a C,
    api_token: String,
}

#[automock]
#[async_trait]
pub trait NotionApiClientTrait {
    async fn query_database(
        &self,
        database_id: &str,
        params: QueryDatabaseParams,
    ) -> Result<QueryDatabaseResponse, Box<dyn Error>>;

    async fn create_page(&self, params: CreatePageParams) -> Result<(), Box<dyn Error>>;
}

impl<'a, C: HttpClientTrait> NotionApiClient<'a, C> {
    pub fn new(http_client: &'a C, api_token: String) -> Self {
        NotionApiClient {
            http_client,
            api_token,
        }
    }

    fn create_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert("Notion-Version", "2022-06-28".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.api_token).parse().unwrap(),
        );

        headers
    }
}

#[async_trait]
impl<'a, C: HttpClientTrait + Sync> NotionApiClientTrait for NotionApiClient<'a, C> {
    async fn query_database(
        &self,
        database_id: &str,
        params: QueryDatabaseParams,
    ) -> Result<QueryDatabaseResponse, Box<dyn Error>> {
        let url = format!("https://api.notion.com/v1/databases/{}/query", database_id);

        let headers = self.create_headers();

        let resp = self
            .http_client
            .post::<QueryDatabaseParams, QueryDatabaseResponse>(&url, headers, &params)
            .await?;

        Ok(resp)
    }

    async fn create_page(&self, params: CreatePageParams) -> Result<(), Box<dyn Error>> {
        let url = "https://api.notion.com/v1/pages";

        let headers = self.create_headers();

        self.http_client
            .post::<CreatePageParams, CreatePageResponse>(url, headers, &params)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use mockall::*;

    use crate::http_client::MockHttpClientTrait;

    use super::*;

    #[tokio::test]
    async fn test_query_database() {
        let api_token = "api_token";
        let database_id = "database_id";
        let params = QueryDatabaseParams {
            filter: QueryDatabaseFilter {
                property: "Date".to_string(),
                date: QueryDatabaseDateFilter {
                    equals: "2023-03-19".to_string(),
                },
            },
        };

        let url = "https://api.notion.com/v1/databases/database_id/query";

        let mut mock = MockHttpClientTrait::new();
        mock.expect_post::<QueryDatabaseParams, QueryDatabaseResponse>()
            .with(predicate::eq(url), predicate::always(), predicate::always())
            .times(1)
            .returning(|_, _, _| Ok(QueryDatabaseResponse { results: vec![] }));
        let notion_api_client = NotionApiClient::new(&mock, api_token.to_string());

        let result = notion_api_client
            .query_database(database_id, params)
            .await
            .unwrap();

        assert_eq!(result.results.len(), 0);
    }

    #[tokio::test]
    async fn test_create_page() {
        let api_token = "api_token";
        let database_id = "database_id";
        let params = CreatePageParams {
            parent: CreatePageParent {
                database_id: database_id.to_string(),
            },
            properties: HashMap::new(),
        };

        let url = "https://api.notion.com/v1/pages";

        let mut mock = MockHttpClientTrait::new();
        mock.expect_post::<CreatePageParams, _>()
            .with(predicate::eq(url), predicate::always(), predicate::always())
            .times(1)
            .returning(|_, _, _| Ok(CreatePageResponse {}));
        let notion_api_client = NotionApiClient::new(&mock, api_token.to_string());

        notion_api_client.create_page(params).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_headers() {
        let mock = MockHttpClientTrait::new();
        let notion_api_client = NotionApiClient::new(&mock, "api_token".to_string());

        let headers = notion_api_client.create_headers();

        assert_eq!(headers.get(ACCEPT).unwrap(), "application/json");
        assert_eq!(headers.get("Notion-Version").unwrap(), "2022-06-28");
        assert_eq!(headers.get(CONTENT_TYPE).unwrap(), "application/json");
        assert_eq!(headers.get(AUTHORIZATION).unwrap(), "Bearer api_token");
    }
}
