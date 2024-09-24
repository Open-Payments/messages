# Project Roadmap

This document outlines the development roadmap for the Open Payments message parsing library. The roadmap is designed to guide future improvements, features, and examples for the library.

---

## Roadmap

### 1. Basic Message Library
- **Goal**: Build the core message library based on `serde` for (de)serialization of ISO 20022 and FedNow messages.
- **Status**: ✅ Completed
- **Details**: The foundational message structures have been implemented, with support for (de)serialization of key ISO 20022 messages using Serde.

### 2. XSD to Library Struct Conversion Automation (In Progress)
- **Goal**: Automate the conversion of XSD schemas into Rust library structs to streamline the development process.
- **Details**: Develop a tool or script that generates Rust structs directly from XSD files, ensuring that the message structures remain in sync with the evolving FedNow and ISO 20022 standards.
- **Status**: ✅ Completed

### 3. Add FedNow Messages into the Library
- **Goal**: Expand the library to include full support for FedNow message formats.
- **Details**: Integrate FedNow-specific messages into the existing library structure, enabling seamless parsing and serialization for these formats.
- **Status**: ✅ Completed

### 4. Write Example Code for Parsing All Sample FedNow XMLs
- **Goal**: Provide comprehensive example code to demonstrate parsing of all available FedNow sample XMLs.
- **Details**: Create example scripts to show how developers can use the library to parse FedNow XMLs into structured data and convert them into JSON format.
- **Status**: ✅ Completed

### 5. Add a Sophisticated Error Framework for XML Parsing and Validation
- **Goal**: Introduce a robust error-handling framework that provides in-depth error information during XML parsing and validation.
- **Details**: Implement error reporting mechanisms to catch and report specific issues in the XML structure, schema validation errors, or content mismatches.
- **Status**: ⚙️ Work in progress

### 6. Add Support for Format Transformation (ISO20022 -> MT103)
- **Goal**: Enable transformation of one message format to another (e.g., ISO 20022 to MT103) within the library.
- **Details**: Create a system that allows users to transform between different financial message formats, using configuration options to control the mapping.
- **Status**: ⏳ Planned

### 7. Add Examples for Format Conversion
- **Goal**: Provide example scripts showing how to convert between different message formats using simple configuration files.
- **Details**: Demonstrate how to configure and run format conversions (e.g., ISO20022 to MT103) using example XML or JSON files.
- **Status**: ⏳ Planned

---

## Future Ideas

- **Improved Performance Optimizations**: Explore opportunities to enhance the library’s performance, particularly for handling large message batches.
- **Enhanced Documentation**: Add detailed guides for each message type and example code in multiple languages, if applicable.
- **Community Contributions**: Actively encourage contributions to extend support for additional message formats or tools.

---

## How to Contribute

If you're interested in helping us achieve any of these milestones, feel free to check out our [Contributing Guide](CONTRIBUTING.md) and open an issue or pull request on our [GitHub repository](https://github.com/Open-Payments/messages).

---
