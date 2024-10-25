// Main entry point for the ISO20022 message object application.
// This application cretes a simple object for ISO20022 message.

use open_payments_iso20022::document::Document;
use open_payments_iso20022_admi::admi_002_001_01::Admi00200101;

fn main() {
    let doc = Document::Admi00200101(Box::new(Admi00200101::default()));

    println!("{:?}", doc)
}
