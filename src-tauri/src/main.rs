// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chess::Board;
use std::str::FromStr;
use shallow_red_engine::engine::{enter_engine, EngineReturn, Statistics};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![run_engine])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn run_engine(current_fen: String) -> EngineReturn {
  let board = Board::from_str(&current_fen).expect("Valid Position"); // setup the board

  let (chess_move, engine_result_maybe) = enter_engine(board).await; // find the best move
  match engine_result_maybe {
    Some(engine_result) => return engine_result,
    None => return EngineReturn{engine_move: chess_move.to_string(), engine_stats: Default::default()},
  }
}