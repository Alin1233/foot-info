use chrono::{DateTime, Local, NaiveDate, NaiveTime, TimeZone, Datelike};
use chrono_tz::US::Eastern;


pub fn convert_utc_to_local(iso_string: &str) -> Option<(String, String)> {
    let parsed_utc = DateTime::parse_from_rfc3339(iso_string).ok()?;
    let local_time: DateTime<Local> = parsed_utc.with_timezone(&Local);
    Some((
        local_time.format("%a %d %b %Y").to_string(),
        local_time.format("%H:%M").to_string()
    ))
}
pub fn convert_et_to_local(date_str: &str, time_str: &str) -> Option<(String, String)> {
    let clean_time = time_str.trim().trim_end_matches(" ET").trim();
    let time = NaiveTime::parse_from_str(clean_time, "%I:%M %p").ok()?;
    let current_date = Local::now().date_naive();
    let current_year = current_date.year();

    let date_with_year_str = format!("{}, {}", date_str, current_year);
    let mut date = NaiveDate::parse_from_str(&date_with_year_str, "%A, %B %d, %Y").ok()?;
    if date < current_date - chrono::Duration::days(30) {
        date = date.with_year(current_year + 1)?;
    }
    let naive_datetime = date.and_time(time);
    let et_datetime = Eastern.from_local_datetime(&naive_datetime).single()?;
    let local_time: DateTime<Local> = et_datetime.with_timezone(&Local);

    Some((
        local_time.format("%a %d %b %Y").to_string(),
        local_time.format("%H:%M").to_string()
    ))
}
