use chrono::{DateTime, Duration, Local, NaiveDateTime, TimeZone, Utc};

// // parse RFC3339 into a timezone-aware DateTime
// let dt_utc: DateTime<Utc> = "2023-11-14T12:34:56Z".parse().unwrap();

// // parse a naive datetime (no zone)
// let naive: NaiveDateTime = NaiveDateTime::parse_from_str("2025-11-14 18:45:00", "%Y-%m-%d %H:%M:%S").unwrap();

// // format
// let s = dt_utc.to_rfc3339();

pub fn current_date_time() -> String {
    let now = Local::now();
    let s = now.format("%y-%m-%d %H:%M:%S").to_string();
    s
}
