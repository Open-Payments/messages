#!/bin/bash

input_directory=$1
output_directory=$2
xgen_path="../xgen/cmd/xgen/xgen"

for f in $input_directory/*.xsd; do
    xsd_file_name="${f//-/_}"
    mv "$f" "$xsd_file_name"
    rs_file_name="$output_directory/$(basename "${f%.xsd}" | sed 's/\./_/g').rs"
    $xgen_path -i "$xsd_file_name" -o "$rs_file_name" -l Rust
    sed -i "" '/\/\/ document \.\.\./,/^}$/d' "$rs_file_name"
    sed -i "" '/\/\/ app_hdr \.\.\./,/^}$/d' "$rs_file_name"
    sed -i "" '/\/\/ xchg \.\.\./,/^}$/d' "$rs_file_name"
    # perl -0777 -i -pe "s{\n\n\n}{}gs" "$rs_file_name"
done