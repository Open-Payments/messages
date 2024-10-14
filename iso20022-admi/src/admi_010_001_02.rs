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


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
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


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}


// ReportParameter1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportParameter1 {
	#[serde(rename = "Nm")]
	pub nm: Max70Text,
	#[serde(rename = "Val")]
	pub val: Max350Text,
}


// RequestDetails4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestDetails4 {
	#[serde(rename = "Key")]
	pub key: Max35Text,
	#[serde(rename = "RptData", skip_serializing_if = "Option::is_none")]
	pub rpt_data: Option<Vec<ReportParameter1>>,
}


// RequestDetails5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestDetails5 {
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
	#[serde(rename = "ReqRef")]
	pub req_ref: Max35Text,
	#[serde(rename = "RptKey")]
	pub rpt_key: Vec<RequestDetails4>,
}


// StaticDataReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StaticDataReportV02 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "SttlmSsnIdr", skip_serializing_if = "Option::is_none")]
	pub sttlm_ssn_idr: Option<Exact4AlphaNumericText>,
	#[serde(rename = "RptDtls")]
	pub rpt_dtls: RequestDetails5,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
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
