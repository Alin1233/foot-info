use async_trait::async_trait;
use crate::models::{Match, Country};
use crate::error::AppError;
use super::FootballProvider;

pub struct MockUsProvider;

#[async_trait]
impl FootballProvider for MockUsProvider {
    fn country(&self) -> Country {
        Country::US
    }

    fn name(&self) -> &str {
        "Mock US Provider"
    }

    async fn fetch_matches_channels(&self, team: &str) -> Result<Vec<Match>, AppError> {
        // Simulate a network delay
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        if team.to_lowercase() == "error" {
             return Err(AppError::TeamNotFound(team.to_string()));
        }

        Ok(vec![Match {
            teams: format!("{} vs US Rival", team),
            competition: "MLS".to_string(),
            date: "2026-07-04".to_string(),
            time: "20:00".to_string(),
            channels: vec!["ESPN".to_string(), "Fox Sports".to_string()],
        }])
    }
}
