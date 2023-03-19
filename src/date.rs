use chrono::{DateTime, Datelike, Local, Weekday};

pub struct Date {
    date_time: DateTime<Local>,
}

impl Date {
    pub fn today() -> Date {
        Date {
            date_time: Local::now(),
        }
    }

    pub fn format(&self) -> String {
        self.date_time.format("%Y-%m-%d").to_string()
    }

    pub fn format_with_slash(&self) -> String {
        self.date_time.format("%Y/%m/%d").to_string()
    }

    pub fn ja_weekday(&self) -> String {
        match self.date_time.weekday() {
            Weekday::Sun => "日".to_string(),
            Weekday::Mon => "月".to_string(),
            Weekday::Tue => "火".to_string(),
            Weekday::Wed => "水".to_string(),
            Weekday::Thu => "木".to_string(),
            Weekday::Fri => "金".to_string(),
            Weekday::Sat => "土".to_string(),
        }
    }
}
