#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::{
    command,
    model::{SiteParam, SiteQueryParam},
};
use tauri::generate_handler;

fn main() {
    // let site = command::site::create(SiteParam {
    //     key: "key".to_string(),
    //     url: "url".to_string(),
    //     title: Some("title".to_string()),
    //     site_queries: Vec::from([{
    //         SiteQueryParam {
    //             key: "key".to_string(),
    //             url_pattern: "url_pattern".to_string(),
    //             processor: "processor".to_string(),
    //             url_filter: "url_filter".to_string(),
    //             priority: 10,
    //         }
    //     }]),
    // })
    // .unwrap();
    let site = command::site::list(1, None, None, None).unwrap();

    println!("{:?}", site);

    // tauri::Builder::default()
    //     .invoke_handler(generate_handler![
    //         command::site::list,
    //         command::site::create,
    //         command::site::update,
    //         command::site::delete,
    //     ])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
