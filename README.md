# Open Payments - Message Parsing Library

[![Apache License 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](http://www.apache.org/licenses/LICENSE-2.0)
[![Contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg)](CONTRIBUTING.md)
[![GitHub issues](https://img.shields.io/github/issues/Open-Payments/messages)](https://github.com/Open-Payments/messages/issues)

The Open Payments library provides tools for parsing, validating, and transforming financial messages, with support for ISO 20022 and FedNow message formats. The library is designed to help developers integrate financial message handling into their Rust applications, using serde for (de)serialization.

---

## Features

- **ISO 20022 Support**: Comprehensive support for key ISO 20022 payment message types.
- **FedNow Message**: Full support for FedNow message formats.
- **(De)serialization**: Using serde for easy conversion between XML and JSON.
- **Extensibility**: Easily extendable to support additional message types or custom formats.

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
# This dependency includes support for the entire ISO 20022 message formats.
# The "payments" feature enables various ISO 20022 message categories, such as pacs, pain, camt, etc.
# If you only need specific message types, you can enable just those features (e.g., "pacs", "pain").
open-payments-iso20022 = { version = "0.2.1", features = ["payments"] }

# This dependency provides support for the FedNow message formats.
# You get full support for parsing and serializing FedNow messages out of the box.
open-payments-fednow = "0.2.1"
```

### Features

The ISO20022 message library `open-payments-iso20022` provides several features to allow you to include only the message types relevant to your use case. Here’s a breakdown of the available features:

```toml
[features]
default = ["head"]  # Default feature, includes the basic header message.
iso20022 = ["payments"]  # Enables all payment-related ISO 20022 messages.
payments = ["acmt", "admi", "auth", "camt", "head", "pacs", "pain", "reda", "remt"]  # Includes all payments-related ISO 20022 message types.

# Individual ISO 20022 message modules:
acmt = ["open-payments-iso20022-acmt"]  # Account Management messages
admi = ["open-payments-iso20022-admi"]  # Administrative messages
auth = ["open-payments-iso20022-auth"]  # Authorization messages
camt = ["open-payments-iso20022-camt"]  # Cash Management messages
head = ["open-payments-iso20022-head"]  # Basic Header messages (default)
pacs = ["open-payments-iso20022-pacs"]  # Payment Clearing and Settlement messages
pain = ["open-payments-iso20022-pain"]  # Payment Initiation messages
reda = ["open-payments-iso20022-reda"]  # Reference Data messages
remt = ["open-payments-iso20022-remt"]  # Remittance Advice messages
```

By configuring the features, you can optimize the library for your specific message requirements, minimizing unnecessary dependencies.

### Usage

**Example: Creating an ISO 20022 Message Object**
```rust
use open_payments_iso20022::document::Document;
use open_payments_iso20022_admi::admi_002_001_01::Admi00200101;

fn main() {
    let doc = Document::Admi00200101(Box::new(Admi00200101::default()));

    println!("{:?}", doc)
}
```

**Example: Creating a FedNow Message Object**

Similarly, here’s an example of how to create a FedNow message object:

```rust
use open_payments_fednow::document::Document;
use open_payments_fednow::iso::pacs_008_001_08::FIToFICustomerCreditTransferV08;

fn main() {
    let doc = Document::FIToFICustomerCreditTransferV08(Box::new(FIToFICustomerCreditTransferV08::default()));

    println!("{:?}", doc)
}
```

### Supported Messages

The library supports a variety of financial message formats from both ISO 20022 and FedNow, covering key areas of the payment lifecycle.

#### ISO 20022 Messages
- **pacs**: Payment Clearing and Settlement
- **pain**: Payment Initiation
- **admi**: Administrative messages
- **auth**: Authorization messages
- **camt**: Cash Management
- **reda**: Reference Data
- **remt**: Remittance Advice
- **acmt**: Account Management
- **head**: Header messages

#### FedNow Messages
- **Customer Credit Transfer**
- **Payment Status Report**
- **Payment Return**

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

- Add a Sophisticated Error Framework for XML Parsing and Validation
- Add Support for Format Transformation (ISO20022 -> MT103)
- Add Examples for Format Conversion
- Performance optimizations for large message batches.
- Expanded documentation and examples.

You can view the complete roadmap [here](ROADMAP.md).

---

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE) file for details.

---

## Contact

For any questions or discussions, please reach out via GitHub Issues on our [GitHub Issues](https://github.com/Open-Payments/messages/issues) page.
