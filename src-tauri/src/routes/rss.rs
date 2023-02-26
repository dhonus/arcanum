use rss::Channel;
use std::error::Error;
use std::path::Path;
use crate::routes::parser;

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
}

impl FeedMeta {
    fn new(url: &str) -> FeedMeta {
        let filename = url
            .replace("https://", "")
            .replace("/", "_")
            .replace(":", "_")
            .replace(".", "_");

        // check if file exists
        let filename = format!("../feeds/{}", filename);
        println!("Filename: {}", filename);
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
            feeds.push(FeedMeta::new(record.get(0).unwrap()));
            println!("{:?}", record);
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
}

pub fn main(source: &str) -> Option<Vec<FeedMeta>> {
    match std::fs::create_dir("../feeds"){
        Ok(_) => println!("Created dir"),
        Err(_) => println!("Dir already exists"),
    }

    if source == "" {
        let feeds = FeedMeta::load();
        return Some(feeds);
    }
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
