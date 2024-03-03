use crate::io::single_player_selector;
use crate::io::{get_tricks_achieved, get_tricks_to_achieve};
use crate::models::game::Game;
use crate::points::solo::solo_points;

use super::game_modes::GameMode;

pub fn run_solo(mut game: Game, game_mode: GameMode) -> Game {
    single_player_selector(&mut game, format!("Who is playing {game_mode}?"));
    let tricks_to_achieve = get_tricks_to_achieve(6, 8);
    let tricks_achieved = get_tricks_achieved(0, 13);
    solo_points(&mut game, tricks_to_achieve, tricks_achieved);
    game
}
