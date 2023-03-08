#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::collections::HashMap;
use crate::routes::rss::FeedMeta;

mod routes;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn feed(_url: &str, _category: &str) -> Result<HashMap<String, Vec<FeedMeta>>, String> {
    let data = routes::rss::main(_url, _category);
    match data {
        Ok(feeds) => Ok(feeds.clone()),
        _ => Err("Failed to parse the feed. Please verify the URL is correct.".to_string()),
    }
}

#[tauri::command]
fn mark_read(url: &str, guid: &str){
    println!("Marking {} as read", guid);
    routes::rss::mark_read(url, guid);
}

#[tauri::command]
fn update_feed(url: &str) -> Result<HashMap<String, Vec<FeedMeta>>, String> {
    routes::rss::update(url);

    let data = routes::rss::main( "", "");
    match data {
        Ok(feeds) => Ok(feeds.clone()),
        _ => Err("Failed to parse the feed. Please verify the URL is correct.".to_string()),
    }
}

#[tauri::command]
fn read_feed(url: &str) -> Result<HashMap<String, Vec<FeedMeta>>, String> {
    routes::rss::read(url);

    let data = routes::rss::main( "", "");
    match data {
        Ok(feeds) => Ok(feeds.clone()),
        _ => Err("Failed to parse the feed. Please verify the URL is correct.".to_string()),
    }
}

#[tauri::command]
fn update_all() -> Result<HashMap<String, Vec<FeedMeta>>, String> {
    routes::rss::update_all();
    let data = routes::rss::main("", "");
    match data {
        Ok(feeds) => Ok(feeds.clone()),
        _ => Err("Failed to parse the feed. Please verify the URL is correct.".to_string()),
    }
}

#[tauri::command]
fn delete_feed(url: &str) -> Result<HashMap<String, Vec<FeedMeta>>, String> {
    routes::rss::delete(url);
    let data = routes::rss::main( "", "");
    match data {
        Ok(feeds) => Ok(feeds.clone()),
        _ => Err("Failed to delete the feed. Ooops!".to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![feed, mark_read, update_feed, read_feed, update_all, delete_feed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}