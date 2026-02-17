use chrono::{NaiveDate, Datelike};
use foot_info::providers::matchstv;

fn load_resource(name: &str) -> String {
    let path = format!("{}/tests/resources/{}", env!("CARGO_MANIFEST_DIR"), name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to load test resource '{}': {}", path, e))
}

// =============================================================================
// HTML Parsing Tests (using real HTML)
// =============================================================================

#[test]
fn test_parse_real_html_returns_matches() {
    let html = load_resource("matchstv.html");
    let result = matchstv::parse_html(&html, "Manchester United");
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
        assert!(!m.time.is_empty(), "Time should not be empty");
    }
}

#[test]
fn test_parse_real_html_has_channels() {
    let html = load_resource("matchstv.html");
    let matches = matchstv::parse_html(&html, "Manchester United").unwrap();

    let has_channels = matches.iter().any(|m| !m.channels.is_empty());
    assert!(
        has_channels,
        "At least one match should have channel information"
    );
}

#[test]
fn test_parse_real_html_has_competitions() {
    let html = load_resource("matchstv.html");
    let matches = matchstv::parse_html(&html, "Manchester United").unwrap();

    let has_competition = matches.iter().any(|m| !m.competition.is_empty());
    assert!(
        has_competition,
        "At least one match should have competition info. Got: {:?}",
        matches.iter().map(|m| &m.competition).collect::<Vec<_>>()
    );
}

#[test]
fn test_parse_empty_schedule_returns_error() {
    let html = r#"<html><body><table class="programme-tv fixtures"></table></body></html>"#;
    let result = matchstv::parse_html(html, "FakeTeam");

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("No matches scheduled"),
        "Expected NoMatchesScheduled error, got: {}",
        err_msg
    );
}

// =============================================================================
// French Date Parsing Tests
// =============================================================================

#[test]
fn test_parse_french_date_mars() {
    let result = matchstv::parse_french_date("samedi 15 mars");
    assert!(result.is_some(), "Should parse 'samedi 15 mars'");

    let (formatted, naive) = result.unwrap();
    assert_eq!(naive.month(), 3);
    assert_eq!(naive.day(), 15);
    assert!(!formatted.is_empty());
}

#[test]
fn test_parse_french_date_fevrier_with_accent() {
    let result = matchstv::parse_french_date("mardi 1 février");
    assert!(result.is_some(), "Should parse 'février' with accent");

    let (_, naive) = result.unwrap();
    assert_eq!(naive.month(), 2);
    assert_eq!(naive.day(), 1);
}

#[test]
fn test_parse_french_date_decembre_without_accent() {
    let result = matchstv::parse_french_date("lundi 25 decembre");
    assert!(result.is_some(), "Should parse 'decembre' without accent");

    let (_, naive) = result.unwrap();
    assert_eq!(naive.month(), 12);
    assert_eq!(naive.day(), 25);
}

#[test]
fn test_parse_french_date_invalid_input() {
    assert!(matchstv::parse_french_date("invalid").is_none());
}

#[test]
fn test_parse_french_date_unknown_month() {
    assert!(matchstv::parse_french_date("lundi 1 randommonth").is_none());
}

#[test]
fn test_parse_french_date_too_few_parts() {
    assert!(matchstv::parse_french_date("mars").is_none());
    assert!(matchstv::parse_french_date("15 mars").is_none());
}

// =============================================================================
// French Time Conversion Tests
// =============================================================================

#[test]
fn test_convert_french_time_valid() {
    let date = NaiveDate::from_ymd_opt(2026, 3, 15).unwrap();
    let result = matchstv::convert_french_time_to_local(date, "21h00");
    assert!(result.is_some(), "Should convert '21h00' to local time");

    let (date_str, time_str) = result.unwrap();
    assert!(!date_str.is_empty());
    assert!(!time_str.is_empty());
}

#[test]
fn test_convert_french_time_midday() {
    let date = NaiveDate::from_ymd_opt(2026, 6, 1).unwrap();
    let result = matchstv::convert_french_time_to_local(date, "12h30");
    assert!(result.is_some());
}

#[test]
fn test_convert_french_time_invalid() {
    let date = NaiveDate::from_ymd_opt(2026, 3, 15).unwrap();
    assert!(matchstv::convert_french_time_to_local(date, "not-a-time").is_none());
}

#[test]
fn test_convert_french_time_midnight() {
    let date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    let result = matchstv::convert_french_time_to_local(date, "00h00");
    assert!(result.is_some());
}

// =============================================================================
// Competition Parsing Edge Case
// =============================================================================

#[test]
fn test_competition_takes_first_comma_segment() {
    let html = r##"
    <html><body>
      <table class="programme-tv fixtures">
        <tr>
          <td colspan="4"><h3><a href="/jour/1-mars">samedi 1 mars</a></h3></td>
        </tr>
        <tr>
          <td class="date">20h00</td>
          <td class="fixture">
            <h4><a href="#">Team A - Team B</a></h4>
            <span class="competitions">Ligue 1, Journée 25</span>
          </td>
          <td class="channel"><img title="Canal+" /></td>
        </tr>
      </table>
    </body></html>"##;

    let result = matchstv::parse_html(html, "Test");
    let matches = result.unwrap();
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].competition, "Ligue 1");
    assert_eq!(matches[0].channels, vec!["Canal+"]);
}
