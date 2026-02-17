use async_trait::async_trait;
use crate::models::{Match, Country};
use crate::error::AppError;

pub mod wheresthematch;
pub mod worldsoccertalk;
pub mod matchstv;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FootballProvider: Send + Sync {
    async fn fetch_matches_channels(&self, team: &str) -> Result<Vec<Match>, AppError>;
    fn country(&self) -> Country;
    fn name(&self) -> &str;
}
