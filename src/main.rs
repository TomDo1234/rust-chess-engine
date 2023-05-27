use std::time::Instant;

use crate::chess_engine::{parse_fen, calculate_position};

mod chess_engine;
fn main() { 
    //rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR starting position

    // println!("Enter FEN:");
    // io::stdin().read_line(&mut fen_input)
    //     .expect("Failed to read line");
    let start_time = Instant::now();
    let (board,color_to_play) = parse_fen("r1bq2r1/b4pk1/p1pp1p2/1p2pP2/1P2P1PB/3P4/1PPQ2P1/R3K2R" /*&fen_input.trim() */);
    let (best_move_piece_1,best_move_1,max_1) = calculate_position(&board,color_to_play,5,1,0,999,-999);
    println!("{:?} {} {}",best_move_piece_1,best_move_1,max_1);
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {} seconds, {} milliseconds",
             elapsed_time.as_secs(),
             elapsed_time.subsec_millis());
}