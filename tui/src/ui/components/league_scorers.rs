use foot_info_core::models::LeagueStats;
use crate::ui::theme::{BEIGE, BG_BLACK, GOLD};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
};

pub fn draw(frame: &mut Frame, area: Rect, stats: &LeagueStats, selected_index: usize) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BEIGE))
        .title(" Top Scorers ")
        .style(Style::default().bg(BG_BLACK).fg(BEIGE));

    if stats.top_scorers.is_empty() {
        let empty = Table::new(
            Vec::<Row>::new(), 
            [Constraint::Percentage(100)]
        ).block(block);
        frame.render_widget(empty, area);
        return;
    }

    let header_cells = ["Player", "Team", "Goals"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(GOLD).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::DarkGray))
        .height(1)
        .bottom_margin(1);

    let rows: Vec<Row> = stats
        .top_scorers
        .iter()
        .map(|s| {
            let cells = vec![
                Cell::from(s.player.clone()),
                Cell::from(s.team.clone()),
                Cell::from(s.goals.to_string()),
            ];
            Row::new(cells).height(1)
        })
        .collect();

    let widths = [
        Constraint::Min(25),    // Player
        Constraint::Min(25),    // Team
        Constraint::Length(6),  // Goals
    ];

    let table = Table::new(rows.into_iter(), widths)
        .header(header)
        .block(block)
        .row_highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .fg(GOLD)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let mut state = TableState::default();
    state.select(Some(selected_index));

    frame.render_stateful_widget(table, area, &mut state);
}
