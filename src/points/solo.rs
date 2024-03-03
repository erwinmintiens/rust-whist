use crate::models::game::{Game, Player};
use crate::utils::{add_points_to_players, get_playing_and_opposing_players};

pub fn solo_points(game: &mut Game, tricks_to_achieve: u8, tricks_achieved: u8) {
    let points = match tricks_to_achieve {
        6 => solo_6(game, tricks_achieved),
        7 => solo_7(game, tricks_achieved),
        8 => solo_8(game, tricks_achieved),
        _ => solo_6(game, tricks_achieved),
    };
    points
}

fn solo_6(game: &mut Game, tricks_achieved: u8) {
    let (mut playing_players, mut opposing_players) = get_playing_and_opposing_players(game);
    if playing_players.len() != 1 {
        panic!("The amount of playing players is not equal to 1 in a game of solo!");
    }
    let playing_player = &mut playing_players[0];
    let playing_player_points: i32;
    let opposing_player_points: i32;
    if tricks_achieved < 6 {
        // playing_player LOSES
        playing_player.succeeded_current_round = false;
        for player in opposing_players.iter_mut() {
            player.succeeded_current_round = true;
        }
        playing_player_points = -15 - (5 - tricks_achieved as i32) * 3;
        opposing_player_points = 10 + (2 * (5 - tricks_achieved as i32));
    } else {
        // playing_player WINS
        playing_player.succeeded_current_round = true;
        for player in opposing_players.iter_mut() {
            player.succeeded_current_round = false;
        }
        playing_player_points = 12 + (tricks_achieved as i32 - 6) * 3;
        opposing_player_points = 0;
    }
    add_points_to_players(game, playing_player_points, opposing_player_points);
}

fn solo_7(game: &mut Game, tricks_achieved: u8) {}
fn solo_8(game: &mut Game, tricks_achieved: u8) {}

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

        solo_6(&mut game, 4);
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
        solo_6(&mut game, 8);
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
