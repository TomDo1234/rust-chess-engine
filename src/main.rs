use std::io;

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

impl Piece {
    fn get_moves(&self) -> Vec<i8> {
        match self.piece_type {
            PieceType::Pawn => {
                let direction = match self.color {
                    Color::White => -1,
                    Color::Black => 1
                };
                if self.has_moved {
                    return vec![8,9,7].into_iter().map(|movement| movement * direction).collect()
                }
                else {
                    return vec![8,16,9,7].into_iter().map(|movement| movement * direction).collect()
                }
            },
            PieceType::Knight => vec![-17, -15, -10, -6, 6, 10, 15, 17],
            PieceType::Bishop => vec![-9, -18, -27, -36, -45, -54, -63, -72, -7, -14, -21, -28, -35, -42, -49, -56, 7, 14, 21, 28, 35, 42, 49, 56, 9, 18, 27, 36, 45, 54, 63, 72],
            PieceType::Rook => vec![-8, -16, -24, -32, -40, -48, -56, -64, -1, -2, -3, -4, -5, -6, -7, -8, 1, 2, 3, 4, 5, 6, 7, 8, 8, 16, 24, 32, 40, 48, 56, 64],
            PieceType::Queen => vec![-8, -16, -24, -32, -40, -48, -56, -64, -1, -2, -3, -4, -5, -6, -7, -8, 1, 2, 3, 4, 5, 6, 7, 8, 8, 16, 24, 32, 40, 48, 56, 64
            ,-9, -18, -27, -36, -45, -54, -63, -72, -7, -14, -21, -28, -35, -42, -49, -56, 7, 14, 21, 28, 35, 42, 49, 56, 9, 18, 27, 36, 45, 54, 63, 72],
            PieceType::King => vec![1,-1,9,-9,8,-8,7,-7]
        }
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

        }
    }
    return 0;
}

fn main() { 
    let mut fen_input = String::new();

    println!("Enter FEN:");
    io::stdin().read_line(&mut fen_input)
        .expect("Failed to read line");

    let (board,color_to_play) = parse_fen(&fen_input.trim());
    
    calculate_position(&board,color_to_play);

    println!("{:?}",board);
}
