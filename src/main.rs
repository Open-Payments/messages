use serde_xml_rs::from_reader;
use serde_path_to_error::deserialize;
use std::fs::File;
use std::io::BufReader;
use xml::reader::EventReader;

use messages::message::fednow::fednow_incoming_external::FedNowIncoming;

fn main() {
    let file = File::open("xml/fednow-sample-message-pacs008.xml").expect("Unable to open file");
    let reader = BufReader::new(file);

    // Wrap the BufReader in an EventReader
    let event_reader = EventReader::new(reader);

    // Use serde_path_to_error to get the path where deserialization failed
    let mut deserializer = serde_xml_rs::Deserializer::new(event_reader);
    let result: Result<FedNowIncoming, serde_path_to_error::Error<serde_xml_rs::Error>> =
        deserialize(&mut deserializer);

    match result {
        Ok(data) => println!("Successfully deserialized: {:?}", data),
        Err(e) => {
            println!("Deserialization error at path: {}", e.path());
            println!("Full error: {}", e);
        }
    }
}