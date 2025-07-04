use crate::bitboards;
use crate::piece::{PIECE_TYPES};
use crate::side::{SIDES, Side};
use crate::zobrist::{ZobristKey, ZOBRIST_RANDOMS};
use crate::Board;
use crate::eval;
use crate::Piece;

pub struct Game {
    pub game_state: GameState,
}

pub struct GameState {
    // Board
    pub board: Board, 

    // Bit:
    // 0 = White king side
    // 1 = White queen side
    // 2 = Black king side
    // 3 = Black queen side
    // IE: 0b1111 = all rights, 0b0000 = no more castling
    pub castling_rights: u8,

    // Side to move, black or white
    pub side_to_move: Side,

    // each bit represents a file that can en passant
    // updated each half move
    pub en_passant_file: Option<u8>,

    // zobrist key to represent the current game state
    pub zobrist_key: ZobristKey,

    // # half moves played since pawn capture 
    pub half_move_counter: u8,

    // # full moves played
    pub full_move_count: u8,

    // material value of each side
    pub material: [i16; SIDES],
}

impl GameState {
    pub fn new() -> Self {
        let board = Board::new();
        let material = eval::count_material(&board);

        let mut game_state = GameState {
            board,
            side_to_move: Side::White,
            castling_rights: 0b1111,
            en_passant_file: None,
            zobrist_key: 0,
            half_move_counter: 0,
            full_move_count: 0,
            material: material,
        };

        let zob_key = game_state.init_zobrist_key();

        GameState {
            zobrist_key: zob_key,
            ..game_state
        }
    }
    
    pub fn init_zobrist_key(&mut self) -> ZobristKey {
        let zob = &*ZOBRIST_RANDOMS;
        let mut key: u64 = 0;

        // do piece randoms
        for side in 0..SIDES {
            for piece_type in 0..PIECE_TYPES {
                let mut bb_for_piece = self.board.piece_bbs[side][piece_type];

                while bb_for_piece != 0 {
                    let sq = bitboards::next_bit(&mut bb_for_piece);
                    key ^= zob.pieces[side][piece_type][sq];
                }
            }
        }

        // castling and side randoms
        key ^= zob.castling[self.castling_rights as usize];
        key ^= zob.sides[self.side_to_move as usize];

        if let Some(file) = self.en_passant_file {
            key ^= zob.en_passant[file as usize];
        }

        key
    }

    // remove material (remember material is an integer of piece scores summed)
    pub fn remove_material(&mut self, side: Side, piece: Piece) {
        self.material[side as usize] -= eval::PIECE_VALUES[piece as usize];
    }

    // add material (remember material is an integer of piece scores summed)
    pub fn add_material(&mut self, side: Side, piece: Piece) {
        self.material[side as usize] += eval::PIECE_VALUES[piece as usize];
    }




}