use std::collections::HashMap;

use crate::{chess_engine::{parse_fen, calculate_position, Color, transposition_table::ZobristHash}, components::chess_board::ChessBoard};
mod chess_engine;
mod components;

use chess_engine::Piece;
use wasm_bindgen::JsCast;
use yew::{prelude::*};

use gloo::{console::log, timers::callback::Timeout};
use web_sys::HtmlInputElement;

fn computer_moves(board_state_hook: UseStateHandle<[Option<Piece>; 64]>,mut new_board: [Option<Piece>; 64]) {
    log!("Thinking...");
    let mut transposition_table: HashMap<u64, i32> = HashMap::new();
    let (best_move_original_position,best_move,_) = calculate_position(&new_board,Color::Black,5,1,0.0,999,-999,&ZobristHash::new(),&mut transposition_table);

    let new_position = best_move_original_position as i8 + best_move;
    new_board[new_position as usize] = new_board[best_move_original_position];
    new_board[best_move_original_position] = None;
    board_state_hook.set(new_board);
}

#[function_component]
fn App() -> Html {
    let board = use_state(|| parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").0);
    let whos_move = use_state(|| Color::White);

    { //Closure so useEffect works
        let whos_move = whos_move.clone();
        let board = board.clone();
        use_effect_with_deps(move |whos_move| { 
            //Wait for it to be visually noticable that the component has rerendered
            let whos_move = whos_move.clone();
            let board_state_hook = board.clone();
            let timeout = Timeout::new(50,move || {
                match *whos_move {
                    Color::Black => {computer_moves(board_state_hook, *board); whos_move.set(Color::White)},
                    Color::White => ()
                };
            });
            timeout.forget();
        },whos_move);
    }

    let submit_fen = {
        let board = board.clone();
        let whos_move = whos_move.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() != "Enter".to_owned() {
                return;
            }
            let target = event.target().unwrap();
            let input = target.unchecked_into::<HtmlInputElement>();
            let value = input.value();
            log!(value.clone());
            let (result_board,whos_move_from_fen) = parse_fen(&value.clone().trim());
            board.set(result_board);
            whos_move.clone().set(whos_move_from_fen);
        })
    };

    let on_piece_drop = {
        let board = board.clone();
        let whos_move = whos_move.clone();
        Callback::from(move |from_and_to: (Option<usize>,usize)| {
            let new_board = *board;
            let (from,to) = from_and_to;
            
            let from = match from {
                Some(from) => from,
                None => return
            };

            let moved_piece = match &new_board[from] {
                Some(piece) => piece,
                None => return
            };

            let movement = to as i8 - from as i8;

            let moves = match moved_piece.get_moves(&new_board) {
                Ok(moves) => moves,
                Err(_) => return
            };

            if moves.contains(&movement) {
                let new_board = match moved_piece.do_move(&new_board, movement) {
                    Ok((_,_,new_board)) => new_board,
                    Err(_) => return
                };

                board.set(new_board);

                if *whos_move == Color::Black {
                    whos_move.set(Color::White);
                }
                else {
                    whos_move.set(Color::Black);
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