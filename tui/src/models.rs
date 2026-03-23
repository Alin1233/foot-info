#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMode {
    Search,
    TopMatches,
    League,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LeagueTab {
    Fixtures,
    Table,
    TopScorers,
}

impl LeagueTab {
    pub fn next(self) -> Self {
        match self {
            Self::Fixtures => Self::Table,
            Self::Table => Self::TopScorers,
            Self::TopScorers => Self::Fixtures,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Fixtures => Self::TopScorers,
            Self::Table => Self::Fixtures,
            Self::TopScorers => Self::Table,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Fixtures => "Fixtures",
            Self::Table => "Table",
            Self::TopScorers => "Top Scorers",
        }
    }
}
