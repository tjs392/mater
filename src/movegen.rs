// movegen.rs handle move generation and related functions
use once_cell::sync::Lazy;
use crate::util;
use crate::board::{Bitboard, Board};
use crate::bitboards;

pub static KNIGHT_ATTACKS: Lazy<[Bitboard; 64]> = Lazy::new(|| generate_knight_attacks());

// knight move generation
// precompute possible moves per square
pub fn generate_knight_attacks() -> [Bitboard; 64] {
    let mut attacks = [0u64; 64];

    const KNIGHT_MOVES: [(i8, i8); 8] = [
        (2,1), (1,2), (-1,2), (-2,1), (-2,-1), (-1,-2), (1,-2), (2,-1),
    ];

    for sq in 0..64 {
        let rank = (sq / 8) as i8;
        let file = (sq % 8) as i8;

        let mut attack_mask = 0u64;

        for &(dr, df) in &KNIGHT_MOVES {
            let r = rank + dr;
            let f = file + df;

            if (0..8).contains(&r) && (0..8).contains(&f) {
                let dest = (r * 8 + f) as usize;
                attack_mask |= 1u64 << dest;
            }
        }

        util::print_bitboard(attack_mask);
        println!();
        attacks[sq] = attack_mask;
    }
    attacks
}

// use as: 
// let mut knights = board.piece_bbs[Side::White as usize][Piece::Knight as usize];
// let knight_moves = generate_knight_moves(knights);
pub fn generate_knight_moves(knights: Bitboard, own_pieces: Bitboard) -> Bitboard {
    let mut moves = 0;
    let mut knights_c = knights;

    while knights_c != 0 {
        let from = bitboards::next_bit(&mut knights_c);
        let attacks = KNIGHT_ATTACKS[from];

        moves |= attacks & !own_pieces;
    }

    moves
}