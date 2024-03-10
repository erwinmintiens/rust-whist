use crate::games::game_modes::GameMode;
use crate::io::get_tricks_achieved;
use crate::models::game::Game;
use crate::utils::{add_points_to_players, get_playing_and_opposing_players};

pub fn miserie_points(game: &mut Game, game_mode: GameMode) {
    let (mut playing_players, mut opposing_players) = get_playing_and_opposing_players(game);
    for player in playing_players.iter_mut() {
        let tricks_achieved = get_tricks_achieved(0, 13);
        player.tricks_achieved_current_round = Some(tricks_achieved);
    }
    match game_mode {
        GameMode::KleineMiserie => kleine_miserie_points(game),
        GameMode::GroteMiserie => grote_miserie_points(game),
        GameMode::MiserieOpTafel => miserie_op_tafel_points(game),
        _ => panic!(
            "Unexpected game mode during handling of miserie: {}",
            game_mode
        ),
    }
}

pub fn kleine_miserie_points(game: &mut Game) {
    let playing_players = game.get_all_playing_players();
}
pub fn grote_miserie_points(game: &mut Game) {}
pub fn miserie_op_tafel_points(game: &mut Game) {}
