use crate::io::{get_tricks_achieved, get_tricks_to_achieve};
use crate::utils::single_player_selector;

use super::game_modes::GameMode;

pub fn run_solo(players: &[String], game_mode: GameMode) {
    let player = single_player_selector(players, &game_mode);
    let tricks_to_achieve = get_tricks_to_achieve();
    let tricks_achieved = get_tricks_achieved();
}
