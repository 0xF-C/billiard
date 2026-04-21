use chrono::{DateTime, Utc};

/// Get current time in Asia/Shanghai timezone
pub fn now_local() -> DateTime<chrono_tz::Tz> {
    Utc::now().with_timezone(&chrono_tz::Asia::Shanghai)
}

/// Get today's date string in Asia/Shanghai timezone (YYYY-MM-DD)
pub fn today_local() -> String {
    now_local().format("%Y-%m-%d").to_string()
}

/// Convert a UTC datetime string to local date string
/// Handles cross-midnight orders (e.g. 23:00-02:00)
#[allow(dead_code)]
pub fn to_local_date(utc_str: &str) -> String {
    match DateTime::parse_from_rfc3339(utc_str) {
        Ok(dt) => dt.with_timezone(&chrono_tz::Asia::Shanghai).format("%Y-%m-%d").to_string(),
        Err(_) => Utc::now().format("%Y-%m-%d").to_string(),
    }
}
