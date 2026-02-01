use chrono::{DateTime, Local};

/// Converts an ISO 8601 UTC timestamp (e.g., "2026-02-01T14:00:00Z")
/// to the user's local date and time.
/// Returns Some((formatted_date, formatted_time)) if parsing succeeds, otherwise None.
pub fn convert_utc_to_local(iso_string: &str) -> Option<(String, String)> {
    let parsed_utc = DateTime::parse_from_rfc3339(iso_string).ok()?;
    let local_time: DateTime<Local> = parsed_utc.with_timezone(&Local);
    Some((
        local_time.format("%a %d %b %Y").to_string(),
        local_time.format("%H:%M").to_string()
    ))
}
