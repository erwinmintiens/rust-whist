use crate::games::game_modes::GameMode;
use crate::models::game::{Game, Player};
use crate::utils::get_playing_and_opposing_players;

pub fn points(game: &mut Game, tricks_to_achieve: u8, tricks_achieved: u8, game_mode: GameMode) {
    let (mut playing_players, mut opposing_players) = get_playing_and_opposing_players(game);
    if !validate_players(&playing_players, &opposing_players) {
        return;
    }
    let player = &mut playing_players[0];
    match game_mode {
        GameMode::Abondance => abondance_points(
            player,
            &mut opposing_players,
            tricks_achieved,
            tricks_to_achieve,
        ),
        GameMode::SoloSlim => solo_slim_points(
            player,
            &mut opposing_players,
            tricks_achieved,
            tricks_to_achieve,
        ),
        _ => {
            eprintln!(
                "Unexpected game_mode while handling Abondance/Solo Slim: {}",
                game_mode
            );
        }
    }
}

fn abondance_points(
    playing_player: &mut Player,
    opposing_players: &mut Vec<&mut Player>,
    tricks_achieved: u8,
    tricks_to_achieve: u8,
) {
    if tricks_achieved >= tricks_to_achieve {
        // Player SUCCESS
        playing_player.succeeded_current_round = true;
        playing_player.add_points(30 + 15 * (tricks_to_achieve as i32 - 9));

        fail_opposing_players(opposing_players);
    } else {
        // Player FAILURE
        playing_player.succeeded_current_round = false;
        playing_player.add_points(-30 - 15 * (tricks_to_achieve as i32 - 9));
        for opposing_player in opposing_players.iter_mut() {
            opposing_player.succeeded_current_round = true;
            opposing_player.add_points(20 + 10 * (tricks_to_achieve as i32 - 9));
        }
    }
}

fn solo_slim_points(
    playing_player: &mut Player,
    opposing_players: &mut Vec<&mut Player>,
    tricks_achieved: u8,
    tricks_to_achieve: u8,
) {
    if tricks_achieved >= tricks_to_achieve {
        // Player SUCCESS
        playing_player.succeeded_current_round = true;
        playing_player.add_points(if tricks_to_achieve == 12 { 100 } else { 150 });
        fail_opposing_players(opposing_players);
    } else {
        // Player FAILURE
        playing_player.succeeded_current_round = false;
        playing_player.add_points(if tricks_to_achieve == 12 { -100 } else { -150 });
        for player in opposing_players.iter_mut() {
            player.succeeded_current_round = true;
            player.add_points(if tricks_to_achieve == 12 { 66 } else { 100 });
        }
    }
}

fn fail_opposing_players(opposing_players: &mut Vec<&mut Player>) {
    for player in opposing_players.iter_mut() {
        player.succeeded_current_round = false;
        player.add_points(0);
    }
}

fn validate_players(
    playing_players: &Vec<&mut Player>,
    opposing_players: &Vec<&mut Player>,
) -> bool {
    if playing_players.len() != 1 {
        eprintln!(
            "The amount of playing players cannot be equal to {} for a game of abondance/solo slim",
            playing_players.len()
        );
        return false;
    }
    if opposing_players.len() != 3 {
        eprintln!("The amount of opposing players cannot be equal to {} for a game of abondance/solo slim", opposing_players.len());
        return false;
    }
    true
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

    macro_rules! abondance_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (to_achieve, achieved, player_expected, opposing_players_points) = $value;
                let mut game = setup_game();
                let mut players = vec![&mut game.player2, &mut game.player3, &mut game.player4];
                let player = &mut game.player1;

                abondance_points(
                    player,
                    &mut players,
                    achieved,
                    to_achieve,
                );

                assert_eq!(player.total_points(), player_expected);
                for player in players {
                    assert_eq!(player.total_points(), opposing_players_points);
                }
            }
        )*
        }
    }

    abondance_tests! {
        abondance_9_achieved_9: (9, 9, 30, 0),
        abondance_9_achieves_11: (9, 11, 30, 0),
        abondance_10_achieves_10: (10, 10, 45, 0),
        abondance_10_achieves_13: (10, 13, 45, 0),
        abondance_11_achieves_11: (11, 11, 60, 0),
        abondance_11_achieves_12: (11, 12, 60, 0),
    }
}
