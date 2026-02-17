use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::app::Action;
use crate::state::AppState;

pub fn handle_key_event(state: &mut AppState, key_event: KeyEvent) -> Option<Action> {
    match key_event.code {
        KeyCode::Esc => {
            state.exit = true;
            None
        }
        KeyCode::Enter => {
            if !state.search_input.is_empty() {
                state.status_message = None;
                Some(Action::Search(state.search_input.clone()))
            } else {
                None
            }
        }
        KeyCode::Char('s') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            if !state.search_input.is_empty() {
                state.config.favorite_team = Some(state.search_input.clone());
                if let Err(e) = state.config.save() {
                    state.error_message = Some(format!("Failed to save config: {}", e));
                } else {
                    state.status_message = Some(format!("Saved favorite: {}", state.search_input));
                }
            }
            None
        }
        KeyCode::Char('f') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Some(team) = &state.config.favorite_team {
                state.search_input = team.clone();
                state.status_message = Some(format!("Loaded favorite: {}", team));
                Some(Action::Search(team.clone()))
            } else {
                state.status_message = Some("No favorite team saved.".to_string());
                None
            }
        }
        KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            state.current_provider_index = (state.current_provider_index + 1) % state.providers.len();
            let provider = state.get_current_provider();
            state.status_message = Some(format!("Switched to: {} ({})", provider.country(), provider.name()));
            None
        }
        KeyCode::Char(c) => {
            state.search_input.push(c);
            None
        }
        KeyCode::Backspace => {
            state.search_input.pop();
            None
        }
        _ => None,
    }
}

pub fn handle_action(state: &mut AppState, action: &Action) -> bool {
    match action {
        Action::Search(_) => {
            state.is_loading = true;
            state.error_message = None;
            state.matches.clear();
            true // caller should spawn the async fetch task
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
    }
}
