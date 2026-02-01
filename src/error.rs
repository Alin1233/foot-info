use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Team '{0}' not found. Please check the spelling and try again.")]
    TeamNotFound(String),

    #[error("No upcoming matches found for '{0}'.")]
    NoMatchesScheduled(String),

    #[error("Unknown error: {0}")]
    Other(String),
}
