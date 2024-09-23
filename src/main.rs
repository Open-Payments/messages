use serde_xml_rs::from_reader;
use serde_path_to_error::deserialize;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::Path;
use std::time::Instant;
use xml::reader::EventReader;
use serde_json::json;

use messages::FednowMessage;

fn main() {
    // Define the directory containing XML files
    let xml_directory = "xml";
    let start_time = Instant::now();

    for n in 0..100 {
        // Iterate over all files in the directory
        for entry in fs::read_dir(xml_directory).expect("Unable to read directory") {
            let entry = entry.expect("Unable to get directory entry");
            let path = entry.path();

            // Check if the file has an .xml extension
            if path.extension().and_then(|e| e.to_str()) == Some("xml") {
                // Parse the XML file and convert to JSON
                parse_and_convert_to_json(&path);
            }
        }
    }
    let elapsed = start_time.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn parse_and_convert_to_json(xml_path: &Path) {
    // Open the XML file
    let file = File::open(xml_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    // Wrap the BufReader in an EventReader
    let event_reader = EventReader::new(reader);

    // Use serde_path_to_error to get the path where deserialization failed
    let mut deserializer = serde_xml_rs::Deserializer::new(event_reader);
    let result: Result<FednowMessage, serde_path_to_error::Error<serde_xml_rs::Error>> = deserialize(&mut deserializer);

    print!("Converting {}", xml_path.display());

    match result {
        Ok(data) => {
            // Convert to pretty JSON
            let json_data = serde_json::to_string_pretty(&data).unwrap();

            // Create the new JSON file path (same name as XML but with .json extension)
            let json_file_path = xml_path.with_extension("json");

            // Write the JSON data to the new file
            let mut json_file = File::create(&json_file_path).expect("Unable to create JSON file");
            json_file.write_all(json_data.as_bytes()).expect("Unable to write JSON data");

            print!("\t[Successfully]\n");
        },
        Err(e) => {
            print!("\t[Failed]\n");
            println!("Deserialization error in {} at path: {}", xml_path.display(), e.path());
            println!("Full error: {}", e);
        }
    }
}