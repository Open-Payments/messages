// Main entry point for the FedNow message parsing application.
// This application reads XML files from a specified directory,
// parses them using Serde, and converts them to JSON format.

use serde_xml_rs;  // For parsing XML into Rust structs
use serde_path_to_error::deserialize;  // For error reporting with detailed path info
use std::fs::{self, File};  // File system utilities for reading files
use std::io::{BufReader, Write};  // For reading and writing files
use std::path::Path;  // Path utility for handling file paths
use std::time::Instant;  // For tracking time elapsed
use xml::reader::EventReader;  // XML event-based parser
use serde_json;  // JSON serialization utility

use open_payments_fednow::FednowMessage;  // Import the FednowMessage struct

fn main() {
    // Define the directory containing XML files
    let xml_directory = "samples";
    
    // Start measuring the execution time
    let start_time = Instant::now();

    // Run the conversion 100 times for benchmarking purposes
    for _ in 0..100 {
        // Iterate over all files in the XML directory
        for entry in fs::read_dir(xml_directory).expect("Unable to read directory") {
            let entry = entry.expect("Unable to get directory entry");
            let path = entry.path();

            // Check if the file has an .xml extension
            if path.extension().and_then(|e| e.to_str()) == Some("xml") {
                // If it's an XML file, parse and convert it to JSON
                parse_and_convert_to_json(&path);
            }
        }
    }

    // Calculate and print the elapsed time for the whole operation
    let elapsed = start_time.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

// Function to parse an XML file and convert its content to a JSON file
fn parse_and_convert_to_json(xml_path: &Path) {
    // Open the XML file for reading
    let file = File::open(xml_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    // Wrap the BufReader in an EventReader to parse the XML
    let event_reader = EventReader::new(reader);

    // Use serde_path_to_error to capture detailed error paths during deserialization
    let mut deserializer = serde_xml_rs::Deserializer::new(event_reader);
    
    // Deserialize the XML into a FednowMessage struct
    let result: Result<FednowMessage, serde_path_to_error::Error<serde_xml_rs::Error>> = deserialize(&mut deserializer);

    // Log the conversion status
    print!("Converting {}", xml_path.display());

    match result {
        Ok(data) => {
            // Convert the deserialized data into pretty-printed JSON
            let json_data = serde_json::to_string_pretty(&data).unwrap();

            // Create a new file path for the JSON output (same name as XML but with .json extension)
            let json_file_path = xml_path.with_extension("json");

            // Write the JSON data to the newly created file
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