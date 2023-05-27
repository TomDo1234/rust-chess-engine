use core::fmt;
use std::{io, error::Error,ptr, time::Instant};

#[derive(Clone,PartialEq,Debug,Copy)]
enum Color {
    White,
    Black,
}

#[derive(Clone,PartialEq,Debug,Copy)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Clone,PartialEq,Debug,Copy)]
struct Piece {
    color: Color,
    piece_type: PieceType,
    value: u8, 
    has_moved: bool
}

#[derive(Debug)]
struct ChessEngineError {
    message: String
}

impl fmt::Display for ChessEngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chess Engine Error: {}", self.message)
    }
}

impl Error for ChessEngineError {}



static WHITE_PAWN: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Pawn,
    value: 1,
    has_moved: false
};

static WHITE_KNIGHT: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Knight,
    value: 3,
    has_moved: false
};

static WHITE_BISHOP: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Bishop,
    value: 3,
    has_moved: false
};

static WHITE_ROOK: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Rook,
    value: 5,
    has_moved: false
};

static WHITE_QUEEN: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Queen,
    value: 9,
    has_moved: false
};

static WHITE_KING: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::King,
    value: 255,
    has_moved: false
};

static BLACK_PAWN: Piece = Piece {
    color: Color::Black,
    piece_type: PieceType::Pawn,
    value: 1,
    has_moved: false
};

static BLACK_KNIGHT: Piece = Piece {
    color: Color::Black,
    piece_type: PieceType::Knight,
    value: 3,
    has_moved: false
};

static BLACK_BISHOP: Piece = Piece {
    color: Color::Black,
    piece_type: PieceType::Bishop,
    value: 3,
    has_moved: false
};

static BLACK_ROOK: Piece = Piece {
    color: Color::Black,
    piece_type: PieceType::Rook,
    value: 5,
    has_moved: false
};

static BLACK_QUEEN: Piece = Piece {
    color: Color::Black,
    piece_type: PieceType::Queen,
    value: 9,
    has_moved: false
};

static BLACK_KING: Piece = Piece {
    color: Color::Black,
    piece_type: PieceType::King,
    value: 255,
    has_moved: false
};

impl Piece {
    fn get_moves(&self,board: &[Option<Piece> ; 64]) -> Result<Vec<i8>,ChessEngineError> {
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
                if board[(position as i8 + 8 * direction) as usize] == Option::None {
                    available_moves.push(8 * direction);
                    if !self.has_moved && board[(position as i8 + 16 * direction) as usize] == Option::None {
                        available_moves.push(16 * direction);
                    }
                }
                if let Some(piece) = &board[(position as i8 + 9 * direction) as usize] {
                    if piece.color != self.color {
                        available_moves.push(9 * direction);
                    }
                }
                if let Some(piece) = &board[(position as i8 + 7 * direction) as usize] {
                    if piece.color != self.color {
                        available_moves.push(7 * direction);
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
                    else if let Some(piece) = &board[new_position as usize] {
                        if piece.color == self.color {
                            continue;
                        }
                    }
                    else if (movement == 10 || movement == 6 || movement == -10 || movement == -6) && (new_position % 8 - position as i8 % 8).abs() != 2 {
                        continue;
                    }
                    else if (movement == 17 || movement == 15 || movement == -15 || movement == -17) && (new_position % 8 - position as i8 % 8).abs() != 1 {
                        continue;
                    }
                    available_moves.push(movement);
                }
                available_moves
            }
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
                for movement in directions {
                    let new_position = position as i8 + movement;
                    if new_position > 63 || new_position < 0 {
                        continue;
                    }
                    if let Some(piece) = &board[new_position as usize] {
                        if piece.color != self.color {
                            available_moves.push(movement);
                        }
                        continue;
                    }
                    available_moves.push(movement);
                }
                available_moves
            }
        };
        Ok(moves)
    }

    fn do_move(&self,board: &[Option<Piece>; 64],movement: i8) -> Result<(u8,[Option<Piece>; 64]),ChessEngineError> {
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
        new_board[position] = Option::None;
        new_board[new_position] = Some(*self);

        Ok((piece_there_value,new_board))
    }
}


fn parse_fen(fen: &str) -> ([Option<Piece>; 64],Color) {
    let mut board: [Option<Piece>; 64] = [(); 64].map(|_| None);
    let mut offset = 0;
    for (rank, fen_rank) in fen.split('/').enumerate() {
        for (index,c) in fen_rank.chars().enumerate() {
            match c {
                '1'..='8' => {
                    for i in 0..c.to_digit(10).unwrap() {
                        board[rank * 8 + offset + index + i as usize] = Option::None;
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

    (board,Color::White)
}

fn calculate_position(board: &[Option<Piece> ; 64],whos_move: Color,recursion_level: u8,current_recursion: u8,value: i32,mut alpha: i32,mut beta: i32) -> (Option<PieceType>,i8,i32) {
    let sign = match whos_move {
        Color::White => 1,
        Color::Black => -1
    };
    
    let mut best_score: i32 = -sign * 500;
    let mut best_move = 0;
    let mut best_move_piece = Option::None;
    for square in board {
        let piece = match square {
            Some(piece) => piece,
            None => continue
        };

        if piece.color == whos_move {
            if let Ok(moves) = piece.get_moves(board) {
                for movement in moves {
                    let (mut take_value,new_board): (i32,[Option<Piece>; 64]) =  match piece.do_move(board, movement) {
                        Ok((value,new_board)) => (value as i32,new_board),
                        Err(_) => (0,*board)
                    };
                    take_value *= sign;
                    
                    if take_value.abs() == 255 { //if king stop immediately, prevents it from thinking it can kill other king next turn to equalize
                        return (Some(piece.piece_type),movement,take_value)
                    }
                    
                    if recursion_level != current_recursion {
                        let foresight_value = calculate_position(&new_board, if whos_move == Color::White { Color::Black } else { Color::White },
                                                    recursion_level, current_recursion + 1,value + take_value,alpha,beta).2;
                                                    
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
                        best_move_piece = Some(piece.piece_type);
                    };

                    if whos_move == Color::Black && best_score < beta {
                        return (best_move_piece ,best_move,best_score)
                    }
                    else if whos_move == Color::White && best_score > alpha {
                        return (best_move_piece ,best_move,best_score)
                    }
                }
            }
        }
    }
    return (best_move_piece,best_move,best_score);
}

fn main() { 
    let mut fen_input = String::new();
    //rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR starting position

    println!("Enter FEN:");
    io::stdin().read_line(&mut fen_input)
        .expect("Failed to read line");

    let (board,color_to_play) = parse_fen(&fen_input /*&fen_input.trim() */);
    let (best_move_piece_1,best_move_1,max_1) = calculate_position(&board,color_to_play,1,1,0,0,0);
    println!("{:?} {} {}",best_move_piece_1,best_move_1,max_1);
}
