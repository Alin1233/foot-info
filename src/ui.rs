use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crate::app::App;
use crate::theme::{BEIGE, BG_BLACK, GOLD, RUST_ORANGE};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Instructions
    let instructions = Line::from(vec![
        Span::raw(" Quit "),
        Span::styled("<Esc> ", Style::default().fg(RUST_ORANGE).add_modifier(Modifier::BOLD)),
        Span::raw("| Search "),
        Span::styled("<Enter> ", Style::default().fg(GOLD).add_modifier(Modifier::BOLD)),
        Span::raw("| Save Fav "),
        Span::styled("<Ctrl+s> ", Style::default().fg(GOLD).add_modifier(Modifier::BOLD)),
        Span::raw("| Load Fav "),
        Span::styled("<Ctrl+f> ", Style::default().fg(GOLD).add_modifier(Modifier::BOLD)),
        Span::raw("| Switch Country "),
        Span::styled("<c> ", Style::default().fg(GOLD).add_modifier(Modifier::BOLD)),
    ]);

    let current_provider = app.get_current_provider();
    let title = format!(" FOOTBALL MATCH CHANNELS INFO [{}] ", current_provider.country());

    // Main Block
    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BEIGE))
        .title(Span::styled(title, Style::default().fg(GOLD).add_modifier(Modifier::BOLD)))
        .title_alignment(Alignment::Center)
        .title_bottom(instructions.centered())
        .style(Style::default().bg(BG_BLACK).fg(BEIGE));

    frame.render_widget(main_block.clone(), area);
    let inner_area = main_block.inner(area);

    // Layout: Input at top, Status Message, Results below
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Input
            Constraint::Length(1), // Spacer/Status
            Constraint::Min(1),    // Results
        ])
        .split(inner_area);

    // Input Section
    let input_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(vertical_layout[0]);

    let input_text = Line::from(vec![
        Span::styled("Enter Team: ", Style::default().fg(GOLD).add_modifier(Modifier::BOLD)),
        Span::raw(&app.search_input),
    ]);

    let input_paragraph = Paragraph::new(input_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(BEIGE))
                .style(Style::default().bg(BG_BLACK))
        )
        .alignment(Alignment::Center);

    frame.render_widget(input_paragraph, input_layout[1]);

    // Status Message Section
    if let Some(msg) = &app.status_message {
        let status_text = Paragraph::new(msg.clone())
            .style(Style::default().fg(GOLD))
            .alignment(Alignment::Center);
        frame.render_widget(status_text, vertical_layout[1]);
    }

    // Results Section
    let results_area = vertical_layout[2];

    if app.is_loading {
        let loading = Paragraph::new("Scraping matches... please wait.")
            .style(Style::default().fg(GOLD).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::NONE));
        frame.render_widget(loading, results_area);
    } else if let Some(err) = &app.error_message {
        let error_msg = Paragraph::new(format!("Error: {}", err))
            .style(Style::default().fg(RUST_ORANGE).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        frame.render_widget(error_msg, results_area);
    } else if !app.matches.is_empty() {
        let items: Vec<ListItem> = app.matches.iter().map(|m| {
            let header = Line::from(vec![
                Span::styled(format!(" {} ", m.teams), Style::default().fg(GOLD).add_modifier(Modifier::BOLD)),
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

            let content = Text::from(vec![
                header,
                time_info,
                channel_info,
                Line::raw(""),
            ]);
            
            ListItem::new(content)
        }).collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::NONE))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        let list_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(results_area);
            
        frame.render_widget(list, list_layout[1]);
    } else if app.search_input.is_empty() {
    } else {
         // No results found (but not initial state)
         // We might want to handle "No matches found" here if a search was performed
    }
}