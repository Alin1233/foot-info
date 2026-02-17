use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Main vertical layout: Input (3 rows) | Status (1 row) | Results (rest).
pub fn main_vertical(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Input
            Constraint::Length(1), // Status
            Constraint::Min(1),    // Results
        ])
        .split(area)
        .to_vec()
}

/// Center-column layout for the search input (20% | 60% | 20%).
pub fn input_horizontal(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(area)
        .to_vec()
}

/// Center-column layout for the results list (10% | 80% | 10%).
pub fn results_horizontal(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(area)
        .to_vec()
}
