use std::error::Error;

use async_trait::async_trait;

use crate::date::Date;
use crate::factory::NotionParamsFactory;
use crate::notion::NotionApiClientTrait;

#[async_trait]
pub trait ServiceTrait {
    async fn create_diary_page(&self, id: &str, date: &Date) -> Result<(), Box<dyn Error>>;
}

pub struct Service<C: NotionApiClientTrait> {
    notion_client: C,
}

impl<C: NotionApiClientTrait> Service<C> {
    pub fn new(notion_client: C) -> Self {
        Service { notion_client }
    }
}

#[async_trait]
impl<C: NotionApiClientTrait + Sync + Send> ServiceTrait for Service<C> {
    async fn create_diary_page(&self, id: &str, date: &Date) -> Result<(), Box<dyn Error>> {
        println!("Creating diary page started.");

        let params = NotionParamsFactory::build_query_database_params(date);
        let resp = self.notion_client.query_database(id, params).await?;
        if !resp.results.is_empty() {
            println!("Today's diary page was already created.");
            return Ok(());
        }

        let params = NotionParamsFactory::build_create_page_params(id, date);
        self.notion_client.create_page(params).await?;

        println!("Today's diary page was created successfully.");

        Ok(())
    }
}
