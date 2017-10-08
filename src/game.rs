use std::fmt;
use std::fs::File;
use std::io::Read;

use regex::Regex;

use board::{Board, Stone};

#[derive(Clone, Debug)]
pub struct Game {
    board: Board,
    black_player: Option<String>,
    white_player: Option<String>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            black_player: None,
            white_player: None,
        }
    }

    pub fn from_file(filename: &str) -> Game {
        let mut f = File::open(filename).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();

        let mut game = Game::new();

        let re = Regex::new(r"(\w{1,2})\[(.+?)\]").expect("invalid regex");

        for cap in re.captures_iter(&contents) {
            let val = &cap[2].to_string();

            match &cap[1] {
                "B" | "AB" => {
                    let (x, y) = Self::alpha_to_xy(val);
                    game.board[(x, y)] = Stone::Black;
                }
                "W" | "AW" => {
                    let (x, y) = Self::alpha_to_xy(val);
                    game.board[(x, y)] = Stone::White;
                }
                "PB" => {
                    game.black_player = Some(val.to_string());
                },
                "PW" => {
                    game.white_player = Some(val.to_string());
                },
                _ => {}
            }
        }

        game
    }

    fn alpha_to_xy(alpha: &str) -> (usize, usize) {
        let mut chars = alpha.chars();
        let x = chars.next().expect("expected 2 characters");
        let y = chars.next().expect("expected 2 characters");

        (x as usize - b'a' as usize, y as usize - b'a' as usize)
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let unknown = String::from("<unknown>");

        let black_player = self.clone().black_player.unwrap_or(unknown.clone());
        let white_player = self.clone().white_player.unwrap_or(unknown.clone());

        writeln!(f, "Black Player: {}", black_player)?;
        writeln!(f, "White Player: {}", white_player)?;
        write!(f, "{}", self.board)
    }
}

#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn new_game() {
        let game = Game::new();
        assert_eq!(game.black_player, None);
        assert_eq!(game.white_player, None);
    }
}
