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

pub mod fednow {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	
	// FedNowMessageSignatureKeyStatus ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FedNowMessageSignatureKeyStatus {
		#[cfg_attr( feature = "derive_serde", serde(rename = "KeyStatus") )]
		pub key_status: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StatusDateTime") )]
		pub status_date_time: String,
	}
	
	impl FedNowMessageSignatureKeyStatus {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// FedNowMessageSignatureKey ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FedNowMessageSignatureKey {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowKeyID") )]
		pub fed_now_key_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Name") )]
		pub name: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EncodedPublicKey") )]
		pub encoded_public_key: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Encoding") )]
		pub encoding: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Algorithm", skip_serializing_if = "Option::is_none") )]
		pub algorithm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KeyCreationDateTime", skip_serializing_if = "Option::is_none") )]
		pub key_creation_date_time: Option<String>,
	}
	
	impl FedNowMessageSignatureKey {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Za-z0-9\\-_]{1,300}").unwrap();
			if !pattern.is_match(&self.fed_now_key_id) {
				return Err(ValidationError::new(1005, "fed_now_key_id does not match the required pattern".to_string()));
			}
			let pattern = Regex::new("[A-Za-z0-9\\-_]{1,300}").unwrap();
			if !pattern.is_match(&self.name) {
				return Err(ValidationError::new(1005, "name does not match the required pattern".to_string()));
			}
			let pattern = Regex::new("[A-Za-z0-9\\-_]{1,50}").unwrap();
			if !pattern.is_match(&self.encoding) {
				return Err(ValidationError::new(1005, "encoding does not match the required pattern".to_string()));
			}
			if let Some(ref val) = self.algorithm {
				let pattern = Regex::new("[A-Za-z0-9\\-_]{1,50}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "algorithm does not match the required pattern".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// KeyAddition ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct KeyAddition {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Key", skip_serializing_if = "Option::is_none") )]
		pub key: Option<FedNowMessageSignatureKey>,
	}
	
	impl KeyAddition {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.key { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
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
				if !pattern.is_match(&val) {
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
			if let Some(ref val) = self.key_addition { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// FedNowPublicKeyResponse ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FedNowPublicKeyResponse {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowMessageSignatureKeyStatus") )]
		pub fed_now_message_signature_key_status: FedNowMessageSignatureKeyStatus,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowMessageSignatureKey") )]
		pub fed_now_message_signature_key: FedNowMessageSignatureKey,
	}
	
	impl FedNowPublicKeyResponse {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.fed_now_message_signature_key_status.validate() { return Err(e); }
			if let Err(e) = self.fed_now_message_signature_key.validate() { return Err(e); }
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
			for item in &self.public_keys { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
}