use foot_info_core::providers::league_stats;

fn load_resource(name: &str) -> String {
    let path = format!("{}/tests/resources/{}", env!("CARGO_MANIFEST_DIR"), name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to load test resource '{}': {}", path, e))
}

#[test]
fn test_parse_real_html_returns_league_stats() {
    let html = load_resource("livesoccertv_league.html");
    let result = league_stats::parse_html(&html);
    assert!(result.is_ok(), "Expected Ok, got: {:?}", result);

    let stats = result.unwrap();
    assert_eq!(stats.competition, "Premier League");
    
    // Check fixtures
    assert!(!stats.fixtures.is_empty(), "Expected some fixtures");
    assert!(stats.fixtures.len() >= 5, "Expected at least 5 fixtures");
    assert_eq!(stats.fixtures[0].home_team, "Everton");
    assert_eq!(stats.fixtures[0].away_team, "Chelsea");
    
    // Check table
    assert!(!stats.table.is_empty(), "Expected league table");
    assert_eq!(stats.table.len(), 20, "Expected 20 teams in Premier League table");
    assert_eq!(stats.table[0].team, "Arsenal");
    assert_eq!(stats.table[0].position, 1);
    
    // Check top scorers
    assert!(!stats.top_scorers.is_empty(), "Expected top scorers");
    assert_eq!(stats.top_scorers[0].player, "E. Haaland");
    assert_eq!(stats.top_scorers[0].team, "Manchester City");
}

#[test]
fn test_parse_empty_html_returns_error() {
    let result = league_stats::parse_html("<html><body></body></html>");
    assert!(result.is_err());
}
