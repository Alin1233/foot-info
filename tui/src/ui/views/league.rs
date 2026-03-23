use crate::models::LeagueTab;
use crate::state::AppState;
use crate::ui::components::{league_fixtures, league_scorers, league_table, status_bar};
use crate::ui::theme::{BEIGE, BG_BLACK, GOLD, RUST_ORANGE};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::Span,
    widgets::{Block, Borders, Paragraph, Tabs},
};

pub fn draw(frame: &mut Frame, area: Rect, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Tabs
                Constraint::Min(0),    // Content
                Constraint::Length(1), // StatusBar
            ]
            .as_ref(),
        )
        .split(area);

    // 1. Tabs
    let tab_titles = vec![
        LeagueTab::Fixtures.label(),
        LeagueTab::Table.label(),
        LeagueTab::TopScorers.label(),
    ];
    let selected_tab_index = match app.league_tab {
        LeagueTab::Fixtures => 0,
        LeagueTab::Table => 1,
        LeagueTab::TopScorers => 2,
    };

    let tabs = Tabs::new(tab_titles.into_iter().map(Span::raw).collect::<Vec<_>>())
        .block(Block::default().borders(Borders::ALL).title("League Data"))
        .select(selected_tab_index)
        .highlight_style(Style::default().fg(GOLD))
        .divider(Span::raw("|"));
    frame.render_widget(tabs, chunks[0]);

    // 2. Content Zone
    let content_area = chunks[1];
    
    if app.is_loading {
        let loading_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(BG_BLACK).fg(BEIGE));
        let p = Paragraph::new("Loading...").alignment(Alignment::Center).block(loading_block);
        frame.render_widget(p, content_area);
    } else if let Some(err) = &app.error_message {
        let err_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(BG_BLACK).fg(RUST_ORANGE));
        let p = Paragraph::new(err.to_string()).alignment(Alignment::Center).block(err_block);
        frame.render_widget(p, content_area);
    } else if let Some(stats) = &app.league_stats {
        match app.league_tab {
            LeagueTab::Fixtures => league_fixtures::draw(frame, content_area, stats, app.selected_fixture_index),
            LeagueTab::Table => league_table::draw(frame, content_area, stats, app.selected_table_index),
            LeagueTab::TopScorers => league_scorers::draw(frame, content_area, stats, app.selected_scorer_index),
        }
    } else {
        let empty_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(BG_BLACK).fg(BEIGE));
        frame.render_widget(empty_block, content_area);
    }

    // 3. Status Bar
    status_bar::render(frame, chunks[2], app.status_message.as_deref());
}
