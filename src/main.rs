use core::fmt;
use std::{io, error::Error,ptr};

#[derive(Clone,PartialEq,Debug)]
enum Color {
    White,
    Black,
}

#[derive(Clone,PartialEq,Debug)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Clone,PartialEq,Debug)]
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
                if !self.has_moved && board[(position as i8 + 16 * direction) as usize] == Option::None {
                    available_moves.push(16 * direction);
                }
                if board[(position as i8 + 8 * direction) as usize] == Option::None {
                    available_moves.push(8 * direction);
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
                    else if board[new_position as usize] != Option::None {
                        continue;
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
                    for i in 0..9 {
                        let movement: i8 = direction * i;
                        let new_position = position as i8 + movement;
                        if new_position > 63 || new_position < 0 {
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
                    for i in 0..8 {
                        let movement: i8 = direction * i;
                        let new_position = position as i8 + movement;
                        if new_position > 63 || new_position < 0 {
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
                    for i in 0..8 {
                        let movement: i8 = direction * i;
                        let new_position = position as i8 + movement;
                        if new_position > 63 || new_position < 0 {
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
                available_moves
            }
        };
        Ok(moves)
    }

    fn do_move(&self,board: &[Option<Piece>; 64],movement: i8) -> Option<u8> {
        let position = board.iter().position(|r| match r {
            None => false,
            Some(r) => r == self
        });

        let position = match position {
            None => return None,
            Some(index) => index
        };

        let piece_there =  match board.get((position as i8 + movement) as usize) {
            None => return None,
            Some(square) => square,
        };

        match piece_there {
            None => {
                if self.piece_type == PieceType::Pawn && ![8,9,-8,-9].contains(&movement) {
                    None
                }
                else {
                    Some(0)
                }
            },
            Some(piece) => Some(piece.value)
        }
    }
}


fn parse_fen(fen: &str) -> ([Option<Piece>; 64],Color) {
    let mut board: [Option<Piece>; 64] = [(); 64].map(|_| None);

    for (rank, fen_rank) in fen.split('/').enumerate() {
        for (index,c) in fen_rank.chars().enumerate() {
            match c {
                '1'..='8' => {
                    for i in 0..c.to_digit(10).unwrap() {
                        board[rank * 8 + index + i as usize] = Option::None;
                    }
                },
                _ => {
                    let piece: Option<Piece> = if c.is_uppercase() { 
                        match c.to_ascii_lowercase() {
                            'p' => Some(WHITE_PAWN.clone()),
                            'r' => Some(WHITE_ROOK.clone()),
                            'n' => Some(WHITE_KNIGHT.clone()),
                            'b' => Some(WHITE_BISHOP.clone()),
                            'q' => Some(WHITE_QUEEN.clone()),
                            'k' => Some(WHITE_KING.clone()),
                            _ => panic!("Invalid character in FEN string {c}"),
                        }
                    }
                    else { 
                        match c.to_ascii_lowercase() {
                            'p' => Some(BLACK_PAWN.clone()),
                            'r' => Some(BLACK_ROOK.clone()),
                            'n' => Some(BLACK_KNIGHT.clone()),
                            'b' => Some(BLACK_BISHOP.clone()),
                            'q' => Some(BLACK_QUEEN.clone()),
                            'k' => Some(BLACK_KING.clone()),
                            _ => panic!("Invalid character in FEN string {c}"),
                        }
                    };
                    
                    board[rank * 8 + index] = piece;
                }
            }
        }
    }

    (board,Color::White)
}

fn calculate_position(board: &[Option<Piece> ; 64],whos_move: Color) -> i8 {
    for square in board {
        let piece = match square {
            Some(piece) => piece,
            None => continue
        };

        if piece.color == whos_move {
            println!("{:?} {:?}",piece.piece_type,piece.get_moves(board));
        }
    }
    return 0;
}

fn main() { 
    let mut fen_input = String::new();
    //rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR starting position

    // println!("Enter FEN:");
    // io::stdin().read_line(&mut fen_input)
    //     .expect("Failed to read line");

    let (board,color_to_play) = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR" /*&fen_input.trim() */);
    
    calculate_position(&board,color_to_play);
}
