// FedNow Message Parsing Library
// https://github.com/Open-Payments/messages
//
// This library is designed to parse FedNow message formats based on ISO 20022 standards.
// It handles various message types, including administrative notifications, payment status reports, 
// customer credit transfers, and more, using Serde for efficient serialization and deserialization.
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


// ISODateTime ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// Max20000Text ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct Max20000Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 20000)]
	#[serde(rename = "Max20000Text")]
	pub max20000_text: String,
}


// Max350Text ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct Max350Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 350)]
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct Max35Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// MessageReference ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct MessageReference {
	#[serde(rename = "Ref")]
	pub ref_attr: String,
}


// RejectionReason2 ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct RejectionReason2 {
	#[serde(rename = "RjctgPtyRsn")]
	pub rjctg_pty_rsn: String,
	#[serde(rename = "RjctnDtTm")]
	pub rjctn_dt_tm: Option<String>,
	#[serde(rename = "ErrLctn")]
	pub err_lctn: Option<String>,
	#[serde(rename = "RsnDesc")]
	pub rsn_desc: Option<String>,
	#[serde(rename = "AddtlData")]
	pub addtl_data: Option<String>,
}


// Admi00200101 ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct Admi00200101 {
	#[serde(rename = "RltdRef")]
	pub rltd_ref: MessageReference,
	#[serde(rename = "Rsn")]
	pub rsn: RejectionReason2,
}
