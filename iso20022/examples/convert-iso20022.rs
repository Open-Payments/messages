// Main entry point for the ISO20022 message parsing application.
// This application reads XML files from a specified directory,
// parses them using Serde, and converts them to JSON format.

use quick_xml::de::Deserializer;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::Path;
use serde_json;
use std::time::Instant;
use std::time::Duration;

use open_payments_iso20022::document::Document;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename = "Document")]
pub struct ISO20022Message {
    #[serde(rename = "$value")]
    document: Document,
}

fn main() {
    let mut total_duration = Duration::new(0, 0);
    let xml_directory = "samples";

    for entry in fs::read_dir(xml_directory).expect("Unable to read directory") {
        let entry = entry.expect("Unable to get directory entry");
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) == Some("xml") {
            parse_and_convert_to_json(&path, &mut total_duration);
        }
    }

    println!("\nTotal time taken for all conversions: {:.2?}", total_duration);
}

fn parse_and_convert_to_json(xml_path: &Path, total_duration: &mut Duration) {
    let file = File::open(xml_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let start = Instant::now();

    // Create quick-xml deserializer and deserialize directly
    let mut de = Deserializer::from_reader(reader);
    let result: Result<ISO20022Message, quick_xml::de::DeError> = ISO20022Message::deserialize(&mut de);

    let duration = start.elapsed();
    *total_duration += duration;

    print!("Converting {} (parsed in {:.2?})", xml_path.display(), duration);

    match result {
        Ok(data) => {
            let json_data = serde_json::to_string_pretty(&data).unwrap();
            let json_file_path = xml_path.with_extension("json");
            let mut json_file = File::create(&json_file_path).expect("Unable to create JSON file");
            json_file.write_all(json_data.as_bytes()).expect("Unable to write JSON data");
            print!("\t[Successfully]\n");
        },
        Err(e) => {
            print!("\t[Failed]\n");
            println!("Error: {}", e);
        }
    }
}