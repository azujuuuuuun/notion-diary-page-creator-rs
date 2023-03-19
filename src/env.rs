use std::env;

use dotenv::dotenv;

pub struct Env {
    pub database_id: String,
    pub api_token: String,
}

pub fn load_env() -> Env {
    dotenv().ok();

    Env {
        database_id: env::var("NOTION_DIARY_DATABASE_ID")
            .expect("NOTION_DIARY_DATABASE_ID must be set."),
        api_token: env::var("NOTION_API_TOKEN").expect("NOTION_API_TOKEN must be set."),
    }
}
