use crate::chess_engine::{parse_fen, calculate_position};
mod chess_engine;

use wasm_bindgen::JsCast;
use yew::{prelude::*};
use gloo::console::log;
use web_sys::HtmlInputElement;

#[function_component]
fn App() -> Html {
    let submit_fen = Callback::from(|event: KeyboardEvent| {
        if event.key() != "Enter".to_owned() {
            return;
        }
        let target = event.target().unwrap();
        let input = target.unchecked_into::<HtmlInputElement>();
        let value = input.value();
        log!(value.clone());
        let (board,color_to_play) = parse_fen(&value.clone() /*&fen_input.trim() */);
        let (best_move_piece_1,best_move_1,max_1) = calculate_position(&board,color_to_play,4,1,0,999,-999);
        log!(format!("{:?} {} {}",best_move_piece_1,best_move_1,max_1));
    });

    html! {
        <div>
            <input onkeypress={submit_fen} />
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