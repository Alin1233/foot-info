use foot_info::models::{Match, TopMatch, ViewMode};
use foot_info::state::AppState;
use foot_info::ui::views;
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

// ── Search view tests ────────────────────────────────────────────────────

#[test]
fn test_search_view_renders_search_bar() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut state = AppState::new();
    state.search_input = "Arsenal".into();

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 80, 24);
            views::search::draw(f, area, &state);
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(output.contains("Enter Team:"));
    assert!(output.contains("Arsenal"));
}

#[test]
fn test_search_view_with_loading_state() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut state = AppState::new();
    state.is_loading = true;

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 80, 24);
            views::search::draw(f, area, &state);
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(output.contains("Scraping matches"));
}

#[test]
fn test_search_view_with_error() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut state = AppState::new();
    state.error_message = Some("Team not found".into());

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 80, 24);
            views::search::draw(f, area, &state);
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(output.contains("Team not found"));
}

#[test]
fn test_search_view_with_matches() {
    let backend = TestBackend::new(100, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut state = AppState::new();
    state.matches = vec![Match {
        teams: "Arsenal v Chelsea".into(),
        competition: "PL".into(),
        date: "Sat 22 Feb".into(),
        time: "15:00".into(),
        channels: vec!["Sky".into()],
    }];

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 100, 24);
            views::search::draw(f, area, &state);
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(output.contains("Arsenal v Chelsea"));
}

#[test]
fn test_search_view_with_status_message() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut state = AppState::new();
    state.status_message = Some("Saved favorite: Arsenal".into());

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 80, 24);
            views::search::draw(f, area, &state);
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(output.contains("Saved favorite: Arsenal"));
}

// ── TopMatches view tests ────────────────────────────────────────────────

#[test]
fn test_top_matches_view_renders_loading() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut state = AppState::new();
    state.view_mode = ViewMode::TopMatches;
    state.is_loading = true;

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 80, 24);
            views::top_matches::draw(f, area, &state);
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(output.contains("Fetching upcoming top matches"));
}

#[test]
fn test_top_matches_view_renders_error() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut state = AppState::new();
    state.view_mode = ViewMode::TopMatches;
    state.error_message = Some("Network error".into());

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 80, 24);
            views::top_matches::draw(f, area, &state);
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(output.contains("Network error"));
}

#[test]
fn test_top_matches_view_renders_matches() {
    let backend = TestBackend::new(100, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut state = AppState::new();
    state.view_mode = ViewMode::TopMatches;
    state.top_matches = vec![TopMatch {
        teams: "Team A - Team B".into(),
        date: "Mon 20 Feb 2026".into(),
        time: "20:00".into(),
        match_url: "/match/1".into(),
    }];

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 100, 24);
            views::top_matches::draw(f, area, &state);
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(output.contains("Team A - Team B"));
}
