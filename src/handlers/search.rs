use crate::app::Action;
use crate::models::ViewMode;
use crate::state::AppState;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles key events when the app is in Search mode.
pub fn handle(state: &mut AppState, key_event: KeyEvent) -> Option<Action> {
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
        KeyCode::Char('t') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            state.view_mode = ViewMode::TopMatches;
            state.error_message = None;
            state.status_message = Some("Fetching upcoming top matches...".to_string());
            Some(Action::FetchTopMatches)
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
