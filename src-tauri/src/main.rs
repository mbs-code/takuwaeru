#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::db;

fn main() {
    let conn = db::get_conn().unwrap();

    // tauri::Builder::default()
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
