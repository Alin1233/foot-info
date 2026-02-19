use foot_info::config::Config;
use foot_info::models::{Country, ViewMode};
use foot_info::state::AppState;

// ── AppState::new tests ──────────────────────────────────────────────────

#[test]
fn test_new_state_has_empty_search_input() {
    let state = AppState::new();
    assert!(state.search_input.is_empty());
}

#[test]
fn test_new_state_has_no_matches() {
    let state = AppState::new();
    assert!(state.matches.is_empty());
}

#[test]
fn test_new_state_has_no_error() {
    let state = AppState::new();
    assert!(state.error_message.is_none());
}

#[test]
fn test_new_state_is_not_loading() {
    let state = AppState::new();
    assert!(!state.is_loading);
}

#[test]
fn test_new_state_is_not_exit() {
    let state = AppState::new();
    assert!(!state.exit);
}

#[test]
fn test_new_state_starts_in_search_mode() {
    let state = AppState::new();
    assert_eq!(state.view_mode, ViewMode::Search);
}

#[test]
fn test_new_state_has_three_providers() {
    let state = AppState::new();
    assert_eq!(state.providers.len(), 3);
}

#[test]
fn test_new_state_starts_at_first_provider() {
    let state = AppState::new();
    assert_eq!(state.current_provider_index, 0);
}

#[test]
fn test_new_state_has_empty_top_matches() {
    let state = AppState::new();
    assert!(state.top_matches.is_empty());
    assert_eq!(state.selected_top_match_index, 0);
}

// ── get_current_provider tests ───────────────────────────────────────────

#[test]
fn test_get_current_provider_returns_uk_first() {
    let state = AppState::new();
    let provider = state.get_current_provider();
    assert_eq!(provider.country(), Country::UK);
}

#[test]
fn test_get_current_provider_cycles_correctly() {
    let mut state = AppState::new();

    state.current_provider_index = 1;
    assert_eq!(state.get_current_provider().country(), Country::US);

    state.current_provider_index = 2;
    assert_eq!(state.get_current_provider().country(), Country::FR);
}

// ── Country::Display tests ───────────────────────────────────────────────

#[test]
fn test_country_display_uk() {
    assert_eq!(format!("{}", Country::UK), "UK");
}

#[test]
fn test_country_display_us() {
    assert_eq!(format!("{}", Country::US), "US");
}

#[test]
fn test_country_display_fr() {
    assert_eq!(format!("{}", Country::FR), "FR");
}

// ── Config tests ─────────────────────────────────────────────────────────

#[test]
fn test_config_default_has_no_favorite() {
    let config = Config::default();
    assert!(config.favorite_team.is_none());
}

#[test]
fn test_config_load_returns_config() {
    // Config::load() should always return a Config (either from file or default)
    let config = Config::load();
    // Just verify it doesn't panic and returns something
    let _ = config.favorite_team;
}

#[test]
fn test_config_save_and_load_roundtrip() {
    // Load current config to restore later
    let original = Config::load();

    // Save a test value
    let mut test_config = Config::default();
    test_config.favorite_team = Some("TestTeam123".to_string());
    test_config.save().expect("Failed to save config");

    // Load it back
    let loaded = Config::load();
    assert_eq!(loaded.favorite_team, Some("TestTeam123".to_string()));

    // Restore original config
    original.save().expect("Failed to restore config");
}
