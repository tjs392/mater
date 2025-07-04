// piece.rs has Piece enums and util functions
use crate::board::Bitboard;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum Piece {
    King = 0,
    Queen = 1,
    Rook = 2,
    Bishop = 3,
    Knight = 4,
    Pawn = 5,
    None = 6,
}

pub const PIECE_TYPES: usize = 7;

impl From<usize> for Piece {
    fn from(val: usize) -> Self {
        match val {
            0 => Piece::King,
            1 => Piece::Queen,
            2 => Piece::Rook,
            3 => Piece::Bishop,
            4 => Piece::Knight,
            5 => Piece::Pawn,
            _ => Piece::None,
        }
    }
}

struct SlidingPiece {
    deltas: [(i8, i8); 4],
}

impl SlidingPiece {
    pub fn get_relevant_blockers(&self, sq: u64) -> Bitboard {
        let mut blockers: Bitboard = 0u64;

        blockers
    }
}

const ROOK: SlidingPiece = SlidingPiece {
    deltas: [(1, 0), (0, -1), (-1, 0), (0, 1)],
};

const BISHOP: SlidingPiece = SlidingPiece {
    deltas: [(1, 1), (1, -1), (-1, -1), (-1, 1)],
};
