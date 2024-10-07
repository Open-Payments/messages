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


// AdministrationProprietaryMessageV02 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AdministrationProprietaryMessageV02 {
	#[validate]
	#[serde(rename = "MsgId")]
	pub msg_id: Option<MessageReference>,
	#[validate]
	#[serde(rename = "Rltd")]
	pub rltd: Option<MessageReference>,
	#[validate]
	#[serde(rename = "Prvs")]
	pub prvs: Option<MessageReference>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<MessageReference>,
	#[validate]
	#[serde(rename = "PrtryData")]
	pub prtry_data: ProprietaryData5,
}


// Max35Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max35Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// MessageReference ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MessageReference {
	#[serde(rename = "Ref")]
	pub ref_attr: String,
}


// ProprietaryData5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProprietaryData5 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[validate]
	#[serde(rename = "Data")]
	pub data: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}
