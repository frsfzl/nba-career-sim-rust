use std::io;
use std::error::Error;

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod ui;
mod player;

use crate::{
    app::{App, CurrentScreen, PlayerCreationInput, ActionItem},
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
        )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::StartScreen => match key.code {
                    KeyCode::Enter => {
                        app.current_screen = CurrentScreen::PlayerCreation;
                    },
                    KeyCode::Esc => {
                        return Ok(false);
                    },
                    _ => {}
                },
                CurrentScreen::PlayerCreation => {
                    match key.code {
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::StartScreen;
                        },
                        KeyCode::Char(value) => {
                            match app.player_creation_input {
                                PlayerCreationInput::FirstName => {
                                    app.player.first_name.push(value);
                                },
                                PlayerCreationInput::LastName => {
                                    app.player.last_name.push(value);
                                },
                                PlayerCreationInput::HeightFeet => {
                                    app.player.height[0].push(value);
                                    let result: Result<i32, _> = app.player.height[0].parse();
                                    match result {
                                        Ok(n) => (),
                                        Err(e) => {
                                            app.player.height[0].pop();
                                        }
                                    }
                                }
                                PlayerCreationInput::HeightInches => {
                                    app.player.height[1].push(value);
                                    let result: Result<i32, _> = app.player.height[1].parse();
                                    match result {
                                        Ok(n) => (),
                                        Err(e) => {
                                            app.player.height[1].pop();
                                        }
                                    }
                                }
                                PlayerCreationInput::JerseyNumber => {
                                    app.player.jersey_number.push(value);
                                    let result: Result<i32, _> = app.player.jersey_number.parse();
                                    match result {
                                        Ok(n) => (),
                                        Err(e) => {
                                            app.player.jersey_number.pop();
                                        }
                                    }
                                }
                                PlayerCreationInput::HighSchool => {
                                    app.player.team.push(value);
                                },
                                _ => {}
                            }
                        },
                        KeyCode::Tab | KeyCode::Enter | KeyCode::Down => {
                            match app.player_creation_input {
                                PlayerCreationInput::FirstName => {
                                    app.player_creation_input = PlayerCreationInput::LastName;
                                },
                                PlayerCreationInput::LastName => {
                                    app.player_creation_input = PlayerCreationInput::HeightFeet;
                                },
                                PlayerCreationInput::HeightFeet => match key.code {
                                    KeyCode::Tab | KeyCode::Enter => {
                                        app.player_creation_input = PlayerCreationInput::HeightInches;
                                    }
                                    _ => {
                                        app.player_creation_input = PlayerCreationInput::Position;
                                        if app.player.position == 0 {
                                            app.player.position = 1;
                                        }
                                    }
                                }
                                PlayerCreationInput::HeightInches => {
                                    app.player_creation_input = PlayerCreationInput::Position;
                                    if app.player.position == 0 {
                                        app.player.position = 1;
                                    }
                                }
                                PlayerCreationInput::Position => {
                                    app.player_creation_input = PlayerCreationInput::JerseyNumber;
                                },
                                PlayerCreationInput::JerseyNumber => {
                                    app.player_creation_input = PlayerCreationInput::HighSchool;
                                },
                                PlayerCreationInput::HighSchool => {
                                    app.player_creation_input = PlayerCreationInput::Continue;
                                }
                                PlayerCreationInput::Continue => match key.code {
                                    KeyCode::Tab => {
                                        app.player_creation_input = PlayerCreationInput::FirstName;
                                    }
                                    KeyCode::Enter => {
                                        // let feet = app.player.height[0].parse::<i32>().unwrap();
                                        // let inches = app.player.height[1].parse::<i32>().unwrap();
                                        // if feet < 5 || feet > 7 || inches > 11 {
                                        //     app.player_creation_input = PlayerCreationInput::Popup;
                                        // }
                                        app.current_screen = CurrentScreen::HighSchool;
                                    }
                                    _ => {}
                                }
                                _ => {}
                            }
                        },
                        KeyCode::Up => {
                            match app.player_creation_input {
                                PlayerCreationInput::LastName => {
                                    app.player_creation_input = PlayerCreationInput::FirstName;
                                },
                                PlayerCreationInput::HeightFeet | PlayerCreationInput::HeightInches => {
                                    app.player_creation_input = PlayerCreationInput::LastName;
                                }
                                PlayerCreationInput::Position => {
                                    app.player_creation_input = PlayerCreationInput::HeightFeet;
                                },
                                PlayerCreationInput::JerseyNumber => {
                                    app.player_creation_input = PlayerCreationInput::Position;
                                },
                                PlayerCreationInput::HighSchool => {
                                    app.player_creation_input = PlayerCreationInput::JerseyNumber;
                                },
                                PlayerCreationInput::Continue => {
                                    app.player_creation_input = PlayerCreationInput::HighSchool;
                                },
                                _ => {}
                            }
                        }
                        KeyCode::Backspace => {
                            match app.player_creation_input {
                                PlayerCreationInput::FirstName => {
                                    app.player.first_name.pop();
                                },
                                PlayerCreationInput::LastName => {
                                    app.player.last_name.pop();
                                },
                                PlayerCreationInput::HeightFeet => {
                                    app.player.height[0].pop();
                                }
                                PlayerCreationInput::HeightInches => {
                                    app.player.height[1].pop();
                                }
                                PlayerCreationInput::JerseyNumber => {
                                    app.player.jersey_number.pop();
                                },
                                PlayerCreationInput::HighSchool => {
                                    app.player.team.pop();
                                },
                                _ => {}
                            }
                        }
                        KeyCode::Right => {
                            match app.player_creation_input {
                                PlayerCreationInput::HeightFeet => {
                                    app.player_creation_input = PlayerCreationInput::HeightInches;
                                }
                                PlayerCreationInput::Position => {
                                    if app.player.position != 5 {
                                        app.player.position += 1;
                                    }                                
                                    else {
                                        app.player.position = 1;
                                    }
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Left => {
                            match app.player_creation_input {
                                PlayerCreationInput::HeightInches => {
                                    app.player_creation_input = PlayerCreationInput::HeightFeet;
                                }
                                PlayerCreationInput::Position => {
                                    if app.player.position != 1 {
                                        app.player.position -= 1;
                                    }                                
                                    else {
                                        app.player.position = 5;
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                },
                CurrentScreen::HighSchool => match key.code {
                    KeyCode::Enter => {
                        app.current_screen = CurrentScreen::PlayerCreation;
                    },
                    KeyCode::Esc => {
                        return Ok(false);
                    },
                    KeyCode::Right => {
                        match app.action_item {
                            ActionItem::TodaysGame => {
                                app.action_item = ActionItem::Messages;
                            }
                            ActionItem::Messages => {
                                app.action_item = ActionItem::PlayerStats;
                            }
                            ActionItem::PlayerStats => {
                                app.action_item = ActionItem::GameLog;
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Left => {
                        match app.action_item {
                            ActionItem::Messages => {
                                app.action_item = ActionItem::TodaysGame;
                            }
                            ActionItem::PlayerStats => {
                                app.action_item = ActionItem::Messages;
                            }
                            ActionItem::GameLog => {
                                app.action_item = ActionItem::PlayerStats;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
