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

    pub fn fail_all_players_except(&mut self, player: &mut Player) {
        self.fail_all_players();
        player.succeeded_current_round = true;
    }

    pub fn fail_all_players(&mut self) {
        self.player1.succeeded_current_round = false;
        self.player2.succeeded_current_round = false;
        self.player3.succeeded_current_round = false;
        self.player4.succeeded_current_round = false;
    }

    pub fn succeed_all_players(&mut self) {
        self.player1.succeeded_current_round = true;
        self.player2.succeeded_current_round = true;
        self.player3.succeeded_current_round = true;
        self.player4.succeeded_current_round = true;
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
}

impl Player {
    pub fn new(id: u8, name: String) -> Self {
        Player {
            id,
            name,
            points: vec![],
            succeeded_current_round: true,
            playing_player: false,
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
