use rss::Channel;
use std::error::Error;
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
        parser::pull(url, filename.as_str());

        let feed = parser::parse(filename.clone());

        FeedMeta {
            url: url.to_string(),
            filename,
            feed
        }
    }
}

pub fn main(source: &str) -> Option<FeedMeta> {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    match runtime.block_on(obtain_feed(source)) {
        Ok(channel) => {
            channel.write_to(::std::io::sink()).unwrap(); // write to the channel to a writer
            let feed_meta = FeedMeta::new(source.clone());
            return Some(feed_meta);
        }
        Err(e) => {
            println!("bad Error: {}", e);
        }
    }
    None
}
