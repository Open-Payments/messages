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





// Event2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Event2 {
	#[serde(rename = "EvtCd")]
	pub evt_cd: Max4AlphaNumericText,
	#[serde(rename = "EvtParam", skip_serializing_if = "Option::is_none")]
	pub evt_param: Option<Vec<Max35Text>>,
	#[serde(rename = "EvtDesc", skip_serializing_if = "Option::is_none")]
	pub evt_desc: Option<Max1000Text>,
	#[serde(rename = "EvtTm", skip_serializing_if = "Option::is_none")]
	pub evt_tm: Option<String>,
}

impl Event2 {
	pub fn validate(&self) -> bool {
		if !self.evt_cd.validate() { return false }
		if let Some(ref evt_param_vec) = self.evt_param { for item in evt_param_vec { if !item.validate() { return false; } } }
		if let Some(ref evt_desc_value) = self.evt_desc { if !evt_desc_value.validate() { return false; } }
		return true
	}
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}

impl ISODateTime {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Max1000Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max1000Text {
	#[serde(rename = "$value")]
	pub max1000_text: String,
}

impl Max1000Text {
	pub fn validate(&self) -> bool {
		if self.max1000_text.chars().count() < 1 {
			return false
		}
		if self.max1000_text.chars().count() > 1000 {
			return false
		}
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


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "$value")]
	pub max4_alpha_numeric_text: String,
}

impl Max4AlphaNumericText {
	pub fn validate(&self) -> bool {
		if self.max4_alpha_numeric_text.chars().count() < 1 {
			return false
		}
		if self.max4_alpha_numeric_text.chars().count() > 4 {
			return false
		}
		let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
		if !pattern.is_match(&self.max4_alpha_numeric_text) {
			return false
		}
		return true
	}
}


// SystemEventNotificationV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemEventNotificationV02 {
	#[serde(rename = "EvtInf")]
	pub evt_inf: Event2,
}

impl SystemEventNotificationV02 {
	pub fn validate(&self) -> bool {
		if !self.evt_inf.validate() { return false }
		return true
	}
}
