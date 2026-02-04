use async_trait::async_trait;
use reqwest::{StatusCode};
use scraper::{Html, Selector};
use crate::models::Match;
use crate::user;
use crate::error::AppError;
use super::FootballProvider;

pub struct WheresTheMatchProvider;

#[async_trait]
impl FootballProvider for WheresTheMatchProvider {
    fn name(&self) -> &str {
        "WheresTheMatch Scraper"
    }

    async fn fetch_matches(&self, team_name: &str) -> Result<Vec<Match>, AppError> {
        let formatted_name = team_name.trim().replace(" ", "-");
        let url = format!("https://www.wheresthematch.com/Football/{}.asp", formatted_name);

        let response = reqwest::get(&url).await?;
        
        // Check for 404
        if response.status() == StatusCode::NOT_FOUND {
            return Err(AppError::TeamNotFound(team_name.to_string()));
        }

        let final_url = response.url().as_str();
        // If redirected to search results, it's a fail
        if final_url.contains("search-results.asp") {
             return Err(AppError::TeamNotFound(team_name.to_string()));
        }

        let body = response.text().await?;
        
        // SPECIFIC CHECK FOR "Invalid URL Format"
        if body.contains("Invalid URL Format") {
            return Err(AppError::TeamNotFound(team_name.to_string()));
        }

        let document = Html::parse_document(&body);

        let h1_selector = Selector::parse("h1.intro").unwrap();
        if document.select(&h1_selector).next().is_none() {
            return Err(AppError::TeamNotFound(team_name.to_string()));
        }

        let row_selector = Selector::parse("#teamswrapper table tbody tr").unwrap();
        let teams_selector = Selector::parse(".fixture-details .fixture").unwrap(); 
        let competition_selector = Selector::parse(".competition-name").unwrap();
        let time_selector = Selector::parse(".start-details").unwrap();
        let channel_selector = Selector::parse(".channel-details img").unwrap();

        let mut matches = Vec::new();

        for row in document.select(&row_selector) {
            if let Some(teams_element) = row.select(&teams_selector).next() {
                let raw_text = teams_element.text().collect::<Vec<_>>().join(" ");
                let teams = raw_text.split_whitespace().collect::<Vec<_>>().join(" ");
                
                if teams.is_empty() 
                   || teams.contains("WATCH TODAY'S GAME LIVE!") 
                   || teams.contains("SKY DEALS") 
                {
                    continue;
                }

                let competition = row
                    .select(&competition_selector)
                    .next()
                    .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                    .unwrap_or_default();

                let (date, time) = if let Some(time_el) = row.select(&time_selector).next() {
                    let iso_string = time_el.value().attr("content");
                    if let Some(iso) = iso_string {
                        if let Some((local_date, local_time)) = user::convert_utc_to_local(iso) {
                            (local_date, local_time)
                        } else {
                            parse_text_date(time_el)
                        }
                    } else {
                        parse_text_date(time_el)
                    }
                } else {
                    ("Unknown Date".to_string(), "Unknown Time".to_string())
                };

                let channels: Vec<String> = row
                    .select(&channel_selector)
                    .filter_map(|img| {
                        img.value().attr("alt").map(|s| {
                            s.trim_end_matches(" logo").to_string()
                        })
                    })
                    .collect();

                matches.push(Match {
                    teams,
                    competition,
                    date,
                    time,
                    channels,
                });
            }
        }

        if matches.is_empty() {
            return Err(AppError::NoMatchesScheduled(team_name.to_string()));
        }

        Ok(matches)
    }
}

fn parse_text_date(element: scraper::ElementRef) -> (String, String) {
    let datetime_text = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
    let parts: Vec<&str> = datetime_text.split_whitespace().collect();
    if let Some(last) = parts.last() {
        if last.contains(':') {
            let d = parts[..parts.len()-1].join(" ");
            (d, last.to_string())
        } else {
            (datetime_text, "".to_string())
        }
    } else {
        (datetime_text, "".to_string())
    }
}
