use core::fmt;
use std::{error::Error,ptr, collections::HashMap};

use self::transposition_table::ZobristHash;
pub mod transposition_table;

#[derive(Clone,PartialEq,Debug,Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone,PartialEq,Debug,Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Clone,PartialEq,Debug,Copy)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
    value: u8, 
    has_moved: bool
}

#[derive(Debug)]
pub struct ChessEngineError {
    message: String
}

impl fmt::Display for ChessEngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chess Engine Error: {}", self.message)
    }
}

impl Error for ChessEngineError {}



const WHITE_PAWN: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Pawn,
    value: 1,
    has_moved: false
};

const WHITE_KNIGHT: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Knight,
    value: 3,
    has_moved: false
};

const WHITE_BISHOP: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Bishop,
    value: 3,
    has_moved: false
};

const WHITE_ROOK: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Rook,
    value: 5,
    has_moved: false
};

const WHITE_QUEEN: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Queen,
    value: 9,
    has_moved: false
};

const WHITE_KING: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::King,
    value: 255,
    has_moved: false
};

const BLACK_PAWN: Piece = Piece {
    color: Color::Black,
    piece_type: PieceType::Pawn,
    value: 1,
    has_moved: false
};

const BLACK_KNIGHT: Piece = Piece {
    color: Color::Black,
    piece_type: PieceType::Knight,
    value: 3,
    has_moved: false
};

const BLACK_BISHOP: Piece = Piece {
    color: Color::Black,
    piece_type: PieceType::Bishop,
    value: 3,
    has_moved: false
};

const BLACK_ROOK: Piece = Piece {
    color: Color::Black,
    piece_type: PieceType::Rook,
    value: 5,
    has_moved: false
};

const BLACK_QUEEN: Piece = Piece {
    color: Color::Black,
    piece_type: PieceType::Queen,
    value: 9,
    has_moved: false
};

const BLACK_KING: Piece = Piece {
    color: Color::Black,
    piece_type: PieceType::King,
    value: 255,
    has_moved: false
};

