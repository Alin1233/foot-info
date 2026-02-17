use async_trait::async_trait;
use reqwest::StatusCode;
use scraper::{Html, Selector};
use chrono::{NaiveDate, NaiveTime, Datelike, Local, TimeZone};
use chrono_tz::Europe::Paris;
use crate::models::{Match, Country};
use crate::error::AppError;
use super::FootballProvider;

pub struct MatchsTvProvider;

#[async_trait]
impl FootballProvider for MatchsTvProvider {
    fn country(&self) -> Country {
        Country::FR
    }

    fn name(&self) -> &str {
        "Matchs.tv Scraper"
    }

    async fn fetch_matches_channels(&self, team_name: &str) -> Result<Vec<Match>, AppError> {
        // Pattern: https://matchs.tv/club/manchester-united/
        let formatted_name = team_name.trim().to_lowercase().replace(" ", "-");
        let url = format!("https://matchs.tv/club/{}/", formatted_name);

        let response = reqwest::get(&url).await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Err(AppError::TeamNotFound(team_name.to_string()));
        }

        let body = response.text().await?;

        parse_html(&body, team_name)
    }
}

/// Parse raw HTML from Matchs.tv and extract match data.
/// Separated from the HTTP layer for testability.
pub fn parse_html(body: &str, team_name: &str) -> Result<Vec<Match>, AppError> {
    let document = Html::parse_document(body);

    let mut matches = Vec::new();

    let row_selector = Selector::parse("table.programme-tv.fixtures tr").unwrap();
    let date_link_selector = Selector::parse("h3 a").unwrap();
    
    let time_selector = Selector::parse("td.date").unwrap();
    let fixture_selector = Selector::parse("td.fixture h4 a").unwrap();
    let competition_selector = Selector::parse("td.fixture .competitions").unwrap();
    let channel_selector = Selector::parse("td.channel img").unwrap();

    let mut current_date_str = String::new();
    let mut current_naive_date: Option<NaiveDate> = None;

    for row in document.select(&row_selector) {
        if let Some(header) = row.select(&date_link_selector).next() {
            let raw_date = header.text().collect::<Vec<_>>().join(" ").trim().to_string();
            if let Some((formatted, naive)) = parse_french_date(&raw_date) {
                current_date_str = formatted;
                current_naive_date = Some(naive);
            } else {
                current_date_str = raw_date;
                current_naive_date = None;
            }
            continue;
        }

        if let Some(time_el) = row.select(&time_selector).next() {
            let raw_time = time_el.text().collect::<Vec<_>>().join(" ").trim().to_string();
            
            let (date_display, time_display) = if let Some(naive_date) = current_naive_date {
                if let Some((local_date, local_time)) = convert_french_time_to_local(naive_date, &raw_time) {
                    (local_date, local_time)
                } else {
                    (current_date_str.clone(), raw_time)
                }
            } else {
                (current_date_str.clone(), raw_time)
            };
            
            let teams = row.select(&fixture_selector)
                .next()
                .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .unwrap_or_else(|| "Unknown Teams".to_string());

            let competition_raw = row.select(&competition_selector)
                .next()
                .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .unwrap_or_default();
            
            let competition = competition_raw.split(',').next().unwrap_or(&competition_raw).trim().to_string();

            let channels: Vec<String> = row.select(&channel_selector)
                .filter_map(|img| img.value().attr("title").map(|s| s.to_string()))
                .collect();

            if !teams.is_empty() {
                matches.push(Match {
                    teams,
                    competition,
                    date: date_display,
                    time: time_display,
                    channels,
                });
            }
        }
    }

    if matches.is_empty() {
        return Err(AppError::NoMatchesScheduled(team_name.to_string()));
    }

    Ok(matches)
}

pub fn parse_french_date(french_date: &str) -> Option<(String, NaiveDate)> {
    // Input: "samedi 7 février"
    let parts: Vec<&str> = french_date.split_whitespace().collect();
    if parts.len() < 3 { return None; }
    
    let day_num = parts[1].parse::<u32>().ok()?;
    let month_str = parts[2].to_lowercase();
    
    let month = match month_str.as_str() {
        "janvier" => 1,
        "février" | "fevrier" => 2,
        "mars" => 3,
        "avril" => 4,
        "mai" => 5,
        "juin" => 6,
        "juillet" => 7,
        "août" | "aout" => 8,
        "septembre" => 9,
        "octobre" => 10,
        "novembre" => 11,
        "décembre" | "decembre" => 12,
        _ => return None,
    };

    let current_date = Local::now().date_naive();
    let current_year = current_date.year();
    
    let mut date = NaiveDate::from_ymd_opt(current_year, month, day_num)?;
    
    if date < current_date - chrono::Duration::days(30) {
        date = NaiveDate::from_ymd_opt(current_year + 1, month, day_num)?;
    }

    Some((date.format("%a %d %b %Y").to_string(), date))
}

pub fn convert_french_time_to_local(date: NaiveDate, time_str: &str) -> Option<(String, String)> {
    let clean_time = time_str.replace("h", ":");
    let time = NaiveTime::parse_from_str(&clean_time, "%H:%M").ok()?;

    let naive_datetime = date.and_time(time);
    let paris_datetime = Paris.from_local_datetime(&naive_datetime).single()?;
    let local_time = paris_datetime.with_timezone(&Local);

    Some((
        local_time.format("%a %d %b %Y").to_string(),
        local_time.format("%H:%M").to_string()
    ))
}
