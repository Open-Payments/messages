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

pub mod fednow {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	
	// Max300AlphaNumericString ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max300AlphaNumericString {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max300_alpha_numeric_string: String,
	}
	
	impl Max300AlphaNumericString {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Za-z0-9\\-_]{1,300}").unwrap();
			if !pattern.is_match(&self.max300_alpha_numeric_string) {
				return Err(ValidationError::new(1005, "max300_alpha_numeric_string does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max50AlphaNumericString ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max50AlphaNumericString {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max50_alpha_numeric_string: String,
	}
	
	impl Max50AlphaNumericString {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Za-z0-9\\-_]{1,50}").unwrap();
			if !pattern.is_match(&self.max50_alpha_numeric_string) {
				return Err(ValidationError::new(1005, "max50_alpha_numeric_string does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max300Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max300Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max300_text: String,
	}
	
	impl Max300Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max300_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max300_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max300_text.chars().count() > 300 {
				return Err(ValidationError::new(1002, "max300_text exceeds the maximum length of 300".to_string()));
			}
			Ok(())
		}
	}
	
	
	// RoutingNumberFRS1: This is a routing number used by the Service participant in connection with the message.

	//                 

	//                 Note: This may be a master account routing number or a subaccount routing number.
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct RoutingNumberFRS1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub routing_number_frs_1: String,
	}
	
	impl RoutingNumberFRS1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{9,9}").unwrap();
			if !pattern.is_match(&self.routing_number_frs_1) {
				return Err(ValidationError::new(1005, "routing_number_frs_1 does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// fed_now_key_id: The key finger print used as the key id.
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct fed_now_key_id {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowKeyID") )]
		pub fed_now_key_id: Max300AlphaNumericString,
	}
	
	impl fed_now_key_id {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.fed_now_key_id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// fed_now_status_description: A description for the status update intended for the key owner.
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct fed_now_status_description {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowStatusDescription") )]
		pub fed_now_status_description: Max300Text,
	}
	
	impl fed_now_status_description {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.fed_now_status_description.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FedNowMessageSignatureKeyStatus ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// status_date_time ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct status_date_time {
		#[cfg_attr( feature = "derive_serde", serde(rename = "StatusDateTime") )]
		pub status_date_time: String,
	}
	
	impl status_date_time {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// key_status ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct key_status {
		#[cfg_attr( feature = "derive_serde", serde(rename = "KeyStatus") )]
		pub key_status: String,
	}
	
	impl key_status {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// FedNowMessageSignatureKey ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FedNowMessageSignatureKey {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowKeyID") )]
		pub fed_now_key_id: Max300AlphaNumericString,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Name") )]
		pub name: Max300AlphaNumericString,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EncodedPublicKey") )]
		pub encoded_public_key: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Encoding") )]
		pub encoding: Max50AlphaNumericString,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Algorithm", skip_serializing_if = "Option::is_none") )]
		pub algorithm: Option<Max50AlphaNumericString>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KeyCreationDateTime", skip_serializing_if = "Option::is_none") )]
		pub key_creation_date_time: Option<String>,
	}
	
	impl FedNowMessageSignatureKey {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.fed_now_key_id.validate() { return Err(e); }
			if let Err(e) = self.name.validate() { return Err(e); }
			if let Err(e) = self.encoding.validate() { return Err(e); }
			if let Some(ref algorithm_value) = self.algorithm { if let Err(e) = algorithm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// key_creation_date_time: The creation datetime for the key. This value is overwritten on submission based on the
	//                             System timezone for FedNow
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct key_creation_date_time {
		#[cfg_attr( feature = "derive_serde", serde(rename = "KeyCreationDateTime", skip_serializing_if = "Option::is_none") )]
		pub key_creation_date_time: Option<String>,
	}
	
	impl key_creation_date_time {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// xs:string ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum XsString {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "expired") )]
		CodeEXPIRED,
		#[cfg_attr( feature = "derive_serde", serde(rename = "revoked") )]
		CodeREVOKED,
		#[cfg_attr( feature = "derive_serde", serde(rename = "compromised") )]
		CodeCOMPROMISED,
		#[cfg_attr( feature = "derive_serde", serde(rename = "active") )]
		CodeACTIVE,
	}
	
	impl XsString {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// key_expiration_date_time: The Expiration datetime. This is the date and time the customer intends for their keys to be
	//                         automatically expired.
	//                         This value should be no more than 365 days from the submission time and will be used as a point
	//                         in time to the nearest system date.
	//                         If the expiration time is 3:00 AM Wednesday, Coordinated Universal Time (UTC)
	//                         this translates to 10:00 PM Tuesday, Eastern Time (ET)
	// 
	//                         For this time, we will take expire this at midnight on the given date.
	//                         In the example above, the key will not be valid at 12:01 AM Wednesday, Eastern Time (ET).
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct key_expiration_date_time {
		#[cfg_attr( feature = "derive_serde", serde(rename = "KeyExpirationDateTime") )]
		pub key_expiration_date_time: String,
	}
	
	impl key_expiration_date_time {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// target_rtn: Identifier of the RTN to associate this key
	//                         with. If providing the ETI, the scope of this key is all RTNs managed by that connection party.
	//                         If providing a single RTN, FedNow will check and only accept the key for operations against that RTN only
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct target_rtn {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TargetRTN", skip_serializing_if = "Option::is_none") )]
		pub target_rtn: Option<RoutingNumberFRS1>,
	}
	
	impl target_rtn {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref target_rtn_value) = self.target_rtn { if let Err(e) = target_rtn_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// KeyAddition ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct KeyAddition {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Key", skip_serializing_if = "Option::is_none") )]
		pub key: Option<FedNowMessageSignatureKey>,
	}
	
	impl KeyAddition {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref key_value) = self.key { if let Err(e) = key_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// KeyRevocation ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct KeyRevocation {
		#[cfg_attr( feature = "derive_serde", serde(rename = "KeyRevocation", skip_serializing_if = "Option::is_none") )]
		pub key_revocation: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowStatusDescription", skip_serializing_if = "Option::is_none") )]
		pub fed_now_status_description: Option<Max300Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowKeyID", skip_serializing_if = "Option::is_none") )]
		pub fed_now_key_id: Option<Max300AlphaNumericString>,
	}
	
	impl KeyRevocation {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref fed_now_status_description_value) = self.fed_now_status_description { if let Err(e) = fed_now_status_description_value.validate() { return Err(e); } }
			if let Some(ref fed_now_key_id_value) = self.fed_now_key_id { if let Err(e) = fed_now_key_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FedNowMessageSignatureKeyExchange ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FedNowMessageSignatureKeyExchange {
		#[cfg_attr( feature = "derive_serde", serde(rename = "KeyAddition", skip_serializing_if = "Option::is_none") )]
		pub key_addition: Option<KeyAddition>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KeyRevocation", skip_serializing_if = "Option::is_none") )]
		pub key_revocation: Option<String>,
	}
	
	impl FedNowMessageSignatureKeyExchange {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref key_addition_value) = self.key_addition { if let Err(e) = key_addition_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FedNowCustomerMessageSignatureKeyOperationResponse ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FedNowCustomerMessageSignatureKeyOperationResponse {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowKeyID") )]
		pub fed_now_key_id: Max300AlphaNumericString,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Status") )]
		pub status: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ErrorCode", skip_serializing_if = "Option::is_none") )]
		pub error_code: Option<String>,
	}
	
	impl FedNowCustomerMessageSignatureKeyOperationResponse {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.fed_now_key_id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// error_code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct error_code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ErrorCode", skip_serializing_if = "Option::is_none") )]
		pub error_code: Option<String>,
	}
	
	impl error_code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// GetAllFedNowActivePublicKeys ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GetAllFedNowActivePublicKeys {
	}
	
	impl GetAllFedNowActivePublicKeys {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// GetAllCustomerPublicKeys ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GetAllCustomerPublicKeys {
	}
	
	impl GetAllCustomerPublicKeys {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// FedNowPublicKeyResponse ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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