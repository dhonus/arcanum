use tauri::App;

#[cfg(mobile)]
mod mobile;
#[cfg(mobile)]
pub use mobile::*;

pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;

#[derive(Default)]
pub struct AppBuilder {
    setup: Option<SetupHook>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn setup<F>(mut self, setup: F) -> Self
        where
            F: FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        self.setup.replace(Box::new(setup));
        self
    }

    pub fn run(self) {
        let setup = self.setup;
        tauri::Builder::default()
            .setup(move |app| {
                if let Some(setup) = setup {
                    (setup)(app)?;
                }
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![feed, mark_read, update_feed, read_feed, update_all, delete_feed])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}


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
    match routes::rss::mark_read(url, guid) {
        Ok(_) => println!("Marked {} as read", guid),
        Err(e) => println!("Error marking as read: {}", e),
    }
}

#[tauri::command]
fn update_feed(url: &str) -> Result<HashMap<String, Vec<FeedMeta>>, String> {
    match routes::rss::update(url) {
        Ok(_) => println!("Updated feed"),
        Err(e) => println!("Error updating feed: {}", e),
    }

    let data = routes::rss::main( "", "");
    match data {
        Ok(feeds) => Ok(feeds.clone()),
        _ => Err("Failed to parse the feed. Please verify the URL is correct.".to_string()),
    }
}

#[tauri::command]
fn read_feed(url: &str) -> Result<HashMap<String, Vec<FeedMeta>>, String> {
    match routes::rss::read(url) {
        Ok(_) => println!("Read feed"),
        Err(e) => println!("Error reading feed: {}", e),
    }

    let data = routes::rss::main( "", "");
    match data {
        Ok(feeds) => Ok(feeds.clone()),
        _ => Err("Failed to parse the feed. Please verify the URL is correct.".to_string()),
    }
}

#[tauri::command]
fn update_all() -> Result<HashMap<String, Vec<FeedMeta>>, String> {
    match routes::rss::update_all() {
        Ok(_) => println!("Updated all feeds"),
        Err(e) => println!("Error updating all feeds: {}", e),
    }
    let data = routes::rss::main("", "");
    match data {
        Ok(feeds) => Ok(feeds.clone()),
        _ => Err("Failed to parse the feed. Please verify the URL is correct.".to_string()),
    }
}

#[tauri::command]
fn delete_feed(url: &str) -> Result<HashMap<String, Vec<FeedMeta>>, String> {
    match routes::rss::delete(url) {
        Ok(_) => println!("Deleted feed"),
        Err(e) => println!("Error deleting feed: {}", e),
    }
    let data = routes::rss::main( "", "");
    match data {
        Ok(feeds) => Ok(feeds.clone()),
        _ => Err("Failed to delete the feed. Ooops!".to_string()),
    }
}