// Main entry point for the FedNow message object application.
// This application cretes a simple object for FedNow message.

use open_payments_fednow::document::Document;
use open_payments_fednow::iso::pacs_008_001_08::FIToFICustomerCreditTransferV08;

fn main() {
    let doc = Document::FIToFICustomerCreditTransferV08(Box::new(FIToFICustomerCreditTransferV08::default()));
    let json_data = serde_json::to_string_pretty(&doc).unwrap();

    println!("{:?}", json_data)
}