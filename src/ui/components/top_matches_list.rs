use crate::models::TopMatch;
use crate::ui::theme::{BEIGE, GOLD, RUST_ORANGE};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
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

/// Groups matches by date and renders them in responsive columns (max 3 per row).
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

    let num_groups = groups.len();
    if num_groups == 0 {
        return;
    }

    // Determine how many columns we can fit per row (targeting 3, falling back to 2 or 1 if extremely narrow)
    // Assuming a minimum readable width of 25 chars per column.
    let mut max_cols_target = 3;
    while max_cols_target > 1 && (area.width / max_cols_target as u16) < 25 {
        max_cols_target -= 1;
    }
    let cols_per_row = max_cols_target.max(1).min(num_groups);
    let num_rows = (num_groups + cols_per_row - 1) / cols_per_row;
    let row_constraints = vec![Constraint::Ratio(1, num_rows as u32); num_rows];
    let row_areas = Layout::vertical(row_constraints).split(area);

    // Chunk the groups and render each chunk into a row area
    for (row_idx, chunk) in groups.chunks(cols_per_row).enumerate() {
        let current_row_area = row_areas[row_idx];

        let col_constraints = vec![Constraint::Ratio(1, cols_per_row as u32); cols_per_row];
        let col_areas = Layout::horizontal(col_constraints).split(current_row_area);

        for (col_idx, (date, day_matches)) in chunk.iter().enumerate() {
            let col_area = col_areas[col_idx];

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
            let mut borders = Borders::ALL;
            if col_idx > 0 {
                borders.remove(Borders::LEFT);
            }
            if row_idx > 0 {
                borders.remove(Borders::TOP);
            }

            let list = List::new(items).block(
                Block::default()
                    .borders(borders)
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
}
pub enum TopMatchesState<'a> {
    Loading,
    Error(&'a str),
    Matches {
        matches: &'a [TopMatch],
        selected_index: usize,
    },
    Empty,
}
