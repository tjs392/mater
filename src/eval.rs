// eval.rs handles board and material evaluation
use crate::piece::{Piece, PIECE_TYPES};
use crate::board::{Board};
use crate::side::{Side, SIDES};
use crate::bitboards;
use crate::game::GameState;

pub const PIECE_VALUES: [i16; 7] = [
    0,      // king
    900,    // queen
    500,    // rook
    300,    // bishop
    300,    // knight
    100,    // pawn
    0,      // none
];

pub fn count_material(board: &Board) -> [i16; SIDES] {
    let mut white_material: i16 = 0;
    let mut black_material: i16 = 0;

    let white_bitboard = board.piece_bbs[Side::White as usize];
    let black_bitboard = board.piece_bbs[Side::Black as usize];

    for (piece_type, (w, b)) in white_bitboard.iter().zip(black_bitboard.iter()).enumerate() {
        let mut white_pieces = *w;
        let mut black_pieces = *b;

        while white_pieces > 0 {
            white_material += PIECE_VALUES[piece_type];
            bitboards::next_bit(&mut white_pieces);
        }

        while black_pieces > 0 {
            black_material += PIECE_VALUES[piece_type];
            bitboards::next_bit(&mut black_pieces);
        }
    }

    [white_material, black_material]
}

// static eval for now, just counting pieces with some scores
pub fn evaluate_position(game_state: &GameState) -> i16 {
    let material = &game_state.material;
    material[Side::White as usize] - material[Side::Black as usize]
}