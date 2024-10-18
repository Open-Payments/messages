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
use regex::Regex;





// AdministrationProprietaryMessageV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdministrationProprietaryMessageV02 {
	#[serde(rename = "MsgId", skip_serializing_if = "Option::is_none")]
	pub msg_id: Option<MessageReference>,
	#[serde(rename = "Rltd", skip_serializing_if = "Option::is_none")]
	pub rltd: Option<MessageReference>,
	#[serde(rename = "Prvs", skip_serializing_if = "Option::is_none")]
	pub prvs: Option<MessageReference>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<MessageReference>,
	#[serde(rename = "PrtryData")]
	pub prtry_data: ProprietaryData5,
}

impl AdministrationProprietaryMessageV02 {
	pub fn validate(&self) -> bool {
		if let Some(ref msg_id_value) = self.msg_id { if !msg_id_value.validate() { return false; } }
		if let Some(ref rltd_value) = self.rltd { if !rltd_value.validate() { return false; } }
		if let Some(ref prvs_value) = self.prvs { if !prvs_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		if !self.prtry_data.validate() { return false }
		return true
	}
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}

impl Max35Text {
	pub fn validate(&self) -> bool {
		if self.max35_text.chars().count() < 1 {
			return false
		}
		if self.max35_text.chars().count() > 35 {
			return false
		}
		return true
	}
}


// MessageReference ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageReference {
	#[serde(rename = "Ref")]
	pub ref_attr: Max35Text,
}

impl MessageReference {
	pub fn validate(&self) -> bool {
		if !self.ref_attr.validate() { return false }
		return true
	}
}


// ProprietaryData5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryData5 {
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
	#[serde(rename = "Data")]
	pub data: SupplementaryDataEnvelope1,
}

impl ProprietaryData5 {
	pub fn validate(&self) -> bool {
		if !self.tp.validate() { return false }
		if !self.data.validate() { return false }
		return true
	}
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> bool {
		return true
	}
}
