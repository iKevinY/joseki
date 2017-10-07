use std::fmt;
use std::ops::{Index, IndexMut};

const BOARD_SIZE: usize = 19;

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
}

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.char())
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    state: Vec<Stone>,
    size: usize,
}

impl Board {
    pub fn new() -> Board {
        Board {
            state: vec![Stone::Empty; BOARD_SIZE * BOARD_SIZE],
            size: BOARD_SIZE,
        }
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
    use super::{Board, Stone, BOARD_SIZE};

    #[test]
    fn empty_board() {
        let board = Board::new();

        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                assert_eq!(board[(x, y)], Stone::Empty);
            }
        }
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
    fn invalid_position() {
        let board = Board::new();
        board[(20, 20)];
    }
}
