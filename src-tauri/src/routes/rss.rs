use std::collections::HashMap;
use std::env;
use rss::Channel;
use std::error::Error;
use std::path::Path;
use crate::routes::parser;
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::fs::{OpenOptions};
use tauri::api::path;
use std::time::Duration;
use chrono::{DateTime};


async fn obtain_feed(source: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(source)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    channel.write_to(::std::io::sink())?; // write to the channel to a writer
    Ok(channel)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FeedMeta {
    pub url: String,
    pub filename: String,
    pub category: String,
    pub feed: Channel,
    pub read: Vec<String>,
    pub unread: i32,
}

impl FeedMeta {
    fn new(url: &str, mut category: &str) -> Result<FeedMeta, reqwest::Error> {
        if category == ""{
            category = "Uncategorized";
        }
        let filename = url
            .replace("https://", "")
            .replace("/", "_")
            .replace(":", "_")
            .replace(".", "_");

        let path = format!("../feeds/{}", filename);
        match std::fs::create_dir_all(path){
            Ok(_) => println!("Created dir"),
            Err(_) => println!("Dir already exists"),
        }

        // check if file exists
        let filename = format!("../feeds/{}/feed.xml", filename);
        println!("Filename: {}", filename);
        let path = Path::new(filename.as_str());
        let client = reqwest::Client::builder()
            .gzip(true)
            .timeout(Duration::from_secs(8))
            .build()?;

        let runtime = tokio::runtime::Runtime::new().unwrap_or_else(|e| {
            panic!("failed to create Tokio runtime: {}", e); // we can't really recover from this
        });
        if !path.exists() {
            match runtime.block_on(parser::pull(url, filename.as_str(), &client)) {
                Ok(_) => println!("File created"),
                Err(_) => println!("File not created"),
            }
        }
        let feed = parser::parse(filename.clone());
        let unread: i32 = feed.items().len() as i32;

        Ok(FeedMeta {
            url: url.to_string(),
            filename,
            category: category.to_string(),
            feed,
            read: Vec::new(),
            unread
        })
    }
    fn load() -> Result<Vec<FeedMeta>, Box<dyn Error>> {
        match std::fs::create_dir("../feeds"){
            Ok(_) => println!("Created dir"),
            Err(_) => println!("Dir already exists"),
        }
        let mut feeds = Vec::new();
        match csv::Reader::from_path("../feeds/db.csv") {
            Ok(_) => println!("File exists"),
            Err(_) => {
                println!("File does not exist");
                return Ok(feeds);
            }
        }
        let mut reader = csv::Reader::from_path("../feeds/db.csv")?;
        let headers = reader.headers();
        println!("Headers: {:?}", headers);
        for result in reader.records() {
            // The iterator yields Result<StringRecord, Error>, so we check the
            // error here.
            let record = result?;
            assert_eq!(record.len(), 4);
            let mut meta = FeedMeta::new(record.get(0).unwrap(), record.get(3).unwrap())?;
            println!("{:?}", record);

            let logfile_name = format!("{}.log", meta.filename.clone());
            let log = FeedMeta::read_log(logfile_name.as_str());
            match log {
                Ok(log) => {
                    meta.read = log;
                    meta.unread = 0;
                    for item in meta.feed.items() {
                        let guid = item.guid();
                        match guid {
                            Some(guid) => {
                                if !meta.read.contains(&guid.value().to_string()) {
                                    meta.unread += 1;
                                }
                            }
                            None => {
                                println!("No guid");
                            }
                        }
                    }
                }
                Err(_) => {
                    println!("Error reading log for {}. It is usually safe to ignore.", meta.filename);
                }
            }
            feeds.push(meta);
        }

        return Ok(feeds);
    }
    fn save(mut feeds: Vec<FeedMeta>) -> Result<Vec<FeedMeta>, Box<dyn Error>> {
        match std::fs::create_dir("../feeds"){
            Ok(_) => println!("Created dir"),
            Err(_) => println!("Dir already exists"),
        }

        // deduplicate feeds
        feeds.sort_by(|a, b| a.url.cmp(&b.url));
        feeds.dedup_by(|a, b| a.url == b.url);

        let mut writer = csv::Writer::from_path("../feeds/db.csv")?;

        writer.write_record(&["url", "filename", "feed", "category"])?;
        for entry in feeds.iter() {
            writer.write_record(&[entry.url.as_str(), entry.filename.as_str(), entry.feed.title.as_str(), entry.category.as_str()])?;
        }
        writer.flush().expect("Failed to flush writer");
        return Ok(feeds);
    }
    fn read_log(filename: &str) -> Result<Vec<String>, std::io::Error> {
        let file = OpenOptions::new()
            .read(true)
            .open(filename)?;
        let reader = BufReader::new(file);
        let mut lines = Vec::new();
        for line in reader.lines() {
            lines.push(line?);
        }
        Ok(lines)
    }
    // take guid and save into lof of read articles
    fn re_index_and_mark_all_read(filename: &str, lines: &mut FeedMeta) -> Result<(), std::io::Error> {
        println!("{} is the filename", filename);
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)?;

        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))?;
        let mut writer = std::io::BufWriter::new(&file);
        for line in lines.feed.items.clone() {
            writer.write(line.guid.unwrap().value.as_bytes())?;
            writer.write(b"\n")?;
        }
        writer.flush()?;

        Ok(())
    }

    fn modify_file(filename: &str, adding: &str) -> Result<(), std::io::Error> {
        // Open the file for reading or create it if it doesn't exist
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)?;

        // Read the file contents line by line into a vector
        let reader = BufReader::new(&file);
        let mut lines = reader.lines().collect::<Result<Vec<String>, _>>()?;

        // Add the new string to the vector
        lines.push(String::from(adding));
        lines.sort_by(|a, b| a.cmp(&b));
        lines.dedup_by(|a, b| a == b);

        // limit size to 200
        if lines.len() > 200 {
            lines = lines[lines.len()-200..].to_vec();
        }

        // Truncate the file and write the updated vector back to it
        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))?;
        let mut writer = std::io::BufWriter::new(&file);
        for line in lines {
            writer.write(line.as_bytes())?;
            writer.write(b"\n")?;
        }
        writer.flush()?;

        Ok(())
    }
}

