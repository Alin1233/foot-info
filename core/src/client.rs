use crate::error::AppError;
use crate::models::{Country, Match, TopMatch};
use crate::providers::{
    matchstv::MatchsTvProvider, wheresthematch::WheresTheMatchProvider,
    worldsoccertalk::WorldSoccerTalkProvider, FootballProvider,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct FootballClient {
    providers: Vec<Arc<dyn FootballProvider>>,
}

impl FootballClient {
    pub fn new() -> Self {
        Self {
            providers: vec![
                Arc::new(WheresTheMatchProvider),
                Arc::new(WorldSoccerTalkProvider),
                Arc::new(MatchsTvProvider),
            ],
        }
    }

    pub fn providers(&self) -> &[Arc<dyn FootballProvider>] {
        &self.providers
    }

    pub async fn fetch_top_matches(&self) -> Result<Vec<TopMatch>, AppError> {
        crate::providers::livesoccertv::fetch_top_matches().await
    }

    pub async fn search_team(&self, team: &str, provider: Country) -> Result<Vec<Match>, AppError> {
        if let Some(p) = self.providers.iter().find(|p| p.country() == provider) {
            p.fetch_matches_channels(team).await
        } else {
            Err(AppError::TeamNotFound("Provider not found".to_string()))
        }
    }
}

impl Default for FootballClient {
    fn default() -> Self {
        Self::new()
    }
}
