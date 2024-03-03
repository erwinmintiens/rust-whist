use std::fmt;

use crate::models::game::{Game, Player, Points};
use crate::utils::{add_points_to_players, get_playing_and_opposing_players};

enum SoloType {
    Solo6,
    Solo7,
    Solo8,
}

impl fmt::Display for SoloType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SoloType::Solo6 => write!(f, "Solo6"),
            SoloType::Solo7 => write!(f, "Solo7"),
            SoloType::Solo8 => write!(f, "Solo8"),
        }
    }
}

pub fn solo_points(game: &mut Game, tricks_to_achieve: u8, tricks_achieved: u8) {
    let solo_type = match tricks_to_achieve {
        6 => SoloType::Solo6,
        7 => SoloType::Solo7,
        8 => SoloType::Solo8,
        _ => SoloType::Solo6,
    };
    solo(game, tricks_achieved, solo_type);
}

fn solo(game: &mut Game, tricks_achieved: u8, solo_type: SoloType) {
    let (mut playing_players, mut opposing_players) = get_playing_and_opposing_players(game);
    match validate_players(&playing_players, &opposing_players) {
        Ok(_) => {}
        Err(e) => panic!(
            "Validating players failed for solo game {}: {}",
            solo_type, e
        ),
    };
    let playing_player = &mut playing_players[0];
    let points = match solo_type {
        SoloType::Solo6 => solo_6(tricks_achieved),
        SoloType::Solo7 => solo_7(tricks_achieved),
        SoloType::Solo8 => solo_8(tricks_achieved),
    };
    if points.playing_points < 0 {
        playing_player.succeeded_current_round = false;
        for player in opposing_players.iter_mut() {
            player.succeeded_current_round = true;
        }
    } else {
        playing_player.succeeded_current_round = true;
        for player in opposing_players.iter_mut() {
            player.succeeded_current_round = false;
        }
    }
    add_points_to_players(game, points.playing_points, points.opposing_points);
}

fn solo_6(tricks_achieved: u8) -> Points {
    let mut points = Points::new();
    if tricks_achieved < 6 {
        // playing_player LOSES
        points.playing_points = -15 - 3 * (5 - tricks_achieved as i32);
        points.opposing_points = 10 + 2 * (5 - tricks_achieved as i32);
    } else {
        // playing_player WINS
        points.playing_points = 12 + 3 * (tricks_achieved as i32 - 6);
        points.opposing_points = 0;
    }
    points
}

fn solo_7(tricks_achieved: u8) -> Points {
    let mut points = Points::new();
    if tricks_achieved < 7 {
        // playing player LOSES
        points.playing_points = -18 - 3 * (6 - tricks_achieved as i32);
        points.opposing_points = 12 + 2 * (6 - tricks_achieved as i32);
    } else {
        // playing player WINS
        points.playing_points = 15 + 3 * (tricks_achieved as i32 - 7);
        points.opposing_points = 0;
    }
    points
}
fn solo_8(tricks_achieved: u8) -> Points {
    let mut points = Points::new();
    if tricks_achieved < 8 {
        // playing player LOSES
        points.playing_points = -24 - 3 * (7 - tricks_achieved as i32);
        points.opposing_points = 16 + 2 * (7 - tricks_achieved as i32);
    } else {
        // playing player WINS
        points.playing_points = 21 + 3 * (tricks_achieved as i32 - 8);
        points.opposing_points = 0;
    }
    points
}

