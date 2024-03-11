use std::{fmt, io::Write};

#[derive(Debug)]
pub struct Game {
    pub player1: Player,
    pub player2: Player,
    pub player3: Player,
    pub player4: Player,
}

impl Game {
    pub fn new(player1: Player, player2: Player, player3: Player, player4: Player) -> Self {
        Game {
            player1,
            player2,
            player3,
            player4,
        }
    }

    pub fn get_all_playing_players(&self) -> Vec<&Player> {
        let mut result: Vec<&Player> = vec![];
        if self.player1.playing_player {
            result.push(&self.player1);
        }
        if self.player2.playing_player {
            result.push(&self.player2);
        }
        if self.player3.playing_player {
            result.push(&self.player3);
        }
        if self.player4.playing_player {
            result.push(&self.player4);
        }
        result
    }

    pub fn reset_all_playing_players(&mut self) {
        self.player1.playing_player = false;
        self.player2.playing_player = false;
        self.player3.playing_player = false;
        self.player4.playing_player = false;
    }

    pub fn clear_all_player_points(&mut self) {
        self.player1.clear_points();
        self.player2.clear_points();
        self.player3.clear_points();
        self.player4.clear_points();
    }

    pub fn add_points_to_players_except(&mut self, points: i32, player: &mut Player) {
        match player.id {
            1_u8 => {
                self.player2.add_points(points);
                self.player3.add_points(points);
                self.player4.add_points(points);
            }
            2_u8 => {
                self.player1.add_points(points);
                self.player3.add_points(points);
                self.player4.add_points(points);
            }
            3_u8 => {
                self.player1.add_points(points);
                self.player2.add_points(points);
                self.player4.add_points(points);
            }
            4_u8 => {
                self.player1.add_points(points);
                self.player2.add_points(points);
                self.player3.add_points(points);
            }
            _ => {}
        }
    }

    pub fn display_all_player_points(&self) {
        println!("{}: {}", self.player1.name, self.player1.total_points());
        println!("{}: {}", self.player2.name, self.player2.total_points());
        println!("{}: {}", self.player3.name, self.player3.total_points());
        println!("{}: {}", self.player4.name, self.player4.total_points());
    }
}

#[derive(Debug)]
pub struct Player {
    pub id: u8,
    pub name: String,
    pub points: Vec<i32>,
    pub succeeded_current_round: bool,
    pub playing_player: bool,
    pub tricks_achieved_current_round: Option<u8>,
}

impl Player {
    pub fn new(id: u8, name: String) -> Self {
        Player {
            id,
            name,
            points: vec![],
            succeeded_current_round: true,
            playing_player: false,
            tricks_achieved_current_round: None,
        }
    }

    pub fn total_points(&self) -> i32 {
        let sum = self.points.iter().sum();
        sum
    }

    pub fn clear_points(&mut self) {
        self.points.clear();
    }

    pub fn add_points(&mut self, points: i32) {
        self.points.push(points);
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

#[derive(Debug, Eq, PartialOrd, Ord)]
pub struct Points {
    pub playing_points: i32,
    pub opposing_points: i32,
}

impl Points {
    pub fn new() -> Self {
        Points {
            playing_points: 0,
            opposing_points: 0,
        }
    }
}

impl PartialEq for Points {
    fn eq(&self, other: &Self) -> bool {
        self.playing_points == other.playing_points && self.opposing_points == other.opposing_points
    }
}

#[cfg(test)]
mod test {
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

    mod game {
        use super::*;

        #[test]
        fn test_resetting_playing_players() {
            let mut game = setup_game();
            game.player1.playing_player = true;
            game.player2.playing_player = true;
            game.player3.playing_player = true;
            game.player4.playing_player = true;

            game.reset_all_playing_players();

            assert_eq!(game.player1.playing_player, false);
            assert_eq!(game.player2.playing_player, false);
            assert_eq!(game.player3.playing_player, false);
            assert_eq!(game.player4.playing_player, false);
        }
    }

    mod points {
        use super::*;

        #[test]
        fn test_partial_eq() {
            let points1 = Points {
                playing_points: 12,
                opposing_points: -5,
            };
            let points2 = Points {
                playing_points: 12,
                opposing_points: -5,
            };
            let points3 = Points {
                playing_points: 12,
                opposing_points: 0,
            };
            let points4 = Points {
                playing_points: -2,
                opposing_points: -5,
            };

            assert_eq!(points1 == points2, true);
            assert_eq!(points1 == points3, false);
            assert_eq!(points2 == points3, false);
            assert_eq!(points1 == points4, false);
        }
    }
}
