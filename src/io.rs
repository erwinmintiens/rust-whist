use crate::games::game_modes::GameMode;
use std::io;

pub fn game_mode_selector() -> GameMode {
    loop {
        println!(
            "Please select your game mode:\n1\t{}\n2\t{}\n3\t{}\n4\t{}\n5\t{}\n6\t{}\n7\t{}\n8\t{}\n9\t{}\n",
            GameMode::Solo,
            GameMode::VragenEnMeegaan,
            GameMode::Troel,
            GameMode::Piccolo,
            GameMode::KleineMiserie,
            GameMode::GroteMiserie,
            GameMode::MiserieOpTafel,
            GameMode::Abondance,
            GameMode::SoloSlim
        );
        let mut game_mode = String::new();
        io::stdin()
            .read_line(&mut game_mode)
            .expect("Failed to read game mode");
        let game_mode: u8 = game_mode
            .trim()
            .parse()
            .expect("Please select a valid number");
        let selected_game_mode = match game_mode {
            1 => GameMode::Solo,
            2 => GameMode::VragenEnMeegaan,
            3 => GameMode::Troel,
            4 => GameMode::Piccolo,
            5 => GameMode::KleineMiserie,
            6 => GameMode::GroteMiserie,
            7 => GameMode::MiserieOpTafel,
            8 => GameMode::Abondance,
            9 => GameMode::SoloSlim,
            _ => {
                println!("Error: please provide a valid number\n");
                continue;
            }
        };
        return selected_game_mode;
    }
}

pub fn get_tricks_to_achieve() -> u8 {
    println!("How many tricks do they have to achieve?");
    return get_u8_input();
}

pub fn get_tricks_achieved() -> u8 {
    println!("How many tricks were achieved?");
    return get_u8_input();
}

fn get_u8_input() -> u8 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input: u8 = match input.trim().parse() {
            Ok(value) => value,
            Err(e) => {
                println!("Please provide a valid number");
                continue;
            }
        };
        return input;
    }
}
