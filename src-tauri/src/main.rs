// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod client;

use client::evaluate;
use log::debug;

fn main() {
    // Start the logger
    flexi_logger::Logger::try_with_env()
        .unwrap()
        .start()
        .unwrap();

    debug!("Stariting...");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![evaluate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