pub fn mark_read(url: &str, guid: &str) -> Result<(), Box<dyn Error>>{
    let filename = url
        .replace("https://", "")
          .replace("/", "_")
        .replace(":", "_")
        .replace(".", "_");
    let filename = format!("../feeds/{}/feed.xml.log", filename);
    FeedMeta::modify_file(filename.as_str(), guid)?;
    Ok(())
}

pub fn update(source: &str) -> Result<(), Box<dyn Error>>{
    let mut feeds: Vec<FeedMeta> = FeedMeta::load()?;
    if feeds.len() == 0 {
        println!("No feeds");
        return Ok(());
    }
    let mut index = 0;
    for (i, feed) in feeds.iter().enumerate() {
        if feed.filename == source {
            index = i;
        }
    }
    let runtime = tokio::runtime::Runtime::new().unwrap_or_else(|e| {
        panic!("failed to create Tokio runtime: {}", e); // we can't really recover from this
    });

    let client = reqwest::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(8))
        .build()?;
    let feed = &mut feeds[index];
    match runtime.block_on(parser::pull(feed.url.as_str(), feed.filename.as_str(), &client)) {
        Ok(_) => {
            println!("Updated {}", feed.filename);
        }
        Err(e) => {
            println!("Error updating {}: {}", feed.filename, e);
        }
    }

    FeedMeta::save(feeds)?;
    println!("Saved.");
    Ok(())
}

use futures::future::join_all;
pub fn update_all() -> Result<(), Box<dyn Error>> {
    let feeds: Vec<FeedMeta> = FeedMeta::load()?;
    if feeds.len() == 0 {
        println!("No feeds");
        return Ok(());
    }
    let client = reqwest::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(8))
        .build()?;

    let mut updated_feeds = Vec::with_capacity(feeds.len());
    for feed in feeds.iter() {
        updated_feeds.push(parser::pull(feed.url.as_str(), feed.filename.as_str(), &client));
    }

    // we create a runtime to run out updates in parallel
    let runtime = tokio::runtime::Runtime::new().unwrap_or_else(|e| {
        panic!("failed to create Tokio runtime: {}", e); // we can't really recover from this
    });

    runtime.block_on(join_all(updated_feeds));

    FeedMeta::save(feeds)?;
    println!("Saved.");
    Ok(())
}

