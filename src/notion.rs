use std::{collections::HashMap, error::Error};

use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

use crate::http_client::HttpClientTrait;

pub struct NotionApiClient<C: HttpClientTrait> {
    http_client: C,
    api_token: String,
}

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

impl<C: HttpClientTrait> NotionApiClient<C> {
    pub fn new(http_client: C, api_token: String) -> Self {
        NotionApiClient {
            http_client,
            api_token,
        }
    }

    pub async fn query_database(
        &self,
        id: &str,
        params: &QueryDatabaseParams,
    ) -> Result<QueryDatabaseResponse, Box<dyn Error>> {
        let url = "https://api.notion.com/v1/databases/".to_owned() + id + "/query";

        let headers = self.create_headers();

        let resp = self
            .http_client
            .post::<QueryDatabaseParams, QueryDatabaseResponse>(&url, headers, params)
            .await?;

        Ok(resp)
    }

    pub async fn create_page(&self, params: &CreatePageParams) -> Result<(), Box<dyn Error>> {
        let url = "https://api.notion.com/v1/pages";

        let headers = self.create_headers();

        self.http_client
            .post::<CreatePageParams, _>(url, headers, params)
            .await?;

        Ok(())
    }

    fn create_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert("Notion-Version", "2022-06-28".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(
            AUTHORIZATION,
            ("Bearer ".to_owned() + &self.api_token).parse().unwrap(),
        );
        return headers;
    }
}

#[cfg(test)]
mod tests {
    use crate::http_client::MockHttpClientTrait;

    use super::*;

    #[tokio::test]
    async fn test_query_database() {
        let api_token = "api_token";
        let id = "id";
        let params = QueryDatabaseParams {
            filter: QueryDatabaseFilter {
                property: "Date".to_string(),
                date: QueryDatabaseDateFilter {
                    equals: "2023-03-19".to_string(),
                },
            },
        };

        let mut mock = MockHttpClientTrait::new();
        mock.expect_post::<QueryDatabaseParams, QueryDatabaseResponse>()
            .times(1)
            .returning(|_, _, _| Ok(QueryDatabaseResponse { results: vec![] }));
        let notion_api_client = NotionApiClient::new(mock, api_token.to_string());

        let result = notion_api_client.query_database(id, &params).await.unwrap();

        assert_eq!(result.results.len(), 0);
    }
}
