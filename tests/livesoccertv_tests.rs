use foot_info::providers::livesoccertv;

fn load_resource(name: &str) -> String {
    let path = format!("{}/tests/resources/{}", env!("CARGO_MANIFEST_DIR"), name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to load test resource '{}': {}", path, e))
}

#[test]
fn test_parse_real_html_returns_top_matches() {
    let html = load_resource("livesoccertv.html");
    let result = livesoccertv::parse_html(&html);
    assert!(result.is_ok(), "Expected Ok, got: {:?}", result);

    let matches = result.unwrap();
    assert!(
        !matches.is_empty(),
        "Expected at least one top match from real HTML"
    );

    // Should have a good number of upcoming matches
    assert!(
        matches.len() >= 5,
        "Expected at least 5 top matches, got {}",
        matches.len()
    );
}

#[test]
fn test_parse_real_html_has_valid_structure() {
    let html = load_resource("livesoccertv.html");
    let matches = livesoccertv::parse_html(&html).unwrap();

    for m in &matches {
        assert!(!m.teams.is_empty(), "Teams should not be empty");
        assert!(!m.date.is_empty(), "Date should not be empty");
        assert!(
            m.teams.contains(" - "),
            "Teams should contain ' - ' separator, got: '{}'",
            m.teams
        );
    }
}

#[test]
fn test_parse_real_html_has_known_teams() {
    let html = load_resource("livesoccertv.html");
    let matches = livesoccertv::parse_html(&html).unwrap();

    // Check that some well-known teams appear
    let all_teams: String = matches
        .iter()
        .map(|m| m.teams.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    let has_big_team = matches.iter().any(|m| {
        m.teams.contains("Bayern")
            || m.teams.contains("Real Madrid")
            || m.teams.contains("Barcelona")
            || m.teams.contains("Liverpool")
            || m.teams.contains("Arsenal")
            || m.teams.contains("Chelsea")
            || m.teams.contains("Manchester")
    });

    assert!(
        has_big_team,
        "Expected at least one major team in top matches. Got: {}",
        all_teams
    );
}

#[test]
fn test_parse_real_html_has_match_urls() {
    let html = load_resource("livesoccertv.html");
    let matches = livesoccertv::parse_html(&html).unwrap();

    let has_urls = matches.iter().any(|m| m.match_url.starts_with("/match/"));
    assert!(
        has_urls,
        "At least one match should have a URL starting with /match/"
    );
}

#[test]
fn test_parse_empty_html_returns_error() {
    let result = livesoccertv::parse_html("<html><body></body></html>");
    assert!(result.is_err());
}

#[test]
fn test_parse_html_without_top_matches_section() {
    let html = r##"
    <html><body>
      <div class="fheader">Some Other Section</div>
      <div>Some content</div>
    </body></html>"##;

    let result = livesoccertv::parse_html(html);
    assert!(result.is_err());
}

#[test]
fn test_parse_html_with_empty_top_matches_section() {
    let html = r##"
    <html><body>
      <div class="fheader">Upcoming Top Matches</div>
      <div class="fheader">Next Section</div>
    </body></html>"##;

    let result = livesoccertv::parse_html(html);
    assert!(result.is_err());
}
