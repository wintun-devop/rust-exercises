use chrono::{Local, Utc};

pub fn current_date_time() -> String {
    let now = Local::now();
    let s = now.format("%y-%m-%d %H:%M:%S").to_string();
    s
}

pub fn current_utc_time() -> String {
    let now_utc = Utc::now();
    let s = now_utc.format("%y-%m-%d %H:%M:%S").to_string();
    s
}

pub fn current_utc_iso() -> String {
    let now_utc = Utc::now().to_rfc3339();
    now_utc
}
