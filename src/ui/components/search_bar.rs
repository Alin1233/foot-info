use crate::ui::theme::{BEIGE, BG_BLACK, GOLD};
use ratatui::layout::Rect;
use ratatui::{
    Frame,
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

/// Renders the search input bar.
pub fn render(frame: &mut Frame, area: Rect, search_input: &str) {
    let input_text = Line::from(vec![
        Span::styled(
            "Enter Team: ",
            Style::default().fg(GOLD).add_modifier(Modifier::BOLD),
        ),
        Span::raw(search_input),
    ]);

    let input_paragraph = Paragraph::new(input_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(BEIGE))
                .style(Style::default().bg(BG_BLACK)),
        )
        .alignment(Alignment::Center);

    frame.render_widget(input_paragraph, area);
}
