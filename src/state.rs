use crate::config::Config;
use crate::models::{Match, TopMatch, ViewMode};
use crate::providers::{
    FootballProvider, matchstv::MatchsTvProvider, wheresthematch::WheresTheMatchProvider,
    worldsoccertalk::WorldSoccerTalkProvider,
};
use std::sync::Arc;

/// Pure application state — no channels, no async, no side effects.
/// This is what the UI reads from and what handlers mutate.
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
    pub view_mode: ViewMode,
    pub top_matches: Vec<TopMatch>,
    pub selected_top_match_index: usize,
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
            view_mode: ViewMode::Search,
            top_matches: Vec::new(),
            selected_top_match_index: 0,
        }
    }

    pub fn get_current_provider(&self) -> Arc<dyn FootballProvider> {
        self.providers[self.current_provider_index].clone()
    }
}
