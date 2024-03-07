use crate::models::game::{Game, Player, Points};
use crate::utils::{add_points_to_players, get_playing_and_opposing_players};

pub fn troel_points(game: &mut Game, tricks_to_achieve: u8, tricks_achieved: u8) {
    let points: Points = points(game, tricks_to_achieve, tricks_achieved);
    add_points_to_players(game, points.playing_points, points.opposing_points);
}

fn points(game: &mut Game, tricks_to_achieve: u8, tricks_achieved: u8) -> Points {
    let (mut playing_players, mut opposing_players) = get_playing_and_opposing_players(game);
    let playing_points: i32;
    let opposing_points: i32;
    if tricks_achieved >= tricks_to_achieve {
        // playing players SUCCESS
        playing_points = 15;
        opposing_points = 0;
        for player in playing_players.iter_mut() {
            player.succeeded_current_round = true;
        }
        for player in opposing_players.iter_mut() {
            player.succeeded_current_round = false;
        }
    } else {
        // playing players FAILURE
        playing_points = 0;
        opposing_points = 15;
        for player in playing_players.iter_mut() {
            player.succeeded_current_round = false;
        }
        for player in opposing_players.iter_mut() {
            player.succeeded_current_round = true;
        }
    }
    Points {
        playing_points,
        opposing_points,
    }
}

#[cfg(test)]
mod tests {
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
    mod playing_players_success {
        use super::*;

        #[test]
        fn test_complete_flow() {
            let mut game = setup_game();
            game.player1.playing_player = true;
            game.player2.playing_player = true;
            troel_points(&mut game, 8, 8);
            assert!(game.player1.total_points() == 15);
            assert!(game.player2.total_points() == 15);
            assert!(game.player3.total_points() == 0);
            assert!(game.player4.total_points() == 0);

            assert!(game.player1.succeeded_current_round == true);
            assert!(game.player2.succeeded_current_round == true);
            assert!(game.player3.succeeded_current_round == false);
            assert!(game.player4.succeeded_current_round == false);
        }

        #[test]
        fn test_8_tricks_needed() {
            let mut game = setup_game();
            assert_eq!(
                points(&mut game, 8, 8),
                Points {
                    playing_points: 15,
                    opposing_points: 0
                }
            );
            assert_eq!(
                points(&mut game, 8, 10),
                Points {
                    playing_points: 15,
                    opposing_points: 0
                }
            );
            assert_eq!(
                points(&mut game, 8, 13),
                Points {
                    playing_points: 15,
                    opposing_points: 0
                }
            );
        }

        #[test]
        fn test_9_tricks_needed() {
            let mut game = setup_game();
            assert_eq!(
                points(&mut game, 9, 9),
                Points {
                    playing_points: 15,
                    opposing_points: 0
                }
            );
            assert_eq!(
                points(&mut game, 9, 10),
                Points {
                    playing_points: 15,
                    opposing_points: 0
                }
            );
            assert_eq!(
                points(&mut game, 9, 13),
                Points {
                    playing_points: 15,
                    opposing_points: 0
                }
            );
        }
    }

    mod playing_players_failure {
        use super::*;

        #[test]
        fn test_complete_flow() {
            let mut game = setup_game();
            game.player1.playing_player = true;
            game.player2.playing_player = true;
            troel_points(&mut game, 8, 6);
            assert!(game.player1.total_points() == 0);
            assert!(game.player2.total_points() == 0);
            assert!(game.player3.total_points() == 15);
            assert!(game.player4.total_points() == 15);

            assert!(game.player1.succeeded_current_round == false);
            assert!(game.player2.succeeded_current_round == false);
            assert!(game.player3.succeeded_current_round == true);
            assert!(game.player4.succeeded_current_round == true);
        }

        #[test]
        fn test_8_tricks_needed() {
            let mut game = setup_game();
            assert_eq!(
                points(&mut game, 8, 7),
                Points {
                    playing_points: 0,
                    opposing_points: 15
                }
            );
            assert_eq!(
                points(&mut game, 8, 4),
                Points {
                    playing_points: 0,
                    opposing_points: 15
                }
            );
            assert_eq!(
                points(&mut game, 8, 0),
                Points {
                    playing_points: 0,
                    opposing_points: 15
                }
            );
        }

        #[test]
        fn test_9_tricks_needed() {
            let mut game = setup_game();
            assert_eq!(
                points(&mut game, 9, 8),
                Points {
                    playing_points: 0,
                    opposing_points: 15
                }
            );
            assert_eq!(
                points(&mut game, 9, 5),
                Points {
                    playing_points: 0,
                    opposing_points: 15
                }
            );
            assert_eq!(
                points(&mut game, 9, 0),
                Points {
                    playing_points: 0,
                    opposing_points: 15
                }
            );
        }
    }
}
