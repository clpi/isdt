#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub mod cmd;
pub mod util;
pub mod error;
pub mod data;
pub mod server;

use cmd::{cfg_cmd, init_demo, init_capture};
use data::{Demo, Section, Step};
use tauri::{generate_handler, generate_context};

pub use error::{IsdtError, IsdtResult};

fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![
            cfg_cmd,
            init_capture,
            init_demo
          ])
        .run(generate_context!())
        .expect("error while running tauri application");
}