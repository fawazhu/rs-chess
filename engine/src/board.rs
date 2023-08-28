#[derive(Debug)]
pub struct GridError {
    message: String,
}
impl std::fmt::Display for GridError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}.", self.message)
    }
}
impl std::error::Error for GridError {}

pub trait Grid<T> {
    fn get_unchecked(&self, x: usize, y: usize) -> T;
    fn set_unchecked(&mut self, x: usize, y: usize, value: T);
    fn get(&self, x: usize, y: usize) -> Result<T, GridError>;
    fn set(&mut self, x: usize, y: usize, value: T) -> Option<GridError>;
}

pub type Board = [u64; 4];
pub type BoardItem = u8;
pub struct BoardItems {
    pub none: BoardItem,
    pub white_pawn: BoardItem,
    pub white_knight: BoardItem,
    pub white_bishop: BoardItem,
    pub white_rook: BoardItem,
    pub white_queen: BoardItem,
    pub white_king: BoardItem,
    pub black_pawn: BoardItem,
    pub black_knight: BoardItem,
    pub black_bishop: BoardItem,
    pub black_rook: BoardItem,
    pub black_queen: BoardItem,
    pub black_king: BoardItem,
}
pub const BOARD_ITEMS: BoardItems = BoardItems {
    none: 0,
    white_pawn: 1,
    white_knight: 2,
    white_bishop: 3,
    white_rook: 4,
    white_queen: 5,
    white_king: 6,
    black_pawn: 7,
    black_knight: 8,
    black_bishop: 9,
    black_rook: 10,
    black_queen: 11,
    black_king: 12,
};

pub const BOARD_WIDTH: usize = 8;
pub const BOARD_ITEM_BITS: usize = 4;
pub const BOARD_ITEM_MASK: usize = (1 << BOARD_ITEM_BITS) - 1;

pub struct BoardBuilder;
impl BoardBuilder {
    pub fn build() -> Board {
        return [0 as u64, 0 as u64, 0 as u64, 0 as u64];
    }
}

impl Grid<BoardItem> for Board {
    fn get(&self, x: usize, y: usize) -> Result<BoardItem, GridError> {
        if x >= BOARD_WIDTH || y >= BOARD_WIDTH {
            return Err(GridError {
                message: format!("Coordinates [{}, {}] out of bounds", x, y),
            });
        }
        return Ok(self.get_unchecked(x, y));
    }
    fn set(&mut self, x: usize, y: usize, value: BoardItem) -> Option<GridError> {
        if x >= BOARD_WIDTH || y >= BOARD_WIDTH {
            return Some(GridError {
                message: format!("Coordinates [{}, {}] out of bounds", x, y),
            });
        }
        self.set_unchecked(x, y, value);
        return None;
    }

    fn get_unchecked(&self, x: usize, y: usize) -> BoardItem {
        let position = ((y * BOARD_WIDTH) + x) as u64;
        let board_section = self[(position / 16) as usize];
        let shift_amt = (position % 16) * (BOARD_ITEM_BITS as u64);
        let position = (BOARD_ITEM_MASK as u64) << shift_amt;
        let item = (board_section & position) >> shift_amt;
        return item as BoardItem;
    }
    fn set_unchecked(&mut self, x: usize, y: usize, value: BoardItem) {
        let position = ((y * BOARD_WIDTH) + x) as u64;
        let board_section_idx = (position / 16) as usize;
        let shift_amt = (position % 16) * (BOARD_ITEM_BITS as u64);
        let position = (BOARD_ITEM_MASK as u64) << shift_amt;
        let value = (value as u64) << shift_amt;
        self[board_section_idx] = (self[board_section_idx] & (!position)) ^ value;
    }
}

#[cfg(test)]
mod tests {
    use crate::board::BoardBuilder;

    use super::{Grid, BOARD_ITEMS};

    #[test]
    fn board_get_retrieves_piece_at_given_position() {
        let mut board = BoardBuilder::build();
        board[0] =
            0b0000_0000_0000_0000_0000_0000_0000_0000__1100_0000_0000_0000_0000_0000_0000_0001;
        board[1] =
            0b0000_0000_0000_0000_0000_0000_0000_0000__0000_0000_0000_0000_0000_0001_0000_0000;
        board[2] =
            0b0000_0000_0000_0000_0000_0000_0000_0000__0000_0000_0000_0000_0000_0000_0000_0000;
        board[3] =
            0b1111_0000_0001_0000_0000_0000_0000_0000__0000_0000_0000_0000_0000_0000_0000_0000;
        assert_eq!(board.get(0, 0).unwrap(), 0b0001);
        assert_eq!(board.get(7, 0).unwrap(), 0b1100);
        assert_eq!(board.get(2, 2).unwrap(), 0b0001);
        assert_eq!(board.get(5, 7).unwrap(), 0b0001);
        assert_eq!(board.get(6, 7).unwrap(), 0b0000);
        assert_eq!(board.get(7, 7).unwrap(), 0b1111);
    }

    #[test]
    fn board_set_places_piece_at_given_position() {
        let mut board = BoardBuilder::build();
        board.set(0, 0, BOARD_ITEMS.white_pawn);
        assert_eq!(board.get(0, 0).unwrap(), BOARD_ITEMS.white_pawn);
        board.set(0, 0, BOARD_ITEMS.black_bishop);
        assert_eq!(board.get(0, 0).unwrap(), BOARD_ITEMS.black_bishop);
        board.set(0, 0, BOARD_ITEMS.none);
        assert_eq!(board.get(0, 0).unwrap(), BOARD_ITEMS.none);
        board.set(3, 3, BOARD_ITEMS.white_queen);
        assert_eq!(board.get(3, 3).unwrap(), BOARD_ITEMS.white_queen);
        board.set(7, 7, BOARD_ITEMS.black_king);
        assert_eq!(board.get(7, 7).unwrap(), BOARD_ITEMS.black_king);
    }

    #[test]
    fn board_get_errors_when_out_of_range() {
        let board = BoardBuilder::build();
        assert!(board.get(8, 0).is_err());
        assert!(board.get(0, 8).is_err());
        assert!(board.get(25, 23).is_err());
    }

    #[test]
    fn board_set_errors_when_out_of_range() {
        let mut board = BoardBuilder::build();
        assert!(board.set(8, 0, BOARD_ITEMS.white_pawn).is_some());
        assert!(board.set(0, 8, BOARD_ITEMS.white_pawn).is_some());
        assert!(board.set(25, 23, BOARD_ITEMS.white_pawn).is_some());
    }
}
