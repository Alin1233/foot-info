use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use foot_info::app::Action;
use foot_info::error::AppError;
use foot_info::handlers::{handle_action, handle_key_event};
use foot_info::models::{Match, TopMatch, ViewMode};
use foot_info::state::AppState;

// ── Helpers ──────────────────────────────────────────────────────────────

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

fn ctrl(c: char) -> KeyEvent {
    KeyEvent::new(KeyCode::Char(c), KeyModifiers::CONTROL)
}

fn make_state() -> AppState {
    AppState::new()
}

fn sample_top_matches() -> Vec<TopMatch> {
    vec![
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
        TopMatch {
            teams: "Team E - Team F".into(),
            date: "Tue 21 Feb 2026".into(),
            time: "18:00".into(),
            match_url: "/match/3".into(),
        },
        TopMatch {
            teams: "Team G - Team H".into(),
            date: "Tue 21 Feb 2026".into(),
            time: "20:00".into(),
            match_url: "/match/4".into(),
        },
    ]
}

// ── Global shortcut tests ────────────────────────────────────────────────

#[test]
fn test_ctrl_c_cycles_provider() {
    let mut state = make_state();
    assert_eq!(state.current_provider_index, 0);

    let action = handle_key_event(&mut state, ctrl('c'));
    assert!(action.is_none());
    assert_eq!(state.current_provider_index, 1);
    assert!(state.status_message.is_some());

    // Cycle again
    handle_key_event(&mut state, ctrl('c'));
    assert_eq!(state.current_provider_index, 2);

    // Wrap around
    handle_key_event(&mut state, ctrl('c'));
    assert_eq!(state.current_provider_index, 0);
}

// ── Search mode tests ────────────────────────────────────────────────────

#[test]
fn test_search_esc_sets_exit() {
    let mut state = make_state();
    let action = handle_key_event(&mut state, key(KeyCode::Esc));
    assert!(action.is_none());
    assert!(state.exit);
}

#[test]
fn test_search_enter_with_input_triggers_search() {
    let mut state = make_state();
    state.search_input = "Arsenal".into();
    let action = handle_key_event(&mut state, key(KeyCode::Enter));
    assert!(matches!(action, Some(Action::Search(ref t)) if t == "Arsenal"));
}

#[test]
fn test_search_enter_empty_does_nothing() {
    let mut state = make_state();
    let action = handle_key_event(&mut state, key(KeyCode::Enter));
    assert!(action.is_none());
}

#[test]
fn test_search_char_input_appends() {
    let mut state = make_state();
    handle_key_event(&mut state, key(KeyCode::Char('a')));
    handle_key_event(&mut state, key(KeyCode::Char('b')));
    assert_eq!(state.search_input, "ab");
}

#[test]
fn test_search_backspace_removes_char() {
    let mut state = make_state();
    state.search_input = "abc".into();
    handle_key_event(&mut state, key(KeyCode::Backspace));
    assert_eq!(state.search_input, "ab");
}

#[test]
fn test_search_backspace_on_empty_is_safe() {
    let mut state = make_state();
    handle_key_event(&mut state, key(KeyCode::Backspace));
    assert_eq!(state.search_input, "");
}

#[test]
fn test_search_ctrl_t_switches_to_top_matches() {
    let mut state = make_state();
    let action = handle_key_event(&mut state, ctrl('t'));
    assert_eq!(state.view_mode, ViewMode::TopMatches);
    assert!(state.status_message.is_some());
    assert!(matches!(action, Some(Action::FetchTopMatches)));
}

#[test]
fn test_search_ctrl_s_saves_favorite() {
    let mut state = make_state();
    state.search_input = "Liverpool".into();
    let action = handle_key_event(&mut state, ctrl('s'));
    assert!(action.is_none());
    assert_eq!(state.config.favorite_team, Some("Liverpool".to_string()));
    assert!(state.status_message.as_ref().unwrap().contains("Saved"));
}

#[test]
fn test_search_ctrl_s_empty_does_nothing() {
    let mut state = make_state();
    state.config.favorite_team = None; // Clear any loaded favorite
    let action = handle_key_event(&mut state, ctrl('s'));
    assert!(action.is_none());
    // favorite_team should remain None when search_input is empty
    assert!(state.config.favorite_team.is_none());
}

#[test]
fn test_search_ctrl_f_loads_favorite() {
    let mut state = make_state();
    state.config.favorite_team = Some("Chelsea".into());
    let action = handle_key_event(&mut state, ctrl('f'));
    assert_eq!(state.search_input, "Chelsea");
    assert!(matches!(action, Some(Action::Search(ref t)) if t == "Chelsea"));
}

