use crate::models::Match;
use crate::ui::layout;
use crate::ui::theme::{BEIGE, GOLD, RUST_ORANGE};
use ratatui::layout::Rect;
use ratatui::{
    Frame,
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

/// Renders the results area: loading spinner, error message, or match list.
pub fn render(frame: &mut Frame, area: Rect, state: &ResultsState) {
    match state {
        ResultsState::Loading => {
            let loading = Paragraph::new("Scraping matches... please wait.")
                .style(Style::default().fg(GOLD).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(loading, area);
        }
        ResultsState::Error(err) => {
            let error_msg = Paragraph::new(format!("Error: {}", err))
                .style(
                    Style::default()
                        .fg(RUST_ORANGE)
                        .add_modifier(Modifier::BOLD),
                )
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            frame.render_widget(error_msg, area);
        }
        ResultsState::Matches(matches) => {
            let items: Vec<ListItem> = matches
                .iter()
                .map(|m| {
                    let header = Line::from(vec![
                        Span::styled(
                            format!(" {} ", m.teams),
                            Style::default().fg(GOLD).add_modifier(Modifier::BOLD),
                        ),
                        Span::raw(" - "),
                        Span::styled(&m.competition, Style::default().fg(BEIGE)),
                    ]);

                    let time_info = Line::from(vec![
                        Span::raw(" 📅 "),
                        Span::raw(&m.date),
                        Span::raw(" ⏰ "),
                        Span::raw(&m.time),
                    ]);

                    let channels = if m.channels.is_empty() {
                        "No TV info".to_string()
                    } else {
                        m.channels.join(", ")
                    };

                    let channel_info = Line::from(vec![
                        Span::styled(" 📺 ", Style::default().fg(RUST_ORANGE)),
                        Span::raw(channels),
                    ]);

                    let content = Text::from(vec![header, time_info, channel_info, Line::raw("")]);

                    ListItem::new(content)
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::NONE))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD));

            let cols = layout::results_horizontal(area);
            frame.render_widget(list, cols[1]);
        }
        ResultsState::Empty => {}
    }
}

/// Describes the current state of the results area.
pub enum ResultsState<'a> {
    Loading,
    Error(&'a str),
    Matches(&'a [Match]),
    Empty,
}
