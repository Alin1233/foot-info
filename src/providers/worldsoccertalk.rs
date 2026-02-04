use async_trait::async_trait;
use reqwest::StatusCode;
use scraper::{Html, Selector};
use crate::models::{Match, Country};
use crate::user;
use crate::error::AppError;
use super::FootballProvider;

pub struct WorldSoccerTalkProvider;

#[async_trait]
impl FootballProvider for WorldSoccerTalkProvider {
    fn country(&self) -> Country {
        Country::US
    }

    fn name(&self) -> &str {
        "WorldSoccerTalk Scraper"
    }

    async fn fetch_matches_channels(&self, team_name: &str) -> Result<Vec<Match>, AppError> {
        let formatted_name = team_name.trim().to_lowercase().replace(" ", "-");
        let url = format!("https://worldsoccertalk.com/teams/{}-tv-schedule/", formatted_name);

        let response = reqwest::get(&url).await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Err(AppError::TeamNotFound(team_name.to_string()));
        }

        let body = response.text().await?;
        let document = Html::parse_document(&body);

        // Check if we actually have a schedule
        let date_selector = Selector::parse("h3.text-stvsDate").unwrap();
        if document.select(&date_selector).next().is_none() {
             return Err(AppError::NoMatchesScheduled(team_name.to_string()));
        }

        let mut matches = Vec::new();

        let match_row_selector = Selector::parse("li.border-stvsMatchBorderColor").unwrap();
        let hour_selector = Selector::parse(".text-stvsMatchHour").unwrap();
        let title_selector = Selector::parse(".text-stvsMatchTitle").unwrap();
        let provider_selector = Selector::parse(".text-stvsProviderLink a.hidden.md\\:inline-block").unwrap();
        let provider_fallback_selector = Selector::parse(".text-stvsProviderLink a").unwrap();

        let content_selector = Selector::parse("div.flex.flex-col.w-full > div").unwrap();

        for date_group in document.select(&content_selector) {
            let date_str = date_group
                .select(&date_selector)
                .next()
                .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string());
            
            if let Some(current_date_str) = date_str {
                for row in date_group.select(&match_row_selector) {
                    let raw_time = row
                        .select(&hour_selector)
                        .next()
                        .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                        .unwrap_or_else(|| "Unknown Time".to_string());

                    // Try to convert ET to Local
                    let (date, time) = if let Some((local_date, local_time)) = user::convert_et_to_local(&current_date_str, &raw_time) {
                        (local_date, local_time)
                    } else {
                        // Fallback to raw string if parsing fails
                        (current_date_str.clone(), raw_time)
                    };

                    let full_title = row
                        .select(&title_selector)
                        .next()
                        .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                        .unwrap_or_default();

                    let (teams, competition) = if let Some(idx) = full_title.rfind('(') {
                        let t = full_title[..idx].trim().to_string();
                        let c = full_title[idx+1..].trim_end_matches(')').trim().to_string();
                        (t, c)
                    } else {
                        (full_title, "Unknown Competition".to_string())
                    };

                    let mut channels: Vec<String> = row
                        .select(&provider_selector)
                        .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                        .collect();
                    
                    if channels.is_empty() {
                        let mut raw_channels: Vec<String> = row
                            .select(&provider_fallback_selector)
                            .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                            .collect();
                        raw_channels.sort();
                        raw_channels.dedup();
                        channels = raw_channels;
                    }

                    matches.push(Match {
                        teams,
                        competition,
                        date,
                        time,
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
}
