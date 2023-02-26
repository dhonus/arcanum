use rss::Channel;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use crate::routes::parser;
use serde::Deserialize;

async fn obtain_feed(source: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(source)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    channel.write_to(::std::io::sink()).unwrap(); // write to the channel to a writer
    Ok(channel)
}

#[derive(Debug, Deserialize, Clone)]
pub struct FeedMeta {
    pub url: String,
    pub filename: String,
    pub feed: parser::Feed
}

impl FeedMeta {
    fn new(url: &str) -> FeedMeta {
        let filename = url
            .replace("https://", "")
            .replace("/", "_")
            .replace(":", "_")
            .replace(".", "_");

        let filename = format!("../{}", filename);
        // check if file exists
        let path = Path::new(filename.as_str());
        if !path.exists() {
            parser::pull(url, filename.as_str());
        }
        let feed = parser::parse(filename.clone());

        FeedMeta {
            url: url.to_string(),
            filename,
            feed
        }
    }
    fn load() -> Vec<FeedMeta> {
        let mut feeds = Vec::new();
        let mut reader = csv::Reader::from_path("../db.csv").unwrap();
        let headers = reader.headers();
        println!("Headers: {:?}", headers);
        for result in reader.records() {
            // The iterator yields Result<StringRecord, Error>, so we check the
            // error here.
            let record = result.unwrap();
            assert_eq!(record.len(), 3);
            feeds.push(FeedMeta::new(record.get(0).unwrap()));
            println!("{:?}", record);
        }

        return feeds;
    }
    fn save(feeds: Vec<FeedMeta>) -> Vec<FeedMeta> {
        use std::fs::OpenOptions;
        use std::io::prelude::*;

        let mut writer = csv::Writer::from_path("../db.csv").unwrap();

        writer.write_record(&["url", "filename", "feed"]).unwrap();
        for entry in feeds.iter() {
            writer.write_record(&[entry.url.as_str(), entry.filename.as_str(), entry.feed.title.as_str()]).unwrap();
        }
        writer.flush().expect("Failed to flush writer");
        return feeds;
    }
}

pub fn main(source: &str) -> Option<Vec<FeedMeta>> {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    match runtime.block_on(obtain_feed(source)) {
        Ok(channel) => {
            channel.write_to(::std::io::sink()).unwrap(); // write to the channel to a writer

            let mut feeds: Vec<FeedMeta> = Vec::new();
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
