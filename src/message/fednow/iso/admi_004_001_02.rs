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


// Event2 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Event2 {
	#[serde(rename = "EvtCd")]
	pub evt_cd: String,
	#[serde(rename = "EvtParam")]
	pub evt_param: Option<Vec<String>>,
	#[serde(rename = "EvtDesc")]
	pub evt_desc: Option<String>,
	#[serde(rename = "EvtTm")]
	pub evt_tm: Option<String>,
}


// ISODateTime ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// Max1000Text ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max1000Text {
	#[serde(rename = "Max1000Text")]
	pub max1000_text: String,
}


// Max35Text ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max4AlphaNumericText ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "Max4AlphaNumericText")]
	pub max4_alpha_numeric_text: String,
}


// SystemEventNotificationV02 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SystemEventNotificationV02 {
	#[serde(rename = "EvtInf")]
	pub evt_inf: Event2,
}
