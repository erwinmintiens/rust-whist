use crate::models::game::{Game, Player, Points};
use crate::utils::{add_points_to_players, get_playing_and_opposing_players};

enum SoloType {
    Solo6,
    Solo7,
    Solo8,
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
    validate_players(&playing_players, &opposing_players);
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
        points.playing_points = -21 - 3 * (7 - tricks_achieved as i32);
        points.opposing_points = 16 + 2 * (7 - tricks_achieved as i32);
    } else {
        // playing player WINS
        points.playing_points = 21 + 3 * (tricks_achieved as i32 - 7);
        points.opposing_points = 0;
    }
    points
}

fn validate_players(playing_players: &Vec<&mut Player>, opposing_players: &Vec<&mut Player>) {
    if playing_players.len() != 1 {
        panic!("The amount of playing players is not equal to 1 in a game of solo!");
    }
    if opposing_players.len() != 3 {
        panic!("The amount of opposing_players is not equal to 3 in a game of solo!");
    }
}

#[cfg(test)]
mod tests {
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
    fn test_solo_6_points() {
        let mut game = setup_game();
        game.player1.playing_player = true;

        solo(&mut game, 4, SoloType::Solo6);
        assert_eq!(game.player1.total_points(), -18);
        assert_eq!(game.player2.total_points(), 12);
        assert_eq!(game.player3.total_points(), 12);
        assert_eq!(game.player4.total_points(), 12);
        assert!(game.player1.succeeded_current_round == false);
        assert!(game.player2.succeeded_current_round == true);
        assert!(game.player3.succeeded_current_round == true);
        assert!(game.player4.succeeded_current_round == true);

        game.clear_all_player_points();

        game.player1.playing_player = false;
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
}
