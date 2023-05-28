use rand::Rng;

const BOARD_SIZE: usize = 64;  // 8x8 board.
const PIECE_TYPES: usize = 12;  // 12 piece types (e.g., Black and White + 6 from each).

pub struct ZobristHash {
    zobrist_table: [[u64; BOARD_SIZE]; PIECE_TYPES],
}

impl ZobristHash {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut zobrist_table = [[0; BOARD_SIZE]; PIECE_TYPES];

        for i in 0..PIECE_TYPES {
            for j in 0..BOARD_SIZE {
                zobrist_table[i][j] = rng.gen::<u64>();
            }
        }

        Self {
            zobrist_table
        }
    }

    pub fn hash(&self, board: &[usize; BOARD_SIZE]) -> u64 {
        let mut h = 0;

        for i in 0..PIECE_TYPES {
            for j in 0..BOARD_SIZE {
                if board[j] == i {
                    h ^= self.zobrist_table[i][j];
                }
            }
        }

        h
    }
}
