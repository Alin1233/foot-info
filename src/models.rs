#[derive(Debug, Clone)]
pub struct Match {
    pub teams: String,
    pub competition: String,
    pub date: String,
    pub time: String,
    pub channels: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Country {
    UK,
    US,
}

impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Country::UK => write!(f, "UK"),
            Country::US => write!(f, "US"),
        }
    }
}
