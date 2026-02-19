use foot_info::models::{Match, TopMatch};
use foot_info::ui::components::match_list::{self, ResultsState};
use foot_info::ui::components::search_bar;
use foot_info::ui::components::status_bar;
use foot_info::ui::components::top_matches_list::{self, TopMatchesState};
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;

fn buffer_to_string(terminal: &Terminal<TestBackend>) -> String {
    let buf = terminal.backend().buffer().clone();
    let mut output = String::new();
    for y in 0..buf.area.height {
        for x in 0..buf.area.width {
            output.push_str(buf[(x, y)].symbol());
        }
        output.push('\n');
    }
    output
}

// ── search_bar tests ─────────────────────────────────────────────────────

#[test]
fn test_search_bar_renders_prompt() {
    let backend = TestBackend::new(60, 3);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 60, 3);
            search_bar::render(f, area, "Arsenal");
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(output.contains("Enter Team:"), "Should show prompt label");
    assert!(
        output.contains("Arsenal"),
        "Should show the typed team name"
    );
}

#[test]
fn test_search_bar_renders_empty_input() {
    let backend = TestBackend::new(60, 3);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 60, 3);
            search_bar::render(f, area, "");
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(
        output.contains("Enter Team:"),
        "Should show prompt even with no input"
    );
}

// ── status_bar tests ─────────────────────────────────────────────────────

#[test]
fn test_status_bar_renders_message() {
    let backend = TestBackend::new(60, 1);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 60, 1);
            status_bar::render(f, area, Some("Saved favorite: Arsenal"));
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(output.contains("Saved favorite: Arsenal"));
}

#[test]
fn test_status_bar_renders_nothing_when_none() {
    let backend = TestBackend::new(60, 1);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 60, 1);
            status_bar::render(f, area, None);
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    // Should be blank (just spaces)
    assert_eq!(output.trim(), "");
}

// ── match_list tests ─────────────────────────────────────────────────────

#[test]
fn test_match_list_renders_loading() {
    let backend = TestBackend::new(80, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 80, 10);
            match_list::render(f, area, &ResultsState::Loading);
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(output.contains("Scraping matches"));
}

#[test]
fn test_match_list_renders_error() {
    let backend = TestBackend::new(80, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 80, 10);
            match_list::render(f, area, &ResultsState::Error("Team not found"));
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(output.contains("Error:"));
    assert!(output.contains("Team not found"));
}

#[test]
fn test_match_list_renders_matches() {
    let backend = TestBackend::new(100, 15);
    let mut terminal = Terminal::new(backend).unwrap();

    let matches = vec![
        Match {
            teams: "Arsenal v Chelsea".into(),
            competition: "Premier League".into(),
            date: "Sat 22 Feb".into(),
            time: "15:00".into(),
            channels: vec!["Sky Sports".into()],
        },
        Match {
            teams: "Liverpool v Man City".into(),
            competition: "FA Cup".into(),
            date: "Sun 23 Feb".into(),
            time: "14:00".into(),
            channels: vec![],
        },
    ];

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 100, 15);
            match_list::render(f, area, &ResultsState::Matches(&matches));
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(
        output.contains("Arsenal v Chelsea"),
        "Should show first match teams"
    );
    assert!(output.contains("Sky Sports"), "Should show channel info");
    assert!(
        output.contains("No TV info"),
        "Should show fallback for empty channels"
    );
}

#[test]
fn test_match_list_renders_empty() {
    let backend = TestBackend::new(80, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 80, 10);
            match_list::render(f, area, &ResultsState::Empty);
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    // Empty state should be blank
    assert_eq!(output.trim(), "");
}

// ── top_matches_list tests ───────────────────────────────────────────────

#[test]
fn test_top_matches_list_renders_loading() {
    let backend = TestBackend::new(80, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 80, 10);
            top_matches_list::render(f, area, &TopMatchesState::Loading);
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(output.contains("Fetching upcoming top matches"));
}

#[test]
fn test_top_matches_list_renders_error() {
    let backend = TestBackend::new(80, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 80, 10);
            top_matches_list::render(f, area, &TopMatchesState::Error("Network error"));
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(output.contains("Error:"));
    assert!(output.contains("Network error"));
}

#[test]
fn test_top_matches_list_renders_matches_with_columns() {
    let backend = TestBackend::new(100, 20);
    let mut terminal = Terminal::new(backend).unwrap();

    let matches = vec![
        TopMatch {
            teams: "Team A - Team B".into(),
            date: "Mon 20 Feb 2026".into(),
            time: "20:00".into(),
            match_url: "/match/1".into(),
        },
        TopMatch {
            teams: "Team C - Team D".into(),
            date: "Tue 21 Feb 2026".into(),
            time: "18:00".into(),
            match_url: "/match/2".into(),
        },
    ];

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 100, 20);
            top_matches_list::render(
                f,
                area,
                &TopMatchesState::Matches {
                    matches: &matches,
                    selected_index: 0,
                },
            );
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(
        output.contains("Team A - Team B"),
        "Should show first match"
    );
    assert!(
        output.contains("Team C - Team D"),
        "Should show second match"
    );
    assert!(
        output.contains("Mon 20 Feb 2026"),
        "Should show first date as column header"
    );
    assert!(
        output.contains("Tue 21 Feb 2026"),
        "Should show second date as column header"
    );
}

#[test]
fn test_top_matches_list_renders_selection_marker() {
    let backend = TestBackend::new(100, 20);
    let mut terminal = Terminal::new(backend).unwrap();

    let matches = vec![
        TopMatch {
            teams: "Team A - Team B".into(),
            date: "Mon 20 Feb 2026".into(),
            time: "20:00".into(),
            match_url: "/match/1".into(),
        },
        TopMatch {
            teams: "Team C - Team D".into(),
            date: "Mon 20 Feb 2026".into(),
            time: "21:00".into(),
            match_url: "/match/2".into(),
        },
    ];

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 100, 20);
            top_matches_list::render(
                f,
                area,
                &TopMatchesState::Matches {
                    matches: &matches,
                    selected_index: 0,
                },
            );
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(
        output.contains("▸"),
        "Should show selection marker for selected item"
    );
}

#[test]
fn test_top_matches_list_renders_empty() {
    let backend = TestBackend::new(80, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 80, 10);
            top_matches_list::render(f, area, &TopMatchesState::Empty);
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert_eq!(output.trim(), "");
}
