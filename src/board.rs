// board.rs handles Board implementation

use crate::bitboards;
use crate::piece::{Piece, PIECE_TYPES};
use crate::side::{Side, SIDES};

pub type Bitboard = u64;

pub struct Board {
    //  12 Bitboards representing pieces of each side
    //  [ ~WK, ~WQ, ~WR, ~WB, ~WN, ~WP, ],
    //  [ ~BK, ~BQ, ~BR, ~BB, ~BN, ~BP, ],
    pub piece_bbs: [[Bitboard; PIECE_TYPES]; SIDES],

    // 2 Bitboards representing all pieces on each side
    // [ ~W, ~B, ]
    pub side_bbs: [Bitboard; SIDES],
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            piece_bbs: [[0; PIECE_TYPES]; SIDES],
            side_bbs: [0; SIDES],
        };
        board.init_starting_positions();
        board
    }
    
    fn init_starting_positions(&mut self) {
        let white_back_rank = [
            (Piece::Rook, 0),
            (Piece::Knight, 1),
            (Piece::Bishop, 2),
            (Piece::Queen, 3),
            (Piece::King, 4),
            (Piece::Bishop, 5),
            (Piece::Knight, 6),
            (Piece::Rook, 7),
        ];

        for (piece, sq) in white_back_rank {
            bitboards::set_bit(&mut self.piece_bbs[Side::White as usize][piece as usize], sq);
            bitboards::set_bit(&mut self.side_bbs[Side::White as usize], sq);
        }

        for sq in 8..16 {
            bitboards::set_bit(&mut self.piece_bbs[Side::White as usize][Piece::Pawn as usize], sq);
            bitboards::set_bit(&mut self.side_bbs[Side::White as usize], sq);
        }

        let black_back_rank = [
            (Piece::Rook, 63),
            (Piece::Knight, 62),
            (Piece::Bishop, 61),
            (Piece::King, 60),
            (Piece::Queen, 59),
            (Piece::Bishop, 58),
            (Piece::Knight, 57),
            (Piece::Rook, 56),
        ];

        for (piece, sq) in black_back_rank {
            bitboards::set_bit(&mut self.piece_bbs[Side::Black as usize][piece as usize], sq);
            bitboards::set_bit(&mut self.side_bbs[Side::Black as usize], sq);
        }

        for sq in 48..56 {
            bitboards::set_bit(&mut self.piece_bbs[Side::Black as usize][Piece::Pawn as usize], sq);
            bitboards::set_bit(&mut self.side_bbs[Side::Black as usize], sq);
        }
    }

    // piece_list is an array of size 64
    // each square on the board is represented
    // piece side does not matter, since piece_list will be used
    // to track captures, and the side is always the opposite of the capture
    pub fn init_piece_list(&self) -> [Piece; 64] {
        let bb_w = self.piece_bbs[Side::White as usize];
        let bb_b = self.piece_bbs[Side::Black as usize];

        let mut piece_list = [Piece::None; 64];

        for (piece_type, (w, b)) in bb_w.iter().zip(bb_b.iter()).enumerate() {
            let mut white_pieces = *w;
            let mut black_pieces = *b;

            while white_pieces > 0 {
                let sq = bitboards::next_bit(&mut white_pieces);
                piece_list[sq] = Piece::from(piece_type);
            }

            while black_pieces > 0 {
                let sq = bitboards::next_bit(&mut black_pieces);
                piece_list[sq] = Piece::from(piece_type);
            }
        }
        piece_list
    }
}