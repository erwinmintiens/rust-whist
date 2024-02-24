use crate::games::game_modes::GameMode;
use std::{error::Error, io};

pub fn single_player_selector<'a>(players: &'a [String], game_mode: &GameMode) -> &'a String {
    loop {
        println!(
            "Who is playing {}?\n1\t{}\n2\t{}\n3\t{}\n4\t{}\n",
            game_mode, &players[0], &players[1], &players[2], &players[3]
        );
        let player = select_player(&players).unwrap();
        return player;
    }
}

pub fn double_player_selector<'a>(players: &'a [String], game_mode: &GameMode) -> [&'a String; 2] {
    loop {
        println!(
            "Who is playing {}?\n1\t{}\n2\t{}\n3\t{}\n4\t{}\n",
            game_mode, &players[0], &players[1], &players[2], &players[3]
        );
        println!("Player 1:");
        let player1 = select_player(&players).unwrap();
        println!("Player 2:");
        let player2 = select_player(&players).unwrap();
        return [player1, player2];
    }
}

fn select_player<'a>(players: &'a [String]) -> Result<&'a String, Box<dyn Error>> {
    let mut player = String::new();
    io::stdin().read_line(&mut player)?;
    let player: usize = player.trim().parse()?;
    return Ok(&players[player - 1]);
}

pub fn read_player(mut string_to_read_to: String, player_number: u8) -> String {
    println!("Player {}:", player_number);
    io::stdin()
        .read_line(&mut string_to_read_to)
        .expect("Failed to read player name");
    string_to_read_to.trim().to_string()
}
