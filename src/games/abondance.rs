use super::game_modes::GameMode;
use crate::io::{do_you_want_to_return_to_main_menu, single_player_selector};
use crate::io::{get_tricks_achieved, get_tricks_to_achieve};
use crate::models::game::Game;
use crate::points::abondance::points;

pub fn run_abondance(mut game: Game, game_mode: GameMode) -> Game {
    loop {
        let playing_player =
            match single_player_selector(&mut game, format!("Who is playing {game_mode}?")) {
                Ok(player) => player,
                Err(_) => {
                    println!("Unexpected input...");
                    if let Some(true) = do_you_want_to_return_to_main_menu() {
                        return game;
                    }
                    continue;
                }
            };
        playing_player.playing_player = true;
        break;
    }
    let minimum: u8;
    let maximum: u8 = 13;
    match game_mode {
        GameMode::Abondance => {
            minimum = 9;
        }
        GameMode::SoloSlim => {
            minimum = 12;
        }
        _ => {
            eprintln!(
                "Unexpected game mode while executing Abondance/Solo Slim: {}",
                game_mode
            );
            return game;
        }
    }
    let tricks_to_achieve = get_tricks_to_achieve(minimum, maximum);
    let tricks_achieved = get_tricks_achieved(0, 13, None);
    points(&mut game, tricks_to_achieve, tricks_achieved, game_mode);
    game
}
