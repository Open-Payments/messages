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


// Exact4AlphaNumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[validate(pattern = "[a-zA-Z0-9]{4}")]
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
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


// Max70Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max70Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 70)]
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// ReportParameter1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReportParameter1 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Val")]
	pub val: String,
}


// RequestDetails4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RequestDetails4 {
	#[serde(rename = "Key")]
	pub key: String,
	#[validate]
	#[serde(rename = "RptData")]
	pub rpt_data: Option<Vec<ReportParameter1>>,
}


// RequestDetails5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RequestDetails5 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "ReqRef")]
	pub req_ref: String,
	#[validate]
	#[serde(rename = "RptKey")]
	pub rpt_key: Vec<RequestDetails4>,
}


// StaticDataReportV02 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct StaticDataReportV02 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "SttlmSsnIdr")]
	pub sttlm_ssn_idr: Option<String>,
	#[validate]
	#[serde(rename = "RptDtls")]
	pub rpt_dtls: RequestDetails5,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
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
