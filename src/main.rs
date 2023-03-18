mod env;
mod notion;

use std::{error::Error, process::exit};

use chrono::{Datelike, Local, Weekday};

use crate::notion::{
    CreatePageDate, CreatePageDateProperty, CreatePageNameProperty, CreatePageParams,
    CreatePageParent, CreatePageProperties, CreatePageTitle, CreatePageTitleText,
    QueryDatabaseDateFilter, QueryDatabaseFilter, QueryDatabaseParams,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = env::load_env();

    println!("Creating diary page started.");

    let notion_client = notion::Client::new(env.api_token);

    let local = Local::now();
    let params = QueryDatabaseParams {
        filter: QueryDatabaseFilter {
            property: "Date".to_string(),
            date: QueryDatabaseDateFilter {
                equals: local.format("%Y-%m-%d").to_string(),
            },
        },
    };

    let resp = notion_client
        .query_database(&env.database_id, &params)
        .await?;
    if resp.results.len() > 0 {
        println!("Today's diary page was already created.");
        exit(0);
    }

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
    let params = CreatePageParams {
        parent: CreatePageParent {
            database_id: env.database_id.to_string(),
        },
        properties: CreatePageProperties {
            Name: CreatePageNameProperty {
                title: vec![CreatePageTitle {
                    text: CreatePageTitleText { content: title },
                }],
            },
            Date: CreatePageDateProperty {
                date: CreatePageDate {
                    start: local.format("%Y-%m-%d").to_string(),
                },
            },
        },
    };

    notion_client.create_page(&params).await?;
    println!("Today's diary page was created successfully.");

    Ok(())
}
