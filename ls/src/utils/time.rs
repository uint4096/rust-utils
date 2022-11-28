use time::{OffsetDateTime, format_description};

pub fn systemtime_strftime<T>(dt: T) -> String 
   where T: Into<OffsetDateTime> {
    let date_format = format_description::parse("[month repr:short] [day] [hour]:[minute]").expect("Failed to format!");
    let date_time: OffsetDateTime = dt.into();
    match date_time.format(&date_format) {
        Ok(dt_time) => { dt_time },
        Err(_) => { String::from("") }
    }
}
