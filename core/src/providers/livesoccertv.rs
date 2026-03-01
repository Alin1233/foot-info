use crate::error::AppError;
use crate::models::TopMatch;
use chrono::{Local, TimeZone, Utc};
use scraper::{Html, Selector};
use wreq::Client;
use wreq_util::Emulation;

const LIVESOCCERTV_URL: &str = "https://www.livesoccertv.com/schedules/";

/// Fetches the HTML using wreq with Chrome TLS emulation (bypasses Cloudflare fingerprinting).
pub async fn fetch_top_matches() -> Result<Vec<TopMatch>, AppError> {
    let client = Client::builder()
        .emulation(Emulation::Chrome136)
        .build()
        .map_err(|e| AppError::NoMatchesScheduled(format!("Failed to build HTTP client: {}", e)))?;

    let response = client
        .get(LIVESOCCERTV_URL)
        .send()
        .await
        .map_err(|e| AppError::NoMatchesScheduled(format!("Request failed: {}", e)))?;

    if !response.status().is_success() {
        return Err(AppError::NoMatchesScheduled(format!(
            "HTTP {} from livesoccertv.com",
            response.status()
        )));
    }

    let body = response
        .text()
        .await
        .map_err(|e| AppError::NoMatchesScheduled(format!("Failed to read response: {}", e)))?;

    if body.is_empty() {
        return Err(AppError::NoMatchesScheduled(
            "Empty response from livesoccertv.com".to_string(),
        ));
    }

    parse_html(&body)
}

/// Converts a Unix millisecond timestamp to a local date string like "Sat 21 Feb 2026"
/// and time string like "18:30".
fn timestamp_to_local(millis: i64) -> (String, String) {
    let utc_dt = Utc.timestamp_millis_opt(millis);
    match utc_dt.single() {
        Some(dt) => {
            let local_dt = dt.with_timezone(&Local);
            let date = local_dt.format("%a %d %b %Y").to_string();
            let time = local_dt.format("%H:%M").to_string();
            (date, time)
        }
        None => ("Unknown date".to_string(), "??:??".to_string()),
    }
}

pub fn parse_html(body: &str) -> Result<Vec<TopMatch>, AppError> {
    let document = Html::parse_document(body);

    let fheader_selector = Selector::parse("div.fheader").expect("Invalid selector");
    let span_selector = Selector::parse("span.ts").expect("Invalid selector");
    let a_selector = Selector::parse("a").expect("Invalid selector");

    let mut matches = Vec::new();
    let mut in_section = false;

    for fheader in document.select(&fheader_selector) {
        let text: String = fheader.text().collect();
        if text.contains("Upcoming Top Matches") {
            if let Some(parent) = fheader.parent() {
                let fheader_id = fheader.id();
                let mut past_header = false;

                for child in parent.children() {
                    if child.id() == fheader_id {
                        past_header = true;
                        continue;
                    }

                    if !past_header {
                        continue;
                    }

                    if let Some(el) = child.value().as_element() {
                        if el.name() == "div" {
                            if el.classes().any(|c| c == "fheader") {
                                break;
                            }

                            if let Some(el_ref) = scraper::ElementRef::wrap(child) {
                                let span = el_ref.select(&span_selector).next();

                                // Extract date and time from the `dv` attribute (Unix millis)
                                let (date, time) = span
                                    .as_ref()
                                    .and_then(|s| s.value().attr("dv"))
                                    .and_then(|dv| dv.parse::<i64>().ok())
                                    .map(timestamp_to_local)
                                    .unwrap_or_else(|| {
                                        // Fallback: use the text content of the span
                                        let fallback_date = span
                                            .as_ref()
                                            .map(|s| {
                                                s.text().collect::<String>().trim().to_string()
                                            })
                                            .unwrap_or_default();
                                        (fallback_date, "??:??".to_string())
                                    });

                                let link = el_ref.select(&a_selector).next();
                                let teams = link
                                    .as_ref()
                                    .map(|a| a.text().collect::<String>().trim().to_string());
                                let match_url = link
                                    .as_ref()
                                    .and_then(|a| a.value().attr("href"))
                                    .unwrap_or("")
                                    .to_string();

                                if let Some(teams) = teams {
                                    if !teams.is_empty() && !date.is_empty() {
                                        matches.push(TopMatch {
                                            teams,
                                            date,
                                            time,
                                            match_url,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
            in_section = true;
            break;
        }
    }

    if !in_section || matches.is_empty() {
        return Err(AppError::NoMatchesScheduled(
            "No upcoming top matches found".to_string(),
        ));
    }

    Ok(matches)
}
