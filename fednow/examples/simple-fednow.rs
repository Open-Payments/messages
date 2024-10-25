// Main entry point for the FedNow message object application.
// This application cretes a simple object for FedNow message.

use open_payments_fednow::document::*;
use open_payments_fednow::iso::pacs_008_001_08::FIToFICustomerCreditTransferV08;
use serde_xml_rs;

fn main() {
    // Create the document object with the FedNow message
    let mut doc = Document::FIToFICustomerCreditTransferV08(Box::new(FIToFICustomerCreditTransferV08::default()));

    // Match the Document enum and modify the field
    if let Document::FIToFICustomerCreditTransferV08(ref mut message) = doc {
        message.grp_hdr.msg_id = "Hello".to_string();
        println!("{:?}", message.validate());
    }

    let reserialized_item = serde_xml_rs::to_string(&doc).unwrap();

    println!("{:?}", reserialized_item)
}