pub mod prelude;
mod start_screen;
mod player_creation;
mod high_school;

use self::prelude::*;

pub fn ui(frame: &mut Frame, app: &App) {
    match app.current_screen {
       CurrentScreen::StartScreen => {
           start_screen::draw_start_screen(frame);
       }
       CurrentScreen::PlayerCreation => {
           player_creation::draw_player_creation(frame, app);
       }
       CurrentScreen::HighSchool => {
           high_school::draw_high_school(frame, app);
       }
       _ => {}
    }
}


