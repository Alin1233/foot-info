use super::components::match_list::ResultsState;
use super::components::{match_list, search_bar, status_bar};
use super::layout;
use super::theme::{BEIGE, BG_BLACK, GOLD, RUST_ORANGE};
use crate::state::AppState;
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

    // Instructions bar
    let instructions = Line::from(vec![
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
            "<Ctrl + c> ",
            Style::default().fg(GOLD).add_modifier(Modifier::BOLD),
        ),
    ]);

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

    // Split into vertical sections
    let sections = layout::main_vertical(inner_area);
    let input_cols = layout::input_horizontal(sections[0]);

    // Render components
    search_bar::render(frame, input_cols[1], &app.search_input);
    status_bar::render(frame, sections[1], app.status_message.as_deref());

    // Determine results state
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