#[test]
fn test_search_ctrl_f_no_favorite_shows_message() {
    let mut state = make_state();
    state.config.favorite_team = None; // Clear any loaded favorite
    let action = handle_key_event(&mut state, ctrl('f'));
    assert!(action.is_none());
    assert!(
        state
            .status_message
            .as_ref()
            .unwrap()
            .contains("No favorite")
    );
}

#[test]
fn test_search_unknown_key_does_nothing() {
    let mut state = make_state();
    let action = handle_key_event(&mut state, key(KeyCode::F(1)));
    assert!(action.is_none());
    assert!(!state.exit);
}

// ── TopMatches mode tests ────────────────────────────────────────────────

#[test]
fn test_top_matches_esc_returns_to_search() {
    let mut state = make_state();
    state.view_mode = ViewMode::TopMatches;
    let action = handle_key_event(&mut state, key(KeyCode::Esc));
    assert!(action.is_none());
    assert_eq!(state.view_mode, ViewMode::Search);
}

#[test]
fn test_top_matches_ctrl_t_returns_to_search() {
    let mut state = make_state();
    state.view_mode = ViewMode::TopMatches;
    let action = handle_key_event(&mut state, ctrl('t'));
    assert!(action.is_none());
    assert_eq!(state.view_mode, ViewMode::Search);
}

#[test]
fn test_top_matches_down_moves_selection() {
    let mut state = make_state();
    state.view_mode = ViewMode::TopMatches;
    state.top_matches = sample_top_matches();
    state.selected_top_match_index = 0;

    handle_key_event(&mut state, key(KeyCode::Down));
    assert_eq!(state.selected_top_match_index, 1);
}

#[test]
fn test_top_matches_down_at_bottom_stays() {
    let mut state = make_state();
    state.view_mode = ViewMode::TopMatches;
    state.top_matches = sample_top_matches();
    state.selected_top_match_index = 1; // Last in first column (Mon has indices 0,1)

    handle_key_event(&mut state, key(KeyCode::Down));
    // Should stay at 1 because it's the last item in the "Mon" column
    assert_eq!(state.selected_top_match_index, 1);
}

#[test]
fn test_top_matches_up_moves_selection() {
    let mut state = make_state();
    state.view_mode = ViewMode::TopMatches;
    state.top_matches = sample_top_matches();
    state.selected_top_match_index = 1;

    handle_key_event(&mut state, key(KeyCode::Up));
    assert_eq!(state.selected_top_match_index, 0);
}

#[test]
fn test_top_matches_up_at_top_stays() {
    let mut state = make_state();
    state.view_mode = ViewMode::TopMatches;
    state.top_matches = sample_top_matches();
    state.selected_top_match_index = 0;

    handle_key_event(&mut state, key(KeyCode::Up));
    assert_eq!(state.selected_top_match_index, 0);
}

#[test]
fn test_top_matches_right_moves_to_next_column() {
    let mut state = make_state();
    state.view_mode = ViewMode::TopMatches;
    state.top_matches = sample_top_matches();
    state.selected_top_match_index = 0; // First item in Mon column

    handle_key_event(&mut state, key(KeyCode::Right));
    // Should move to first item in Tue column (index 2)
    assert_eq!(state.selected_top_match_index, 2);
}

#[test]
fn test_top_matches_right_at_last_column_stays() {
    let mut state = make_state();
    state.view_mode = ViewMode::TopMatches;
    state.top_matches = sample_top_matches();
    state.selected_top_match_index = 2; // First item in Tue column (last column)

    handle_key_event(&mut state, key(KeyCode::Right));
    assert_eq!(state.selected_top_match_index, 2); // Stays
}

#[test]
fn test_top_matches_left_moves_to_prev_column() {
    let mut state = make_state();
    state.view_mode = ViewMode::TopMatches;
    state.top_matches = sample_top_matches();
    state.selected_top_match_index = 2; // First item in Tue column

    handle_key_event(&mut state, key(KeyCode::Left));
    // Should move to first item in Mon column (index 0)
    assert_eq!(state.selected_top_match_index, 0);
}

#[test]
fn test_top_matches_left_at_first_column_stays() {
    let mut state = make_state();
    state.view_mode = ViewMode::TopMatches;
    state.top_matches = sample_top_matches();
    state.selected_top_match_index = 0;

    handle_key_event(&mut state, key(KeyCode::Left));
    assert_eq!(state.selected_top_match_index, 0);
}

