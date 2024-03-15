use std::fmt;

use crate::games::game_modes::GameMode;

#[derive(Debug)]
pub struct UnexpectedGameModeError {
    game_mode: GameMode,
    details: String,
}

impl UnexpectedGameModeError {
    pub fn new(game_mode: GameMode, msg: &str) -> Self {
        UnexpectedGameModeError {
            game_mode,
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for UnexpectedGameModeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for UnexpectedGameModeError {
    fn description(&self) -> &str {
        &self.details
    }
}
