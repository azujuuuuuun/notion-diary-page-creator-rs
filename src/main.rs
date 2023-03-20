mod date;
mod env;
mod factory;
mod http_client;
mod notion;

use std::{error::Error, process::exit};

use dotenv::dotenv;
use reqwest::Client as ReqwestClient;

use crate::date::Date;
use crate::env::load_env;
use crate::factory::NotionParamsFactory;
use crate::http_client::HttpClient;
use crate::notion::{NotionApiClient, NotionApiClientTrait};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Creating diary page started.");

    dotenv().ok();
    let env = load_env();

    let reqwest_client = ReqwestClient::new();
    let http_client = HttpClient::new(reqwest_client);
    let notion_client = NotionApiClient::new(&http_client, env.api_token);

    let today = Date::today();
    let params = NotionParamsFactory::build_query_database_params(&today);

    let resp = notion_client
        .query_database(&env.database_id, &params)
        .await?;
    if !resp.results.is_empty() {
        println!("Today's diary page was already created.");
        exit(0);
    }

    let params = NotionParamsFactory::build_create_page_params(&env.database_id, &today);

    notion_client.create_page(&params).await?;
    println!("Today's diary page was created successfully.");

    Ok(())
}
