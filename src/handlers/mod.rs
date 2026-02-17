mod search;
mod top_matches;

use crate::app::Action;
use crate::models::ViewMode;
use crate::state::AppState;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles a key press event, mutating state and optionally returning an Action to dispatch.
pub fn handle_key_event(state: &mut AppState, key_event: KeyEvent) -> Option<Action> {
    // Shared shortcuts available on all screens
    if let Some(action) = handle_global(state, key_event) {
        return action;
    }

    // Mode-specific shortcuts
    match state.view_mode {
        ViewMode::Search => search::handle(state, key_event),
        ViewMode::TopMatches => top_matches::handle(state, key_event),
    }
}

/// Shortcuts available on every screen (e.g. switch country).
/// Returns `Some(Some(action))` if an async action is needed,
/// `Some(None)` if consumed but no action needed,
/// `None` if the key wasn't handled here (fall through to mode handler).
fn handle_global(state: &mut AppState, key_event: KeyEvent) -> Option<Option<Action>> {
    match key_event.code {
        KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            state.current_provider_index =
                (state.current_provider_index + 1) % state.providers.len();
            let provider = state.get_current_provider();
            state.status_message = Some(format!(
                "Switched to: {} ({})",
                provider.country(),
                provider.name()
            ));
            Some(None) // Consumed, no async action needed
        }
        _ => None, // Not handled, fall through to mode handler
    }
}

/// Applies an incoming Action to state. Returns true if an async task should be spawned.
pub fn handle_action(state: &mut AppState, action: &Action) -> bool {
    match action {
        Action::Search(_) => {
            state.is_loading = true;
            state.error_message = None;
            state.matches.clear();
            true
        }
        Action::MatchesFound(matches) => {
            state.is_loading = false;
            state.matches = matches.clone();
            false
        }
        Action::Error(e) => {
            state.is_loading = false;
            state.error_message = Some(e.to_string());
            false
        }
        Action::FetchTopMatches => {
            state.is_loading = true;
            state.error_message = None;
            state.top_matches.clear();
            state.selected_top_match_index = 0;
            true
        }
        Action::TopMatchesFound(top_matches) => {
            state.is_loading = false;
            state.top_matches = top_matches.clone();
            state.selected_top_match_index = 0;
            state.status_message = Some(format!("Found {} upcoming matches", top_matches.len()));
            false
        }
    }
}
