use crate::games::game_modes::GameMode;
use crate::models::game::{Game, Player};
use inquire::{MultiSelect, Select, Text};
use std::io;

pub fn game_mode_selector() -> GameMode {
    loop {
        let options: Vec<GameMode> = vec![
            GameMode::Solo,
            GameMode::VragenEnMeegaan,
            GameMode::Troel,
            GameMode::Piccolo,
            GameMode::KleineMiserie,
            GameMode::GroteMiserie,
            GameMode::MiserieOpTafel,
            GameMode::Abondance,
            GameMode::SoloSlim,
        ];

        let answer = Select::new("Select your game mode:", options).prompt();
        match answer {
            Ok(choice) => return choice,
            Err(_) => {
                println!("An error occurred while selecting choice. Please try again.");
                continue;
            }
        };
    }
}

pub fn get_tricks_to_achieve(minimum: u8, maximum: u8) -> u8 {
    let message = "How many tricks do they have to achieve?";
    return get_u8_input(message, minimum, maximum);
}

pub fn get_tricks_achieved(minimum: u8, maximum: u8) -> u8 {
    let message = "How many tricks were achieved?";
    return get_u8_input(message, minimum, maximum);
}

fn get_u8_input(message: &str, minimum: u8, maximum: u8) -> u8 {
    loop {
        let result = Select::new(message, (minimum..=maximum).collect()).prompt();
        match result {
            Ok(value) => return value,
            Err(_) => {
                println!("An error occurred while parsing selected value. Please try again.");
                continue;
            }
        }
    }
}

pub fn read_player(player_number: u8) -> String {
    let message = format!("Player {player_number} name:");
    loop {
        let player_name = Text::new(&message).prompt();
        match player_name {
            Ok(name) => {
                println!();
                return name;
            }
            Err(e) => {
                println!("An error occurred: {}, please try again.", e);
                continue;
            }
        }
    }
}

pub fn single_player_selector<'a>(game: &'a mut Game, message: String) {
    loop {
        let player = Select::new(
            &message,
            vec![&game.player1, &game.player2, &game.player3, &game.player4],
        )
        .prompt();
        match player {
            Ok(_) => return,
            Err(_) => {
                println!("An error occurred while selecting value. Please try again.");
                continue;
            }
        }
    }
}

pub fn double_player_selector<'a>(game: &'a Game, game_mode: &GameMode) -> Vec<&'a Player> {
    let msg = format!("Who is playing {game_mode}?");
    loop {
        let players = MultiSelect::new(
            &msg,
            vec![&game.player1, &game.player2, &game.player3, &game.player4],
        )
        .prompt();
        match players {
            Ok(player_vec) => {
                if player_vec.len() != 2 {
                    println!("Please select 2 players.");
                    continue;
                }
                return player_vec;
            }
            Err(_) => {
                println!("An error occurred while selecting players. Please try again.");
                continue;
            }
        }
    }
}
