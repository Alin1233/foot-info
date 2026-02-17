use crate::models::TopMatch;
use crate::ui::theme::{BEIGE, GOLD, RUST_ORANGE};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};
use std::collections::BTreeMap;

/// Renders the top matches grouped by date in side-by-side columns.
pub fn render(frame: &mut Frame, area: Rect, state: &TopMatchesState) {
    match state {
        TopMatchesState::Loading => {
            let loading = Paragraph::new("Fetching upcoming top matches... please wait.")
                .style(Style::default().fg(GOLD).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(loading, area);
        }
        TopMatchesState::Error(err) => {
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
        TopMatchesState::Matches {
            matches,
            selected_index,
        } => {
            render_columns(frame, area, matches, *selected_index);
        }
        TopMatchesState::Empty => {}
    }
}

/// Groups matches by date and renders each group in a column.
fn render_columns(frame: &mut Frame, area: Rect, matches: &[TopMatch], selected_index: usize) {
    // Group matches by date, preserving insertion order
    let mut groups: Vec<(String, Vec<(usize, &TopMatch)>)> = Vec::new();
    let mut seen_dates: BTreeMap<String, usize> = BTreeMap::new();

    for (i, m) in matches.iter().enumerate() {
        if let Some(&group_idx) = seen_dates.get(&m.date) {
            groups[group_idx].1.push((i, m));
        } else {
            let idx = groups.len();
            seen_dates.insert(m.date.clone(), idx);
            groups.push((m.date.clone(), vec![(i, m)]));
        }
    }

    let num_cols = groups.len();
    if num_cols == 0 {
        return;
    }

    // Create equal-width columns
    let constraints: Vec<Constraint> = groups
        .iter()
        .map(|_| Constraint::Ratio(1, num_cols as u32))
        .collect();

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    for (col_idx, (date, day_matches)) in groups.iter().enumerate() {
        let col_area = columns[col_idx];

        // Build list items for this day
        let mut items: Vec<ListItem> = Vec::new();

        for (global_idx, m) in day_matches {
            let is_selected = *global_idx == selected_index;

            let marker = if is_selected { " ▸ " } else { "   " };

            let header_style = if is_selected {
                Style::default().fg(GOLD).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(BEIGE)
            };

            let header = Line::from(vec![
                Span::styled(marker, header_style),
                Span::styled(&m.teams, header_style),
            ]);

            let time_line = Line::from(vec![
                Span::raw("   ⏰ "),
                Span::styled(&m.time, Style::default().fg(BEIGE)),
            ]);

            items.push(ListItem::new(Text::from(vec![
                header,
                time_line,
                Line::raw(""),
            ])));
        }

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(BEIGE))
                .title(Span::styled(
                    format!(" 📅 {} ", date),
                    Style::default()
                        .fg(RUST_ORANGE)
                        .add_modifier(Modifier::BOLD),
                ))
                .title_alignment(Alignment::Center),
        );

        frame.render_widget(list, col_area);
    }
}

/// State for the top matches list component.
pub enum TopMatchesState<'a> {
    Loading,
    Error(&'a str),
    Matches {
        matches: &'a [TopMatch],
        selected_index: usize,
    },
    Empty,
}
