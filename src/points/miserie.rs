use crate::games::game_modes::GameMode;
use crate::io::get_tricks_achieved;
use crate::models::game::{Game, Player};
use crate::utils::get_playing_and_opposing_players;

pub fn miserie_points(game: &mut Game, game_mode: GameMode) {
    let (mut playing_players, mut opposing_players) = get_playing_and_opposing_players(game);
    let mut number_of_failed_players: u8 = 0;
    for player in playing_players.iter_mut() {
        let tricks_achieved = get_tricks_achieved(0, 13, Some(&player.name));
        if !handle_playing_player(player, tricks_achieved, &game_mode) {
            number_of_failed_players += 1;
        }
    }
    handle_opposing_players(&mut opposing_players, number_of_failed_players, &game_mode);
}

/// Handles the points of a playing player in case of win or loss.
/// Returns `true` in case the player succeeded, `false` in case the player failed.
fn handle_playing_player(player: &mut Player, tricks_achieved: u8, game_mode: &GameMode) -> bool {
    player.tricks_achieved_current_round = Some(tricks_achieved);
    if (tricks_achieved == 0 && *game_mode != GameMode::Piccolo)
        || (tricks_achieved == 1 && *game_mode == GameMode::Piccolo)
    {
        player.succeeded_current_round = true;
        player.add_points(match game_mode {
            GameMode::KleineMiserie => 18,
            GameMode::Piccolo => 27,
            GameMode::GroteMiserie => 36,
            GameMode::MiserieOpTafel => 72,
            _ => {
                eprintln!("Unexpected GameMode while handling miserie: {}", game_mode);
                0
            }
        });
        return true;
    } else {
        player.succeeded_current_round = false;
        player.add_points(match game_mode {
            GameMode::KleineMiserie => -18,
            GameMode::Piccolo => -27,
            GameMode::GroteMiserie => -36,
            GameMode::MiserieOpTafel => -72,
            _ => {
                eprintln!("Unexpected GameMode while handling miserie: {}", game_mode);
                0
            }
        });
        return false;
    }
}

