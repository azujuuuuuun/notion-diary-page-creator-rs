use std::collections::HashMap;

use chrono::{Datelike, Local, Weekday};

use crate::notion::{
    CreatePageDate, CreatePageParams, CreatePageParent, CreatePageProperty, CreatePageTitle,
    CreatePageTitleText, QueryDatabaseDateFilter, QueryDatabaseFilter, QueryDatabaseParams,
};

pub struct NotionParamsFactory;

impl NotionParamsFactory {
    pub fn build_query_database_params() -> QueryDatabaseParams {
        let local = Local::now();
        QueryDatabaseParams {
            filter: QueryDatabaseFilter {
                property: "Date".to_string(),
                date: QueryDatabaseDateFilter {
                    equals: local.format("%Y-%m-%d").to_string(),
                },
            },
        }
    }

    pub fn build_create_page_params(database_id: &str) -> CreatePageParams {
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
                    start: local.format("%Y-%m-%d").to_string(),
                },
            },
        );
        CreatePageParams { parent, properties }
    }
}
