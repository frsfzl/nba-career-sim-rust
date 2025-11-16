use super::prelude::*;

pub fn draw_player_creation(frame: &mut Frame, app: &App) {
    let area = centered_rect(40, 75, frame.area());
    let content = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10), // Left padding
            Constraint::Percentage(80), // Content of form
            Constraint::Percentage(10), // Right padding
        ])
        .split(area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10), // Top padding
            Constraint::Percentage(15), // Create Your Player title
            Constraint::Percentage(10), // First Name Input
            Constraint::Percentage(10), // Last Name Input
            Constraint::Percentage(10), // Height Input
            Constraint::Percentage(10), // Position Multi-Select
            Constraint::Percentage(10), // Jersey Number Input
            Constraint::Percentage(10), // High School Name Input
            Constraint::Percentage(10), // Continue Button
            Constraint::Percentage(5), // Bottom padding
        ])
        .split(content[1]);

    let title = Paragraph::new("Create Your Player").style(Style::default().add_modifier(Modifier::BOLD)).centered();

    let mut first_name_spans = vec![
        Span::raw("First Name: "),
        Span::styled(&app.player.first_name, Style::new().fg(Color::Yellow)),
    ];

    
    let mut last_name_spans = vec![
        Span::raw("Last Name: "),
        Span::styled(&app.player.last_name, Style::new().fg(Color::Yellow)),
    ];

    let mut height_spans = vec![
        Span::raw("Height: "),
        Span::styled(&app.player.height[0], Style::new().fg(Color::Yellow)),
        Span::raw(" ft "),
        Span::styled(&app.player.height[1], Style::new().fg(Color::Yellow)),
        Span::raw(" in "),
    ];
    let mut height_line = Line::from(height_spans);

    let mut position_spans = vec![
        Span::raw("Position: "),
        Span::raw(" PG "),
        Span::raw(" SG "),
        Span::raw(" SF "),
        Span::raw(" PF "),
        Span::raw(" C ")
    ];
    
    if app.player.position != 0 {
        position_spans[app.player.position as usize] = position_spans[app.player.position as usize].clone().set_style(Style::default().bg(Color::Yellow).fg(Color::Black));
    }

    let mut jersey_number_spans = vec![
        Span::raw("Jersey Number: "),
        Span::styled(&app.player.jersey_number, Style::new().fg(Color::Yellow)),
    ];

    let mut high_school_spans = vec![
        Span::raw("High School: "),
        Span::styled(&app.player.team, Style::new().fg(Color::Yellow)),
    ];

    let mut continue_button = Line::from("Continue").style(Style::default().add_modifier(Modifier::BOLD)).centered();

    let cursor_span = Span::styled("█", Style::new().fg(Color::Yellow).slow_blink());
    match app.player_creation_input {
        PlayerCreationInput::FirstName => {
            first_name_spans.push(cursor_span);
        }
        PlayerCreationInput::LastName => {
            last_name_spans.push(cursor_span);
        }
        PlayerCreationInput::HeightFeet => {
            height_line.spans.insert(2,cursor_span);
        }
        PlayerCreationInput::HeightInches => {
            height_line.spans.insert(4,cursor_span);
        }
        PlayerCreationInput::Position => {
            position_spans[app.player.position as usize] = position_spans[app.player.position as usize].clone().set_style(Style::default().bg(Color::Yellow).fg(Color::Black).rapid_blink());
        }
        PlayerCreationInput::JerseyNumber => {
            jersey_number_spans.push(cursor_span);
        }
        PlayerCreationInput::HighSchool => {
            high_school_spans.push(Span::styled("█", Style::new().fg(Color::Yellow).slow_blink()));
        }
        PlayerCreationInput::Continue => {
            continue_button = continue_button.clone().set_style(Style::default().bg(Color::Yellow).fg(Color::Black));
        }
        PlayerCreationInput::Popup => {
            let feet = app.player.height[0].parse::<i32>().unwrap();
            let inches = app.player.height[1].parse::<i32>().unwrap();
            if feet < 5 || feet > 7 || inches > 11 {
                let popup_area = centered_rect(35, 15, frame.area());
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(20),
                        Constraint::Percentage(40),
                        Constraint::Percentage(40),
                        Constraint::Percentage(20),
                    ])
                    .split(popup_area);
                frame.render_widget((Line::from("Invalid Height")).style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)).centered(), chunks[1]);
                frame.render_widget(Paragraph::new("Please enter a height between 5'0\" and 7'11\"").centered().wrap(Wrap { trim: true}), chunks[2]);
                frame.render_widget(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)), popup_area);
            }
        }
        _ => {}
    }
    
    let first_name_line = Line::from(first_name_spans);
    let first_name_form = Paragraph::new(first_name_line).wrap(Wrap { trim: true });
    let last_name_line = Line::from(last_name_spans);
    let last_name_form = Paragraph::new(last_name_line).wrap(Wrap { trim: true });
    let height_form = Paragraph::new(height_line);
    let position_line = Line::from(position_spans);
    let position_form = Paragraph::new(position_line);
    let jersey_number_line = Line::from(jersey_number_spans);
    let jersey_number_form = Paragraph::new(jersey_number_line);
    let high_school_line = Line::from(high_school_spans);
    let high_school_form = Paragraph::new(high_school_line).wrap(Wrap { trim: true });
    
    frame.render_widget(Clear, area);
    frame.render_widget(title, chunks[1]);
    frame.render_widget(first_name_form, chunks[2]);
    frame.render_widget(last_name_form, chunks[3]);
    frame.render_widget(height_form, chunks[4]);
    frame.render_widget(position_form, chunks[5]);
    frame.render_widget(jersey_number_form, chunks[6]);
    frame.render_widget(high_school_form, chunks[7]);
    frame.render_widget(continue_button, chunks[8]);
    frame.render_widget(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)).border_type(BorderType::Rounded), area);
}
