#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chess::Board;
use std::str::FromStr;
use shallow_red_engine::engine::enter_engine;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![run_engine])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn run_engine(current_fen: &str) -> String {
  let board = Board::from_str(&current_fen).expect("Valid Position"); // setup the board
  let engine_move = enter_engine(board); // find the best move

  return engine_move.to_string(); // return the best move
}