use crate::models::game::{Game, Player, Points};
use crate::utils::{add_points_to_players, get_playing_and_opposing_players};

pub fn vragen_en_meegaan_points(game: &mut Game, tricks_to_achieve: u8, tricks_achieved: u8) {
    let (mut playing_players, mut opposing_players) = get_playing_and_opposing_players(game);
    match validate_players(&playing_players, &opposing_players) {
        Ok(_) => {}
        Err(e) => {
            panic!("Validating player failed: {}", e);
        }
    }
}

fn validate_players(
    playing_players: &Vec<&mut Player>,
    opposing_players: &Vec<&mut Player>,
) -> Result<(), &'static str> {
    if playing_players.len() != 2 {
        return Err("The amount of playing players is not equal to 2!");
    }
    if opposing_players.len() != 2 {
        return Err("The amount of opposing players is not equal to 2!");
    }
    Ok(())
}
