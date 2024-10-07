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


// ApplicationSpecifics1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ApplicationSpecifics1 {
	#[serde(rename = "SysUsr")]
	pub sys_usr: Option<String>,
	#[validate]
	#[serde(rename = "Sgntr")]
	pub sgntr: Option<SignatureEnvelope>,
	#[serde(rename = "TtlNbOfDocs")]
	pub ttl_nb_of_docs: f64,
}


// BusinessFileHeaderV01 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BusinessFileHeaderV01 {
	#[validate]
	#[serde(rename = "PyldDesc")]
	pub pyld_desc: PayloadDescription2,
	#[validate]
	#[serde(rename = "Pyld")]
	pub pyld: Option<Vec<LaxPayload>>,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// LaxPayload ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LaxPayload {
}


// ManifestData2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ManifestData2 {
	#[serde(rename = "DocTp")]
	pub doc_tp: String,
	#[serde(rename = "NbOfDocs")]
	pub nb_of_docs: f64,
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


// Max35Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max35Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Number ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// PayloadData2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PayloadData2 {
	#[serde(rename = "PyldIdr")]
	pub pyld_idr: String,
	#[serde(rename = "CreDtAndTm")]
	pub cre_dt_and_tm: String,
	#[serde(rename = "PssblDplctFlg")]
	pub pssbl_dplct_flg: Option<bool>,
}


// PayloadDescription2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PayloadDescription2 {
	#[validate]
	#[serde(rename = "PyldData")]
	pub pyld_data: PayloadData2,
	#[validate]
	#[serde(rename = "ApplSpcfcs")]
	pub appl_spcfcs: Option<ApplicationSpecifics1>,
	#[serde(rename = "PyldTp")]
	pub pyld_tp: String,
	#[validate]
	#[serde(rename = "MnfstData")]
	pub mnfst_data: Option<Vec<ManifestData2>>,
}


// SignatureEnvelope ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SignatureEnvelope {
}


// TrueFalseIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}
