use foot_info_core::client::FootballClient;
pub use foot_info_core::models::{Country, Match, TopMatch};

// Instruct flutter_rust_bridge to generate transparent bindings for our core structs
// instead of treating them as Opaque pointers
#[flutter_rust_bridge::frb(mirror(Match))]
pub struct _Match {
    pub teams: String,
    pub competition: String,
    pub date: String,
    pub time: String,
    pub channels: Vec<String>,
}

#[flutter_rust_bridge::frb(mirror(TopMatch))]
pub struct _TopMatch {
    pub teams: String,
    pub date: String,
    pub time: String,
    pub match_url: String,
}

#[flutter_rust_bridge::frb(mirror(Country))]
pub enum _Country {
    UK,
    US,
    FR,
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub async fn search_team(team: String, country: Country) -> Vec<Match> {
    let client = FootballClient::new();
    client.search_team(&team, country).await.unwrap_or_default()
}

pub async fn fetch_top_matches() -> Vec<TopMatch> {
    let client = FootballClient::new();
    client.fetch_top_matches().await.unwrap_or_default()
}
