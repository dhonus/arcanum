use rss::Channel;
use std::error::Error;
use tokio::runtime::Runtime;

use crate::routes::parser;

async fn example_feed(source: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(source)
        .await?
        .bytes()
        .await?;
    //println!("Content: {:?}", content);
    let channel = Channel::read_from(&content[..])?;
    channel.write_to(::std::io::sink()).unwrap(); // write to the channel to a writer
    Ok(channel)
}

pub fn main() -> String {
    println!("Hello from Rust!");
    const SOURCE: &str = "https://tofudreams.com/index.xml";

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut string = String::new(); // convert the channel to a string

    match runtime.block_on(example_feed(SOURCE)) {
        Ok(channel) => {
            println!("ok");
            channel.write_to(::std::io::sink()).unwrap(); // write to the channel to a writer
            let filename = parser::pull(SOURCE.clone());
            parser::parse(filename);
            string = channel.to_string();
            //println!("string: {}", string);
        }
        Err(e) => {
            println!("bad Error: {}", e);
        }
    }
    return string;
}
