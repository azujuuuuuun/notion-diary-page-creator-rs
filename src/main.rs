mod date;
mod env;
mod factory;
mod http_client;
mod notion;
mod service;

use std::error::Error;

use dotenv::dotenv;
use reqwest::Client as ReqwestClient;

use crate::env::Env;
use crate::http_client::HttpClient;
use crate::notion::NotionApiClient;
use crate::service::{Service, ServiceTrait};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let env = Env::load_env();

    let reqwest_client = ReqwestClient::new();
    let http_client = HttpClient::new(reqwest_client);
    let notion_client = NotionApiClient::new(&http_client, env.api_token);
    let service = Service::new(notion_client);

    service.create_diary_page(&env.database_id).await?;

    Ok(())
}
