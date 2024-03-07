use rust_whist::new_game;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    new_game();
}
