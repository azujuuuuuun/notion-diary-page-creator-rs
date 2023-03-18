use std::error::Error;

use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

pub struct Client {
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

#[derive(Serialize)]
pub struct CreatePageParent {
    pub database_id: String,
}

#[derive(Serialize)]
pub struct CreatePageTitleText {
    pub content: String,
}

#[derive(Serialize)]
pub struct CreatePageTitle {
    pub text: CreatePageTitleText,
}

#[derive(Serialize)]
pub struct CreatePageNameProperty {
    pub title: Vec<CreatePageTitle>,
}

#[derive(Serialize)]
pub struct CreatePageDate {
    pub start: String,
}

#[derive(Serialize)]
pub struct CreatePageDateProperty {
    pub date: CreatePageDate,
}

#[derive(Serialize)]
pub struct CreatePageProperties {
    pub Name: CreatePageNameProperty,
    pub Date: CreatePageDateProperty,
}

#[derive(Serialize)]
pub struct CreatePageParams {
    pub parent: CreatePageParent,
    pub properties: CreatePageProperties,
}

impl Client {
    pub fn new(api_token: String) -> Client {
        Client { api_token }
    }

    pub async fn query_database(
        &self,
        id: &str,
        params: &QueryDatabaseParams,
    ) -> Result<QueryDatabaseResponse, Box<dyn Error>> {
        let url = "https://api.notion.com/v1/databases/".to_owned() + &id + "/query";

        let headers = self.create_headers();

        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .headers(headers)
            .json(params)
            .send()
            .await?
            .json::<QueryDatabaseResponse>()
            .await?;

        Ok(resp)
    }

    pub async fn create_page(&self, params: &CreatePageParams) -> Result<(), Box<dyn Error>> {
        let url = "https://api.notion.com/v1/pages";

        let headers = self.create_headers();

        let client = reqwest::Client::new();
        client
            .post(url)
            .headers(headers)
            .json(params)
            .send()
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
