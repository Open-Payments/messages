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


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// CountryCodeAndName3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCodeAndName3 {
	#[serde(rename = "Cd")]
	pub cd: CountryCode,
	#[serde(rename = "Nm")]
	pub nm: Max70Text,
}


// FinancialInstrumentReportingMarketIdentificationCodeReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentReportingMarketIdentificationCodeReportV02 {
	#[serde(rename = "MktId")]
	pub mkt_id: Vec<MarketIdentification95>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}


// MICEntityType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum MICEntityType1Code {
	#[default]
	#[serde(rename = "APPA")]
	CodeAPPA,
	#[serde(rename = "CTPS")]
	CodeCTPS,
	#[serde(rename = "MLTF")]
	CodeMLTF,
	#[serde(rename = "OTFS")]
	CodeOTFS,
	#[serde(rename = "RMKT")]
	CodeRMKT,
	#[serde(rename = "SINT")]
	CodeSINT,

}


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[serde(rename = "$value")]
	pub mic_identifier: String,
}


// MarketIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum MarketIdentification1Code {
	#[default]
	#[serde(rename = "SGMT")]
	CodeSGMT,
	#[serde(rename = "OPRT")]
	CodeOPRT,

}


// MarketIdentification95 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketIdentification95 {
	#[serde(rename = "Oprg")]
	pub oprg: MICIdentifier,
	#[serde(rename = "Sgmt")]
	pub sgmt: MICIdentifier,
	#[serde(rename = "Tp")]
	pub tp: MarketIdentification1Code,
	#[serde(rename = "Ctgy", skip_serializing_if = "Option::is_none")]
	pub ctgy: Option<MICEntityType1Code>,
	#[serde(rename = "InstnNm")]
	pub instn_nm: Max450Text,
	#[serde(rename = "Acrnm", skip_serializing_if = "Option::is_none")]
	pub acrnm: Option<Max35Text>,
	#[serde(rename = "City", skip_serializing_if = "Option::is_none")]
	pub city: Option<Max35Text>,
	#[serde(rename = "Ctry")]
	pub ctry: CountryCodeAndName3,
	#[serde(rename = "AuthrtyNm", skip_serializing_if = "Option::is_none")]
	pub authrty_nm: Option<Max450Text>,
	#[serde(rename = "WebSite", skip_serializing_if = "Option::is_none")]
	pub web_site: Option<Max210Text>,
	#[serde(rename = "Note", skip_serializing_if = "Option::is_none")]
	pub note: Option<Max450Text>,
	#[serde(rename = "Mod", skip_serializing_if = "Option::is_none")]
	pub mod_attr: Option<Modification1Code>,
	#[serde(rename = "CreDt", skip_serializing_if = "Option::is_none")]
	pub cre_dt: Option<String>,
	#[serde(rename = "VldtyPrd")]
	pub vldty_prd: Period4Choice,
	#[serde(rename = "StsDt", skip_serializing_if = "Option::is_none")]
	pub sts_dt: Option<String>,
	#[serde(rename = "LastUpdtdDt", skip_serializing_if = "Option::is_none")]
	pub last_updtd_dt: Option<String>,
}


// Max210Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max210Text {
	#[serde(rename = "$value")]
	pub max210_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}


// Max450Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max450Text {
	#[serde(rename = "$value")]
	pub max450_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}


// Modification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Modification1Code {
	#[default]
	#[serde(rename = "NOCH")]
	CodeNOCH,
	#[serde(rename = "MODI")]
	CodeMODI,
	#[serde(rename = "DELE")]
	CodeDELE,
	#[serde(rename = "ADDD")]
	CodeADDD,

}


// Period2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// Period4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period4Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
	pub to_dt: Option<String>,
	#[serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt_to_dt: Option<Period2>,
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}
