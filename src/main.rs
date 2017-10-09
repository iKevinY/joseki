extern crate joseki;

use std::env;

use joseki::Game;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("Usage: joseki <filename>");
    } else {
        println!("{}", Game::from_sgf(&args[1]));
    }
}
