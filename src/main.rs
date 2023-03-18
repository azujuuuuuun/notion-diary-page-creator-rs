mod env;

use std::{error::Error, process::exit};

use chrono::{Datelike, Local, Weekday};
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct QueryDatabaseResult {}

#[derive(Deserialize, Debug)]
struct QueryDatabaseResponse {
    results: Vec<QueryDatabaseResult>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = env::load_env();

    println!("Creating diary page started.");

    let url = "https://api.notion.com/v1/databases/".to_owned() + &env.database_id + "/query";

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Notion-Version", "2022-06-28".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(
        AUTHORIZATION,
        ("Bearer ".to_owned() + &env.api_token).parse().unwrap(),
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

    if resp.results.len() > 0 {
        println!("Today's diary page was already created.");
        exit(0);
    }

    let url = "https://api.notion.com/v1/pages";

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Notion-Version", "2022-06-28".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(
        AUTHORIZATION,
        ("Bearer ".to_owned() + &env.api_token).parse().unwrap(),
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
            "database_id": env.database_id,
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

    println!("Today's diary page was created successfully.");

    Ok(())
}
