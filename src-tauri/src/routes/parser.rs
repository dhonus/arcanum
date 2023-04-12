use std::fs::File;
use std::io::BufReader;
use std::{io};
use reqwest::Client;
use rss::{Channel};
use std::error::Error;
extern crate rand;

pub async fn pull(url: &str, filename: &str, client: &Client) -> Result<(), Box<dyn Error>> {
    use rand::distributions::{Alphanumeric, DistString};
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let string = format!("{}?{}", url, string);

    let resp = client.get(string).send().await;
    let body = resp?.text().await;
    println!("Trying to remove {}", filename);
    match std::fs::remove_file(filename){
        Ok(_) => println!("Removed file"),
        Err(_) => println!("File does not exist"),
    }
    println!("Trying to create file {}", filename);
    let stripped = filename.strip_suffix("feed.xml").unwrap();
    match std::fs::create_dir_all(stripped){
        Ok(_) => println!("Created dir"),
        Err(_) => println!("Dir already exists"),
    }

    let mut out = File::create(filename).expect("failed to create file");
    io::copy(&mut body?.as_bytes(), &mut out).expect("failed to copy content");
    
    Ok(())
}

pub fn parse(file: String) -> Channel {
    let xmlfile = File::open(file.clone()).unwrap();
    let channel = Channel::read_from(BufReader::new(xmlfile));
    return match channel {
        Ok(channel) => channel,
        Err(err) => {
            let mut bad_channel = Channel::default();
            bad_channel.title = "Error parsing".to_string();
            bad_channel.description = format!("{} cannot be parsed because: {}. To fix, try re-adding the feed, or checking the source website for errors.", file, err).to_string();
            bad_channel
        }
    }

}