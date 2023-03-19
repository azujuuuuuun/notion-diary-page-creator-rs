use chrono::{DateTime, Datelike, Local, Weekday};

pub struct Date {
    date_time: DateTime<Local>,
}

impl Date {
    #[cfg(test)]
    fn new(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> Date {
        use chrono::TimeZone;

        Date {
            date_time: Local
                .with_ymd_and_hms(year, month, day, hour, min, sec)
                .unwrap(),
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        assert_eq!(Date::new(2023, 03, 19, 0, 0, 0).format(), "2023-03-19")
    }

    #[test]
    fn test_format_with_slash() {
        assert_eq!(
            Date::new(2023, 03, 19, 0, 0, 0).format_with_slash(),
            "2023/03/19"
        )
    }

    #[test]
    fn test_ja_weekday() {
        assert_eq!(Date::new(2023, 03, 19, 0, 0, 0).ja_weekday(), "日");
        assert_eq!(Date::new(2023, 03, 20, 0, 0, 0).ja_weekday(), "月");
        assert_eq!(Date::new(2023, 03, 21, 0, 0, 0).ja_weekday(), "火");
        assert_eq!(Date::new(2023, 03, 22, 0, 0, 0).ja_weekday(), "水");
        assert_eq!(Date::new(2023, 03, 23, 0, 0, 0).ja_weekday(), "木");
        assert_eq!(Date::new(2023, 03, 24, 0, 0, 0).ja_weekday(), "金");
        assert_eq!(Date::new(2023, 03, 25, 0, 0, 0).ja_weekday(), "土");
    }
}