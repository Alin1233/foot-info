use foot_info::models::ViewMode;
use foot_info::state::AppState;
use foot_info::ui;
use ratatui::Terminal;
use ratatui::backend::TestBackend;

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

// ── render::draw tests (top-level draw function) ─────────────────────────

#[test]
fn test_draw_search_mode_shows_title_and_instructions() {
    let backend = TestBackend::new(120, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    let state = AppState::new();

    terminal
        .draw(|f| {
            ui::draw(f, &state);
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(
        output.contains("FOOTBALL MATCH CHANNELS INFO"),
        "Should show app title"
    );
    assert!(output.contains("Esc"), "Should show Esc shortcut");
    assert!(output.contains("Enter"), "Should show Enter shortcut");
    assert!(output.contains("Ctrl+t"), "Should show Ctrl+t shortcut");
    assert!(output.contains("Ctrl+c"), "Should show Ctrl+c shortcut");
}

#[test]
fn test_draw_search_mode_shows_country_in_title() {
    let backend = TestBackend::new(120, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    let state = AppState::new();

    terminal
        .draw(|f| {
            ui::draw(f, &state);
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(
        output.contains("[UK]"),
        "Should show the default provider country in title"
    );
}

#[test]
fn test_draw_top_matches_mode_shows_different_instructions() {
    let backend = TestBackend::new(120, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut state = AppState::new();
    state.view_mode = ViewMode::TopMatches;

    terminal
        .draw(|f| {
            ui::draw(f, &state);
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(
        output.contains("FOOTBALL MATCH CHANNELS INFO"),
        "Should show app title"
    );
    assert!(output.contains("Back"), "Should show Back label for Esc");
    assert!(output.contains("Navigate"), "Should show Navigate label");
    assert!(
        output.contains("Select Match"),
        "Should show match selection label"
    );
}

#[test]
fn test_draw_with_different_provider_shows_country() {
    let backend = TestBackend::new(120, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut state = AppState::new();
    state.current_provider_index = 1; // US provider

    terminal
        .draw(|f| {
            ui::draw(f, &state);
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(output.contains("[US]"), "Title should reflect US provider");
}

#[test]
fn test_draw_fr_provider_shows_country() {
    let backend = TestBackend::new(120, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut state = AppState::new();
    state.current_provider_index = 2; // FR provider

    terminal
        .draw(|f| {
            ui::draw(f, &state);
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(output.contains("[FR]"), "Title should reflect FR provider");
}

#[test]
fn test_draw_search_contains_search_bar() {
    let backend = TestBackend::new(120, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut state = AppState::new();
    state.search_input = "Liverpool".into();

    terminal
        .draw(|f| {
            ui::draw(f, &state);
        })
        .unwrap();

    let output = buffer_to_string(&terminal);
    assert!(
        output.contains("Enter Team:"),
        "Should contain search bar inside main frame"
    );
    assert!(output.contains("Liverpool"));
}
