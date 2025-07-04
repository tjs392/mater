mod bitboards;
mod board;
mod piece;
mod side;
mod util;
mod zobrist;
mod game;
mod eval;

use board::Board;
use piece::{Piece, PIECE_TYPES};
use side::{Side, SIDES};
use util::{print_board, print_bitboard};
use zobrist::{ZOBRIST_RANDOMS};
use crate::game::{GameState};
use crate::eval::evaluate_position;

fn main() {
    // Initialize GameState with starting position
    let mut game_state = GameState::new();
    let board = &game_state.board;

    println!("===== INITIAL BOARD STATE =====");
    print_board(board);

    game_state.add_material(Side::White, Piece::Bishop);

    println!();
    println!("==== Board evaluation ====");
    println!("{:#?}", eval::evaluate_position(&game_state));

    // println!("\n===== PIECE BITBOARDS =====");
    // for side in 0..SIDES {
    //     for piece in 0..PIECE_TYPES {
    //         let bb = board.piece_bbs[side][piece];
    //         if bb != 0 {
    //             println!("{:?} {:?}:", Side::from(side), Piece::from(piece));
    //             print_bitboard(bb);
    //             println!();
    //         }
    //     }
    // }

    // println!("===== SIDE BITBOARDS =====");
    // for side in 0..SIDES {
    //     println!("{:?} side bitboard:", Side::from(side));
    //     print_bitboard(board.side_bbs[side]);
    //     println!();
    // }

    // println!("===== PIECE LIST =====");
    // let piece_list = game_state.board.init_piece_list();
    // for (sq, piece) in piece_list.iter().enumerate() {
    //     if *piece != Piece::None {
    //         println!("Square {}: {:?}", sq, piece);
    //     }
    // }

    // println!("\n===== CASTLING RIGHTS =====");
    // println!("Castling Rights: {:04b}", game_state.castling_rights);

    // println!("\n===== EN PASSANT FILE =====");
    // match game_state.en_passant_file {
    //     Some(file) => println!("En Passant File: {}", file),
    //     None => println!("En Passant File: None"),
    // }

    // println!("\n===== SIDE TO MOVE =====");
    // println!("{:?}", game_state.side_to_move);

    // println!("\n===== ZOBRIST HASH =====");
    // println!("Zobrist Hash: 0x{:016X}", game_state.zobrist_key);

    // println!("\n===== ZOBRIST RANDOMS (Partial Dump) =====");
    // let zob = &*ZOBRIST_RANDOMS;

    // println!("Zobrist Piece Hashes:");
    // for side in 0..SIDES {
    //     for piece in 0..PIECE_TYPES {
    //         for sq in 0..64 {
    //             let key = zob.pieces[side][piece][sq];
    //             println!("side={:?} piece={:?} square={} => 0x{:016X}", Side::from(side), Piece::from(piece), sq, key);
    //         }
    //     }
    // }

    // println!("\nZobrist Castling Rights:");
    // for (i, val) in zob.castling.iter().enumerate() {
    //     println!("rights={:04b} => 0x{:016X}", i, val);
    // }

    // println!("\nZobrist En Passant Files:");
    // for (i, val) in zob.en_passant.iter().enumerate() {
    //     println!("file={} => 0x{:016X}", i, val);
    // }

    // println!("\nZobrist Side to Move:");
    // for (i, val) in zob.sides.iter().enumerate() {
    //     println!("{:?} => 0x{:016X}", Side::from(i), val);
    // }

    // println!("\n===== DEBUG COMPLETE =====");
}
