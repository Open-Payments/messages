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


// ApplicationSpecifics1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ApplicationSpecifics1 {
	#[serde(rename = "SysUsr", skip_serializing_if = "Option::is_none")]
	pub sys_usr: Option<Max140Text>,
	#[serde(rename = "Sgntr", skip_serializing_if = "Option::is_none")]
	pub sgntr: Option<SignatureEnvelope>,
	#[serde(rename = "TtlNbOfDocs")]
	pub ttl_nb_of_docs: f64,
}


// BusinessFileHeaderV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BusinessFileHeaderV01 {
	#[serde(rename = "PyldDesc")]
	pub pyld_desc: PayloadDescription2,
	#[serde(rename = "Pyld", skip_serializing_if = "Option::is_none")]
	pub pyld: Option<Vec<LaxPayload>>,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}


// LaxPayload ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LaxPayload {
}


// ManifestData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ManifestData2 {
	#[serde(rename = "DocTp")]
	pub doc_tp: Max35Text,
	#[serde(rename = "NbOfDocs")]
	pub nb_of_docs: f64,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max256Text {
	#[serde(rename = "$value")]
	pub max256_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}


// PayloadData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PayloadData2 {
	#[serde(rename = "PyldIdr")]
	pub pyld_idr: Max35Text,
	#[serde(rename = "CreDtAndTm")]
	pub cre_dt_and_tm: String,
	#[serde(rename = "PssblDplctFlg", skip_serializing_if = "Option::is_none")]
	pub pssbl_dplct_flg: Option<bool>,
}


// PayloadDescription2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PayloadDescription2 {
	#[serde(rename = "PyldData")]
	pub pyld_data: PayloadData2,
	#[serde(rename = "ApplSpcfcs", skip_serializing_if = "Option::is_none")]
	pub appl_spcfcs: Option<ApplicationSpecifics1>,
	#[serde(rename = "PyldTp")]
	pub pyld_tp: Max256Text,
	#[serde(rename = "MnfstData", skip_serializing_if = "Option::is_none")]
	pub mnfst_data: Option<Vec<ManifestData2>>,
}


// SignatureEnvelope ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SignatureEnvelope {
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}
