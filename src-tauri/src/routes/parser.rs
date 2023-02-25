extern crate xml;

use std::fs::File;
use std::io::BufReader;
use rss::Channel;
use std::io;
use xml::reader::{EventReader, XmlEvent};

struct Feed {
    title: String,
    //link: String,
    description: String,
    last_build_date: String,
    generator: String,
    item: Vec<Item>,
}
struct Item {
    title: String,
    link: String,
    description: String,
    pub_date: String,
    guid: String,
    //enclosure: String,
    //category: String,
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

pub fn pull(url: &str) -> String {
    let resp = reqwest::blocking::get(url).expect("request failed");
    let body = resp.text().expect("body invalid");
    let url_stripped = url.replace("https://", "");
    let url_stripped = url_stripped.replace("/", "_");
    let url_stripped = url_stripped.replace(":", "_");
    let url_stripped = url_stripped.replace(".", "_");
    // concat strings
    let url_stripped = format!("../{}", url_stripped);

    let mut out = File::create(url_stripped.clone()).expect("failed to create file");
    io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
    return url_stripped;
}

// https://crates.io/crates/xml-rs
// https://mainmatter.com/blog/2020/12/31/xml-and-rust/
pub fn parse(file: String) {

    let mut record = Feed {
        title: String::new(),
        description: String::new(),
        last_build_date: String::new(),
        generator: String::new(),
        item: Vec::new(),
    };

    let file = File::open(file).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        let mut item = Item {
            title: String::new(),
            link: String::new(),
            description: String::new(),
            pub_date: String::new(),
            guid: String::new(),
        };
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}+{}", indent(depth), name);
                depth += 1;
            }
            Ok(XmlEvent::Characters(text)) => {
                item.description = text;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}