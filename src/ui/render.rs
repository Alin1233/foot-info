use crate::models::ViewMode;
use crate::state::AppState;
use crate::ui::components::match_list::ResultsState;
use crate::ui::components::top_matches_list::TopMatchesState;
use crate::ui::components::{match_list, search_bar, status_bar, top_matches_list};
use crate::ui::layout;
use crate::ui::theme::{BEIGE, BG_BLACK, GOLD, RUST_ORANGE};
use ratatui::{
    Frame,
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders},
};

/// Main draw function — composes all UI components onto the frame.
pub fn draw(frame: &mut Frame, app: &AppState) {
    let area = frame.area();

    // Instructions bar (changes based on mode)
    let instructions = match app.view_mode {
        ViewMode::Search => Line::from(vec![
            Span::raw(" Quit "),
            Span::styled(
                "<Esc> ",
                Style::default()
                    .fg(RUST_ORANGE)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("| Search "),
            Span::styled(
                "<Enter> ",
                Style::default().fg(GOLD).add_modifier(Modifier::BOLD),
            ),
            Span::raw("| Save Fav "),
            Span::styled(
                "<Ctrl+s> ",
                Style::default().fg(GOLD).add_modifier(Modifier::BOLD),
            ),
            Span::raw("| Load Fav "),
            Span::styled(
                "<Ctrl+f> ",
                Style::default().fg(GOLD).add_modifier(Modifier::BOLD),
            ),
            Span::raw("| Switch Country "),
            Span::styled(
                "<Ctrl+c> ",
                Style::default().fg(GOLD).add_modifier(Modifier::BOLD),
            ),
            Span::raw("| Top Matches "),
            Span::styled(
                "<Ctrl+t> ",
                Style::default().fg(GOLD).add_modifier(Modifier::BOLD),
            ),
        ]),
        ViewMode::TopMatches => Line::from(vec![
            Span::raw(" Back "),
            Span::styled(
                "<Esc> ",
                Style::default()
                    .fg(RUST_ORANGE)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("| Select Match "),
            Span::styled(
                "<Enter> ",
                Style::default().fg(GOLD).add_modifier(Modifier::BOLD),
            ),
            Span::raw("| Navigate "),
            Span::styled(
                "<↑/↓/←/→> ",
                Style::default().fg(GOLD).add_modifier(Modifier::BOLD),
            ),
            Span::raw("| Switch Country "),
            Span::styled(
                "<Ctrl+c> ",
                Style::default().fg(GOLD).add_modifier(Modifier::BOLD),
            ),
        ]),
    };

    let current_provider = app.get_current_provider();
    let title = format!(
        " FOOTBALL MATCH CHANNELS INFO [{}] ",
        current_provider.country()
    );

    // Main block (border + title + instructions)
    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BEIGE))
        .title(Span::styled(
            title,
            Style::default().fg(GOLD).add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center)
        .title_bottom(instructions.centered())
        .style(Style::default().bg(BG_BLACK).fg(BEIGE));

    frame.render_widget(main_block.clone(), area);
    let inner_area = main_block.inner(area);

    match app.view_mode {
        ViewMode::Search => draw_search_mode(frame, inner_area, app),
        ViewMode::TopMatches => draw_top_matches_mode(frame, inner_area, app),
    }
}

fn draw_search_mode(frame: &mut Frame, inner_area: ratatui::layout::Rect, app: &AppState) {
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

fn draw_top_matches_mode(frame: &mut Frame, inner_area: ratatui::layout::Rect, app: &AppState) {
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
