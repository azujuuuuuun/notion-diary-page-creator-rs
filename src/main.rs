mod date;
mod diary_repository;
mod diary_service;
mod env;
mod factory;
mod http_client;
mod notion;

use std::error::Error;

use date::Date;
use dotenv::dotenv;
use reqwest::Client as ReqwestClient;

use crate::diary_service::{DiaryService, DiaryServiceTrait};
use crate::env::Env;
use crate::http_client::HttpClient;
use crate::notion::NotionApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let env = Env::load_env();

    let reqwest_client = ReqwestClient::new();
    let http_client = HttpClient::new(reqwest_client);
    let notion_api_client = NotionApiClient::new(&http_client, env.api_token);
    let diary_service = DiaryService::new(notion_api_client);

    let today = Date::today();

    diary_service
        .create_diary_page(&env.database_id, &today)
        .await?;

    Ok(())
}
