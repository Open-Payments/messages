#!/bin/bash

# ISO 20022 XSDs
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

mv iso20022-acmt/src/common.rs open-payments-common/src/common-acmt.rs
mv iso20022-admi/src/common.rs open-payments-common/src/common-admi.rs
mv iso20022-auth/src/common.rs open-payments-common/src/common-auth.rs
mv iso20022-camt/src/common.rs open-payments-common/src/common-camt.rs
mv iso20022-head/src/common.rs open-payments-common/src/common-head.rs
mv iso20022-pacs/src/common.rs open-payments-common/src/common-pacs.rs
mv iso20022-pain/src/common.rs open-payments-common/src/common-pain.rs
mv iso20022-reda/src/common.rs open-payments-common/src/common-reda.rs
mv iso20022-remt/src/common.rs open-payments-common/src/common-remt.rs
cp common.rs open-payments-common/src
python3 generate-common.py open-payments-common/src 0
rm open-payments-common/src/common-acmt.rs
rm open-payments-common/src/common-admi.rs
rm open-payments-common/src/common-auth.rs
rm open-payments-common/src/common-camt.rs
rm open-payments-common/src/common-head.rs
rm open-payments-common/src/common-pacs.rs
rm open-payments-common/src/common-pain.rs
rm open-payments-common/src/common-reda.rs
rm open-payments-common/src/common-remt.rs

# FedNow XSDs
cp common.rs fednow/src
sh generate-fednow.sh xsd/fednow fednow/src
sh generate-fednow.sh xsd/fednow/fednow fednow/src/fednow_extra
sh generate-fednow.sh xsd/fednow/iso fednow/src/iso

cat fednow/src/iso/common.rs >> fednow/src/common.rs
rm fednow/src/iso/common.rs
cat fednow/src/fednow_extra/common.rs >> fednow/src/common.rs
rm fednow/src/fednow_extra/common.rs
