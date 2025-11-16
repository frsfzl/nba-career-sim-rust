use super::prelude::*;

pub fn draw_high_school(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(2),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(78),
        ])
        .split(frame.area());
    let info_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(2),
            Constraint::Percentage(36),
            Constraint::Percentage(60),
            Constraint::Percentage(2),
        ])
        .split(chunks[1]);
    let action_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(20),
        ])
        .split(chunks[2]);
    
    let position = ["PG", "SG", "SF", "PF", "C"];
    let player_info_text = Text::from(vec![
        Line::from(format!("{} {}", app.player.first_name, app.player.last_name)).left_aligned(),
        Line::from(format!("{} #{}", position[(app.player.position as usize) - 1], app.player.jersey_number)),
    ]);
    
    let high_school_text = Text::from(vec![
        Line::from(format!("{} Varsity Basketball Team", app.player.team)).right_aligned(),
        Line::from("Record: 0-0").right_aligned(),
    ]);

    let mut todays_game = Span::raw("Today's Game").add_modifier(Modifier::BOLD).to_centered_line();
    let mut messages = Span::raw("Messages").add_modifier(Modifier::BOLD).to_centered_line();
    let mut player_stats = Span::raw("Player's Stats").add_modifier(Modifier::BOLD).to_centered_line();
    let mut game_log = Span::raw("Game Log").add_modifier(Modifier::BOLD).to_centered_line();

    match app.action_item {
        ActionItem::TodaysGame => {
            todays_game = todays_game.clone().set_style(Style::default().bg(Color::Yellow).fg(Color::Black));
            frame.render_widget(Span::raw("Testing...").to_centered_line(), chunks[3]);
        },
        ActionItem::Messages => {
            messages = messages.clone().set_style(Style::default().bg(Color::Yellow).fg(Color::Black));
        },
        ActionItem::PlayerStats => {
            player_stats = player_stats.clone().set_style(Style::default().bg(Color::Yellow).fg(Color::Black));
        },
        ActionItem::GameLog => {
            game_log = game_log.clone().set_style(Style::default().bg(Color::Yellow).fg(Color::Black));
        },
        _ => {}
    }

    // frame.render_widget(Clear, frame.area());
    frame.render_widget(Paragraph::new(player_info_text), info_chunks[1]);
    frame.render_widget(high_school_text, info_chunks[2]);
    frame.render_widget(todays_game, action_chunks[1]);
    frame.render_widget(messages, action_chunks[2]);
    frame.render_widget(player_stats, action_chunks[3]);
    frame.render_widget(game_log, action_chunks[4]);
    frame.render_widget(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)).border_type(BorderType::Rounded), frame.area());
    


}
