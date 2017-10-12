#![allow(dead_code)]
use std::collections::HashSet;
use std::fmt;
use std::ops::{Index, IndexMut};

const DEFAULT_BOARD_SIZE: usize = 19;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Stone {
    Empty,
    Black,
    White,
}

impl Stone {
    fn char(&self) -> char {
        match *self {
            Stone::Empty => '⋅', // U+22C5 DOT OPERATOR
            Stone::Black => '●', // U+25CF BLACK CIRCLE
            Stone::White => '○', // U+25CB WHITE CIRCLE
        }
    }

    fn not(&self) -> Stone {
        match *self {
            Stone::Empty => Stone::Empty,
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
        }
    }
}

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.char())
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    state: Vec<Stone>,
    pub size: usize,
}

impl Board {
    /// Creates a new 19x19 `Board`.
    pub fn new() -> Board {
        Self::with_size(DEFAULT_BOARD_SIZE)
    }

    /// Creates a new `Board` with a given `size`.
    pub fn with_size(size: usize) -> Board {
        Board {
            state: vec![Stone::Empty; size * size],
            size,
        }
    }

    /// Creates a new `Board` from a string representation of the board state.
    pub fn from_str(board: &str) -> Board {
        let state: Vec<_> = board.chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| match c {
                'B' | 'X' | 'x' | '#' => Stone::Black,
                'W' | 'O' | 'o' | '0' => Stone::White,
                _ => Stone::Empty,
            })
            .collect();

        let size = (state.len() as f64).sqrt() as usize;

