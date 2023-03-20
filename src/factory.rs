use std::collections::HashMap;

use crate::date::Date;
use crate::notion::{
    CreatePageDate, CreatePageParams, CreatePageParent, CreatePageProperty, CreatePageTitle,
    CreatePageTitleText, QueryDatabaseDateFilter, QueryDatabaseFilter, QueryDatabaseParams,
};

pub struct NotionParamsFactory;

impl NotionParamsFactory {
    pub fn build_query_database_params(today: &Date) -> QueryDatabaseParams {
        QueryDatabaseParams {
            filter: QueryDatabaseFilter {
                property: "Date".to_string(),
                date: QueryDatabaseDateFilter {
                    equals: today.format(),
                },
            },
        }
    }

    pub fn build_create_page_params(database_id: &str, today: &Date) -> CreatePageParams {
        let title = format!("{}({})", today.format_with_slash(), today.ja_weekday());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_query_database_params() {
        let today = Date::new(2023, 03, 19, 0, 0, 0);

        let params = NotionParamsFactory::build_query_database_params(&today);

        assert_eq!(params.filter.property, "Date");
        assert_eq!(params.filter.date.equals, "2023-03-19");
    }

    #[test]
    fn test_build_create_page_params() {
        let database_id = "database_id";
        let today = Date::new(2023, 03, 19, 0, 0, 0);

        let params = NotionParamsFactory::build_create_page_params(database_id, &today);

        assert_eq!(params.parent.database_id, database_id);
        if let CreatePageProperty::Title { title } = params.properties.get("Name").unwrap() {
            assert_eq!(title.get(0).unwrap().text.content, "2023/03/19(日)")
        }
        if let CreatePageProperty::Date { date } = params.properties.get("Date").unwrap() {
            assert_eq!(date.start, "2023-03-19")
        }
    }
}
