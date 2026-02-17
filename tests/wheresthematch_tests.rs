use foot_info::providers::wheresthematch;

fn load_resource(name: &str) -> String {
    let path = format!("{}/tests/resources/{}", env!("CARGO_MANIFEST_DIR"), name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to load test resource '{}': {}", path, e))
}

#[test]
fn test_parse_real_html_returns_matches() {
    let html = load_resource("wheresthematch.html");
    let result = wheresthematch::parse_html(&html, "Manchester United");
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
        assert!(
            !m.teams.contains("WATCH TODAY'S GAME LIVE!"),
            "Ad rows should be filtered out"
        );
        assert!(
            !m.teams.contains("SKY DEALS"),
            "Ad rows should be filtered out"
        );
    }
}

#[test]
fn test_parse_real_html_contains_man_utd() {
    let html = load_resource("wheresthematch.html");
    let matches = wheresthematch::parse_html(&html, "Manchester United").unwrap();

    let has_man_utd = matches.iter().any(|m| {
        m.teams.contains("Man") || m.teams.contains("Manchester")
    });
    assert!(
        has_man_utd,
        "At least one match should reference Manchester United. Got: {:?}",
        matches.iter().map(|m| &m.teams).collect::<Vec<_>>()
    );
}

#[test]
fn test_parse_real_html_has_channels() {
    let html = load_resource("wheresthematch.html");
    let matches = wheresthematch::parse_html(&html, "Manchester United").unwrap();

    let has_channels = matches.iter().any(|m| !m.channels.is_empty());
    assert!(
        has_channels,
        "At least one match should have channel information"
    );
}

#[test]
fn test_parse_invalid_html_returns_team_not_found() {
    let html = "<html><body><p>Invalid URL Format</p></body></html>";
    let result = wheresthematch::parse_html(html, "FakeTeam");

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("not found"),
        "Expected TeamNotFound error, got: {}",
        err_msg
    );
}

#[test]
fn test_parse_empty_page_returns_team_not_found() {
    let html = "<html><body><p>Nothing here</p></body></html>";
    let result = wheresthematch::parse_html(html, "NoTeam");

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("not found"),
        "Expected TeamNotFound error, got: {}",
        err_msg
    );
}

#[test]
fn test_parse_page_with_header_but_no_matches() {
    let html = r#"
    <html><body>
      <h1 class="intro">Some Team on TV</h1>
      <div id="teamswrapper"><table><tbody></tbody></table></div>
    </body></html>"#;
    let result = wheresthematch::parse_html(html, "Some Team");

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("No matches scheduled"),
        "Expected NoMatchesScheduled error, got: {}",
        err_msg
    );
}
