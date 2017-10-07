use std::ops::{Index, IndexMut};

const BOARD_SIZE: usize = 19;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Stone {
    Empty,
    Black,
    White,
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


#[cfg(test)]
mod tests {
    use super::{Board, Stone, BOARD_SIZE};

    #[test]
    fn empty_board() {
        let board = Board::new();

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
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
