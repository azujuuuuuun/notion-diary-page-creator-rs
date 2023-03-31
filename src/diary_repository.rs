use std::{collections::HashMap, error::Error};

use async_trait::async_trait;
use mockall::automock;

use crate::{
    date::Date,
    notion::{
        CreatePageDate, CreatePageParams, CreatePageParent, CreatePageProperty, CreatePageTitle,
        CreatePageTitleText, NotionApiClientTrait, QueryDatabaseDateFilter, QueryDatabaseFilter,
        QueryDatabaseParams,
    },
};

#[automock]
#[async_trait]
pub trait DiaryRepositoryTrait {
    async fn exist(&self, database_id: &str, date: &Date) -> Result<bool, Box<dyn Error>>;

    async fn create_page(&self, database_id: &str, date: &Date) -> Result<(), Box<dyn Error>>;
}

pub struct DiaryRepository<C: NotionApiClientTrait> {
    notion_api_client: C,
}

impl<C: NotionApiClientTrait> DiaryRepository<C> {
    pub fn new(notion_api_client: C) -> Self {
        DiaryRepository { notion_api_client }
    }
}

#[async_trait]
impl<C: NotionApiClientTrait + Sync + Send> DiaryRepositoryTrait for DiaryRepository<C> {
    async fn exist(&self, database_id: &str, date: &Date) -> Result<bool, Box<dyn Error>> {
        let params = QueryDatabaseParams {
            filter: QueryDatabaseFilter {
                property: "Date".to_string(),
                date: QueryDatabaseDateFilter {
                    equals: date.format(),
                },
            },
        };

        let resp = self
            .notion_api_client
            .query_database(database_id, params)
            .await?;

        if resp.results.is_empty() {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    async fn create_page(&self, database_id: &str, date: &Date) -> Result<(), Box<dyn Error>> {
        let title = format!("{}({})", date.format_with_slash(), date.ja_weekday());
        let parent = CreatePageParent {
            database_id: database_id.to_string(),
        };
        let mut properties: HashMap<String, CreatePageProperty> = HashMap::new();
        properties.insert(
            "Name".to_string(),
            CreatePageProperty::Title {
                title: vec![CreatePageTitle {
                    text: CreatePageTitleText { content: title },
                }],
            },
        );
        properties.insert(
            "Date".to_string(),
            CreatePageProperty::Date {
                date: CreatePageDate {
                    start: date.format(),
                },
            },
        );
        let params = CreatePageParams { parent, properties };

        self.notion_api_client.create_page(params).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate;

    use crate::notion::{MockNotionApiClientTrait, QueryDatabaseResponse, QueryDatabaseResult};

    use super::*;

    #[tokio::test]
    async fn test_exist_false() {
        let database_id = "database_id";
        let date = Date::new(2023, 3, 29, 0, 0, 0);

        let mut notion_api_client = MockNotionApiClientTrait::new();
        notion_api_client
            .expect_query_database()
            .with(predicate::eq(database_id), predicate::always())
            .times(1)
            .returning(|_, _| Ok(QueryDatabaseResponse { results: vec![] }));
        let diary_repository = DiaryRepository::new(notion_api_client);

        let exist = diary_repository.exist(database_id, &date).await.unwrap();

        assert!(!exist);
    }

    #[tokio::test]
    async fn test_exist_true() {
        let database_id = "database_id";
        let date = Date::new(2023, 3, 29, 0, 0, 0);

        let mut notion_api_client = MockNotionApiClientTrait::new();
        notion_api_client
            .expect_query_database()
            .with(predicate::eq(database_id), predicate::always())
            .times(1)
            .returning(|_, _| {
                Ok(QueryDatabaseResponse {
                    results: vec![QueryDatabaseResult {}],
                })
            });
        let diary_repository = DiaryRepository::new(notion_api_client);

        let exist = diary_repository.exist(database_id, &date).await.unwrap();

        assert!(exist);
    }

    #[tokio::test]
    async fn test_create_page() {
        let database_id = "database_id";
        let date = Date::new(2023, 3, 29, 0, 0, 0);

        let mut notion_api_client = MockNotionApiClientTrait::new();
        notion_api_client
            .expect_create_page()
            .with(predicate::always())
            .times(1)
            .returning(|_| Ok(()));
        let diary_repository = DiaryRepository::new(notion_api_client);

        diary_repository
            .create_page(database_id, &date)
            .await
            .unwrap();
    }
}
