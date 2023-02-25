#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use crate::routes::parser::Feed;

mod routes;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(_name: &str) -> String {
    const SOURCE: &str = "https://www.ostravan.cz/feed/";
    // routes::rss::main(SOURCE)
    match routes::rss::main(SOURCE) {
        Some(feed) => println!("Received feed for {}", feed.filename),
        None => println!("Bad RSS"),
    }
    format!("Hello, {}! You've been greeted from Rust!", "Hiya")
}

#[tauri::command]
fn feed(_name: &str) -> Result<Feed, String> {
    const SOURCE: &str = "https://www.theguardian.com/international/rss";
    let data = routes::rss::main(SOURCE);
    match data {
        Some(feed) => Ok(feed.feed),
        _ => Err("Failed to parse the feed. Please verify the URL is correct.".to_string()),
    }
}

fn main() {
    //rss::main()
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, feed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}