impl Piece {
    pub fn get_moves(&self,board: &[Option<Piece> ; 64]) -> Result<Vec<i8>,ChessEngineError> {
        let position: Option<usize> = board.iter().position(|r| match r {
            None => false,
            Some(r) => ptr::eq(r,self) //checking if the actual memory address is equal
        });

        let position = match position {
            None => return Err(ChessEngineError {message: "Piece not in board".to_owned()}),
            Some(index) => index
        };

        let moves = match self.piece_type {
            PieceType::Pawn => {
                let direction: i8 = match self.color {
                    Color::White => -1,
                    Color::Black => 1
                };

                let mut available_moves: Vec<i8> = vec![];
                let new_position_forward_one = position as i8 + 8 * direction;
                let new_position_forward_two = position as i8 + 16 * direction;
                if new_position_forward_one >= 0 && new_position_forward_one <= 63 && board[new_position_forward_one as usize] == None {
                    available_moves.push(8 * direction);
                    if !self.has_moved && new_position_forward_two >= 0 && new_position_forward_two <= 63 && board[new_position_forward_two as usize] == None {
                        available_moves.push(16 * direction);
                    }
                }
                let new_position = position as i8 + 9 * direction;
                if ((new_position / 8) - (position / 8) as i8).abs() == 1 {
                    if let Some(piece) = &board[new_position as usize] {
                        if piece.color != self.color {
                            available_moves.push(9 * direction);
                        }
                    }
                }
                let new_position = position as i8 + 7 * direction;
                if ((new_position / 8) - (position / 8) as i8).abs() == 1 {
                    if let Some(piece) = &board[new_position as usize] {
                        if piece.color != self.color {
                            available_moves.push(7 * direction);
                        }
                    }
                }
                available_moves
            },
            PieceType::Knight => {
                let moves: Vec<i8> = vec![-17, -15, -10, -6, 6, 10, 15, 17];
                let mut available_moves: Vec<i8> = vec![];
                for movement in moves {
                    let new_position = position as i8 + movement;
                    if new_position < 0 || new_position > 63 {
                        continue;
                    }
                    else if (movement == 10 || movement == 6 || movement == -10 || movement == -6) && (new_position % 8 - position as i8 % 8).abs() != 2 {
                        continue;
                    }
                    else if (movement == 17 || movement == 15 || movement == -15 || movement == -17) && (new_position % 8 - position as i8 % 8).abs() != 1 {
                        continue;
                    }
                    if let Some(piece) = &board[new_position as usize] {
                        if piece.color == self.color {
                            continue;
                        }
                    }
                    available_moves.push(movement);
                }
                available_moves
            },
            PieceType::Bishop => {
                let mut available_moves: Vec<i8> = vec![];
                let directions: [i8;4] = [-9,9,7,-7];
                for direction in directions {
                    for i in 1..8 {
                        let movement: i8 = direction * i;
                        let new_position = position as i8 + movement;
                        if new_position > 63 || new_position < 0 {
                            break;
                        }
                        if (((position as i8 + movement) % 8) - (position % 8) as i8).abs() != i {
                            break;
                        }
                        if let Some(piece) = &board[new_position as usize] {
                            if piece.color != self.color {
                                available_moves.push(movement);
                            }
                            break;
                        }
                        available_moves.push(movement);
                    }
                }
                available_moves
            },
            PieceType::Rook => {
                let mut available_moves: Vec<i8> = vec![];
                let directions: [i8;4] = [-1,1,8,-8];
                for direction in directions {
                    for i in 1..8 {
                        let movement: i8 = direction * i;
                        let new_position = position as i8 + movement;
                        if new_position > 63 || new_position < 0 {
                            break;
                        }
                        if direction.abs() == 1 && (new_position / 8) != (position / 8) as i8 {
                            break;
                        }
                        if let Some(piece) = &board[new_position as usize] {
                            if piece.color != self.color {
                                available_moves.push(movement);
                            }
                            break;
                        }
                        available_moves.push(movement);
                    }
                }
                available_moves
            },
            PieceType::Queen => {
                let mut available_moves: Vec<i8> = vec![];
                let directions: [i8;8] = [-1,1,8,-8,-9,9,7,-7];
                for direction in directions {
                    for i in 1..8 {
                        let movement: i8 = direction * i;
                        let new_position = position as i8 + movement;
                        if new_position > 63 || new_position < 0 {
                            break;
                        }
                        if direction.abs() == 1 && (new_position / 8) != (position / 8) as i8 {
                            break;
                        }
                        else if direction.abs() != 1  && ((new_position / 8) - (position / 8) as i8).abs() != i {
                            break;
                        }
                        if let Some(piece) = &board[new_position as usize] {
                            if piece.color != self.color {
                                available_moves.push(movement);
                            }
                            break;
                        }
                        available_moves.push(movement);
                    }
                }
                available_moves
            },
            PieceType::King => {
                let mut available_moves: Vec<i8> = vec![];
                let directions: [i8;8] = [-1,1,8,-8,-9,9,7,-7];

                //Normal move logic
                for movement in directions {
                    let new_position = position as i8 + movement;
                    if new_position > 63 || new_position < 0 {
                        continue;
                    }
                    if movement.abs() == 1 && (new_position / 8) != (position / 8) as i8 {
                        break;
                    }
                    else if movement.abs() != 1  && ((new_position / 8) - (position / 8) as i8).abs() != 1 {
                        break;
                    }
                    if let Some(piece) = &board[new_position as usize] {
                        if piece.color != self.color {
                            available_moves.push(movement);
                        }
                        continue;
                    }
                    available_moves.push(movement);
                }

                //Castling logic
                if !self.has_moved {
                    if board[position + 1].is_none() && board[position + 2].is_none() {
                        available_moves.push(2);
                    }
                    if board[position - 1].is_none() && board[position - 2].is_none() && board[position - 3].is_none() {
                        available_moves.push(-2);
                    }
                }

                available_moves
            }
        };
        Ok(moves)
    }

    pub fn do_move(&self,board: &[Option<Piece>; 64],movement: i8) -> Result<(usize,u8,[Option<Piece>; 64]),ChessEngineError> {
        let position = board.iter().position(|r| match r {
            None => false,
            Some(r) => ptr::eq(r,self)
        });

        let position = match position {
            None => return Err(ChessEngineError {message: "Piece not in board".to_owned()}),
            Some(index) => index
        };
        let new_position = (position as i8 + movement) as usize;

        let piece_there_value = match &board[new_position] {
            None => 0,
            Some(piece) => piece.value,
        };

        let mut new_board = *board;
        new_board[position] = None;

        let moved_piece = match self.piece_type { //Mutate if its a pawn
            PieceType::Pawn | PieceType::King | PieceType::Rook => {
                let mut new_piece = self.clone();
                new_piece.has_moved = true;
                new_piece
            },
            _ => *self
        };

        new_board[new_position] = Some(moved_piece);

        //Castling Logic
        if self.piece_type == PieceType::King {
            if movement == 2 {
                new_board[position + 1] = new_board[position + 3];
                new_board[position + 3] = None;
            }
            else if movement == -2 {
                new_board[position - 1] = new_board[position - 4];
                new_board[position - 4] = None;
            }
        }

        Ok((position,piece_there_value,new_board))
    }
}


