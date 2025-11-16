// --- Ratatui Imports ---
pub use ratatui::prelude::*;
pub use ratatui::widgets::{
    Block, Borders, BorderType, Cell, Clear, List, ListItem, Paragraph, Row, Table, Wrap,
};
pub use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
pub use ratatui::style::{Color, Modifier, Style, Stylize, Styled};
pub use ratatui::text::{Line, Span, Text};

// --- App Imports ---
pub use crate::app::{App, CurrentScreen, PlayerCreationInput, ActionItem};
pub use crate::player::Player;

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let start_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(start_layout[1])[1]
}
