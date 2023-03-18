use std::error::Error;

use chrono::{Datelike, Local, Weekday};
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::json;

pub struct Client {
    api_token: String,
}

#[derive(Deserialize, Debug)]
pub struct QueryDatabaseResult {}

#[derive(Deserialize, Debug)]
pub struct QueryDatabaseResponse {
    pub results: Vec<QueryDatabaseResult>,
}

impl Client {
    pub fn new(api_token: String) -> Client {
        Client { api_token }
    }

    pub async fn query_database(&self, id: &str) -> Result<QueryDatabaseResponse, Box<dyn Error>> {
        let url = "https://api.notion.com/v1/databases/".to_owned() + &id + "/query";

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert("Notion-Version", "2022-06-28".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(
            AUTHORIZATION,
            ("Bearer ".to_owned() + &self.api_token).parse().unwrap(),
        );

        let local = Local::now();
        let params = json!({
            "filter": {
                "property": "Date",
                "date": {
                    "equals": local.format("%Y-%m-%d").to_string()
                }
            }
        });

        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .headers(headers)
            .json(&params)
            .send()
            .await?
            .json::<QueryDatabaseResponse>()
            .await?;

        Ok(resp)
    }

    pub async fn create_page(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let url = "https://api.notion.com/v1/pages";

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert("Notion-Version", "2022-06-28".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(
            AUTHORIZATION,
            ("Bearer ".to_owned() + &self.api_token).parse().unwrap(),
        );

        let local = Local::now();
        let ja_weekday = match local.weekday() {
            Weekday::Sun => "日",
            Weekday::Mon => "月",
            Weekday::Tue => "火",
            Weekday::Wed => "水",
            Weekday::Thu => "木",
            Weekday::Fri => "金",
            Weekday::Sat => "土",
        };
        let title = local.format("%Y/%m/%d").to_string() + "(" + ja_weekday + ")";
        let params = json!({
            "parent": {
                "database_id": id,
            },
            "properties": {
                "Name": {
                    "title": [{
                        "text": {
                            "content": title
                        }
                    }]
                },
                "Date": {
                    "date": {
                        "start": local.format("%Y-%m-%d").to_string()
                    }
                }
            }
        });

        let client = reqwest::Client::new();
        client
            .post(url)
            .headers(headers)
            .json(&params)
            .send()
            .await?;

        Ok(())
    }
}
