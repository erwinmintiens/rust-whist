use std::fmt::format;

use crate::{
    io::{get_tricks_achieved, get_tricks_to_achieve, multiple_player_selector},
    models::game::Game,
    points::vragen_en_meegaan::vragen_en_meegaan_points,
};

use super::game_modes::GameMode;

pub fn run_vragen_en_meegaan(mut game: Game, game_mode: GameMode) -> Game {
    loop {
        let mut playing_players =
            match multiple_player_selector(&mut game, format!("Who is playing {game_mode}?")) {
                Ok(players) => {
                    if players.len() != 2 {
                        eprintln!("Please select 2 players.");
                        continue;
                    }
                    players
                }
                Err(e) => {
                    eprintln!("An error occurred while reading input: {}", e);
                    continue;
                }
            };
        for player in playing_players.iter_mut() {
            player.playing_player = true;
        }
        break;
    }
    let tricks_to_achieve = get_tricks_to_achieve(8, 13);
    let tricks_achieved = get_tricks_achieved(0, 13);
    vragen_en_meegaan_points(&mut game, tricks_to_achieve, tricks_achieved);
    game
}
