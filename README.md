# Open Payments - Message Parsing Library

[![Apache License 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](http://www.apache.org/licenses/LICENSE-2.0)
[![Contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg)](CONTRIBUTING.md)
[![GitHub issues](https://img.shields.io/github/issues/Open-Payments/messages)](https://github.com/Open-Payments/messages/issues)

Open Payments is a robust message parsing library designed to handle various message types from the **FedNow** and **ISO 20022** standards. This library makes it easier to integrate payment messaging formats into your financial applications using efficient parsing, serialization, and deserialization capabilities.

---

## Features

- **Comprehensive Message Support**: Supports a wide range of ISO 20022 message types such as `admi`, `camt`, `pacs`, and `pain`.
- **FedNow Ready**: Fully compatible with FedNow messaging specifications.
- **Serde-powered**: Built with Serde for fast and efficient (de)serialization to/from XML and JSON formats.
- **Easy Integration**: Simple APIs to parse, convert, and handle payment messages within your Rust applications.
- **Extensible**: Designed to be open-source and easy to contribute to, making it suitable for custom implementations.

---

## Getting Started

### Prerequisites

You’ll need the following installed to build and use this library:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (Rust package manager)

### Installation

Add the following to your `Cargo.toml` to start using the library in your Rust project:

```toml
[dependencies]
open-payments = "0.1.0"
```

### Usage

Here's an example of how to parse a FedNow message and convert it to JSON:

```rust
use open_payments::FednowMessage;
use serde_json::json;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("path_to_fednow_message.xml").expect("Unable to open file");
    let reader = BufReader::new(file);

    let message: FednowMessage = serde_xml_rs::from_reader(reader).expect("Unable to parse XML");

    // Convert to JSON
    let json_message = json!(message);
    println!("{}", serde_json::to_string_pretty(&json_message).unwrap());
}
```

### Supported Messages

The library supports various message formats defined by ISO 20022, including:

- **admi.002.001.01**: Administrative notifications - Provides status information regarding system events.
- **admi.004.001.02**: System event notification - Notifies about system events such as maintenance, downtime, or system changes.
- **admi.006.001.01**: Resend request - Requests the resend of previously sent messages.
- **admi.007.001.01**: Receipt acknowledgement - Confirms the receipt of a message.
- **admi.011.001.01**: System event acknowledgement - Acknowledges a system event notification.
- **admi.998.001.02**: Administration proprietary message - Custom or proprietary administration messages for specific use cases.

- **camt.026.001.07**: Payment investigation - Used to initiate an investigation into a payment.
- **camt.028.001.09**: Resolution of investigation - Provides the outcome of a previously initiated payment investigation.
- **camt.029.001.09**: Unable to apply - Used to notify that a payment cannot be applied as expected.
- **camt.052.001.08**: Bank-to-customer account report - Reports on an account's transactions and balances.
- **camt.054.001.08**: Bank-to-customer debit credit notification - Notifies about debits and credits to an account.
- **camt.055.001.09**: Customer payment cancellation request - Requests the cancellation of a payment transaction.
- **camt.056.001.08**: FIToFI payment cancellation request - Requests the cancellation of a payment sent between financial institutions.
- **camt.060.001.05**: Account reporting request - Requests detailed information regarding transactions on an account.

- **pacs.002.001.10**: FIToFI payment status report - Provides the status of a payment sent between financial institutions.
- **pacs.004.001.10**: Payment return - Returns a payment that was unable to be applied.
- **pacs.008.001.08**: FIToFI customer credit transfer - Facilitates the transfer of funds between financial institutions for customer-initiated credit transfers.
- **pacs.009.001.08**: Financial institution credit transfer - Transfers funds between financial institutions.
- **pacs.028.001.03**: FIToFI payment status request - Requests the status of a payment sent between financial institutions.

- **pain.013.001.07**: Creditor payment activation request - Initiates a payment request from the creditor’s side.
- **pain.014.001.07**: Creditor payment activation request status report - Provides a status report for a payment activation request.

---

This extensive support for ISO 20022 messages enables comprehensive coverage of the payment message lifecycle, including administrative processes, investigations, status reports, and transaction instructions.

## Contributing

We welcome contributions from developers to help improve the library. Whether you’re fixing a bug, improving documentation, or adding new message types, your help is appreciated!

1. Fork the repository.
2. Create a new branch with your feature or fix: `git checkout -b feature-branch-name`.
3. Commit your changes: `git commit -m 'Add some feature'`.
4. Push to the branch: `git push origin feature-branch-name`.
5. Open a pull request.

Make sure to review our [contributing guidelines](CONTRIBUTING.md) before you start.

---

## Roadmap

Here’s what’s coming up:

- Support for additional ISO 20022 message types.
- Performance optimizations for large message batches.
- Expanded documentation and examples.

---

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE) file for details.

---

## Contact

For any questions or discussions, please reach out via GitHub Issues on our [GitHub Issues](https://github.com/Open-Payments/messages/issues) page.
