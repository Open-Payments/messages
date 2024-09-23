#!/bin/bash

# Directory to search for input XSD files
input_directory="./xsd"
cd $input_directory

# Directory name for the generated Rust files
library_directory_name="message"

# Directory to write output Rust files (root of the Rust files)
output_directory="../src/"

# Original file extension to search (e.g., .xsd)
input_extension=".xsd"

# New file extension for the generated output (e.g., .rs)
output_extension=".rs"

# Command to convert XSD to Rust
command="/Users/codetiger/Development/xgen/cmd/xgen/xgen"

# Start fresh by cleaning up the previous generated files
rm -rf "$output_directory$library_directory_name"
mkdir -p "$output_directory$library_directory_name"

# Replace dashes in the filenames with underscores
for f in `find "." -iname "*.xsd" -type f -print`; do
    mv "$f" "${f//-/_}"
done

$command -i "." -o "$output_directory$library_directory_name" -l Rust

# Clean up unwanted files like .DS_Store.rs
for f in `find "$output_directory$library_directory_name" -iname "*DS_Store.rs" -type f -print`; do
    rm -rf "$f"
done

# Rename .xsd.rs files to .rs
for f in `find "$output_directory$library_directory_name" -iname "*.xsd.rs" -type f -print`; do
    mv "$f" "${f%.xsd.rs}.rs"
done

# Rename files with '.' in the file names to use '_', excluding the extension
find "$output_directory$library_directory_name" -type f -name "*.rs" | while read -r file; do
    dir=$(dirname "$file")
    base_name=$(basename "$file" .rs | sed 's/\./_/g')  # Replace '.' with '_' only in the file name
    mv "$file" "$dir/$base_name.rs"  # Append the .rs extension correctly
done

# Path to lib.rs
lib_file=$output_directory"lib.rs"

# Start fresh by cleaning up the previous lib.rs
echo "// Auto-generated lib.rs" > "$lib_file"
echo 'pub mod message;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

use crate::message::fednow::fednow_incoming_external::FedNowIncoming;
use crate::message::fednow::fednow_outgoing_external::FedNowOutgoing;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum FednowMessage {
    #[serde(rename = "FedNowIncoming")]
    FedNowIncoming(Box<FedNowIncoming>),

    #[serde(rename = "FedNowOutgoing")]
    FedNowOutgoing(Box<FedNowOutgoing>),
}' >> "$lib_file"

# Function to generate mod.rs in each directory
generate_mod_rs() {
    local dir="$1"
    local mod_file="$dir/mod.rs"

    # Create or overwrite the mod.rs file
    echo "// Auto-generated mod.rs for $(basename "$dir")" > "$mod_file"

    # Add `pub mod` for each subdirectory
    for sub_dir in "$dir"/*/; do
        if [ -d "$sub_dir" ]; then
            module_name=$(basename "$sub_dir" | sed 's/-/_/g')  # Replace - with _ in module name
            echo "pub mod $module_name;" >> "$mod_file"
        fi
    done

    # Add `pub mod` for each .rs file in the directory
    for rs_file in "$dir"/*.rs; do
        if [ "$(basename "$rs_file")" != "mod.rs" ]; then
            module_name=$(basename "$rs_file" .rs)
            echo "pub mod $module_name;" >> "$mod_file"
        fi
    done
}

# Recursively find directories and generate mod.rs for each
find "$output_directory$library_directory_name" -type d | while read -r dir; do
    generate_mod_rs "$dir"
done