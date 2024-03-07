use super::game_modes::GameMode;
use crate::{
    io::{double_player_selector, get_tricks_achieved, get_tricks_to_achieve},
    models::game::Game,
    points::troel::troel_points,
};

pub fn run_troel(mut game: Game, game_mode: GameMode) -> Game {
    loop {
        let mut playing_players =
            match double_player_selector(&mut game, format!("Who is playing {game_mode}?")) {
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
        if playing_players.len() != 2 {
            eprintln!("The amount of playing players is not equal to 2.");
            continue;
        }
        for player in playing_players.iter_mut() {
            player.playing_player = true;
        }
        break;
    }
    let tricks_to_achieve = get_tricks_to_achieve(8, 9);
    let tricks_achieved = get_tricks_achieved(0, 13);
    troel_points(&mut game, tricks_to_achieve, tricks_achieved);
    game
}
