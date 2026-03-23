use crate::app::Action;
use crate::models::{LeagueTab, ViewMode};
use crate::state::AppState;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles key events when the app is in League mode.
pub fn handle(state: &mut AppState, key_event: KeyEvent) -> Option<Action> {
    match key_event.code {
        KeyCode::Esc => {
            state.view_mode = ViewMode::Search;
            state.status_message = None;
            state.error_message = None;
            None
        }
        KeyCode::Char('l') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            state.view_mode = ViewMode::Search;
            state.status_message = None;
            None
        }
        KeyCode::Tab => {
            state.league_tab = state.league_tab.next();
            None
        }
        KeyCode::BackTab => {
            state.league_tab = state.league_tab.prev();
            None
        }
        KeyCode::Up => {
            match state.league_tab {
                LeagueTab::Fixtures => {
                    if state.selected_fixture_index > 0 {
                        state.selected_fixture_index -= 1;
                    }
                }
                LeagueTab::Table => {
                    if state.selected_table_index > 0 {
                        state.selected_table_index -= 1;
                    }
                }
                LeagueTab::TopScorers => {
                    if state.selected_scorer_index > 0 {
                        state.selected_scorer_index -= 1;
                    }
                }
            }
            None
        }
        KeyCode::Down => {
            if let Some(stats) = &state.league_stats {
                match state.league_tab {
                    LeagueTab::Fixtures => {
                        if state.selected_fixture_index + 1 < stats.fixtures.len() {
                            state.selected_fixture_index += 1;
                        }
                    }
                    LeagueTab::Table => {
                        if state.selected_table_index + 1 < stats.table.len() {
                            state.selected_table_index += 1;
                        }
                    }
                    LeagueTab::TopScorers => {
                        if state.selected_scorer_index + 1 < stats.top_scorers.len() {
                            state.selected_scorer_index += 1;
                        }
                    }
                }
            }
            None
        }
        KeyCode::Enter => {
            if state.league_tab == LeagueTab::Fixtures {
                if let Some(stats) = &state.league_stats {
                    if let Some(fixture) = stats.fixtures.get(state.selected_fixture_index) {
                        let team = fixture.home_team.clone();
                        state.search_input = team.clone();
                        state.view_mode = ViewMode::Search;
                        state.status_message = None;
                        return Some(Action::Search(team));
                    }
                }
            }
            None
        }
        KeyCode::Char('r') => {
            let url = state.league_url.clone();
            Some(Action::FetchLeagueStats(url))
        }
        _ => None,
    }
}
