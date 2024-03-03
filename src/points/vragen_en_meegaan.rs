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
        // players SUCCEEDED
        playing_points = playing_players_success_points(tricks_to_achieve, tricks_achieved);
        opposing_points = 0;
        for player in playing_players.iter_mut() {
            player.succeeded_current_round = true;
        }
        for player in opposing_players.iter_mut() {
            player.succeeded_current_round = false;
        }
    } else {
        // players FAILED
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

fn playing_players_success_points(tricks_to_achieve: u8, tricks_achieved: u8) -> i32 {
    if tricks_achieved == 13 {
        return 30;
    }
    return 7
        + 3 * (tricks_to_achieve as i32 - 8)
        + 3 * (tricks_achieved as i32 - tricks_to_achieve as i32);
}

fn playing_players_failure_points(tricks_to_achieve: u8, tricks_achieved: u8) -> i32 {
    return -10
        - 6 * (tricks_to_achieve as i32 - 8)
        - 3 * (tricks_to_achieve as i32 - tricks_achieved as i32);
}

fn opposing_players_success_points(tricks_to_achieve: u8, tricks_achieved: u8) -> i32 {
    return playing_players_failure_points(tricks_to_achieve, tricks_achieved).abs();
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::models::game::Game;

    #[test]
    fn test_playing_players_success_points() {
        assert_eq!(playing_players_success_points(8, 8), 7);
        assert_eq!(playing_players_success_points(8, 9), 10);
        assert_eq!(playing_players_success_points(8, 10), 13);
        assert_eq!(playing_players_success_points(8, 11), 16);
        assert_eq!(playing_players_success_points(8, 12), 19);
        assert_eq!(
            playing_players_success_points(8, 9),
            playing_players_success_points(9, 9)
        );
        assert_eq!(
            playing_players_success_points(8, 10),
            playing_players_success_points(9, 10)
        );
        assert_eq!(
            playing_players_success_points(8, 10),
            playing_players_success_points(10, 10)
        );
        assert_eq!(
            playing_players_success_points(8, 11),
            playing_players_success_points(11, 11)
        );
        assert_eq!(
            playing_players_success_points(8, 12),
            playing_players_success_points(12, 12)
        );
        assert_eq!(
            playing_players_success_points(8, 12),
            playing_players_success_points(9, 12)
        );
    }

    #[test]
    fn test_playing_players_success_points_10_tricks() {
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
    fn test_playing_players_success_points_11_tricks() {
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
        assert_eq!(playing_players_success_points(8, 11), 16);
    }

    #[test]
    fn test_playing_players_success_points_30() {
        assert_eq!(playing_players_success_points(8, 13), 30);
        assert_eq!(playing_players_success_points(9, 13), 30);
        assert_eq!(playing_players_success_points(10, 13), 30);
        assert_eq!(playing_players_success_points(11, 13), 30);
        assert_eq!(playing_players_success_points(12, 13), 30);
        assert_eq!(playing_players_success_points(13, 13), 30);
    }
}
