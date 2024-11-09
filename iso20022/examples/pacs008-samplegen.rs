use std::fs;
use serde_json;
use open_payments_iso20022::document::Document;
use open_payments_iso20022_pacs::pacs_008_001_12::FIToFICustomerCreditTransferV12;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the configuration file
    let config_str = fs::read_to_string("examples/pacs008-config.json")?;
    
    // Parse the configuration into a serde_json::Map
    let config_map: serde_json::Map<String, serde_json::Value> = 
        serde_json::from_str(&config_str)?;

    // Generate a sample using the configuration
    let pacs008 = FIToFICustomerCreditTransferV12::sample_with_config(&config_map)?;

    // Create the document
    let doc = Document::FIToFICustomerCreditTransferV12(Box::new(pacs008));

    println!("{:#?}", doc);
    println!("{}", serde_json::to_string_pretty(&doc).unwrap());

    Ok(())
}