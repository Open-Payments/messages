#!/bin/bash

input_directory=$1
output_directory=$2
xgen_path="../xgen/cmd/xgen/xgen"

for f in $input_directory/*.xsd; do
    xsd_file_name="${f//-/_}"
    mv "$f" "$xsd_file_name"
    rs_file_name="$output_directory/$(basename "${f%.xsd}" | sed 's/\./_/g').rs"
    $xgen_path -i "$xsd_file_name" -o "$rs_file_name" -l Rust -p fednow
done

python3 generate-common.py $output_directory
