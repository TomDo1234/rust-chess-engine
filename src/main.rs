use crate::chess_engine::{parse_fen, calculate_position};
mod chess_engine;

use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let mut fen = "";
    fn submit_fen (e: KeyboardEvent) {
        if e.key() != "Enter" {
            return;
        }
        e.current_target();
    }

    html! {
        <div>
            <input onkeypress={submit_fen} value={fen} />
            {fen}
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