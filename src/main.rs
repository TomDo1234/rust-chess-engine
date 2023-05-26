#[derive(Clone)]
#[derive(PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Clone)]
#[derive(PartialEq)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Clone)]
#[derive(PartialEq)]
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
            PieceType::Knight => vec![-8,-16,-9,-7],
            PieceType::Bishop => vec![-8,-16,-9,-7],
            PieceType::Rook => vec![-8,-16,-9,-7],
            PieceType::Queen => vec![-8,-16,-9,-7],
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
            None => Some(0),
            Some(piece) => Some(piece.value)
        }
    }
}

fn main() {
    let white_pawn = Piece {
        color: Color::White,
        piece_type: PieceType::Pawn,
        value: 1,
        has_moved: false
    };

    let white_knight = Piece {
        color: Color::White,
        piece_type: PieceType::Knight,
        value: 1,
        has_moved: false
    };

    let white_bishop = Piece {
        color: Color::White,
        piece_type: PieceType::Bishop,
        value: 1,
        has_moved: false
    };

    let white_rook = Piece {
        color: Color::White,
        piece_type: PieceType::Rook,
        value: 1,
        has_moved: false
    };

    let white_queen = Piece {
        color: Color::White,
        piece_type: PieceType::Queen,
        value: 1,
        has_moved: false
    };

    let white_king = Piece {
        color: Color::White,
        piece_type: PieceType::King,
        value: 1,
        has_moved: false
    };

    let black_pawn = Piece {
        color: Color::Black,
        piece_type: PieceType::Pawn,
        value: 1,
        has_moved: false
    };

    let black_knight = Piece {
        color: Color::Black,
        piece_type: PieceType::Knight,
        value: 1,
        has_moved: false
    };

    let black_bishop = Piece {
        color: Color::Black,
        piece_type: PieceType::Bishop,
        value: 1,
        has_moved: false
    };

    let black_rook = Piece {
        color: Color::Black,
        piece_type: PieceType::Rook,
        value: 1,
        has_moved: false
    };

    let black_queen = Piece {
        color: Color::Black,
        piece_type: PieceType::Queen,
        value: 1,
        has_moved: false
    };

    let black_king = Piece {
        color: Color::Black,
        piece_type: PieceType::King,
        value: 1,
        has_moved: false
    };

    let board: [Option<Piece>; 64] = [Some(black_rook.clone()),Some(black_knight.clone()),Some(black_bishop.clone()),Some(black_queen),Some(black_king),Some(black_bishop.clone()),Some(black_knight.clone()),Some(black_rook.clone()),
                                     Some(black_pawn.clone()),Some(black_pawn.clone()),Some(black_pawn.clone()),Some(black_pawn.clone()),Some(black_pawn.clone()),Some(black_pawn.clone()),Some(black_pawn.clone()),Some(black_pawn.clone()),
                                     Option::None,Option::None,Option::None,Option::None,Option::None,Option::None,Option::None,Option::None,
                                     Option::None,Option::None,Option::None,Option::None,Option::None,Option::None,Option::None,Option::None,
                                     Option::None,Option::None,Option::None,Option::None,Option::None,Option::None,Option::None,Option::None,
                                     Option::None,Option::None,Option::None,Option::None,Option::None,Option::None,Option::None,Option::None,
                                     Some(white_pawn.clone()),Some(white_pawn.clone()),Some(white_pawn.clone()),Some(white_pawn.clone()),Some(white_pawn.clone()),Some(white_pawn.clone()),Some(white_pawn.clone()),Some(white_pawn.clone()),
                                     Some(white_rook.clone()),Some(white_knight.clone()),Some(white_bishop.clone()),Some(white_queen),Some(white_king),Some(white_bishop.clone()),Some(white_knight.clone()),Some(white_rook.clone())];
}
