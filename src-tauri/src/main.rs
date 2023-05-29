// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod chess_engine;
use chess_engine::{Piece};
use serde::Serialize;
use serde_big_array::BigArray;

use crate::chess_engine::parse_fen;

#[derive(Serialize)]
struct BoardGenerationResponse {
  #[serde(with = "BigArray")]
  board: [Option<Piece>; 64]
}

#[tauri::command]
fn generate_board_from_fen(fen: &str)-> BoardGenerationResponse {
  BoardGenerationResponse {
    board: parse_fen(fen).0
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![generate_board_from_fen])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