        Board { state, size }
    }

    /// Returns true if placing `stone` at `x, y` is a valid play. The ko rule is handled at the
    /// `Game` level, since `Board` doesn't store previous state.
    fn legal_move(&mut self, stone: Stone, x: usize, y: usize) -> bool {
        if stone == Stone::Empty {
            return false;
        } else if self[(x, y)] != Stone::Empty {
            return false;
        } else if (x >= self.size) || (y >= self.size) {
            return false;
        }

        // See if placing stone would cause a capture (perform before self-capture check).
        for (nx, ny) in self.neighbours(x, y) {
            if self[(nx, ny)] == stone.not() {
                let liberties = self.liberties(nx, ny);
                if liberties.len() == 1 && *liberties.iter().next().unwrap() == (x, y) {
                    return true;
                }
            }
        }

        // Prevent self-capture by simulating placing a stone at `(x, y)` and checking liberties.
        self[(x, y)] = stone;
        let liberties = self.liberties(x, y);
        self[(x, y)] = Stone::Empty;

        if liberties.len() == 0 {
            return false;
        }

        true
    }

    /// Places `stone` at `(x, y)`, returning true if it was successful. Handles captures.
    pub fn make_move(&mut self, stone: Stone, x: usize, y: usize) -> bool {
        if self[(x, y)] != Stone::Empty {
            return false;
        } else if stone == Stone::Empty {
            return false;
        } else if !self.legal_move(stone, x, y) {
            return false;
        }

        let opposing_stone = stone.not();

        for (nx, ny) in self.neighbours(x, y) {
            if self[(nx, ny)] == opposing_stone {
                // If the chain that `(nx, ny)` is a part of only has a single liberty at point
                // `(x, y)`, then the entire chain will be captured by making this move.
                let liberties = self.liberties(nx, ny);

                if liberties.len() == 1 && *liberties.iter().next().unwrap() == (x, y) {
                    for (cx, cy) in self.chain_at(nx, ny) {
                        self[(cx, cy)] = Stone::Empty;
                    }
                }
            }
        }

        // Finally, place the stone at `(x, y)`.
        self[(x, y)] = stone;

        true
    }

    /// Returns the positions adjacent to `(x, y)`.
    fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();

        if x > 0 { positions.push((x - 1, y)) }
        if y > 0 { positions.push((x, y - 1)) }
        if x < (self.size - 1) { positions.push((x + 1, y)) }
        if y < (self.size - 1) { positions.push((x, y + 1)) }

        positions
    }

    /// Returns the set of all positions connected to the stone at `(x, y)`.
    fn chain_at(&self, x: usize, y: usize) -> HashSet<(usize, usize)> {
        let mut seen = HashSet::new();
        let stone = self[(x, y)];

        if stone == Stone::Empty {
            return seen;
        }

        seen.insert((x, y));

        // Perform depth-first search starting from `(x, y)`
        let mut horizon: Vec<_> = self.neighbours(x, y).into_iter()
            .filter(|&(x, y)| self[(x, y)] == stone)
            .collect();

        while horizon.len() > 0 {
            let (nx, ny) = horizon.pop().expect("horizon is empty");
            seen.insert((nx, ny));

            for (a, b) in self.neighbours(nx, ny) {
                if self[(a, b)] == stone && !seen.contains(&(a, b)) {
                    horizon.push((a, b));
                }
            }
        }

        seen
    }

    /// Returns the set of liberties of the stone at `(x, y)`.
    fn liberties(&self, x: usize, y: usize) -> HashSet<(usize, usize)> {
        let mut liberties = HashSet::new();
        let mut seen = HashSet::new();
        let stone = self[(x, y)];

        if stone == Stone::Empty {
            return liberties;
        }

        seen.insert((x, y));

        // Perform depth-first search starting from `(x, y)`
        let mut horizon: Vec<_> = self.neighbours(x, y).into_iter().collect();

        while horizon.len() > 0 {
            let (nx, ny) = horizon.pop().expect("horizon is empty");
            seen.insert((nx, ny));

            if self[(nx, ny)] == Stone::Empty {
                liberties.insert((nx, ny));
            }

            if self[(nx, ny)] == stone {
                for (a, b) in self.neighbours(nx, ny) {
                    if !seen.contains(&(a, b)) {
                        horizon.push((a, b));
                    }
                }
            }
        }

        liberties
    }

    /// Returns true if (x, y) is a star point (hoshi) based on the current board size.
    fn star_point(&self, x: usize, y: usize) -> bool {
        match self.size {
            9 => (x == 4 && y == 4) || ((x == 2 || x == 6) && (y == 2 || y == 6)),
            13 => (x == 6 && y == 6) || ((x == 3 || x == 9) && (y == 3 || y == 9)),
            19 => (x == 3 || x == 9 || x == 15) && (y == 3 || y == 9 || y == 15),
            _ => false,
        }
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Stone;

    fn index<'a>(&'a self, index: (usize, usize)) -> &'a Stone {
        let (x, y) = index;
        &self.state[y * self.size + x]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut<'a>(&'a mut self, index: (usize, usize)) -> &'a mut Stone {
        let (x, y) = index;
        &mut self.state[y * self.size + x]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rows = Vec::new();

        for y in 0..self.size {
            let mut row = String::new();

            for x in 0..self.size {
                match self.state[y * self.size + x] {
                    Stone::Empty => {
                        if self.star_point(x, y) {
                            row.push('•'); // U+2022 BULLET
                        } else {
                            row.push(Stone::Empty.char());
                        }
                    }
                    stone => row.push(stone.char()),
                }
                row.push(' ');
            }

            row.pop(); // remove trailing space from row
            rows.push(row);
        }

        write!(f, "{}", rows.join("\n"))
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    use super::{Board, Stone};

    #[test]
    fn empty_board() {
        let board = Board::new();

        for y in 0..board.size {
            for x in 0..board.size {
                assert_eq!(board[(x, y)], Stone::Empty);
            }
        }
    }

    #[test]
    fn board_from_str() {
        let board = Board::from_str("#O#O.O#O#");

        assert_eq!(board.size, 3);
        assert_eq!(board[(0, 0)], Stone::Black);
        assert_eq!(board[(1, 0)], Stone::White);
        assert_eq!(board[(1, 1)], Stone::Empty);
    }

    #[test]
    fn board_equality() {
        let board_1 = Board::from_str("#O#O.O#O#");
        let board_2 = Board::from_str("\
            BWB \
            W.W \
            BWB");

        assert_eq!(board_1, board_2);
    }

    #[test]
    fn access_board() {
        let mut board = Board::new();

        board[(0, 0)] = Stone::Black;
        board[(1, 1)] = Stone::White;

        assert_eq!(board[(0, 0)], Stone::Black);
        assert_eq!(board[(1, 1)], Stone::White);
    }

    #[test]
    #[should_panic]
    fn access_invalid_position() {
        let board = Board::new();
        board[(20, 20)];
    }

    #[test]
    fn position_neighbours() {
        let board = Board::with_size(3);
        assert_eq!(board.neighbours(0, 0).len(), 2);
        assert_eq!(board.neighbours(0, 1).len(), 3);
        assert_eq!(board.neighbours(1, 1).len(), 4);
        assert_eq!(board.neighbours(2, 2).len(), 2);
    }

    #[test]
    fn connected_stones() {
        let board = Board::from_str("\
            .#..# \
            ##.O# \
            ..O.. \
            O.O#O \
            .#OO.");

        assert_eq!(board.chain_at(0, 0).len(), 0);

        let chain_1 = board.chain_at(0, 1);
        assert_eq!(chain_1.len(), 3);
        assert_eq!(chain_1, board.chain_at(1, 1));
        assert_eq!(chain_1, board.chain_at(1, 0));

        let chain_2 = board.chain_at(2, 2);
        assert_eq!(chain_2.len(), 4);
        assert_eq!(chain_2, board.chain_at(2, 3));
        assert_eq!(chain_2, board.chain_at(2, 4));
        assert_eq!(chain_2, board.chain_at(3, 4));
    }

    #[test]
    fn liberties() {
        let board = Board::from_str("\
            ...O. \
            ..### \
            O#.O. \
            OO### \
            .O.O#");

        let chain_1 = board.liberties(0, 3);
        let expected_1 = HashSet::from_iter(vec![(0, 1), (0, 4), (2, 4)]);
        assert_eq!(chain_1, board.liberties(1, 3));
        assert_eq!(chain_1, expected_1);

        let chain_2 = board.liberties(2, 3);
        let expected_2 = HashSet::from_iter(vec![(2, 2), (2, 4), (4, 2)]);
        assert_eq!(chain_2, board.liberties(4, 4));
        assert_eq!(chain_2, expected_2)
    }

    #[test]
    fn no_capture() {
        let mut board = Board::from_str("\
            .!. \
            #.# \
            .#.");

        let expected = Board::from_str("\
            .#. \
            #.# \
            .#.");

        assert!(board.make_move(Stone::Black, 1, 0));
        assert_eq!(board, expected);
    }

    #[test]
    fn invalid_move() {
        let mut board = Board::from_str("\
            ... \
            .B. \
            ...");

        assert!(!board.make_move(Stone::Black, 1, 1));
        assert!(!board.make_move(Stone::White, 1, 1));
    }

    #[test]
    fn simple_capture() {
        let mut board = Board::from_str("\
            .!. \
            #O# \
            .#.");

        let expected = Board::from_str("\
            .#. \
            #.# \
            .#.");

        assert!(board.make_move(Stone::Black, 1, 0));
        assert_eq!(board, expected);
    }

    #[test]
    fn capture_against_corner() {
        let mut board = Board::from_str("\
            OO#.. \
            O#... \
            !.... \
            ..... \
            .....");

        let expected = Board::from_str("\
            ..#.. \
            .#... \
            #.... \
            ..... \
            .....");

        assert!(board.make_move(Stone::Black, 0, 2));
        assert_eq!(board, expected);
    }

    #[test]
    fn another_capture() {
        let mut board = Board::from_str("\
            ...#. \
            #OO#O \
            !##O. \
            #OO.. \
            .....");

        let expected = Board::from_str("\
            ...#. \
            #OO#O \
            O..O. \
            #OO.. \
            .....");

        assert!(board.make_move(Stone::White, 0, 2));
        assert_eq!(board, expected);
    }

    #[test]
    fn double_capture() {
        let mut board = Board::from_str("\
            .#OOO \
            ..!O# \
            .#O#. \
            .O##. \
            .....");

        let expected = Board::from_str("\
            .#... \
            ..#.# \
            .#.#. \
            .O##. \
            .....");

        assert!(board.make_move(Stone::Black, 2, 1));
        assert_eq!(board, expected);
    }

    #[test]
    fn prevent_self_capture() {
        let mut board = Board::from_str("\
            .#. \
            #!# \
            .#.");

        let expected = board.clone();

        assert!(!board.make_move(Stone::White, 1, 1));
        assert_eq!(board, expected);
    }

    #[test]
    fn prevent_complex_self_capture() {
        let mut board = Board::from_str("\
            ..O.. \
            ..#.. \
            .#OOO \
            .O#!O \
            #.O##");

        let expected = board.clone();

        assert!(!board.make_move(Stone::Black, 3, 3));
        assert_eq!(board, expected);
    }

    #[test]
    fn allow_temporary_self_capture() {
        let mut board = Board::from_str("\
            ###O. \
            #!#O. \
            ###O. \
            OOO.. \
            .....");

        let expected = Board::from_str("\
            ...O. \
            .O.O. \
            ...O. \
            OOO.. \
            .....");

        assert!(board.make_move(Stone::White, 1, 1));
        assert_eq!(board, expected);
    }
}
