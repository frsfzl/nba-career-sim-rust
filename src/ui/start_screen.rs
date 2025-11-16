use super::prelude::*;

pub fn draw_start_screen(frame: &mut Frame) {
    let start_area = centered_rect(50,50, frame.area());
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(40),
        ])
        .split(start_area);
    
    let title = Paragraph::new("NBA Career Simulator TUI").style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)).centered();
    let subtitle = Paragraph::new(" - Press Enter to Start - ").style(Style::default().fg(Color::White)).centered();

    frame.render_widget(Clear, start_area);
    frame.render_widget(title, chunks[1]);
    frame.render_widget(subtitle, chunks[2]);
    frame.render_widget(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)).border_type(BorderType::Rounded), start_area);
}
