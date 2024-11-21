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


#![allow(unused_imports)]
use regex::Regex;
use crate::common::*;
use open_payments_common::ValidationError;
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};


// KeyRevocation ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct KeyRevocation {
	#[cfg_attr( feature = "derive_serde", serde(rename = "KeyRevocation", skip_serializing_if = "Option::is_none") )]
	pub key_revocation: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowStatusDescription", skip_serializing_if = "Option::is_none") )]
	pub fed_now_status_description: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowKeyID", skip_serializing_if = "Option::is_none") )]
	pub fed_now_key_id: Option<String>,
}

impl KeyRevocation {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fed_now_status_description {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "fed_now_status_description is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 300 {
				return Err(ValidationError::new(1002, "fed_now_status_description exceeds the maximum length of 300".to_string()));
			}
		}
		if let Some(ref val) = self.fed_now_key_id {
			let pattern = Regex::new("[A-Za-z0-9\\-_]{1,300}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "fed_now_key_id does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// FedNowMessageSignatureKeyExchange ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowMessageSignatureKeyExchange {
	#[cfg_attr( feature = "derive_serde", serde(rename = "KeyAddition", skip_serializing_if = "Option::is_none") )]
	pub key_addition: Option<KeyAddition>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KeyRevocation", skip_serializing_if = "Option::is_none") )]
	pub key_revocation: Option<String>,
}

impl FedNowMessageSignatureKeyExchange {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.key_addition { val.validate()? }
		Ok(())
	}
}


// FedNowCustomerMessageSignatureKeyOperationResponse ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowCustomerMessageSignatureKeyOperationResponse {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowKeyID") )]
	pub fed_now_key_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Status") )]
	pub status: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ErrorCode", skip_serializing_if = "Option::is_none") )]
	pub error_code: Option<String>,
}

impl FedNowCustomerMessageSignatureKeyOperationResponse {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Za-z0-9\\-_]{1,300}").unwrap();
		if !pattern.is_match(&self.fed_now_key_id) {
			return Err(ValidationError::new(1005, "fed_now_key_id does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// GetAllFedNowActivePublicKeys ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GetAllFedNowActivePublicKeys {
}

impl GetAllFedNowActivePublicKeys {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// GetAllCustomerPublicKeys ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GetAllCustomerPublicKeys {
}

impl GetAllCustomerPublicKeys {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FedNowPublicKeyResponses ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowPublicKeyResponses {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PublicKeys") )]
	pub public_keys: Vec<FedNowPublicKeyResponse>,
}

impl FedNowPublicKeyResponses {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.public_keys { item.validate()? }
		Ok(())
	}
}
