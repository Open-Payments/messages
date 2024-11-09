#!/bin/bash

sh generate-iso20022.sh xsd/iso20022/acmt iso20022-acmt/src
sh generate-iso20022.sh xsd/iso20022/admi iso20022-admi/src
sh generate-iso20022.sh xsd/iso20022/auth iso20022-auth/src
sh generate-iso20022.sh xsd/iso20022/camt iso20022-camt/src
sh generate-iso20022.sh xsd/iso20022/head iso20022-head/src
sh generate-iso20022.sh xsd/iso20022/pacs iso20022-pacs/src
sh generate-iso20022.sh xsd/iso20022/pain iso20022-pain/src
sh generate-iso20022.sh xsd/iso20022/reda iso20022-reda/src
sh generate-iso20022.sh xsd/iso20022/remt iso20022-remt/src
cp common.rs iso20022/src

cp common.rs fednow/src
sh generate-fednow.sh xsd/fednow fednow/src
sh generate-fednow.sh xsd/fednow/fednow fednow/src/fednow_extra
sh generate-fednow.sh xsd/fednow/iso fednow/src/iso

cat fednow/src/iso/common.rs >> fednow/src/common.rs
rm fednow/src/iso/common.rs
cat fednow/src/fednow_extra/common.rs >> fednow/src/common.rs
rm fednow/src/fednow_extra/common.rs
