#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use crate::routes::rss::FeedMeta;

mod routes;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(_name: &str) -> String {
    match routes::rss::main(_name) {
        Some(feeds) => println!("Received feed for {}", feeds[0].filename),
        None => println!("Bad RSS"),
    }
    format!("Hello, {}! You've been greeted from Rust!", "Hiya")
}

#[tauri::command]
fn feed(_name: &str) -> Result<Vec<FeedMeta>, String> {
    let data = routes::rss::main(_name);
    match data {
        Some(feeds) => Ok(feeds.clone()),
        _ => Err("Failed to parse the feed. Please verify the URL is correct.".to_string()),
    }
}

#[tauri::command]
fn mark_read(url: &str, guid: &str) -> Result<Vec<FeedMeta>, String> {
    println!("Marking {} as read", guid);
    routes::rss::mark_read(url, guid);
    let data = routes::rss::main("");
    match data {
        Some(feeds) => Ok(feeds.clone()),
        _ => Err("Failed to parse the feed. Please verify the URL is correct.".to_string()),
    }
}
#[tauri::command]
fn update_feed(url: &str) -> Result<Vec<FeedMeta>, String> {
    println!("Updating {}", url);

    routes::rss::update(url);

    let data = routes::rss::main("");
    match data {
        Some(feeds) => Ok(feeds.clone()),
        _ => Err("Failed to parse the feed. Please verify the URL is correct.".to_string()),
    }
}

fn main() {
    //rss::main()
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, feed, mark_read, update_feed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}