pub fn read(source: &str) -> Result<(), Box<dyn Error>> {
    let mut feeds: Vec<FeedMeta> = FeedMeta::load()?;
    if feeds.len() == 0 {
        println!("No feeds");
        return Ok(());
    }
    let mut index = 0;
    for (i, feed) in feeds.iter().enumerate() {
        if feed.filename == source {
            index = i;
        }
    }
    let runtime = tokio::runtime::Runtime::new().unwrap_or_else(|e| {
        panic!("failed to create Tokio runtime: {}", e); // we can't really recover from this
    });

    let client = reqwest::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(8))
        .build()?;
    let feed = &mut feeds[index];

    match runtime.block_on(parser::pull(feed.url.as_str(), feed.filename.as_str(), &client)) {
        Ok(_) => {
            println!("Updated {}", feed.filename);
        }
        Err(e) => {
            println!("Error updating {}: {}", feed.filename, e);
        }
    }

    let filename = format!("{}.log", source);
    FeedMeta::re_index_and_mark_all_read(filename.as_str(), feed)?;

    FeedMeta::save(feeds)?;
    println!("Saved.");
    Ok(())
}

pub fn delete(source: &str) -> Result<(), Box<dyn Error>>{
    let mut feeds: Vec<FeedMeta> = FeedMeta::load()?;
    let mut index = 0;
    for (i, feed) in feeds.iter().enumerate() {
        if feed.filename == source {
            index = i;
        }
    }

    if index == 0 {
        println!("No such feed");
        return Ok(());
    }

    let feed = &mut feeds[index];
    let stripped = feed.filename.as_str().strip_suffix("feed.xml").unwrap();
    match std::fs::remove_dir_all(stripped) {
        Ok(_) => println!("Deleted"),
        Err(_) => println!("Error deleting"),
    }

    feeds.remove(index);

    FeedMeta::save(feeds)?;
    println!("Saved.");
    Ok(())
}

// RFC 2822 date sorting
fn sort_by_date(feeds: &Vec<FeedMeta>) -> Result<HashMap<String, Vec<FeedMeta>>, String> {
    let mut categorized: HashMap<String, Vec<FeedMeta>> = HashMap::new();
    for feed in feeds.iter() {
        let mut f = feed.clone();
        f.feed.items.sort_by(|a, b| {
            let date_a = DateTime::parse_from_rfc2822(&a.pub_date.as_ref().unwrap()).unwrap();
            let date_b = DateTime::parse_from_rfc2822(&b.pub_date.as_ref().unwrap()).unwrap();
            date_b.cmp(&date_a)
        });
        categorized.entry(f.category.clone())
            .or_insert(Vec::new())
            .push(f.clone());
    }
    Ok(categorized)
}

pub fn main(url: &str, category: &str) -> Result<HashMap<String, Vec<FeedMeta>>, Box<dyn Error>> {

    let path = path::data_dir().expect("Could not get data dir");
    let the_path = path.to_str().expect("Could not get path as str");

    println!("The current directory is {} and the suggested if", the_path);

    let root = Path::new(the_path);
    assert!(env::set_current_dir(&root).is_ok());

    // create dir
    match std::fs::create_dir_all("arcanum/feeds"){
        Ok(_) => println!("Created dir"),
        Err(_) => println!("Dir already exists"),
    }
    let the_path = format!("{}/arcanum/feeds", the_path);
    let root = Path::new(the_path.as_str());

    assert!(env::set_current_dir(&root).is_ok());

    // first run returns empty vec
    if url == "" {
        let mut categorized: HashMap<String, Vec<FeedMeta>> = HashMap::new();
        let feeds = FeedMeta::load()?;
        if feeds.len() == 0 {
            categorized.insert("Uncategorized".to_string(), feeds);
        } else {
            categorized = sort_by_date(&mut feeds.clone())?;
            return Ok(categorized)
        }
        return Ok(categorized);
    }

    let runtime = tokio::runtime::Runtime::new().unwrap_or_else(|e| {
        panic!("failed to create Tokio runtime: {}", e); // we can't really recover from this
    });

    match runtime.block_on(obtain_feed(url)) {
        Ok(channel) => {
            channel.write_to(::std::io::sink())?; // write to the channel to a writer

            let mut feeds: Vec<FeedMeta> = FeedMeta::load()?;
            let feed_meta = FeedMeta::new(url.clone(), category.clone())?;
            feeds.push(feed_meta);

            FeedMeta::save(feeds)?;
            let mut feeds = FeedMeta::load()?;

            feeds.sort_by(|a, b| a.category.cmp(&b.category));

            let categorized = sort_by_date(&mut feeds.clone())?;
            return Ok(categorized);
        }
        Err(e) => {
            println!("Real bad Error: {}", e);
        }
    }
    return Err("Error".into());
}
