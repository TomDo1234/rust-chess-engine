use crate::{chess_engine::{parse_fen}, components::chess_board::ChessBoard};
mod chess_engine;
mod components;

use wasm_bindgen::JsCast;
use yew::{prelude::*};
use gloo::console::log;
use web_sys::HtmlInputElement;

#[function_component]
fn App() -> Html {
    let board = use_state(|| parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").0);

    let submit_fen = {
        let board = board.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() != "Enter".to_owned() {
                return;
            }
            let target = event.target().unwrap();
            let input = target.unchecked_into::<HtmlInputElement>();
            let value = input.value();
            log!(value.clone());
            let (result_board,_) = parse_fen(&value.clone() /*&fen_input.trim() */);
            board.set(result_board);
        })
    };

    let on_piece_drop = {
        let board = board.clone();
        Callback::from(move |from_and_to: (Option<usize>,usize)| {
            let mut new_board = *board;
            let (from,to) = from_and_to;
            if let Some(from) = from {
                if let Some(moved_piece) = &new_board[from] {
                    let movement = to as i8 - from as i8;
                    if let Ok(moves) = moved_piece.get_moves(&new_board) {
                        if moves.contains(&movement) {
                            new_board[to] = new_board[from];
                            new_board[from] = None;
                            board.set(new_board);
                        }
                    }
                }
            }
        })
    };

    html! {
        <div class="flex flex-col justify-center items-center h-screen" >
            <input class={classes!("border border-1 border-black border-solid mb-8".to_owned())} onkeypress={submit_fen} />
            <ChessBoard board={*board} on_piece_drop={on_piece_drop.clone()} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}


// fn main() { 
//     //rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR starting position

//     // println!("Enter FEN:");
//     // io::stdin().read_line(&mut fen_input)
//     //     .expect("Failed to read line");
//     let (board,color_to_play) = parse_fen("r1bq2r1/b4pk1/p1pp1p2/1p2pP2/1P2P1PB/3P4/1PPQ2P1/R3K2R" /*&fen_input.trim() */);
//     let (best_move_piece_1,best_move_1,max_1) = calculate_position(&board,color_to_play,5,1,0,999,-999);
//     println!("{:?} {} {}",best_move_piece_1,best_move_1,max_1);
// }