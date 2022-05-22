#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::command;

fn main() {
    // let site = command::site::create(&"url2".to_string(), &Some("title2".to_string())).unwrap();
    let site = command::site::list(1, None, None, None).unwrap();
    println!("{:?}", site);

    // tauri::Builder::default()
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
