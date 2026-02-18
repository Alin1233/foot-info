use crate::state::AppState;
use crate::ui::components::match_list::{self, ResultsState};
use crate::ui::components::{search_bar, status_bar};
use crate::ui::layout;
use ratatui::Frame;

pub fn draw(frame: &mut Frame, inner_area: ratatui::layout::Rect, app: &AppState) {
    let sections = layout::main_vertical(inner_area);
    let input_cols = layout::input_horizontal(sections[0]);

    search_bar::render(frame, input_cols[1], &app.search_input);
    status_bar::render(frame, sections[1], app.status_message.as_deref());

    let results_state = if app.is_loading {
        ResultsState::Loading
    } else if let Some(ref err) = app.error_message {
        ResultsState::Error(err)
    } else if !app.matches.is_empty() {
        ResultsState::Matches(&app.matches)
    } else {
        ResultsState::Empty
    };

    match_list::render(frame, sections[2], &results_state);
}
