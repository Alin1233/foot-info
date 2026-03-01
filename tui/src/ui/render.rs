use crate::models::ViewMode;
use crate::state::AppState;
use crate::ui::theme::{BEIGE, BG_BLACK, GOLD, RUST_ORANGE};
use crate::ui::views;
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
        ViewMode::Search => views::search::draw(frame, inner_area, app),
        ViewMode::TopMatches => views::top_matches::draw(frame, inner_area, app),
    }
}
