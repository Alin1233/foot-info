use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Network error: {0:?}")]
    Network(#[from] wreq::Error),

    #[error("Provider error: {0}")]
    ProviderError(String),

    #[error("Team '{0}' not found. Please check the spelling and try again.")]
    TeamNotFound(String),

    #[error("No matches scheduled for team: {0}")]
    NoMatchesScheduled(String),
}