fn validate_players(
    playing_players: &Vec<&mut Player>,
    opposing_players: &Vec<&mut Player>,
) -> Result<(), &'static str> {
    if playing_players.len() != 1 {
        return Err("The amount of playing players is not equal to 1 in a game of solo!");
    }
    if opposing_players.len() != 3 {
        return Err("The amount of opposing_players is not equal to 3 in a game of solo!");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::models::game::Game;

    use super::*;

    fn setup_game() -> Game {
        let player1 = Player::new(1_u8, String::from("Player1"));
        let player2 = Player::new(2_u8, String::from("Player2"));
        let player3 = Player::new(3_u8, String::from("Player3"));
        let player4 = Player::new(4_u8, String::from("Player4"));
        Game {
            player1,
            player2,
            player3,
            player4,
        }
    }

    #[test]
    fn test_solo_6_success_points() {
        let mut game = setup_game();
        game.player2.playing_player = true;

        solo(&mut game, 8, SoloType::Solo6);
        assert_eq!(game.player2.total_points(), 18);
        assert_eq!(game.player1.total_points(), 0);
        assert_eq!(game.player3.total_points(), 0);
        assert_eq!(game.player4.total_points(), 0);
        assert!(game.player1.succeeded_current_round == false);
        assert!(game.player2.succeeded_current_round == true);
        assert!(game.player3.succeeded_current_round == false);
        assert!(game.player4.succeeded_current_round == false);
    }
    #[test]
    fn test_solo_6_failure_points() {
        let mut game = setup_game();
        game.player1.playing_player = true;

        solo(&mut game, 4, SoloType::Solo6);
        assert_eq!(game.player2.total_points(), 12);
        assert_eq!(game.player3.total_points(), 12);
        assert_eq!(game.player4.total_points(), 12);

        assert_eq!(game.player1.total_points(), -18);

        assert!(game.player2.succeeded_current_round == true);
        assert!(game.player3.succeeded_current_round == true);
        assert!(game.player4.succeeded_current_round == true);

        assert!(game.player1.succeeded_current_round == false);
    }

    #[test]
    fn test_solo_7_success_points() {
        let mut game = setup_game();
        game.player3.playing_player = true;

        solo(&mut game, 11, SoloType::Solo7);
        assert_eq!(game.player1.total_points(), 0);
        assert_eq!(game.player2.total_points(), 0);
        assert_eq!(game.player4.total_points(), 0);

        assert_eq!(game.player3.total_points(), 27);

        assert!(game.player1.succeeded_current_round == false);
        assert!(game.player2.succeeded_current_round == false);
        assert!(game.player4.succeeded_current_round == false);

        assert!(game.player3.succeeded_current_round == true);
    }

    #[test]
    fn test_solo_7_failure_points() {
        let mut game = setup_game();
        game.player4.playing_player = true;

        solo(&mut game, 3, SoloType::Solo7);
        assert_eq!(game.player1.total_points(), 18);
        assert_eq!(game.player2.total_points(), 18);
        assert_eq!(game.player3.total_points(), 18);

        assert_eq!(game.player4.total_points(), -27);

        assert!(game.player1.succeeded_current_round == true);
        assert!(game.player2.succeeded_current_round == true);
        assert!(game.player3.succeeded_current_round == true);

        assert!(game.player4.succeeded_current_round == false);
    }

    #[test]
    fn test_solo_8_success_points() {
        let mut game = setup_game();
        game.player3.playing_player = true;

        solo(&mut game, 8, SoloType::Solo8);
        assert_eq!(game.player1.total_points(), 0);
        assert_eq!(game.player2.total_points(), 0);
        assert_eq!(game.player4.total_points(), 0);

        assert_eq!(game.player3.total_points(), 21);

        assert!(game.player1.succeeded_current_round == false);
        assert!(game.player2.succeeded_current_round == false);
        assert!(game.player4.succeeded_current_round == false);

        assert!(game.player3.succeeded_current_round == true);
    }

    #[test]
    fn test_solo_8_failure_points() {
        let mut game = setup_game();
        game.player4.playing_player = true;

        solo(&mut game, 7, SoloType::Solo8);
        assert_eq!(game.player1.total_points(), 16);
        assert_eq!(game.player2.total_points(), 16);
        assert_eq!(game.player3.total_points(), 16);

        assert_eq!(game.player4.total_points(), -24);

        assert!(game.player1.succeeded_current_round == true);
        assert!(game.player2.succeeded_current_round == true);
        assert!(game.player3.succeeded_current_round == true);

        assert!(game.player4.succeeded_current_round == false);
    }

    #[test]
    fn test_player_validation_success() {
        let mut game = setup_game();
        let playing_players = vec![&mut game.player1];
        let opposing_players = vec![&mut game.player3, &mut game.player2, &mut game.player4];
        let result = validate_players(&playing_players, &opposing_players);
        assert!(result == Ok(()));
    }

    #[test]
    fn test_player_validation_amount_of_playing_players_failure() {
        let mut game = setup_game();
        let playing_players = vec![&mut game.player1, &mut game.player4];
        let opposing_players = vec![&mut game.player3];
        let result = validate_players(&playing_players, &opposing_players);
        assert!(
            result == Err("The amount of playing players is not equal to 1 in a game of solo!")
        );
    }

    #[test]
    fn test_player_validation_amount_of_opposing_players_failure() {
        let mut game = setup_game();
        let playing_players = vec![&mut game.player1];
        let opposing_players = vec![&mut game.player3];
        let result = validate_players(&playing_players, &opposing_players);
        assert!(
            result == Err("The amount of opposing_players is not equal to 3 in a game of solo!")
        );
    }
}
