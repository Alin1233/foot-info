use ratatui::layout::{Constraint, Layout, Rect};

/// Main vertical layout: Input (3 rows) | Status (1 row) | Results (rest).
pub fn main_vertical(area: Rect) -> Vec<Rect> {
    Layout::vertical([
        Constraint::Length(3), // Input
        Constraint::Length(1), // Status
        Constraint::Fill(1),   // Results
    ])
    .split(area)
    .to_vec()
}

/// Center-column layout for the search input (flexible | max 60 cols | flexible).
pub fn input_horizontal(area: Rect) -> Vec<Rect> {
    Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Max(60),
        Constraint::Fill(1),
    ])
    .split(area)
    .to_vec()
}

/// Center-column layout for the results list (flexible | max 100 cols | flexible).
pub fn results_horizontal(area: Rect) -> Vec<Rect> {
    Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Max(100),
        Constraint::Fill(1),
    ])
    .split(area)
    .to_vec()
}
