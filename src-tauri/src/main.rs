#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::command;
use tauri::generate_handler;

fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![
            command::site::site_count,
            command::site::site_list,
            command::site::site_get,
            command::site::site_create,
            command::site::site_update,
            command::site::site_delete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
