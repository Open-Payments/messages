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


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// Max20000Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max20000Text {
	#[serde(rename = "Max20000Text")]
	pub max20000_text: String,
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


// MessageReference ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageReference {
	#[serde(rename = "Ref")]
	pub ref_attr: String,
}


// RejectionReason2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RejectionReason2 {
	#[serde(rename = "RjctgPtyRsn")]
	pub rjctg_pty_rsn: String,
	#[serde(rename = "RjctnDtTm", skip_serializing_if = "Option::is_none")]
	pub rjctn_dt_tm: Option<String>,
	#[serde(rename = "ErrLctn", skip_serializing_if = "Option::is_none")]
	pub err_lctn: Option<String>,
	#[serde(rename = "RsnDesc", skip_serializing_if = "Option::is_none")]
	pub rsn_desc: Option<String>,
	#[serde(rename = "AddtlData", skip_serializing_if = "Option::is_none")]
	pub addtl_data: Option<String>,
}


// Admi00200101 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Admi00200101 {
	#[serde(rename = "RltdRef")]
	pub rltd_ref: MessageReference,
	#[serde(rename = "Rsn")]
	pub rsn: RejectionReason2,
}
