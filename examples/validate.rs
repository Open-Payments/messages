use serde_xml_rs;  // For parsing XML into Rust structs
use serde_path_to_error::deserialize;  // For error reporting with detailed path info
use serde_valid::Validate;  // For validating deserialized structs
use std::fs::{self, File};  // File system utilities for reading files
use std::io::BufReader;  // For reading files
use std::path::Path;  // Path utility for handling file paths
use xml::reader::EventReader;  // For creating an EventReader from BufReader

use payment_message::FednowMessage;  // Import the FednowMessage struct, ensure it's `Validate`

fn main() {
    // Define the directory containing XML files
    let xml_directory = "xml";

    // Iterate over all files in the XML directory
    for entry in fs::read_dir(xml_directory).expect("Unable to read directory") {
        let entry = entry.expect("Unable to get directory entry");
        let path = entry.path();

        // Check if the file has an .xml extension
        if path.extension().and_then(|e| e.to_str()) == Some("xml") {
            // If it's an XML file, validate its content
            validate_xml(&path);
        }
    }
}

// Function to parse and validate an XML file
fn validate_xml(xml_path: &Path) {
    // Open the XML file for reading
    let file = File::open(xml_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    // Create an EventReader from the BufReader
    let event_reader = EventReader::new(reader);

    // Wrap the EventReader in serde_xml_rs's Deserializer
    let mut deserializer = serde_xml_rs::Deserializer::new(event_reader);

    // Deserialize the XML into a FednowMessage struct to validate
    let result: Result<FednowMessage, serde_path_to_error::Error<serde_xml_rs::Error>> = deserialize(&mut deserializer);

    // Log the validation status
    print!("Validating {}", xml_path.display());

    match result {
        Ok(data) => {
            // If deserialization is successful, perform validation using `serde_valid`
            match data.validate() {
                Ok(_) => {
                    // If validation passes
                    print!("\t[Valid]\n");
                },
                Err(validation_errors) => {
                    // If validation fails, print!("\t[Validation Failed]\n");
                    println!("Validation errors: {:?}", validation_errors);
                }
            }
        },
        Err(e) => {
            // If there's an error during deserialization, log the failure and error details
            print!("\t[Invalid]\n");
            println!("Deserialization error in {} at path: {}", xml_path.display(), e.path());
            println!("Full error: {}", e);
        }
    }
}