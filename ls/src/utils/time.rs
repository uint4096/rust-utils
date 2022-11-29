use std::time::SystemTime;
use chrono::{Utc, Local, DateTime};

pub fn format_system_time(dt: SystemTime) -> String {
    let date_time: DateTime<Utc> = dt.into();
    let chrono_time = date_time.with_timezone(&Local);
    chrono_time.format("%b %d %R").to_string()
}