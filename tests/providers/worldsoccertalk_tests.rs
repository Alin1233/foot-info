use foot_info::providers::worldsoccertalk;

fn load_resource(name: &str) -> String {
    let path = format!("{}/tests/resources/{}", env!("CARGO_MANIFEST_DIR"), name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to load test resource '{}': {}", path, e))
}

#[test]
fn test_parse_real_html_returns_matches() {
    let html = load_resource("worldsoccertalk.html");
    let result = worldsoccertalk::parse_html(&html, "Manchester United");
    assert!(result.is_ok(), "Expected Ok, got: {:?}", result);

    let matches = result.unwrap();
    assert!(
        !matches.is_empty(),
        "Expected at least one match from real HTML"
    );

    // Verify the structure of each parsed match
    for m in &matches {
        assert!(!m.teams.is_empty(), "Teams should not be empty");
        assert!(!m.date.is_empty(), "Date should not be empty");
        assert!(!m.competition.is_empty(), "Competition should not be empty");
    }
}

#[test]
fn test_parse_real_html_has_channels() {
    let html = load_resource("worldsoccertalk.html");
    let matches = worldsoccertalk::parse_html(&html, "Manchester United").unwrap();

    let has_channels = matches.iter().any(|m| !m.channels.is_empty());
    assert!(
        has_channels,
        "At least one match should have channel/provider information"
    );
}

#[test]
fn test_parse_real_html_parses_competition_from_title() {
    let html = load_resource("worldsoccertalk.html");
    let matches = worldsoccertalk::parse_html(&html, "Manchester United").unwrap();

    // Matches with parentheses in the title should have competition extracted
    let has_known_competition = matches.iter().any(|m| m.competition != "Unknown Competition");
    assert!(
        has_known_competition,
        "At least one match should have a parsed competition name. Got: {:?}",
        matches.iter().map(|m| &m.competition).collect::<Vec<_>>()
    );
}

#[test]
fn test_parse_empty_schedule_returns_error() {
    let html = "<html><body><div>No schedule here</div></body></html>";
    let result = worldsoccertalk::parse_html(html, "FakeTeam");

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("No matches scheduled"),
        "Expected NoMatchesScheduled error, got: {}",
        err_msg
    );
}

#[test]
fn test_parse_title_without_parentheses_gives_unknown_competition() {
    // Inline minimal HTML with a title that has no parentheses
    let html = r##"
    <html><body>
      <div class="flex flex-col w-full">
        <div>
          <h3 class="text-stvsDate">Monday, January 1</h3>
          <ul>
            <li class="border-stvsMatchBorderColor">
              <span class="text-stvsMatchHour">3:00 PM ET</span>
              <span class="text-stvsMatchTitle">Some Match No Parens</span>
              <span class="text-stvsProviderLink">
                <a href="#">ESPN</a>
              </span>
            </li>
          </ul>
        </div>
      </div>
    </body></html>"##;

    let result = worldsoccertalk::parse_html(html, "Test");
    let matches = result.unwrap();
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].teams, "Some Match No Parens");
    assert_eq!(matches[0].competition, "Unknown Competition");
}
