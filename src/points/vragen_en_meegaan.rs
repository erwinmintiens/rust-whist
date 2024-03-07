use crate::models::game::{Game, Points};
use crate::utils::{add_points_to_players, get_playing_and_opposing_players};

pub fn vragen_en_meegaan_points(game: &mut Game, tricks_to_achieve: u8, tricks_achieved: u8) {
    let points = points(game, tricks_to_achieve, tricks_achieved);
    add_points_to_players(game, points.playing_points, points.opposing_points);
}

fn points(game: &mut Game, tricks_to_achieve: u8, tricks_achieved: u8) -> Points {
    let (mut playing_players, mut opposing_players) = get_playing_and_opposing_players(game);
    let playing_points: i32;
    let opposing_points: i32;
    if tricks_achieved >= tricks_to_achieve {
        // playing players SUCCESS
        playing_points = playing_players_success_points(tricks_to_achieve, tricks_achieved);
        opposing_points = 0;
        for player in playing_players.iter_mut() {
            player.succeeded_current_round = true;
        }
        for player in opposing_players.iter_mut() {
            player.succeeded_current_round = false;
        }
    } else {
        // playing players FAILURE
        playing_points = playing_players_failure_points(tricks_to_achieve, tricks_achieved);
        opposing_points = opposing_players_success_points(tricks_to_achieve, tricks_achieved);
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

/// Returns points for the playing players if they achieved their tricks.
/// ONLY USE WHEN PLAYING PLAYERS SUCCEEDED
fn playing_players_success_points(tricks_to_achieve: u8, tricks_achieved: u8) -> i32 {
    match tricks_achieved {
        13 => return 30,
        12 => {
            if tricks_to_achieve == 12 {
                return 22;
            } else {
                return 19;
            }
        }
        _ => {
            return 7
                + 3 * (tricks_to_achieve as i32 - 8)
                + 3 * (tricks_achieved as i32 - tricks_to_achieve as i32)
        }
    }
}

/// Returns points for the playing players if they did not achieve their tricks.
/// ONLY USE WHEN PLAYING PLAYERS FAILED
fn playing_players_failure_points(tricks_to_achieve: u8, tricks_achieved: u8) -> i32 {
    match tricks_to_achieve {
        13 => return -33 - 3 * (tricks_to_achieve as i32 - 1 - tricks_achieved as i32),
        12 => return -25 - 3 * (tricks_to_achieve as i32 - 1 - tricks_achieved as i32),
        11 => return -19 - 3 * (tricks_to_achieve as i32 - 1 - tricks_achieved as i32),
        10 => return -16 - 3 * (tricks_to_achieve as i32 - 1 - tricks_achieved as i32),
        9 => return -13 - 3 * (tricks_to_achieve as i32 - 1 - tricks_achieved as i32),
        8 => return -10 - 3 * (tricks_to_achieve as i32 - 1 - tricks_achieved as i32),
        _ => return 0,
    }
}

fn opposing_players_success_points(tricks_to_achieve: u8, tricks_achieved: u8) -> i32 {
    return playing_players_failure_points(tricks_to_achieve, tricks_achieved).abs();
}

#[cfg(test)]
mod tests {

    use crate::models::game::Game;

    mod successful_tests {
        use super::super::*;

        #[test]
        fn test_playing_players_success_points() {
            assert_eq!(playing_players_success_points(8, 8), 7);
            assert_eq!(playing_players_success_points(8, 9), 10);
            assert_eq!(playing_players_success_points(8, 10), 13);
            assert_eq!(playing_players_success_points(8, 11), 16);
            assert_eq!(playing_players_success_points(8, 12), 19);
        }

        #[test]
        /// Tests if all vragen_en_meegaan game modes which are successful
        /// and where 12 tricks are achieved result in 10 points.
        fn test_playing_players_success_points_9_tricks() {
            assert_eq!(playing_players_success_points(8, 9), 10);
            assert_eq!(
                playing_players_success_points(8, 9),
                playing_players_success_points(9, 9)
            );
        }

        #[test]
        /// Tests if all vragen_en_meegaan game modes which are successful
        /// and where 10 tricks are achieved result in 13 points.
        fn test_playing_players_success_points_10_tricks() {
            assert_eq!(playing_players_success_points(8, 10), 13);
            assert_eq!(
                playing_players_success_points(8, 10),
                playing_players_success_points(9, 10)
            );
            assert_eq!(
                playing_players_success_points(8, 10),
                playing_players_success_points(10, 10)
            );
        }

        #[test]
        /// Tests if all vragen_en_meegaan game modes which are successful
        /// and where 11 tricks are achieved result in 16 points.
        fn test_playing_players_success_points_11_tricks() {
            assert_eq!(playing_players_success_points(8, 11), 16);
            assert_eq!(
                playing_players_success_points(8, 11),
                playing_players_success_points(9, 11)
            );
            assert_eq!(
                playing_players_success_points(8, 11),
                playing_players_success_points(10, 11)
            );
            assert_eq!(
                playing_players_success_points(8, 11),
                playing_players_success_points(11, 11)
            );
        }

        #[test]
        /// Tests if all vragen_en_meegaan game modes which are successful
        /// and where 12 tricks are achieved result in 19 points, except when
        /// going for 12 tricks and succeeding. Then, 22 points are awarded.
        fn test_playing_players_success_points_12_tricks() {
            assert_eq!(playing_players_success_points(8, 12), 19);
            assert_eq!(
                playing_players_success_points(8, 12),
                playing_players_success_points(9, 12)
            );
            assert_eq!(
                playing_players_success_points(8, 12),
                playing_players_success_points(10, 12)
            );
            assert_eq!(
                playing_players_success_points(8, 12),
                playing_players_success_points(11, 12)
            );
            assert_eq!(playing_players_success_points(12, 12), 22);
        }

        #[test]
        /// Tests if all vragen_en_meegaan game modes which are successful
        /// and where 13 tricks are achieved result in 30 points.
        fn test_playing_players_success_points_13_tricks() {
            assert_eq!(playing_players_success_points(8, 13), 30);
            assert_eq!(playing_players_success_points(9, 13), 30);
            assert_eq!(playing_players_success_points(10, 13), 30);
            assert_eq!(playing_players_success_points(11, 13), 30);
            assert_eq!(playing_players_success_points(12, 13), 30);
            assert_eq!(playing_players_success_points(13, 13), 30);
        }
    }

    mod failure_tests {
        use crate::points::vragen_en_meegaan::{
            playing_players_failure_points, playing_players_success_points,
        };

        #[test]
        fn test_7_tricks_achieved() {
            assert_eq!(playing_players_failure_points(8, 7), -10);
            assert_eq!(playing_players_failure_points(9, 7), -16);
            assert_eq!(playing_players_failure_points(10, 7), -22);
            assert_eq!(playing_players_failure_points(11, 7), -28);
            assert_eq!(playing_players_failure_points(12, 7), -37);
            assert_eq!(playing_players_failure_points(13, 7), -48);
        }

        #[test]
        fn test_8_tricks_achieved() {
            assert_eq!(playing_players_failure_points(9, 8), -13);
            assert_eq!(playing_players_failure_points(10, 8), -19);
            assert_eq!(playing_players_failure_points(11, 8), -25);
            assert_eq!(playing_players_failure_points(12, 8), -34);
            assert_eq!(playing_players_failure_points(13, 8), -45);
        }

        #[test]
        fn test_9_tricks_achieved() {
            assert_eq!(playing_players_failure_points(10, 9), -16);
            assert_eq!(playing_players_failure_points(11, 9), -22);
            assert_eq!(playing_players_failure_points(12, 9), -31);
            assert_eq!(playing_players_failure_points(13, 9), -42);
        }

        #[test]
        fn test_10_tricks_achieved() {
            assert_eq!(playing_players_failure_points(11, 10), -19);
            assert_eq!(playing_players_failure_points(12, 10), -28);
            assert_eq!(playing_players_failure_points(13, 10), -39);
        }

        #[test]
        fn test_11_tricks_achieved() {
            assert_eq!(playing_players_failure_points(12, 11), -25);
            assert_eq!(playing_players_failure_points(13, 11), -36);
        }

        #[test]
        fn test_12_tricks_achieved() {
            assert_eq!(playing_players_failure_points(13, 12), -33);
        }
    }
}
