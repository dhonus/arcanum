extern crate xml;

use std::fs::File;
use std::io::BufReader;
use std::{io};
use reqwest::Client;
use rss::{Channel};
extern crate rand;

pub async fn pull(url: &str, filename: &str, client: &Client) -> Result<(), String> {
    use rand::distributions::{Alphanumeric, DistString};
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let string = format!("{}?{}", url, string);

    let resp = client.get(string).send().await;
    match resp {
        Ok(resp) => {
            let body = resp.text().await;
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
            io::copy(&mut body.unwrap().as_bytes(), &mut out).expect("failed to copy content");
            Ok(())
        },
        Err(err) => Err(format!("Error in my_function: {}", err)),
    }
}

pub fn parse(file: String) -> Channel {
    let xmlfile = File::open(file.clone()).unwrap();
    let channel = Channel::read_from(BufReader::new(xmlfile)).unwrap();
    //println!("Channel: {:?}", channel);
    return channel;
}