use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use regex::Regex;

use board::{Board, Stone};

#[derive(Clone, Debug, Default, PartialEq)]
struct Player {
    name: Option<String>,
    rank: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Game {
    board: Board,
    last_board: Option<Board>,
    black: Player,
    white: Player,
}

impl Game {
    /// Creates a new game with an empty board state.
    pub fn new() -> Game {
        Game { ..Default::default() }
    }

    /// Creates a new game from a string representation of the board state.
    pub fn from_str(board: &str) -> Game {
        Game {
            board: Board::from_str(board),
            ..Default::default()
        }
    }

    /// Creates a game from a given SGF file.
    pub fn from_sgf<P: AsRef<Path>>(path: P) -> Game {
        let mut f = File::open(path).expect("invalid path");
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();

        let mut game = Game::new();

        // Enum containing various SGF properties
        enum SGF {
            AddStone(Stone),
            Move(Stone),
            PlayerName(Stone),
            PlayerRank(Stone),
            Unknown,
        }

        // TODO: Write an actual SGF parser instead of naively using regexes
        let re = Regex::new(r"(\w{1,2})\[(.+?)\]").expect("invalid regex");

        // Parse captured regex matches into SGF properties
        let properties = re.captures_iter(&contents).map(|cap| {
            let property = match &cap[1] {
                "B"  => SGF::Move(Stone::Black),
                "W"  => SGF::Move(Stone::White),
                "AB" => SGF::AddStone(Stone::Black),
                "AW" => SGF::AddStone(Stone::White),
                "PB" => SGF::PlayerName(Stone::Black),
                "PW" => SGF::PlayerName(Stone::White),
                "BR" => SGF::PlayerRank(Stone::Black),
                "WR" => SGF::PlayerRank(Stone::White),
                _    => SGF::Unknown,
            };

            (property, cap[2].to_string())
        });

        for (prop, val) in properties {
            match prop {
                SGF::Move(stone) => {
                    // Use `Game::make_move` to take into account captures.
                    let (x, y) = Self::alpha_to_xy(&val);
                    game.make_move(stone, x, y);
                },
                SGF::AddStone(stone) => {
                    // Manually assign stone to position.
                    game.board[Self::alpha_to_xy(&val)] = stone;
                },
                SGF::PlayerName(stone) => {
                    if stone == Stone::Black {
                        game.black.name = Some(val);
                    } else {
                        game.white.name = Some(val);
                    }
                },
                SGF::PlayerRank(stone) => {
                    if stone == Stone::Black {
                        game.black.rank = Some(val);
                    } else {
                        game.white.rank = Some(val);
                    }
                }
                _ => {},
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
            if *b == next_board {
                return false;
            }
        }

        self.last_board = Some(self.board.clone());
        self.board = next_board;

        true
    }

    /// Maps "alphabetical coordinates" to `(x, y)` coordinates.
    /// Ex. "ab" => (0, 1); "zz" => (25, 25)
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

        let black_player = self.black.name.as_ref().unwrap_or(&unknown);
        let white_player = self.white.name.as_ref().unwrap_or(&unknown);

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
        assert_eq!(game.black.name, None);
        assert_eq!(game.white.name, None);
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
