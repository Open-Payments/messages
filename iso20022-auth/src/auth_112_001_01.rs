// Open Payment Message Parsing Library
// https://github.com/Open-Payments/messages
//
// This library is designed to parse message formats based on the ISO 20022 standards,
// including but not limited to FedNow messages. It supports various financial message types,
// such as customer credit transfers, payment status reports, administrative notifications, 
// and other ISO 20022 messages, using Serde for efficient serialization and deserialization.
//
// Copyright (c) 2024 Open Payments by Harishankar Narayanan
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// You may obtain a copy of this library at
// https://github.com/Open-Payments/messages

use serde::{Deserialize, Serialize};


// ActiveCurrencyAnd24AmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd24AmountSimpleType {
	#[serde(rename = "ActiveCurrencyAnd24Amount_SimpleType")]
	pub active_currency_and24_amount_simple_type: f64,
}


// ActiveCurrencyAnd24Amount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd24Amount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAndAmount_SimpleType")]
	pub active_currency_and_amount_simple_type: f64,
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// AssetClassDetailedSubProductType16Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType16Code {
	#[serde(rename = "AssetClassDetailedSubProductType16Code")]
	pub asset_class_detailed_sub_product_type16_code: String,
}


// AssetClassDetailedSubProductType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// AssetHolding3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetHolding3 {
	#[serde(rename = "PstHrcutVal")]
	pub pst_hrcut_val: ActiveCurrencyAnd24Amount,
	#[serde(rename = "AsstTp")]
	pub asst_tp: AssetHolding3Choice,
	#[serde(rename = "CollRqrmnt")]
	pub coll_rqrmnt: String,
}


// AssetHolding3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetHolding3Choice {
	#[serde(rename = "Gold", skip_serializing_if = "Option::is_none")]
	pub gold: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Trpty", skip_serializing_if = "Option::is_none")]
	pub trpty: Option<TripartyCollateralAndAmount1>,
	#[serde(rename = "Csh", skip_serializing_if = "Option::is_none")]
	pub csh: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Scty", skip_serializing_if = "Option::is_none")]
	pub scty: Option<SecurityIdentificationAndAmount1>,
	#[serde(rename = "Grnt", skip_serializing_if = "Option::is_none")]
	pub grnt: Option<Guarantee1>,
	#[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
	pub cmmdty: Option<Commodity2>,
}


// CCPInteroperabilityReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPInteroperabilityReportV01 {
	#[serde(rename = "IntrprbltyCCP")]
	pub intrprblty_ccp: Vec<InteroperabilityCCP1>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// CollateralAccountType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralAccountType3Code {
	#[serde(rename = "CollateralAccountType3Code")]
	pub collateral_account_type3_code: String,
}


// CollateralType22Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralType22Choice {
	#[serde(rename = "GnlColl", skip_serializing_if = "Option::is_none")]
	pub gnl_coll: Option<GeneralCollateral4>,
	#[serde(rename = "SpcfcColl", skip_serializing_if = "Option::is_none")]
	pub spcfc_coll: Option<SpecificCollateral3>,
}


// Commodity2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Commodity2 {
	#[serde(rename = "MktVal")]
	pub mkt_val: ActiveCurrencyAnd24Amount,
	#[serde(rename = "CmmdtyTp")]
	pub cmmdty_tp: AssetClassDetailedSubProductType1Choice,
}


// FinancialInstrument104 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument104 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
}


// GeneralCollateral4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GeneralCollateral4 {
	#[serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_id: Option<Vec<FinancialInstrument104>>,
	#[serde(rename = "MktVal")]
	pub mkt_val: ActiveCurrencyAnd24Amount,
}


// GenericIdentification168 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification168 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<String>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<String>,
}


// GenericIdentification36 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<String>,
}


// Guarantee1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Guarantee1 {
	#[serde(rename = "Prvdr")]
	pub prvdr: PartyIdentification118Choice,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "ISINOct2015Identifier")]
	pub isin_oct2015_identifier: String,
}


// InteroperabilityCCP1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InteroperabilityCCP1 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification168,
	#[serde(rename = "TtlInitlMrgn")]
	pub ttl_initl_mrgn: Vec<ActiveCurrencyAndAmount>,
	#[serde(rename = "TrdsClrd", skip_serializing_if = "Option::is_none")]
	pub trds_clrd: Option<f64>,
	#[serde(rename = "GrssNtnlAmt")]
	pub grss_ntnl_amt: Vec<ActiveCurrencyAnd24Amount>,
	#[serde(rename = "AsstHldg")]
	pub asst_hldg: Vec<AssetHolding3>,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max256Text {
	#[serde(rename = "Max256Text")]
	pub max256_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// NonNegativeNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonNegativeNumber {
	#[serde(rename = "NonNegativeNumber")]
	pub non_negative_number: f64,
}


// PartyIdentification118Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification118Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification168>,
}


// ProductType7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProductType7Code {
	#[serde(rename = "ProductType7Code")]
	pub product_type7_code: String,
}


// SecurityIdentificationAndAmount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentificationAndAmount1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "MktVal")]
	pub mkt_val: ActiveCurrencyAnd24Amount,
	#[serde(rename = "FinInstrmTp")]
	pub fin_instrm_tp: String,
}


// SpecificCollateral3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SpecificCollateral3 {
	#[serde(rename = "FinInstrmId")]
	pub fin_instrm_id: FinancialInstrument104,
	#[serde(rename = "MktVal")]
	pub mkt_val: ActiveCurrencyAnd24Amount,
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<String>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// TripartyCollateralAndAmount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TripartyCollateralAndAmount1 {
	#[serde(rename = "Trpty")]
	pub trpty: ActiveCurrencyAndAmount,
	#[serde(rename = "CollTp")]
	pub coll_tp: CollateralType22Choice,
}
