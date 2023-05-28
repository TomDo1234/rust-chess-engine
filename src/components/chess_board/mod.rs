use yew::{function_component, Properties, Html, html, classes};

use crate::chess_engine::{Piece,Color, PieceType};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub board: [Option<Piece>; 64],
}

#[function_component]
pub fn ChessBoard(props: &Props) -> Html {
    let Props { board } = props;
    html! {
        <div class={classes!("flex flex-col w-[500px] h-[500px]".to_owned())} >
            {
                (0..8).map(|rowindex| html! {
                    <div class={classes!("flex-1 flex w-full".to_owned())} >
                        {
                            (0..8).map(|colindex| {
                                let index = 8 * rowindex + colindex;
                                
                                let bg_class = match (index + rowindex) % 2 {
                                    0 => "bg-[#fdce9e]", 
                                    1 => "bg-[#d18b47]",
                                    _ => ""
                                };

                                let piece = match board[index] {
                                    None => return html!{ <img class={classes!(format!("flex-1 {bg_class}"))} src="" /> },
                                    Some(piece) => piece
                                };

                                if piece.color == Color::White {
                                    match piece.piece_type {
                                        PieceType::Pawn => html!{ <img class={classes!(format!("flex-1 {bg_class}"))} src="images/white_pawn.svg" /> },
                                        PieceType::Knight => html!{ <img class={classes!(format!("flex-1 {bg_class}"))} src="images/white_knight.svg" /> },
                                        PieceType::Bishop => html!{ <img class={classes!(format!("flex-1 {bg_class}"))} src="images/white_bishop.svg" /> },
                                        PieceType::Rook => html!{ <img class={classes!(format!("flex-1 {bg_class}"))} src="images/white_rook.svg" /> },
                                        PieceType::Queen => html!{ <img class={classes!(format!("flex-1 {bg_class}"))} src="images/white_queen.svg" /> },
                                        PieceType::King => html!{ <img class={classes!(format!("flex-1 {bg_class}"))} src="images/white_king.svg" /> },
                                    }
                                }
                                else {
                                    match piece.piece_type {
                                        PieceType::Pawn => html!{ <img class={classes!(format!("flex-1 {bg_class}"))} src="images/black_pawn.svg" /> },
                                        PieceType::Knight => html!{ <img class={classes!(format!("flex-1 {bg_class}"))} src="images/black_knight.svg" /> },
                                        PieceType::Bishop => html!{ <img class={classes!(format!("flex-1 {bg_class}"))} src="images/black_bishop.svg" /> },
                                        PieceType::Rook => html!{ <img class={classes!(format!("flex-1 {bg_class}"))} src="images/black_rook.svg" /> },
                                        PieceType::Queen => html!{ <img class={classes!(format!("flex-1 {bg_class}"))} src="images/black_queen.svg" /> },
                                        PieceType::King => html!{ <img class={classes!(format!("flex-1 {bg_class}"))} src="images/black_king.svg" /> },
                                    }
                                }
                            }).collect::<Html>()
                        }
                    </div>
                }).collect::<Html>()
            }
        </div>
    }
}
