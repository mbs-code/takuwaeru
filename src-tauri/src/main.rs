#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::command;

fn main() {
    let site = command::site::update(1, "url".to_string(), Some("title".to_string())).unwrap();
    println!("{:?}", site);

    // tauri::Builder::default()
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
