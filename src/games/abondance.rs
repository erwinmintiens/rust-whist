use super::game_modes::GameMode;
use crate::io::single_player_selector;
use crate::io::{get_tricks_achieved, get_tricks_to_achieve};
use crate::models::game::Game;
use crate::points::abondance::points;

pub fn run_abondance(mut game: Game, game_mode: GameMode) -> Game {
    loop {
        let playing_player =
            match single_player_selector(&mut game, format!("Who is playing {game_mode}?")) {
                Ok(player) => player,
                Err(e) => {
                    eprintln!("An error occurred while reading input: {}", e);
                    continue;
                }
            };
        playing_player.playing_player = true;
        break;
    }
    match game_mode {
        GameMode::Abondance => {
            let tricks_to_achieve = get_tricks_to_achieve(9, 13);
            let tricks_achieved = get_tricks_achieved(0, 13, None);
            points(&mut game, tricks_to_achieve, tricks_achieved, game_mode);
        }
        GameMode::SoloSlim => {
            let tricks_to_achieve = get_tricks_to_achieve(12, 13);
            let tricks_achieved = get_tricks_achieved(0, 13, None);
            points(&mut game, tricks_to_achieve, tricks_achieved, game_mode);
        }
        _ => {
            eprintln!(
                "Unexpected game mode while executing Abondance/Solo Slim: {}",
                game_mode
            );
        }
    }
    game
}
pub fn run_solo_slim(game: Game, game_mode: GameMode) -> Game {
    game
}
