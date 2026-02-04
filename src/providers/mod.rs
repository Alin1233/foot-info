use async_trait::async_trait;
use crate::models::Match;
use crate::error::AppError;

pub mod wheresthematch;

#[async_trait]
pub trait FootballProvider: Send + Sync {
    async fn fetch_matches(&self, team: &str) -> Result<Vec<Match>, AppError>;
    fn name(&self) -> &str;
}
