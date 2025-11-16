#[derive(Debug)]
pub struct Player {
    pub first_name: String,
    pub last_name: String,
    pub height: [String; 2],
    pub position: u8,
    pub team: String,
    pub jersey_number: String,
}

impl Player {
    pub fn new() -> Self {
        Self {
            first_name: String::new(),
            last_name: String::new(),
            height: [String::from(""),String::from("")],
            position: 0,
            team: String::new(),
            jersey_number: String::new(),
        }
    }
}

