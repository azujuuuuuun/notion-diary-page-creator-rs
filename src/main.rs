mod env;
mod notion;

use std::{error::Error, process::exit};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = env::load_env();

    println!("Creating diary page started.");

    let notion_client = notion::Client::new(env.api_token);

    let resp = notion_client.query_database(&env.database_id).await?;
    if resp.results.len() > 0 {
        println!("Today's diary page was already created.");
        exit(0);
    }

    notion_client.create_page(&env.database_id).await?;
    println!("Today's diary page was created successfully.");

    Ok(())
}
