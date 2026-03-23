use foot_info_core::models::LeagueStats;
use crate::ui::theme::{BEIGE, BG_BLACK, GOLD};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};

pub fn draw(frame: &mut Frame, area: Rect, stats: &LeagueStats, selected_index: usize) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BEIGE))
        .title(" Upcoming / Recent Fixtures ")
        .style(Style::default().bg(BG_BLACK).fg(BEIGE));

    if stats.fixtures.is_empty() {
        let empty = List::new(vec![ListItem::new("No fixtures found.")])
            .block(block);
        frame.render_widget(empty, area);
        return;
    }

    let items: Vec<ListItem> = stats
        .fixtures
        .iter()
        .map(|f| {
            let mut spans = vec![];
            
            // Date mapping
            spans.push(Span::styled(
                format!("{:<15} ", f.date),
                Style::default().fg(Color::DarkGray),
            ));

            // Time mapping
            spans.push(Span::styled(
                format!("{:<8} ", f.time),
                Style::default().fg(Color::Yellow),
            ));

            // Teams mapping
            spans.push(Span::styled(
                format!("{} - {}", f.home_team, f.away_team),
                Style::default().fg(BEIGE).add_modifier(Modifier::BOLD),
            ));

            ListItem::new(Line::from(spans))
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .fg(GOLD)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let mut state = ListState::default();
    state.select(Some(selected_index));

    frame.render_stateful_widget(list, area, &mut state);
}
