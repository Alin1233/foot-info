use crate::state::AppState;
use crate::ui::components::status_bar;
use crate::ui::components::top_matches_list::{self, TopMatchesState};
use crate::ui::layout;
use ratatui::Frame;

pub fn draw(frame: &mut Frame, inner_area: ratatui::layout::Rect, app: &AppState) {
    let sections = layout::main_vertical(inner_area);

    // Status bar in the top section
    status_bar::render(frame, sections[0], app.status_message.as_deref());

    // Top matches columns in the full results area
    let top_state = if app.is_loading {
        TopMatchesState::Loading
    } else if let Some(ref err) = app.error_message {
        TopMatchesState::Error(err)
    } else if !app.top_matches.is_empty() {
        TopMatchesState::Matches {
            matches: &app.top_matches,
            selected_index: app.selected_top_match_index,
        }
    } else {
        TopMatchesState::Empty
    };

    top_matches_list::render(frame, sections[2], &top_state);
}
