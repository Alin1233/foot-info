use crate::ui::theme::GOLD;
use ratatui::layout::Rect;
use ratatui::{Frame, layout::Alignment, style::Style, widgets::Paragraph};

/// Renders an optional status message (e.g. "Saved favorite: Arsenal").
pub fn render(frame: &mut Frame, area: Rect, message: Option<&str>) {
    if let Some(msg) = message {
        let status_text = Paragraph::new(msg.to_string())
            .style(Style::default().fg(GOLD))
            .alignment(Alignment::Center);
        frame.render_widget(status_text, area);
    }
}
