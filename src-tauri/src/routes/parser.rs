extern crate xml;

use std::fs::File;
use std::io::BufReader;
use std::io;
use xml::reader::{EventReader, XmlEvent};

#[derive(serde::Serialize)]
pub struct Feed {
    pub title: String,
    //link: String,
    pub description: String,
    pub last_build_date: String,
    pub generator: String,
    pub items: Vec<Item>,
}

#[derive(serde::Serialize)]
pub struct Item {
    pub title: String,
    pub link: String,
    pub description: String,
    pub content: String,
    pub pub_date: String,
    pub guid: String,
}

impl Item {
    fn new() -> Item {
        Item {
            title: String::new(),
            link: String::new(),
            description: String::new(),
            content: String::new(),
            pub_date: String::new(),
            guid: String::new(),
        }
    }
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

pub fn pull(url: &str, filename: &str) {
    let resp = reqwest::blocking::get(url).expect("request failed");
    let body = resp.text().expect("body invalid");

    let mut out = File::create(filename).expect("failed to create file");
    io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
}

// https://crates.io/crates/xml-rs
// https://mainmatter.com/blog/2020/12/31/xml-and-rust/
pub fn parse(file: String) -> Feed {
    let mut record = Feed {
        title: String::new(),
        description: String::new(),
        last_build_date: String::new(),
        generator: String::new(),
        items: Vec::new(),
    };

    let file = File::open(file).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;

    let mut current_field: Option<String> = None;
    let mut item = Item::new();

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}+{}", indent(depth), name);
                depth += 1;
                // Set the current field based on the element name
                match name.local_name.as_ref() {
                    "item" => current_field = Some("item".to_string()),
                    "title" => current_field = Some("title".to_string()),
                    "link" => current_field = Some("link".to_string()),
                    "description" => current_field = Some("description".to_string()),
                    "content" => current_field = Some("content".to_string()),
                    "pubDate" => current_field = Some("pub_date".to_string()),
                    "guid" => current_field = Some("guid".to_string()),
                    _ => {}
                }
            }
            Ok(XmlEvent::Characters(text)) => {
                // Add the text to the appropriate field based on the current field flag
                if let Some(field) = current_field.as_ref() {
                    match field.as_ref() {
                        "title" => {
                            if item.title == "" {
                                item.title = text
                            }
                        }
                        "link" => item.link = text,
                        "description" => {
                            if item.description == "" {
                                item.description = text;
                            }
                        }
                        "content" => item.content = text,
                        "pub_date" => item.pub_date = text,
                        "guid" => item.guid = text,
                        _ => {}
                    }
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                //println!("{}-{}", indent(depth), name);
                // Clear the current field flag when the end element is encountered
                match name.local_name.as_ref() {
                    "title" | "link" | "description" | "pubDate" | "guid" => {
                        current_field = None
                    }
                    "item" => {
                        record.items.push(item);
                        item = Item::new();
                    }
                    _ => {}
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    return record;
}