extern crate xml;

use std::fs::File;
use std::io::BufReader;
use std::io;
use rss::Channel;

pub fn pull(url: &str, filename: &str) {
    let resp = reqwest::blocking::get(url).expect("request failed");
    let body = resp.text().expect("body invalid");

    let mut out = File::create(filename).expect("failed to create file");
    io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
}

pub fn parse(file: String) -> Channel {
    let xmlfile = File::open(file.clone()).unwrap();
    let channel = Channel::read_from(BufReader::new(xmlfile)).unwrap();
    //println!("Channel: {:?}", channel);
    return channel;
}