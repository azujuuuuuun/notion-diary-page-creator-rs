mod date;
mod env;
mod factory;
mod notion;

use std::{error::Error, process::exit};

use crate::date::Date;
use crate::factory::NotionParamsFactory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = env::load_env();

    println!("Creating diary page started.");

    let notion_client = notion::Client::new(env.api_token);

    let today = Date::today();
    let params = NotionParamsFactory::build_query_database_params(&today);

    let resp = notion_client
        .query_database(&env.database_id, &params)
        .await?;
    if resp.results.len() > 0 {
        println!("Today's diary page was already created.");
        exit(0);
    }

    let params = NotionParamsFactory::build_create_page_params(&env.database_id, &today);

    notion_client.create_page(&params).await?;
    println!("Today's diary page was created successfully.");

    Ok(())
}
