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
            command::page::page_count,
            command::page::page_list,
            command::page::page_get,
            command::page::page_create,
            command::page::page_update,
            command::page::page_delete,
            command::page::page_clear,
            command::queue::queue_count,
            command::queue::queue_list,
            command::queue::queue_get,
            command::queue::queue_create,
            command::queue::queue_update,
            command::queue::queue_delete,
            command::queue::queue_clear,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
