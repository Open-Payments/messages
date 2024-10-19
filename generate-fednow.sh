#!/bin/bash

input_directory=$1
output_directory=$2
xgen_path="../xgen/cmd/xgen/xgen"

for f in $input_directory/*.xsd; do
    xsd_file_name="${f//-/_}"
    mv "$f" "$xsd_file_name"
    rs_file_name="$output_directory/$(basename "${f%.xsd}" | sed 's/\./_/g').rs"
    $xgen_path -i "$xsd_file_name" -o "$rs_file_name" -l Rust -p fednow
    # sed -i "" -e "/\/\/ document \.\.\./,/^}$/d" "$rs_file_name"
    # sed -i "" -e "/impl document {/,/^}$/d" "$rs_file_name"

    # sed -i "" -e "/\/\/ app_hdr \.\.\./,/^}$/d" "$rs_file_name"
    # sed -i "" -e "/impl app_hdr {/,/^}$/d" "$rs_file_name"

    # sed -i "" -e "/\/\/ xchg \.\.\./,/^}$/d" "$rs_file_name"
    # sed -i "" -e "/impl xchg {/,/^}$/d" "$rs_file_name"

    # perl -0777 -i -pe "s{\n\n\n}{}gs" "$rs_file_name"
done


# \t// document ...\n\tpub struct document {\n\t\t#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]\n\t\tpub document: Document,\n\t}\n\t\n\timpl document {\t\tpub fn validate(&self) -> Result<(), ValidationError> {\n\t\t\tif let Err(e) = self.document.validate() { return Err(e); }\n\t\t\tOk(())\n\t\t}\n\t}
