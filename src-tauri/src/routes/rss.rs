use rss::Channel;
use std::error::Error;
use std::path::Path;
use crate::routes::parser;
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::fs::{File, OpenOptions};
use tauri::utils::config::parse::read_from;

async fn obtain_feed(source: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(source)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    channel.write_to(::std::io::sink()).unwrap(); // write to the channel to a writer
    Ok(channel)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FeedMeta {
    pub url: String,
    pub filename: String,
    pub feed: Channel,
    pub read: Vec<String>,
}

impl FeedMeta {
    fn new(url: &str) -> FeedMeta {
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
        if !path.exists() {
            parser::pull(url, filename.as_str());
        }
        let feed = parser::parse(filename.clone());

        FeedMeta {
            url: url.to_string(),
            filename,
            feed,
            read: Vec::new(),
        }
    }
    fn load() -> Vec<FeedMeta> {
        match std::fs::create_dir("../feeds"){
            Ok(_) => println!("Created dir"),
            Err(_) => println!("Dir already exists"),
        }
        let mut feeds = Vec::new();
        match csv::Reader::from_path("../feeds/db.csv") {
            Ok(_) => println!("File exists"),
            Err(_) => {
                println!("File does not exist");
                return feeds;
            }
        }
        let mut reader = csv::Reader::from_path("../feeds/db.csv").unwrap();
        let headers = reader.headers();
        println!("Headers: {:?}", headers);
        for result in reader.records() {
            // The iterator yields Result<StringRecord, Error>, so we check the
            // error here.
            let record = result.unwrap();
            assert_eq!(record.len(), 3);
            let mut meta = FeedMeta::new(record.get(0).unwrap());
            println!("{:?}", record);

            let logFilename = format!("{}.log", meta.filename.clone());
            let log = FeedMeta::read_log(logFilename.as_str());
            match log {
                Ok(log) => {
                    meta.read = log;
                }
                Err(_) => {
                    println!("Error reading log");
                }
            }
            feeds.push(meta);
        }

        return feeds;
    }
    fn save(mut feeds: Vec<FeedMeta>) -> Vec<FeedMeta> {
        match std::fs::create_dir("../feeds"){
            Ok(_) => println!("Created dir"),
            Err(_) => println!("Dir already exists"),
        }

        // deduplicate feeds
        feeds.sort_by(|a, b| a.url.cmp(&b.url));
        feeds.dedup_by(|a, b| a.url == b.url);

        let mut writer = csv::Writer::from_path("../feeds/db.csv").unwrap();

        writer.write_record(&["url", "filename", "feed"]).unwrap();
        for entry in feeds.iter() {
            writer.write_record(&[entry.url.as_str(), entry.filename.as_str(), entry.feed.title.as_str()]).unwrap();
        }
        writer.flush().expect("Failed to flush writer");
        return feeds;
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
        writer.flush().unwrap();

        Ok(())
    }
}

pub fn mark_read(url: &str, guid: &str) {
    let filename = url
        .replace("https://", "")
          .replace("/", "_")
        .replace(":", "_")
        .replace(".", "_");
    let filename = format!("../feeds/{}/feed.xml.log", filename);
    FeedMeta::modify_file(filename.as_str(), guid).unwrap();
}

pub fn main(source: &str) -> Option<Vec<FeedMeta>> {
    match std::fs::create_dir("../feeds"){
        Ok(_) => println!("Created dir"),
        Err(_) => println!("Dir already exists"),
    }

    // first run returns empty vec
    if source == "" {
        let feeds = FeedMeta::load();
        return Some(feeds);
    }
    println!("here");

    let runtime = tokio::runtime::Runtime::new().unwrap();

    match runtime.block_on(obtain_feed(source)) {
        Ok(channel) => {
            channel.write_to(::std::io::sink()).unwrap(); // write to the channel to a writer

            let mut feeds: Vec<FeedMeta> = FeedMeta::load();
            let feed_meta = FeedMeta::new(source.clone());
            feeds.push(feed_meta);

            FeedMeta::save(feeds);
            let feeds = FeedMeta::load();

            return Some(feeds);
        }
        Err(e) => {
            println!("bad Error: {}", e);
        }
    }
    None
}
