use crate::app::Action;
use crate::models::{TopMatch, ViewMode};
use crate::state::AppState;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

/// Handles key events when the app is in TopMatches mode.
pub fn handle(state: &mut AppState, key_event: KeyEvent) -> Option<Action> {
    match key_event.code {
        KeyCode::Esc => {
            state.view_mode = ViewMode::Search;
            state.status_message = None;
            state.error_message = None;
            None
        }
        KeyCode::Char('t') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            state.view_mode = ViewMode::Search;
            state.status_message = None;
            None
        }
        KeyCode::Up => {
            if state.selected_top_match_index > 0 {
                state.selected_top_match_index -= 1;
            }
            None
        }
        KeyCode::Down => {
            if state.selected_top_match_index + 1 < state.top_matches.len() {
                state.selected_top_match_index += 1;
            }
            None
        }
        KeyCode::Left => {
            let groups = date_groups(&state.top_matches);
            if let Some((col, row)) = flat_to_col_row(&groups, state.selected_top_match_index) {
                if col > 0 {
                    let target_row = row.min(groups[col - 1].1.len() - 1);
                    state.selected_top_match_index = groups[col - 1].1[target_row];
                }
            }
            None
        }
        KeyCode::Right => {
            let groups = date_groups(&state.top_matches);
            if let Some((col, row)) = flat_to_col_row(&groups, state.selected_top_match_index) {
                if col + 1 < groups.len() {
                    let target_row = row.min(groups[col + 1].1.len() - 1);
                    state.selected_top_match_index = groups[col + 1].1[target_row];
                }
            }
            None
        }
        KeyCode::Enter => {
            if let Some(top_match) = state.top_matches.get(state.selected_top_match_index) {
                let team = top_match
                    .teams
                    .split(" - ")
                    .next()
                    .unwrap_or(&top_match.teams)
                    .trim()
                    .to_string();
                state.search_input = team.clone();
                state.view_mode = ViewMode::Search;
                state.status_message = None;
                Some(Action::Search(team))
            } else {
                None
            }
        }
        _ => None,
    }
}

// ── Column navigation helpers ────────────────────────────────────────────

/// Groups top matches by date, returning (date_string, [flat_indices]) in insertion order.
fn date_groups(matches: &[TopMatch]) -> Vec<(String, Vec<usize>)> {
    let mut groups: Vec<(String, Vec<usize>)> = Vec::new();
    let mut seen: HashMap<String, usize> = HashMap::new();

    for (i, m) in matches.iter().enumerate() {
        if let Some(&idx) = seen.get(&m.date) {
            groups[idx].1.push(i);
        } else {
            let idx = groups.len();
            seen.insert(m.date.clone(), idx);
            groups.push((m.date.clone(), vec![i]));
        }
    }
    groups
}

/// Given date groups and a flat index, returns (column_index, row_within_column).
fn flat_to_col_row(groups: &[(String, Vec<usize>)], flat_idx: usize) -> Option<(usize, usize)> {
    for (col, (_date, indices)) in groups.iter().enumerate() {
        if let Some(row) = indices.iter().position(|&i| i == flat_idx) {
            return Some((col, row));
        }
    }
    None
}
