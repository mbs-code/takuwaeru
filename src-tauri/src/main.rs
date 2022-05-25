#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::command;
use tauri::generate_handler;

fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![
            command::site::list,
            command::site::create,
            command::site::update,
            command::site::delete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
