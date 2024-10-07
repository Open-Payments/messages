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
use serde_valid::Validate;


// ActiveCurrencyAnd24AmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd24AmountSimpleType {
	#[serde(rename = "ActiveCurrencyAnd24Amount_SimpleType")]
	pub active_currency_and24_amount_simple_type: f64,
}


// ActiveCurrencyAnd24Amount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd24Amount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAndAmount_SimpleType")]
	pub active_currency_and_amount_simple_type: f64,
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[validate(pattern = "[A-Z]{3,3}")]
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// AssetClassDetailedSubProductType16Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType16Code {
	#[validate(enumerate = ["FXCR", "FXEM", "FXMJ", "FUEL", "FOIL", "GOIL", "GSLN", "GASP", "HEAT", "IRON", "JTFL", "KERO", "LAMP", "LEAD", "LLSO", "LNGG", "CORN", "MARS", "MWHT", "MOLY", "NAPH", "NBPG", "NASC", "NCGG", "NGLO", "NICK", "OFFP", "ALUM", "ALUA", "BAKK", "BSLD", "BDSL", "BRNT", "BRNX", "CNDA", "CERE", "CBLT", "CCOA", "COND", "CSHP", "COPR", "DSEL", "DBCR", "DUBA", "ERUE", "ESPO", "ETHA", "EUAE", "EUAA", "FWHT", "FITR", "OTHR", "PLDM", "PKLD", "PTNM", "POTA", "RPSD", "BRWN", "RICE", "ROBU", "SLVR", "SOYB", "STEL", "TNKR", "TAPI", "TINN", "TTFG", "URAL", "WHSG", "WTIO", "ZINC"])]
	#[serde(rename = "AssetClassDetailedSubProductType16Code")]
	pub asset_class_detailed_sub_product_type16_code: String,
}


// AssetClassDetailedSubProductType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// AssetHolding1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetHolding1 {
	#[validate]
	#[serde(rename = "PstHrcutVal")]
	pub pst_hrcut_val: ActiveCurrencyAnd24Amount,
	#[validate]
	#[serde(rename = "AsstTp")]
	pub asst_tp: AssetHolding1Choice,
	#[serde(rename = "CollRqrmnt")]
	pub coll_rqrmnt: String,
}


// AssetHolding1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetHolding1Choice {
	#[validate]
	#[serde(rename = "Gold")]
	pub gold: Option<ActiveCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "Trpty")]
	pub trpty: Option<ActiveCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "Csh")]
	pub csh: Option<ActiveCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "Scty")]
	pub scty: Option<SecurityIdentificationAndAmount1>,
	#[validate]
	#[serde(rename = "Grnt")]
	pub grnt: Option<Guarantee1>,
	#[validate]
	#[serde(rename = "Cmmdty")]
	pub cmmdty: Option<Commodity2>,
}


// CCPCollateralReportV01 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CCPCollateralReportV01 {
	#[validate]
	#[serde(rename = "CollAcctOwnr")]
	pub coll_acct_ownr: Vec<CollateralAccount4>,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// CollateralAccount4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CollateralAccount4 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: GenericIdentification165,
	#[validate]
	#[serde(rename = "AsstHldg")]
	pub asst_hldg: Vec<AssetHolding1>,
}


// CollateralAccountType3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CollateralAccountType3Code {
	#[validate(enumerate = ["MGIN", "DFLT"])]
	#[serde(rename = "CollateralAccountType3Code")]
	pub collateral_account_type3_code: String,
}


// Commodity2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Commodity2 {
	#[validate]
	#[serde(rename = "MktVal")]
	pub mkt_val: ActiveCurrencyAnd24Amount,
	#[validate]
	#[serde(rename = "CmmdtyTp")]
	pub cmmdty_tp: AssetClassDetailedSubProductType1Choice,
}


// GenericIdentification165 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification165 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// GenericIdentification168 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification168 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// GenericIdentification36 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// Guarantee1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Guarantee1 {
	#[validate]
	#[serde(rename = "Prvdr")]
	pub prvdr: PartyIdentification118Choice,
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
}


// ISINOct2015Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[validate(pattern = "[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}")]
	#[serde(rename = "ISINOct2015Identifier")]
	pub isin_oct2015_identifier: String,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[validate(pattern = "[A-Z0-9]{18,18}[0-9]{2,2}")]
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max140Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 140)]
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max256Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max256Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 256)]
	#[serde(rename = "Max256Text")]
	pub max256_text: String,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max350Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 350)]
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max35Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// PartyIdentification118Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification118Choice {
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification168>,
}


// ProductType7Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProductType7Code {
	#[validate(enumerate = ["SVGN", "EQUI", "OTHR"])]
	#[serde(rename = "ProductType7Code")]
	pub product_type7_code: String,
}


// SchemeIdentificationType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SchemeIdentificationType1Code {
	#[validate(enumerate = ["MARG", "COLL", "POSI", "CLIM"])]
	#[serde(rename = "SchemeIdentificationType1Code")]
	pub scheme_identification_type1_code: String,
}


// SecurityIdentificationAndAmount1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityIdentificationAndAmount1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[validate]
	#[serde(rename = "MktVal")]
	pub mkt_val: ActiveCurrencyAnd24Amount,
	#[serde(rename = "FinInstrmTp")]
	pub fin_instrm_tp: String,
}


// SupplementaryData1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[validate]
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}
