use crate::models::game::{Game, Player};
use std::{error::Error, io};

fn select_player<'a>(game: &'a Game) -> Result<&'a Player, Box<dyn Error>> {
    let mut player = String::new();
    io::stdin().read_line(&mut player)?;
    let player: usize = player.trim().parse()?;
    match player {
        1 => return Ok(&game.player1),
        2 => return Ok(&game.player2),
        3 => return Ok(&game.player3),
        4 => return Ok(&game.player4),
        _ => return Err(Into::into("Please provide a valid input")),
    }
}

pub fn get_playing_and_opposing_players<'a>(
    game: &'a mut Game,
) -> (Vec<&'a mut Player>, Vec<&'a mut Player>) {
    let mut playing_players: Vec<&mut Player> = Vec::new();
    let mut opposing_players: Vec<&mut Player> = Vec::new();
    if game.player1.playing_player {
        playing_players.push(&mut game.player1);
    } else {
        opposing_players.push(&mut game.player1);
    }
    if game.player2.playing_player {
        playing_players.push(&mut game.player2);
    } else {
        opposing_players.push(&mut game.player2);
    }
    if game.player3.playing_player {
        playing_players.push(&mut game.player3);
    } else {
        opposing_players.push(&mut game.player3);
    }
    if game.player4.playing_player {
        playing_players.push(&mut game.player4);
    } else {
        opposing_players.push(&mut game.player4);
    }
    (playing_players, opposing_players)
}

pub fn add_points_to_players(
    game: &mut Game,
    points_playing_player: i32,
    points_opposing_player: i32,
) {
    if game.player1.playing_player {
        game.player1.add_points(points_playing_player);
    } else {
        game.player1.add_points(points_opposing_player);
    }
    if game.player2.playing_player {
        game.player2.add_points(points_playing_player);
    } else {
        game.player2.add_points(points_opposing_player);
    }
    if game.player3.playing_player {
        game.player3.add_points(points_playing_player);
    } else {
        game.player3.add_points(points_opposing_player);
    }
    if game.player4.playing_player {
        game.player4.add_points(points_playing_player);
    } else {
        game.player4.add_points(points_opposing_player);
    }
}
