// util.rs handles all utility functions

use crate::Board;
use crate::bitboards;
use crate::piece::{Piece, PIECE_TYPES};
use crate::side::{Side, SIDES};

pub fn print_bitboard(bitboard: u64) {
    const LAST_BIT: u64 = 63;
    for rank in 0..8 {
        for file in (0..8).rev() {
            let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
            let char = if bitboard & mask != 0 { '1' } else { '0' };
            print!("{char} ");
        }
        println!();
    }
}

pub fn print_board(board: &Board) {
    for rank in (0..8).rev() {
        print!("{} ", rank + 1);
        for file in 0..8 {
            let sq = rank * 8 + file;
            let mut found = false;

            for side in 0..SIDES {
                for piece in 0..PIECE_TYPES {
                    if bitboards::get_bit(&board.piece_bbs[side][piece], sq) {
                        let piece_char = match (Side::from(side), Piece::from(piece)) {
                            (Side::White, Piece::King) => 'K',
                            (Side::White, Piece::Queen) => 'Q',
                            (Side::White, Piece::Rook) => 'R',
                            (Side::White, Piece::Knight) => 'N',
                            (Side::White, Piece::Bishop) => 'B',
                            (Side::White, Piece::Pawn) => 'P',
                            (Side::Black, Piece::King) => 'k',
                            (Side::Black, Piece::Queen) => 'q',
                            (Side::Black, Piece::Rook) => 'r',
                            (Side::Black, Piece::Knight) => 'n',
                            (Side::Black, Piece::Bishop) => 'b',
                            (Side::Black, Piece::Pawn) => 'p',
                            _ => '.',
                        };
                        print!("{} ", piece_char);
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }

            if !found {
                print!(". ");
            }
        }
        println!();
    }
    println!("  A B C D E F G H")
}