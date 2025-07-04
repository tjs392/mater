// zobrist.rs handles zobrist hashing
// using zobrist hashing to track and hash board positions
// https://www.chessprogramming.org/Zobrist_Hashing

use crate::bitboards;
use crate::piece;
use crate::side;
use crate::Board;

use rand::Rng;
use once_cell::sync::Lazy;
use piece::{PIECE_TYPES};
use side::{SIDES, Side};

pub const NUM_SQUARES: usize = 64;
pub const NUM_CASTLING: usize = 16;
pub const NUM_EN_PASSANT: usize = 8;

pub struct ZobristRandoms {
    pub pieces: [[[u64; NUM_SQUARES]; PIECE_TYPES]; SIDES],
    pub castling: [u64; NUM_CASTLING],
    pub sides: [u64; SIDES],
    pub en_passant: [u64; NUM_EN_PASSANT],
}

// keys required:
// 64 sq * 6 piece types * 2 sides = 768 piece/side/square keys
// 16 keys for castling positions
// 0000 = no castling, 0001 = white castle kingside
// 0010 = white castle queenside, etc.
// 2 keys for white/black to move
// 8 legal en passant keys (8 files, keep track of current side)
// this makes 803 keys total for positions representation

pub static ZOBRIST_RANDOMS: Lazy<ZobristRandoms> = Lazy::new(|| {
    let mut rng = rand::thread_rng();

    let mut pieces = [[[0u64; NUM_SQUARES]; PIECE_TYPES]; SIDES];
    for side in 0..SIDES {
        for piece_type in 0..PIECE_TYPES {
            for sq in 0..NUM_SQUARES {
                pieces[side][piece_type][sq] = rng.r#gen::<u64>();
            }
        }
    }

    let mut castling = [0u64; NUM_CASTLING];
    for i in 0..NUM_CASTLING {
        castling[i] = rng.r#gen::<u64>();
    }

    let mut sides = [0u64; SIDES];
    for i in 0..SIDES {
        sides[i] = rng.r#gen::<u64>();
    }

    let mut en_passant = [0u64; NUM_EN_PASSANT];
    for i in 0..NUM_EN_PASSANT {
        en_passant[i] = rng.r#gen::<u64>();
    }

    ZobristRandoms {
        pieces,
        castling,
        sides,
        en_passant,
    }
});

pub type ZobristKey = u64;