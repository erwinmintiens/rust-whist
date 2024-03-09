mod games;
mod io;
mod models;
mod points;
mod utils;
use crate::io::{do_you_want_to_quit, game_mode_selector, main_menu as menu_io, read_player};
use games::abondance::{run_abondance, run_solo_slim};
use games::game_modes::GameMode;
use games::miserie::{run_grote_miserie, run_kleine_miserie, run_miserie, run_miserie_op_tafel};
use games::piccolo::run_piccolo;
use games::solo::run_solo;
use games::troel::run_troel;
use games::vragen_en_meegaan::run_vragen_en_meegaan;
use models::{
    game::{Game, Player},
    menu::MainMenuOptions,
};

pub fn new_game() {
    let (player1, player2, player3, player4) = player_selector();
    let game = Game::new(player1, player2, player3, player4);
    run_game(game);
}

fn run_game(mut game: Game) {
    loop {
        game = main_menu(game);
    }
}
fn main_menu(game: Game) -> Game {
    let game = match menu_io() {
        Ok(value) => match value {
            MainMenuOptions::NextRound => start_next_round(game),
            MainMenuOptions::Settings => settings(game),
            MainMenuOptions::DisplayScore => {
                game.display_all_player_points();
                game
            }
            MainMenuOptions::Quit => {
                do_you_want_to_quit();
                game
            }
        },
        Err(value) => {
            eprint!("{}", value);
            game
        }
    };
    game
}

pub fn start_next_round(mut game: Game) -> Game {
    game.reset_all_playing_players();
    let game_mode = game_mode_selector();
    println!("Chosen game mode: {game_mode}");
    return run(game, game_mode);
}

fn settings(mut game: Game) -> Game {
    game
}

fn player_selector() -> (Player, Player, Player, Player) {
    println!("Provide 4 players:");
    let player1_name = read_player(1_u8);
    let player2_name = read_player(2_u8);
    let player3_name = read_player(3_u8);
    let player4_name = read_player(4_u8);
    let player1 = Player::new(1_u8, player1_name);
    let player2 = Player::new(2_u8, player2_name);
    let player3 = Player::new(3_u8, player3_name);
    let player4 = Player::new(4_u8, player4_name);
    println!();
    (player1, player2, player3, player4)
}

fn run(mut game: Game, game_mode: GameMode) -> Game {
    game = match game_mode {
        GameMode::Solo => run_solo(game, game_mode),
        GameMode::VragenEnMeegaan => run_vragen_en_meegaan(game, game_mode),
        GameMode::Troel => run_troel(game, game_mode),
        GameMode::Piccolo => run_piccolo(game, game_mode),
        GameMode::KleineMiserie => run_miserie(game, game_mode),
        GameMode::GroteMiserie => run_miserie(game, game_mode),
        GameMode::MiserieOpTafel => run_miserie(game, game_mode),
        GameMode::Abondance => run_abondance(game, game_mode),
        GameMode::SoloSlim => run_solo_slim(game, game_mode),
    };
    game.display_all_player_points();
    game
}
