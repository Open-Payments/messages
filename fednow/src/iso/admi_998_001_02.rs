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


use regex::Regex;
use crate::common::*;
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};


// AdministrationProprietaryMessageV02 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AdministrationProprietaryMessageV02 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId", skip_serializing_if = "Option::is_none") )]
	pub msg_id: Option<MessageReference>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rltd", skip_serializing_if = "Option::is_none") )]
	pub rltd: Option<MessageReference>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prvs", skip_serializing_if = "Option::is_none") )]
	pub prvs: Option<MessageReference>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<MessageReference>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryData") )]
	pub prtry_data: ProprietaryData5,
}

impl AdministrationProprietaryMessageV02 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.msg_id { val.validate()? }
		if let Some(ref val) = self.rltd { val.validate()? }
		if let Some(ref val) = self.prvs { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
		self.prtry_data.validate()?;
		Ok(())
	}
}


// MessageReference ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MessageReference {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
	pub ref_attr: String,
}

impl MessageReference {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.ref_attr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "ref_attr is shorter than the minimum length of 1".to_string()));
		}
		if self.ref_attr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "ref_attr exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// ProprietaryData5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProprietaryData5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Data") )]
	pub data: SupplementaryDataEnvelope1,
}

impl ProprietaryData5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
		}
		if self.tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
		}
		self.data.validate()?;
		Ok(())
	}
}


// SupplementaryDataEnvelope1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
