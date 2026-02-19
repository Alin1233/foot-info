use foot_info::ui::layout;
use ratatui::layout::Rect;

#[test]
fn test_main_vertical_returns_three_sections() {
    let area = Rect::new(0, 0, 80, 24);
    let sections = layout::main_vertical(area);
    assert_eq!(sections.len(), 3);
}

#[test]
fn test_main_vertical_input_section_is_3_rows() {
    let area = Rect::new(0, 0, 80, 24);
    let sections = layout::main_vertical(area);
    assert_eq!(sections[0].height, 3);
}

#[test]
fn test_main_vertical_status_section_is_1_row() {
    let area = Rect::new(0, 0, 80, 24);
    let sections = layout::main_vertical(area);
    assert_eq!(sections[1].height, 1);
}

#[test]
fn test_main_vertical_results_gets_remaining_space() {
    let area = Rect::new(0, 0, 80, 24);
    let sections = layout::main_vertical(area);
    // 24 total - 3 (input) - 1 (status) = 20
    assert_eq!(sections[2].height, 20);
}

#[test]
fn test_input_horizontal_returns_three_columns() {
    let area = Rect::new(0, 0, 100, 3);
    let cols = layout::input_horizontal(area);
    assert_eq!(cols.len(), 3);
}

#[test]
fn test_input_horizontal_center_is_60_percent() {
    let area = Rect::new(0, 0, 100, 3);
    let cols = layout::input_horizontal(area);
    assert_eq!(cols[1].width, 60);
}

#[test]
fn test_results_horizontal_returns_three_columns() {
    let area = Rect::new(0, 0, 100, 20);
    let cols = layout::results_horizontal(area);
    assert_eq!(cols.len(), 3);
}

#[test]
fn test_results_horizontal_center_is_80_percent() {
    let area = Rect::new(0, 0, 100, 20);
    let cols = layout::results_horizontal(area);
    assert_eq!(cols[1].width, 80);
}

#[test]
fn test_layout_with_small_area() {
    let area = Rect::new(0, 0, 10, 6);
    let sections = layout::main_vertical(area);
    // Even with small area, should return 3 sections
    assert_eq!(sections.len(), 3);
    assert_eq!(sections[0].height, 3);
    assert_eq!(sections[1].height, 1);
    assert_eq!(sections[2].height, 2);
}
