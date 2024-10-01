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


// Max300AlphaNumericString ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct Max300AlphaNumericString {
	#[validate(pattern = "[A-Za-z0-9\\-_]{1,300}")]
	#[serde(rename = "Max300AlphaNumericString")]
	pub max300_alpha_numeric_string: String,
}


// Max50AlphaNumericString ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct Max50AlphaNumericString {
	#[validate(pattern = "[A-Za-z0-9\\-_]{1,50}")]
	#[serde(rename = "Max50AlphaNumericString")]
	pub max50_alpha_numeric_string: String,
}


// Max300Text ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct Max300Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 300)]
	#[serde(rename = "Max300Text")]
	pub max300_text: String,
}


// RoutingNumberFRS1 is This is a routing number used by the Service participant in connection with the message.

//                 

//                 Note: This may be a master account routing number or a subaccount routing number.
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct RoutingNumberFRS1 {
	#[validate(pattern = "[0-9]{9,9}")]
	#[serde(rename = "RoutingNumber_FRS_1")]
	pub routing_number_frs_1: String,
}


// FedNowMessageSignatureKeyStatus ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct FedNowMessageSignatureKeyStatus {
	#[serde(rename = "KeyStatus")]
	pub key_status: String,
	#[serde(rename = "StatusDateTime")]
	pub status_date_time: String,
}


// FedNowMessageSignatureKey ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct FedNowMessageSignatureKey {
	#[serde(rename = "FedNowKeyID")]
	pub fed_now_key_id: String,
	#[serde(rename = "Name")]
	pub name: String,
	#[serde(rename = "EncodedPublicKey")]
	pub encoded_public_key: String,
	#[serde(rename = "Encoding")]
	pub encoding: String,
	#[serde(rename = "Algorithm")]
	pub algorithm: Option<String>,
	#[serde(rename = "KeyCreationDateTime")]
	pub key_creation_date_time: Option<String>,
}


// KeyAddition ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct KeyAddition {
	#[validate]
	#[serde(rename = "Key")]
	pub key: Option<FedNowMessageSignatureKey>,
}


// KeyRevocation ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct KeyRevocation {
	#[serde(rename = "KeyRevocation")]
	pub key_revocation: Option<String>,
	#[serde(rename = "FedNowStatusDescription")]
	pub fed_now_status_description: Option<String>,
	#[serde(rename = "FedNowKeyID")]
	pub fed_now_key_id: Option<String>,
}


// FedNowMessageSignatureKeyExchange ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct FedNowMessageSignatureKeyExchange {
	#[validate]
	#[serde(rename = "KeyAddition")]
	pub key_addition: Option<KeyAddition>,
	#[serde(rename = "KeyRevocation")]
	pub key_revocation: Option<String>,
}


// FedNowCustomerMessageSignatureKeyOperationResponse ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct FedNowCustomerMessageSignatureKeyOperationResponse {
	#[serde(rename = "FedNowKeyID")]
	pub fed_now_key_id: String,
	#[serde(rename = "Status")]
	pub status: String,
	#[serde(rename = "ErrorCode")]
	pub error_code: Option<String>,
}


// GetAllFedNowActivePublicKeys ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct GetAllFedNowActivePublicKeys {
}


// GetAllCustomerPublicKeys ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct GetAllCustomerPublicKeys {
}


// FedNowPublicKeyResponse ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct FedNowPublicKeyResponse {
	#[validate]
	#[serde(rename = "FedNowMessageSignatureKeyStatus")]
	pub fed_now_message_signature_key_status: FedNowMessageSignatureKeyStatus,
	#[validate]
	#[serde(rename = "FedNowMessageSignatureKey")]
	pub fed_now_message_signature_key: FedNowMessageSignatureKey,
}


// FedNowPublicKeyResponses ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct FedNowPublicKeyResponses {
	#[validate]
	#[serde(rename = "PublicKeys")]
	pub public_keys: Vec<FedNowPublicKeyResponse>,
}
