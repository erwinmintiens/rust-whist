use super::game_modes::GameMode;
use crate::io::multiple_player_selector;
use crate::models::game::Game;
use crate::points::miserie::miserie_points;

pub fn run_miserie(mut game: Game, game_mode: GameMode) -> Game {
    loop {
        let mut playing_players =
            match multiple_player_selector(&mut game, format!("Who is playing {game_mode}?")) {
                Ok(players) => players,
                Err(e) => {
                    eprintln!("An error occurred while reading input: {}", e);
                    continue;
                }
            };
        if playing_players.len() == 0 {
            eprint!("Please select a minimum of 1 player");
            continue;
        }
        for player in playing_players.iter_mut() {
            player.playing_player = true;
        }
        break;
    }
    miserie_points(&mut game, game_mode);
    game
}
