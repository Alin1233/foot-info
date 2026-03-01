use crate::error::AppError;
use crate::models::{Country, Match};
use async_trait::async_trait;

pub mod livesoccertv;
pub mod matchstv;
pub mod wheresthematch;
pub mod worldsoccertalk;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FootballProvider: Send + Sync {
    async fn fetch_matches_channels(&self, team: &str) -> Result<Vec<Match>, AppError>;
    fn country(&self) -> Country;
    fn name(&self) -> &str;
}