pub fn parse_fen(fen: &str) -> ([Option<Piece>; 64],Color) {
    let mut board: [Option<Piece>; 64] = [(); 64].map(|_| None);
    let mut offset = 0;
    let fen_parts: Vec<&str> = fen.split(' ').collect();
    let fen_board_part = fen_parts[0];
    let fen_whos_move = match fen_parts.get(1) {
        Some(color_char) => if *color_char == "b" { Color::Black } else { Color::White },
        None => Color::White
    };


    for (rank, fen_rank) in fen_board_part.split('/').enumerate() {
        for (index,c) in fen_rank.chars().enumerate() {
            match c {
                '1'..='8' => {
                    for i in 0..c.to_digit(10).unwrap() {
                        board[rank * 8 + offset + index + i as usize] = None;
                    }
                    offset += c.to_digit(10).unwrap() as usize - 1;
                },
                _ => {
                    let piece: Option<Piece> = if c.is_uppercase() { 
                        match c.to_ascii_lowercase() {
                            'p' => Some(WHITE_PAWN),
                            'r' => Some(WHITE_ROOK),
                            'n' => Some(WHITE_KNIGHT),
                            'b' => Some(WHITE_BISHOP),
                            'q' => Some(WHITE_QUEEN),
                            'k' => Some(WHITE_KING),
                            _ => panic!("Invalid character in FEN string {c}"),
                        }
                    }
                    else { 
                        match c.to_ascii_lowercase() {
                            'p' => Some(BLACK_PAWN),
                            'r' => Some(BLACK_ROOK),
                            'n' => Some(BLACK_KNIGHT),
                            'b' => Some(BLACK_BISHOP),
                            'q' => Some(BLACK_QUEEN),
                            'k' => Some(BLACK_KING),
                            _ => panic!("Invalid character in FEN string {c}"),
                        }
                    };
                    
                    board[rank * 8 + offset + index] = piece;
                }
            }
        }
        offset = 0
    }

    (board,fen_whos_move)
}

