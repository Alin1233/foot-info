use std::sync::Arc;
use crate::models::Match;
use crate::providers::{FootballProvider, wheresthematch::WheresTheMatchProvider, worldsoccertalk::WorldSoccerTalkProvider, matchstv::MatchsTvProvider};
use crate::config::Config;

pub struct AppState {
    pub search_input: String,
    pub matches: Vec<Match>,
    pub error_message: Option<String>,
    pub status_message: Option<String>,
    pub is_loading: bool,
    pub exit: bool,
    pub config: Config,
    pub providers: Vec<Arc<dyn FootballProvider>>,
    pub current_provider_index: usize,
}

impl AppState {
    pub fn new() -> Self {
        let config = Config::load();
        Self {
            search_input: String::new(),
            matches: Vec::new(),
            error_message: None,
            status_message: None,
            is_loading: false,
            exit: false,
            config,
            providers: vec![
                Arc::new(WheresTheMatchProvider),
                Arc::new(WorldSoccerTalkProvider),
                Arc::new(MatchsTvProvider),
            ],
            current_provider_index: 0,
        }
    }

    pub fn get_current_provider(&self) -> Arc<dyn FootballProvider> {
        self.providers[self.current_provider_index].clone()
    }
}
