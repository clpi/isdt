#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub mod cmd;
pub mod util;
pub mod data;
pub mod server;

use cmd::{init_demo, init_capture};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
          cfg_cmd,
          init_capture,
          init_demo
          ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn cfg_cmd() {
    println!("Invoked from js");
}