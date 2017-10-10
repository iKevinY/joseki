use std::fmt;
use std::fs::File;
use std::io::Read;

use regex::Regex;

use board::{Board, Stone};

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    board: Board,
    last_board: Option<Board>,
    black_player: Option<String>,
    white_player: Option<String>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            last_board: None,
            black_player: None,
            white_player: None,
        }
    }

    pub fn from_str(board: &str) -> Game {
        Game {
            board: Board::from_str(board),
            last_board: None,
            black_player: None,
            white_player: None,
        }
    }

    pub fn from_sgf(filename: &str) -> Game {
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

    /// Places `stone` at `(x, y)`, returning true if it was successful (respecting the ko rule).
    pub fn make_move(&mut self, stone: Stone, x: usize, y: usize) -> bool {
        let mut next_board = self.board.clone();

        if !next_board.make_move(stone, x, y) {
            return false;
        }

        if let Some(ref b) = self.last_board {
            if b.clone() == next_board {
                return false;
            }
        }

        self.last_board = Some(self.board.clone());
        self.board = next_board;

        true
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
    use board::{Board, Stone};

    #[test]
    fn new_game() {
        let game = Game::new();
        assert_eq!(game.black_player, None);
        assert_eq!(game.white_player, None);
    }

    #[test]
    fn make_move() {
        let mut game = Game::from_str("\
            ... \
            ... \
            ...");

        let expected = Game::from_str("\
            ... \
            .#. \
            ...");

        assert!(game.make_move(Stone::Black, 1, 1));
        assert_eq!(game.board, expected.board);
    }

    #[test]
    fn ko_rule_recapture() {
        let mut game = Game::from_str("\
            .#O.. \
            #O.O. \
            .#O.. \
            ..... \
            .....");

        let expected = Game::from_str("\
            .#O.. \
            #.#O. \
            .#O.. \
            ..... \
            .....");

        // Black capture is a valid play.
        assert!(game.make_move(Stone::Black, 2, 1));
        assert_eq!(game.board, expected.board);

        // White cannot capture due to the ko rule.
        assert!(!game.make_move(Stone::White, 1, 1));
        assert_eq!(game.board, expected.board);
    }

    #[test]
    fn valid_ko_threat_sequence() {
        let mut game = Game::from_str("\
            #.#OO \
            .##O. \
            ##OO. \
            #.#O. \
            ##OO.");

        assert!(game.make_move(Stone::White, 1, 3));
        assert_eq!(game.board, Board::from_str("\
            #.#OO \
            .##O. \
            ##OO. \
            #O.O. \
            ##OO."));

        // Black cannot recapture due to the ko rule, so they play elsewhere instead.
        assert!(!game.make_move(Stone::Black, 2, 3));
        assert!(game.make_move(Stone::Black, 4, 2));
        assert_eq!(game.board, Board::from_str("\
            #.#OO \
            .##O. \
            ##OO# \
            #O.O. \
            ##OO."));

        assert!(game.make_move(Stone::White, 4, 3));
        assert_eq!(game.board, Board::from_str("\
            #.#OO \
            .##O. \
            ##OO# \
            #O.OO \
            ##OO."));

        // Black can capture at the location previously prevented by the ko rule.
        assert!(game.make_move(Stone::Black, 2, 3));
        assert_eq!(game.board, Board::from_str("\
            #.#OO \
            .##O. \
            ##OO# \
            #.#OO \
            ##OO."));

        assert!(game.make_move(Stone::White, 4, 1));
        assert_eq!(game.board, Board::from_str("\
            #.#OO \
            .##OO \
            ##OO. \
            #.#OO \
            ##OO."));

        assert!(game.make_move(Stone::Black, 1, 3));
        assert_eq!(game.board, Board::from_str("\
            #.#OO \
            .##OO \
            ##OO. \
            ###OO \
            ##OO."));
    }
}
