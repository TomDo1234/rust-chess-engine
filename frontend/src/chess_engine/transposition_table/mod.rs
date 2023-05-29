use rand::Rng;

use super::{Piece, BLACK_KING, BLACK_QUEEN, BLACK_BISHOP, BLACK_ROOK, BLACK_KNIGHT, BLACK_PAWN, WHITE_KING, WHITE_BISHOP, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK};

const BOARD_SIZE: usize = 64;  // 8x8 board.
const PIECES: [Piece; 12] = [BLACK_KING,BLACK_QUEEN,BLACK_BISHOP,BLACK_ROOK,BLACK_KNIGHT,BLACK_PAWN,WHITE_KING,WHITE_BISHOP,WHITE_KNIGHT,WHITE_PAWN,WHITE_QUEEN,WHITE_ROOK];

pub struct ZobristHash {
    zobrist_table: [[u64; BOARD_SIZE]; PIECES.len()],
    other_data_table: [u64; 9]
}

impl ZobristHash {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut zobrist_table = [[0; BOARD_SIZE]; PIECES.len()];

        for i in 0..PIECES.len() {
            for j in 0..BOARD_SIZE {
                zobrist_table[i][j] = rng.gen::<u64>();
            }
        }

        let mut other_data_table = [0; 9];
        for i in 0..9 {
            other_data_table[i] = rng.gen::<u64>();
        }

        Self {
            zobrist_table,
            other_data_table
        }
    }

    pub fn hash(&self, board: &[Option<Piece>; BOARD_SIZE],current_recursion: u8) -> u64 {
        let mut h = 0;

        for i in 0..PIECES.len() {
            let checked_piece = PIECES[i];
            for j in 0..BOARD_SIZE {
                if let Some(piece) = board[j] {
                    if piece.color == checked_piece.color && piece.piece_type == checked_piece.piece_type {
                        h ^= self.zobrist_table[i][j];
                    }
                }
            }
        }
        h ^= self.other_data_table[current_recursion as usize - 1];
        
        h
    }
}
