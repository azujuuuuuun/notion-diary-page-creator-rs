use std::collections::HashMap;

use crate::date::Date;
use crate::notion::{
    CreatePageDate, CreatePageParams, CreatePageParent, CreatePageProperty, CreatePageTitle,
    CreatePageTitleText, QueryDatabaseDateFilter, QueryDatabaseFilter, QueryDatabaseParams,
};

pub struct NotionParamsFactory;

impl NotionParamsFactory {
    pub fn build_query_database_params() -> QueryDatabaseParams {
        QueryDatabaseParams {
            filter: QueryDatabaseFilter {
                property: "Date".to_string(),
                date: QueryDatabaseDateFilter {
                    equals: Date::today().format(),
                },
            },
        }
    }

    pub fn build_create_page_params(database_id: &str) -> CreatePageParams {
        let today = Date::today();
        let title = today.format_with_slash() + "(" + &today.ja_weekday() + ")";
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
                    start: today.format(),
                },
            },
        );
        CreatePageParams { parent, properties }
    }
}
