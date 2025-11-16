use crate::player::Player;

pub enum CurrentScreen {
    StartScreen,
    PlayerCreation,
    HighSchool,
}

#[derive(Debug)]
pub enum PlayerCreationInput {
    FirstName,
    LastName,
    HeightFeet,
    HeightInches,
    JerseyNumber,
    Position,
    HighSchool,
    Continue,
    Popup,
}

pub enum ActionItem {
    TodaysGame,
    Messages,
    PlayerStats,
    GameLog,
}
pub struct App {
    pub current_screen: CurrentScreen,
    pub player: Player,
    pub player_creation_input: PlayerCreationInput,
    pub action_item: ActionItem,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::StartScreen,
            player: Player::new(),
            player_creation_input: PlayerCreationInput::FirstName,
            action_item: ActionItem::TodaysGame,
        }
    }
}
