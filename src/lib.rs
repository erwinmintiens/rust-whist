mod games;
mod io;
mod utils;
use crate::io::game_mode_selector;
use crate::utils::read_player;
use games::abondance::{run_abondance, run_solo_slim};
use games::game_modes::GameMode;
use games::miserie::{run_grote_miserie, run_kleine_miserie, run_miserie_op_tafel};
use games::piccolo::run_piccolo;
use games::solo::run_solo;
use games::troel::run_troel;
use games::vragen_en_meegaan::run_vragen_en_meegaan;

pub fn start_game() {
    let players = player_selector();
    let game_mode = game_mode_selector();
    println!("Chosen game mode: {game_mode}");
    run(players, game_mode);
}

fn player_selector() -> [String; 4] {
    let mut player1 = String::new();
    let mut player2 = String::new();
    let mut player3 = String::new();
    let mut player4 = String::new();
    println!("Provide 4 players:");
    player1 = read_player(player1, 1);
    player2 = read_player(player2, 2);
    player3 = read_player(player3, 3);
    player4 = read_player(player4, 4);
    [player1, player2, player3, player4]
}

fn run(players: [String; 4], game_mode: GameMode) {
    match game_mode {
        GameMode::Solo => run_solo(&players, game_mode),
        GameMode::VragenEnMeegaan => run_vragen_en_meegaan(),
        GameMode::Troel => run_troel(),
        GameMode::Piccolo => run_piccolo(),
        GameMode::KleineMiserie => run_kleine_miserie(),
        GameMode::GroteMiserie => run_grote_miserie(),
        GameMode::MiserieOpTafel => run_miserie_op_tafel(),
        GameMode::Abondance => run_abondance(),
        GameMode::SoloSlim => run_solo_slim(),
    }
}
