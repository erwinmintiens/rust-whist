use rust_whist::start_game;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    start_game();
}