fn handle_opposing_players(
    opposing_players: &mut Vec<&mut Player>,
    number_of_failed_players: u8,
    game_mode: &GameMode,
) {
    for player in opposing_players.iter_mut() {
        if number_of_failed_players > 0 {
            player.succeeded_current_round = true;
            player.add_points(match game_mode {
                GameMode::KleineMiserie => 12 * number_of_failed_players as i32,
                GameMode::Piccolo => 18 * number_of_failed_players as i32,
                GameMode::GroteMiserie => 24 * number_of_failed_players as i32,
                GameMode::MiserieOpTafel => 48 * number_of_failed_players as i32,
                _ => {
                    eprintln!("Unexpected GameMode while handling miserie: {}", game_mode);
                    0
                }
            })
        } else {
            player.succeeded_current_round = false;
            player.add_points(0);
        }
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

    mod kleine_miserie {
        use super::*;

        mod success {

            use super::*;

            #[test]
            fn test_1_player_1_succeeds() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player1, 0, &GameMode::KleineMiserie);

                assert_eq!(game.player1.total_points(), 18);
                assert_eq!(game.player2.total_points(), 0);
                assert_eq!(game.player3.total_points(), 0);
                assert_eq!(game.player4.total_points(), 0);
            }

            #[test]
            fn test_2_players_2_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player1, 0, &GameMode::KleineMiserie);
                handle_playing_player(&mut game.player4, 0, &GameMode::KleineMiserie);

                assert_eq!(game.player1.total_points(), 18);
                assert_eq!(game.player4.total_points(), 18);
                assert_eq!(game.player3.total_points(), 0);
                assert_eq!(game.player2.total_points(), 0);
            }

            #[test]
            fn test_3_players_3_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 0, &GameMode::KleineMiserie);
                handle_playing_player(&mut game.player3, 0, &GameMode::KleineMiserie);
                handle_playing_player(&mut game.player4, 0, &GameMode::KleineMiserie);

                assert_eq!(game.player2.total_points(), 18);
                assert_eq!(game.player3.total_points(), 18);
                assert_eq!(game.player4.total_points(), 18);
                assert_eq!(game.player1.total_points(), 0);
            }
        }

        mod failure {
            use super::*;

            #[test]
            fn test_1_player_0_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 1, &GameMode::KleineMiserie);
                handle_opposing_players(
                    &mut vec![&mut game.player1, &mut game.player3, &mut game.player4],
                    1,
                    &GameMode::KleineMiserie,
                );

                assert_eq!(game.player2.total_points(), -18);
                assert_eq!(game.player3.total_points(), 12);
                assert_eq!(game.player4.total_points(), 12);
                assert_eq!(game.player1.total_points(), 12);
            }

            #[test]
            fn test_2_players_0_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player3, 1, &GameMode::KleineMiserie);
                handle_playing_player(&mut game.player4, 1, &GameMode::KleineMiserie);
                handle_opposing_players(
                    &mut vec![&mut game.player1, &mut game.player2],
                    2,
                    &GameMode::KleineMiserie,
                );

                assert_eq!(game.player1.total_points(), 24);
                assert_eq!(game.player2.total_points(), 24);
                assert_eq!(game.player3.total_points(), -18);
                assert_eq!(game.player4.total_points(), -18);
            }

            #[test]
            fn test_2_players_1_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player3, 1, &GameMode::KleineMiserie);
                handle_playing_player(&mut game.player4, 0, &GameMode::KleineMiserie);
                handle_opposing_players(
                    &mut vec![&mut game.player1, &mut game.player2],
                    1,
                    &GameMode::KleineMiserie,
                );

                assert_eq!(game.player1.total_points(), 12);
                assert_eq!(game.player2.total_points(), 12);
                assert_eq!(game.player3.total_points(), -18);
                assert_eq!(game.player4.total_points(), 18);
            }

            #[test]
            fn test_3_players_0_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 1, &GameMode::KleineMiserie);
                handle_playing_player(&mut game.player3, 1, &GameMode::KleineMiserie);
                handle_playing_player(&mut game.player4, 1, &GameMode::KleineMiserie);
                handle_opposing_players(&mut vec![&mut game.player1], 3, &GameMode::KleineMiserie);

                assert_eq!(game.player1.total_points(), 36);
                assert_eq!(game.player2.total_points(), -18);
                assert_eq!(game.player3.total_points(), -18);
                assert_eq!(game.player4.total_points(), -18);
            }
            #[test]
            fn test_3_players_1_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 0, &GameMode::KleineMiserie);
                handle_playing_player(&mut game.player3, 1, &GameMode::KleineMiserie);
                handle_playing_player(&mut game.player4, 1, &GameMode::KleineMiserie);
                handle_opposing_players(&mut vec![&mut game.player1], 2, &GameMode::KleineMiserie);

                assert_eq!(game.player1.total_points(), 24);
                assert_eq!(game.player2.total_points(), 18);
                assert_eq!(game.player3.total_points(), -18);
                assert_eq!(game.player4.total_points(), -18);
            }
            #[test]
            fn test_3_players_2_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 1, &GameMode::KleineMiserie);
                handle_playing_player(&mut game.player3, 0, &GameMode::KleineMiserie);
                handle_playing_player(&mut game.player4, 0, &GameMode::KleineMiserie);
                handle_opposing_players(&mut vec![&mut game.player1], 1, &GameMode::KleineMiserie);

                assert_eq!(game.player1.total_points(), 12);
                assert_eq!(game.player2.total_points(), -18);
                assert_eq!(game.player3.total_points(), 18);
                assert_eq!(game.player4.total_points(), 18);
            }
        }
    }

    mod piccolo {
        use super::*;

        mod success {

            use super::*;

            #[test]
            fn test_1_player_1_succeeds() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player1, 1, &GameMode::Piccolo);

                assert_eq!(game.player1.total_points(), 27);
                assert_eq!(game.player2.total_points(), 0);
                assert_eq!(game.player3.total_points(), 0);
                assert_eq!(game.player4.total_points(), 0);
            }

            #[test]
            fn test_2_players_2_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player1, 1, &GameMode::Piccolo);
                handle_playing_player(&mut game.player4, 1, &GameMode::Piccolo);

                assert_eq!(game.player1.total_points(), 27);
                assert_eq!(game.player4.total_points(), 27);
                assert_eq!(game.player3.total_points(), 0);
                assert_eq!(game.player2.total_points(), 0);
            }

            #[test]
            fn test_3_players_3_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 1, &GameMode::Piccolo);
                handle_playing_player(&mut game.player3, 1, &GameMode::Piccolo);
                handle_playing_player(&mut game.player4, 1, &GameMode::Piccolo);

                assert_eq!(game.player2.total_points(), 27);
                assert_eq!(game.player3.total_points(), 27);
                assert_eq!(game.player4.total_points(), 27);
                assert_eq!(game.player1.total_points(), 0);
            }
        }

        mod failure {
            use super::*;

            #[test]
            fn test_1_player_0_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 2, &GameMode::Piccolo);
                handle_opposing_players(
                    &mut vec![&mut game.player1, &mut game.player3, &mut game.player4],
                    1,
                    &GameMode::Piccolo,
                );

                assert_eq!(game.player2.total_points(), -27);
                assert_eq!(game.player3.total_points(), 18);
                assert_eq!(game.player4.total_points(), 18);
                assert_eq!(game.player1.total_points(), 18);
            }

            #[test]
            fn test_2_players_0_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player3, 0, &GameMode::Piccolo);
                handle_playing_player(&mut game.player4, 0, &GameMode::Piccolo);
                handle_opposing_players(
                    &mut vec![&mut game.player1, &mut game.player2],
                    2,
                    &GameMode::Piccolo,
                );

                assert_eq!(game.player1.total_points(), 36);
                assert_eq!(game.player2.total_points(), 36);
                assert_eq!(game.player3.total_points(), -27);
                assert_eq!(game.player4.total_points(), -27);
            }

            #[test]
            fn test_2_players_1_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player3, 1, &GameMode::Piccolo);
                handle_playing_player(&mut game.player4, 0, &GameMode::Piccolo);
                handle_opposing_players(
                    &mut vec![&mut game.player1, &mut game.player2],
                    1,
                    &GameMode::Piccolo,
                );

                assert_eq!(game.player1.total_points(), 18);
                assert_eq!(game.player2.total_points(), 18);
                assert_eq!(game.player3.total_points(), 27);
                assert_eq!(game.player4.total_points(), -27);
            }

            #[test]
            fn test_3_players_0_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 0, &GameMode::Piccolo);
                handle_playing_player(&mut game.player3, 0, &GameMode::Piccolo);
                handle_playing_player(&mut game.player4, 0, &GameMode::Piccolo);
                handle_opposing_players(&mut vec![&mut game.player1], 3, &GameMode::Piccolo);

                assert_eq!(game.player1.total_points(), 54);
                assert_eq!(game.player2.total_points(), -27);
                assert_eq!(game.player3.total_points(), -27);
                assert_eq!(game.player4.total_points(), -27);
            }
            #[test]
            fn test_3_players_1_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 1, &GameMode::Piccolo);
                handle_playing_player(&mut game.player3, 0, &GameMode::Piccolo);
                handle_playing_player(&mut game.player4, 0, &GameMode::Piccolo);
                handle_opposing_players(&mut vec![&mut game.player1], 2, &GameMode::Piccolo);

                assert_eq!(game.player1.total_points(), 36);
                assert_eq!(game.player2.total_points(), 27);
                assert_eq!(game.player3.total_points(), -27);
                assert_eq!(game.player4.total_points(), -27);
            }
            #[test]
            fn test_3_players_2_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 0, &GameMode::Piccolo);
                handle_playing_player(&mut game.player3, 1, &GameMode::Piccolo);
                handle_playing_player(&mut game.player4, 1, &GameMode::Piccolo);
                handle_opposing_players(&mut vec![&mut game.player1], 1, &GameMode::Piccolo);

                assert_eq!(game.player1.total_points(), 18);
                assert_eq!(game.player2.total_points(), -27);
                assert_eq!(game.player3.total_points(), 27);
                assert_eq!(game.player4.total_points(), 27);
            }
        }
    }

    mod grote_miserie {
        use super::*;

        mod success {

            use super::*;

            #[test]
            fn test_1_player_1_succeeds() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player1, 0, &GameMode::GroteMiserie);

                assert_eq!(game.player1.total_points(), 36);
                assert_eq!(game.player2.total_points(), 0);
                assert_eq!(game.player3.total_points(), 0);
                assert_eq!(game.player4.total_points(), 0);
            }

            #[test]
            fn test_2_players_2_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player1, 0, &GameMode::GroteMiserie);
                handle_playing_player(&mut game.player4, 0, &GameMode::GroteMiserie);

                assert_eq!(game.player1.total_points(), 36);
                assert_eq!(game.player4.total_points(), 36);
                assert_eq!(game.player3.total_points(), 0);
                assert_eq!(game.player2.total_points(), 0);
            }

            #[test]
            fn test_3_players_3_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 0, &GameMode::GroteMiserie);
                handle_playing_player(&mut game.player3, 0, &GameMode::GroteMiserie);
                handle_playing_player(&mut game.player4, 0, &GameMode::GroteMiserie);

                assert_eq!(game.player2.total_points(), 36);
                assert_eq!(game.player3.total_points(), 36);
                assert_eq!(game.player4.total_points(), 36);
                assert_eq!(game.player1.total_points(), 0);
            }
        }

        mod failure {
            use super::*;

            #[test]
            fn test_1_player_0_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 1, &GameMode::GroteMiserie);
                handle_opposing_players(
                    &mut vec![&mut game.player1, &mut game.player3, &mut game.player4],
                    1,
                    &GameMode::GroteMiserie,
                );

                assert_eq!(game.player2.total_points(), -36);
                assert_eq!(game.player3.total_points(), 24);
                assert_eq!(game.player4.total_points(), 24);
                assert_eq!(game.player1.total_points(), 24);
            }

            #[test]
            fn test_2_players_0_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player3, 1, &GameMode::GroteMiserie);
                handle_playing_player(&mut game.player4, 1, &GameMode::GroteMiserie);
                handle_opposing_players(
                    &mut vec![&mut game.player1, &mut game.player2],
                    2,
                    &GameMode::GroteMiserie,
                );

                assert_eq!(game.player1.total_points(), 48);
                assert_eq!(game.player2.total_points(), 48);
                assert_eq!(game.player3.total_points(), -36);
                assert_eq!(game.player4.total_points(), -36);
            }

            #[test]
            fn test_2_players_1_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player3, 1, &GameMode::GroteMiserie);
                handle_playing_player(&mut game.player4, 0, &GameMode::GroteMiserie);
                handle_opposing_players(
                    &mut vec![&mut game.player1, &mut game.player2],
                    1,
                    &GameMode::GroteMiserie,
                );

                assert_eq!(game.player1.total_points(), 24);
                assert_eq!(game.player2.total_points(), 24);
                assert_eq!(game.player3.total_points(), -36);
                assert_eq!(game.player4.total_points(), 36);
            }

            #[test]
            fn test_3_players_0_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 1, &GameMode::GroteMiserie);
                handle_playing_player(&mut game.player3, 1, &GameMode::GroteMiserie);
                handle_playing_player(&mut game.player4, 1, &GameMode::GroteMiserie);
                handle_opposing_players(&mut vec![&mut game.player1], 3, &GameMode::GroteMiserie);

                assert_eq!(game.player1.total_points(), 72);
                assert_eq!(game.player2.total_points(), -36);
                assert_eq!(game.player3.total_points(), -36);
                assert_eq!(game.player4.total_points(), -36);
            }
            #[test]
            fn test_3_players_1_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 0, &GameMode::GroteMiserie);
                handle_playing_player(&mut game.player3, 1, &GameMode::GroteMiserie);
                handle_playing_player(&mut game.player4, 1, &GameMode::GroteMiserie);
                handle_opposing_players(&mut vec![&mut game.player1], 2, &GameMode::GroteMiserie);

                assert_eq!(game.player1.total_points(), 48);
                assert_eq!(game.player2.total_points(), 36);
                assert_eq!(game.player3.total_points(), -36);
                assert_eq!(game.player4.total_points(), -36);
            }
            #[test]
            fn test_3_players_2_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 1, &GameMode::GroteMiserie);
                handle_playing_player(&mut game.player3, 0, &GameMode::GroteMiserie);
                handle_playing_player(&mut game.player4, 0, &GameMode::GroteMiserie);
                handle_opposing_players(&mut vec![&mut game.player1], 1, &GameMode::GroteMiserie);

                assert_eq!(game.player1.total_points(), 24);
                assert_eq!(game.player2.total_points(), -36);
                assert_eq!(game.player3.total_points(), 36);
                assert_eq!(game.player4.total_points(), 36);
            }
        }
    }
    mod open_miserie {
        use super::*;

        mod success {

            use super::*;

            #[test]
            fn test_1_player_1_succeeds() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player1, 0, &GameMode::MiserieOpTafel);

                assert_eq!(game.player1.total_points(), 72);
                assert_eq!(game.player2.total_points(), 0);
                assert_eq!(game.player3.total_points(), 0);
                assert_eq!(game.player4.total_points(), 0);
            }

            #[test]
            fn test_2_players_2_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player1, 0, &GameMode::MiserieOpTafel);
                handle_playing_player(&mut game.player4, 0, &GameMode::MiserieOpTafel);

                assert_eq!(game.player1.total_points(), 72);
                assert_eq!(game.player4.total_points(), 72);
                assert_eq!(game.player3.total_points(), 0);
                assert_eq!(game.player2.total_points(), 0);
            }

            #[test]
            fn test_3_players_3_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 0, &GameMode::MiserieOpTafel);
                handle_playing_player(&mut game.player3, 0, &GameMode::MiserieOpTafel);
                handle_playing_player(&mut game.player4, 0, &GameMode::MiserieOpTafel);

                assert_eq!(game.player2.total_points(), 72);
                assert_eq!(game.player3.total_points(), 72);
                assert_eq!(game.player4.total_points(), 72);
                assert_eq!(game.player1.total_points(), 0);
            }
        }

        mod failure {
            use super::*;

            #[test]
            fn test_1_player_0_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 1, &GameMode::MiserieOpTafel);
                handle_opposing_players(
                    &mut vec![&mut game.player1, &mut game.player3, &mut game.player4],
                    1,
                    &GameMode::MiserieOpTafel,
                );

                assert_eq!(game.player2.total_points(), -72);
                assert_eq!(game.player3.total_points(), 48);
                assert_eq!(game.player4.total_points(), 48);
                assert_eq!(game.player1.total_points(), 48);
            }

            #[test]
            fn test_2_players_0_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player3, 1, &GameMode::MiserieOpTafel);
                handle_playing_player(&mut game.player4, 1, &GameMode::MiserieOpTafel);
                handle_opposing_players(
                    &mut vec![&mut game.player1, &mut game.player2],
                    2,
                    &GameMode::MiserieOpTafel,
                );

                assert_eq!(game.player1.total_points(), 96);
                assert_eq!(game.player2.total_points(), 96);
                assert_eq!(game.player3.total_points(), -72);
                assert_eq!(game.player4.total_points(), -72);
            }

            #[test]
            fn test_2_players_1_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player3, 1, &GameMode::MiserieOpTafel);
                handle_playing_player(&mut game.player4, 0, &GameMode::MiserieOpTafel);
                handle_opposing_players(
                    &mut vec![&mut game.player1, &mut game.player2],
                    1,
                    &GameMode::MiserieOpTafel,
                );

                assert_eq!(game.player1.total_points(), 48);
                assert_eq!(game.player2.total_points(), 48);
                assert_eq!(game.player3.total_points(), -72);
                assert_eq!(game.player4.total_points(), 72);
            }

            #[test]
            fn test_3_players_0_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 1, &GameMode::MiserieOpTafel);
                handle_playing_player(&mut game.player3, 1, &GameMode::MiserieOpTafel);
                handle_playing_player(&mut game.player4, 1, &GameMode::MiserieOpTafel);
                handle_opposing_players(&mut vec![&mut game.player1], 3, &GameMode::MiserieOpTafel);

                assert_eq!(game.player1.total_points(), 144);
                assert_eq!(game.player2.total_points(), -72);
                assert_eq!(game.player3.total_points(), -72);
                assert_eq!(game.player4.total_points(), -72);
            }
            #[test]
            fn test_3_players_1_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 0, &GameMode::MiserieOpTafel);
                handle_playing_player(&mut game.player3, 1, &GameMode::MiserieOpTafel);
                handle_playing_player(&mut game.player4, 1, &GameMode::MiserieOpTafel);
                handle_opposing_players(&mut vec![&mut game.player1], 2, &GameMode::MiserieOpTafel);

                assert_eq!(game.player1.total_points(), 96);
                assert_eq!(game.player2.total_points(), 72);
                assert_eq!(game.player3.total_points(), -72);
                assert_eq!(game.player4.total_points(), -72);
            }
            #[test]
            fn test_3_players_2_succeed() {
                let mut game = setup_game();
                handle_playing_player(&mut game.player2, 1, &GameMode::MiserieOpTafel);
                handle_playing_player(&mut game.player3, 0, &GameMode::MiserieOpTafel);
                handle_playing_player(&mut game.player4, 0, &GameMode::MiserieOpTafel);
                handle_opposing_players(&mut vec![&mut game.player1], 1, &GameMode::MiserieOpTafel);

                assert_eq!(game.player1.total_points(), 48);
                assert_eq!(game.player2.total_points(), -72);
                assert_eq!(game.player3.total_points(), 72);
                assert_eq!(game.player4.total_points(), 72);
            }
        }
    }
}