#[test]
fn test_top_matches_right_clamps_row_to_shorter_column() {
    // Mon column has 2 items (indices 0,1), Tue column has 2 items (indices 2,3).
    // Start at Mon column row 1 (index 1), move right → Tue column row 1 (index 3).
    let mut state = make_state();
    state.view_mode = ViewMode::TopMatches;
    state.top_matches = sample_top_matches();
    state.selected_top_match_index = 1;

    handle_key_event(&mut state, key(KeyCode::Right));
    assert_eq!(state.selected_top_match_index, 3);
}

#[test]
fn test_top_matches_enter_selects_match_and_searches() {
    let mut state = make_state();
    state.view_mode = ViewMode::TopMatches;
    state.top_matches = sample_top_matches();
    state.selected_top_match_index = 0;

    let action = handle_key_event(&mut state, key(KeyCode::Enter));
    assert_eq!(state.view_mode, ViewMode::Search);
    assert_eq!(state.search_input, "Team A");
    assert!(matches!(action, Some(Action::Search(ref t)) if t == "Team A"));
}

#[test]
fn test_top_matches_enter_with_no_matches_does_nothing() {
    let mut state = make_state();
    state.view_mode = ViewMode::TopMatches;
    // No top_matches populated

    let action = handle_key_event(&mut state, key(KeyCode::Enter));
    assert!(action.is_none());
}

#[test]
fn test_top_matches_nav_with_empty_matches_does_nothing() {
    let mut state = make_state();
    state.view_mode = ViewMode::TopMatches;

    handle_key_event(&mut state, key(KeyCode::Down));
    assert_eq!(state.selected_top_match_index, 0);

    handle_key_event(&mut state, key(KeyCode::Up));
    assert_eq!(state.selected_top_match_index, 0);

    handle_key_event(&mut state, key(KeyCode::Left));
    assert_eq!(state.selected_top_match_index, 0);

    handle_key_event(&mut state, key(KeyCode::Right));
    assert_eq!(state.selected_top_match_index, 0);
}

#[test]
fn test_top_matches_unknown_key_does_nothing() {
    let mut state = make_state();
    state.view_mode = ViewMode::TopMatches;
    let action = handle_key_event(&mut state, key(KeyCode::F(1)));
    assert!(action.is_none());
}

// ── handle_action tests ──────────────────────────────────────────────────

#[test]
fn test_action_search_sets_loading() {
    let mut state = make_state();
    state.error_message = Some("old error".into());
    state.matches = vec![Match {
        teams: "X".into(),
        competition: "Y".into(),
        date: "D".into(),
        time: "T".into(),
        channels: vec![],
    }];

    let should_spawn = handle_action(&mut state, &Action::Search("Arsenal".into()));
    assert!(should_spawn);
    assert!(state.is_loading);
    assert!(state.error_message.is_none());
    assert!(state.matches.is_empty());
}

#[test]
fn test_action_matches_found_stores_results() {
    let mut state = make_state();
    state.is_loading = true;

    let matches = vec![Match {
        teams: "Arsenal v Chelsea".into(),
        competition: "Premier League".into(),
        date: "Sat 22 Feb".into(),
        time: "15:00".into(),
        channels: vec!["Sky Sports".into()],
    }];

    let should_spawn = handle_action(&mut state, &Action::MatchesFound(matches.clone()));
    assert!(!should_spawn);
    assert!(!state.is_loading);
    assert_eq!(state.matches.len(), 1);
    assert_eq!(state.matches[0].teams, "Arsenal v Chelsea");
}

#[test]
fn test_action_error_stores_message() {
    let mut state = make_state();
    state.is_loading = true;

    let err = AppError::TeamNotFound("Arsenal".into());
    let should_spawn = handle_action(&mut state, &Action::Error(err));
    assert!(!should_spawn);
    assert!(!state.is_loading);
    assert!(state.error_message.is_some());
    assert!(state.error_message.as_ref().unwrap().contains("Arsenal"));
}

#[test]
fn test_action_fetch_top_matches_sets_loading() {
    let mut state = make_state();
    state.top_matches = sample_top_matches();
    state.selected_top_match_index = 2;

    let should_spawn = handle_action(&mut state, &Action::FetchTopMatches);
    assert!(should_spawn);
    assert!(state.is_loading);
    assert!(state.error_message.is_none());
    assert!(state.top_matches.is_empty());
    assert_eq!(state.selected_top_match_index, 0);
}

#[test]
fn test_action_top_matches_found_stores_results() {
    let mut state = make_state();
    state.is_loading = true;

    let top = sample_top_matches();
    let should_spawn = handle_action(&mut state, &Action::TopMatchesFound(top));
    assert!(!should_spawn);
    assert!(!state.is_loading);
    assert_eq!(state.top_matches.len(), 4);
    assert_eq!(state.selected_top_match_index, 0);
    assert!(state.status_message.as_ref().unwrap().contains("4"));
}
