#[derive(Debug, Clone)]
pub struct Match {
    pub teams: String,
    pub competition: String,
    pub date: String,
    pub time: String,
    pub channels: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TopMatch {
    pub teams: String,
    pub date: String,
    pub time: String,
    pub match_url: String,
}

#[derive(Debug, Clone)]
pub struct LeagueFixture {
    pub home_team: String,
    pub away_team: String,
    pub date: String,
    pub time: String,
    pub score: Option<String>,
    pub channels: Vec<String>,
    pub match_url: String,
}

#[derive(Debug, Clone)]
pub struct StandingRow {
    pub position: u8,
    pub team: String,
    pub played: u8,
    pub won: u8,
    pub drawn: u8,
    pub lost: u8,
    pub goals_for: u16,
    pub goals_against: u16,
    pub goal_diff: i16,
    pub points: u16,
    pub form: Vec<char>,
}

#[derive(Debug, Clone)]
pub struct TopScorer {
    pub player: String,
    pub team: String,
    pub goals: u8,
    pub penalties: u8,
}

#[derive(Debug, Clone)]
pub struct LeagueStats {
    pub competition: String,
    pub fixtures: Vec<LeagueFixture>,
    pub table: Vec<StandingRow>,
    pub top_scorers: Vec<TopScorer>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Country {
    UK,
    US,
    FR,
}

impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Country::UK => write!(f, "UK"),
            Country::US => write!(f, "US"),
            Country::FR => write!(f, "FR"),
        }
    }
}