pub fn calculate_position(board: &[Option<Piece> ; 64],whos_move: Color,recursion_level: u8,current_recursion: u8,
                        value: i32,mut alpha: i32,mut beta: i32,zobrist_hasher: &ZobristHash,mut transposition_table: &mut HashMap<u64,i32>) -> (usize,i8,i32) {

    let sign = match whos_move {
        Color::White => 1,
        Color::Black => -1
    };

    //Checking Transposition table 
    let hash = zobrist_hasher.hash(board,current_recursion);
    if let Some(transposition_table_value) = transposition_table.get(&hash) {
        return (0,0,*transposition_table_value);
    }
    /////

    let mut best_score: i32 = -sign * 500;
    let mut best_move = 0;
    let mut best_piece_position = 0;
    for square in board {
        let piece = match square {
            Some(piece) => piece,
            None => continue
        };
        
        if piece.color == whos_move {
            if let Ok(moves) = piece.get_moves(board) {
                for movement in moves {
                    let (position,mut take_value,new_board): (usize,i32,[Option<Piece>; 64]) =  match piece.do_move(board, movement) {
                        Ok((position,value,new_board)) => (position,value as i32,new_board),
                        Err(_) => (0,0,*board)
                    };
                    take_value *= sign;

                    
                    if take_value.abs() == 255 { //if king stop immediately, prevents it from thinking it can kill other king next turn to equalize
                        return (position,movement,take_value)
                    }
                    
                    if recursion_level != current_recursion {
                        let foresight_value = calculate_position(&new_board, if whos_move == Color::White { Color::Black } else { Color::White },
                                                    recursion_level, current_recursion + 1,value + take_value,alpha,beta,zobrist_hasher,&mut transposition_table).2;
                                                    
                        take_value += foresight_value;

                        if whos_move == Color::White && foresight_value > alpha {
                            alpha = take_value;
                        }
                        else if whos_move == Color::Black && foresight_value < beta {
                            beta = take_value;
                        }
                    }
                    
                    if take_value * sign > best_score * sign {
                        best_score = take_value;
                        best_move = movement;
                        best_piece_position = position;
                    };

                    if whos_move == Color::Black && best_score < beta {
                        return (position,best_move,best_score)
                    }
                    else if whos_move == Color::White && best_score > alpha {
                        return (position,best_move,best_score)
                    }
                }
            }
        }
    }

    transposition_table.insert(hash, best_score);

    return (best_piece_position,best_move,best_score);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{parse_fen,chess_engine::{transposition_table::{ZobristHash}, calculate_position}};

    #[test]
    fn test_simple_take() {
        let mut transposition_table: HashMap<u64, i32> = HashMap::new();
        let (board,color_to_play) = parse_fen("rnb1kbnr/pppppppp/5q2/8/4N3/8/PPPPPPPP/R1BQKBNR");
        let (best_move_piece_1,best_move_1,max_1) = calculate_position(&board,color_to_play,4,1,0,0,0,&ZobristHash::new(),&mut transposition_table);
        assert_eq!((best_move_piece_1,best_move_1,max_1), (36,-15,6));
    }

    #[test]
    fn test_scholar() {
        let mut transposition_table: HashMap<u64, i32> = HashMap::new();
        let (board,color_to_play) = parse_fen("rnbqkbnr/pppppppp/8/8/2B5/4PQ2/PPPP1PPP/RNB1K1NR");
        let (best_move_piece_1,best_move_1,max_1) = calculate_position(&board,color_to_play,4,1,0,999,-999,&ZobristHash::new(),&mut transposition_table);
        assert_eq!((best_move_piece_1,best_move_1,max_1), (34,-21,253));
    }

    #[test]
    fn test_back_rank() {
        let mut transposition_table: HashMap<u64, i32> = HashMap::new();
        let (board,color_to_play) = parse_fen("6k1/5ppp/8/8/8/8/8/1Q2K3");
        let (best_move_piece_1,best_move_1,max_1) = calculate_position(&board,color_to_play,3,1,0,999,-999,&ZobristHash::new(),&mut transposition_table);
        assert_eq!((best_move_piece_1,best_move_1,max_1), (57,-56,255));
    }

    #[test]
    fn test_fork() {
        let mut transposition_table: HashMap<u64, i32> = HashMap::new();
        let (board,color_to_play) = parse_fen("2r3k1/5ppp/8/3N4/8/8/8/4K3");
        let (best_move_piece_1,best_move_1,max_1) = calculate_position(&board,color_to_play,3,1,0,999,-999,&ZobristHash::new(),&mut transposition_table);
        assert_eq!((best_move_piece_1,best_move_1,max_1), (27,-15,5));
    }

    #[test]
    fn test_smother() {
        let mut transposition_table: HashMap<u64, i32> = HashMap::new();
        let (board,color_to_play) = parse_fen("6rk/6pp/8/4N3/8/8/B7/4K3");
        let (best_move_piece_1,best_move_1,max_1) = calculate_position(&board,color_to_play,4,1,0,999,-999,&ZobristHash::new(),&mut transposition_table);
        assert_eq!((best_move_piece_1,best_move_1,max_1), (28,-15,255));
    }

    #[test]
    fn test_two_move() {
        let mut transposition_table: HashMap<u64, i32> = HashMap::new();
        let (board,color_to_play) = parse_fen("2r4k/6pp/8/4N3/8/1Q6/B7/4K3");
        let (best_move_piece_1,best_move_1,max_1) = calculate_position(&board,color_to_play,5,1,0,999,-999,&ZobristHash::new(),&mut transposition_table);
        assert_eq!((best_move_piece_1,best_move_1,max_1), (28,-6,252));
    }

    #[test]
    fn test_two_move_2() {
        let mut transposition_table: HashMap<u64, i32> = HashMap::new();
        let (board,color_to_play) = parse_fen("r1bq2r1/b4pk1/p1pp1p2/1p2pP2/1P2P1PB/3P4/1PPQ2P1/R3K2R");
        let (best_move_piece_1,best_move_1,max_1) = calculate_position(&board,color_to_play,5,1,0,999,-999,&ZobristHash::new(),&mut transposition_table);
        assert_eq!((best_move_piece_1,best_move_1,max_1), (51,-28,245));
    }

    #[test]
    fn knight_correct_restrictions() {
        let (board,_) = parse_fen("rnbqkb1r/ppppp1p1/5p1p/8/n5N1/8/PPPPPPPP/RNBQKB1R");
        for (i,piece) in board.iter().enumerate() {
            if i == 32 {
                let piece = match piece {
                    Some(piece) => piece,
                    None => continue
                };
                
                assert!(!piece.get_moves(&board).unwrap().contains(&6));
            }
        }
    }
}