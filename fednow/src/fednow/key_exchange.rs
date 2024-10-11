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


// Max300AlphaNumericString ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max300AlphaNumericString {
	#[serde(rename = "Max300AlphaNumericString")]
	pub max300_alpha_numeric_string: String,
}


// Max50AlphaNumericString ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max50AlphaNumericString {
	#[serde(rename = "Max50AlphaNumericString")]
	pub max50_alpha_numeric_string: String,
}


// Max300Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max300Text {
	#[serde(rename = "Max300Text")]
	pub max300_text: String,
}


// RoutingNumberFRS1 is This is a routing number used by the Service participant in connection with the message.

//                 

//                 Note: This may be a master account routing number or a subaccount routing number.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RoutingNumberFRS1 {
	#[serde(rename = "RoutingNumber_FRS_1")]
	pub routing_number_frs_1: String,
}



// FedNowMessageSignatureKeyStatus ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowMessageSignatureKeyStatus {
	#[serde(rename = "KeyStatus")]
	pub key_status: String,
	#[serde(rename = "StatusDateTime")]
	pub status_date_time: String,
}



// FedNowMessageSignatureKey ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowMessageSignatureKey {
	#[serde(rename = "FedNowKeyID")]
	pub fed_now_key_id: String,
	#[serde(rename = "Name")]
	pub name: String,
	#[serde(rename = "EncodedPublicKey")]
	pub encoded_public_key: String,
	#[serde(rename = "Encoding")]
	pub encoding: String,
	#[serde(rename = "Algorithm", skip_serializing_if = "Option::is_none")]
	pub algorithm: Option<String>,
	#[serde(rename = "KeyCreationDateTime", skip_serializing_if = "Option::is_none")]
	pub key_creation_date_time: Option<String>,
}


// KeyAddition ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct KeyAddition {
	#[serde(rename = "Key", skip_serializing_if = "Option::is_none")]
	pub key: Option<FedNowMessageSignatureKey>,
}


// KeyRevocation ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct KeyRevocation {
	#[serde(rename = "KeyRevocation", skip_serializing_if = "Option::is_none")]
	pub key_revocation: Option<String>,
	#[serde(rename = "FedNowStatusDescription", skip_serializing_if = "Option::is_none")]
	pub fed_now_status_description: Option<String>,
	#[serde(rename = "FedNowKeyID", skip_serializing_if = "Option::is_none")]
	pub fed_now_key_id: Option<String>,
}


// FedNowMessageSignatureKeyExchange ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowMessageSignatureKeyExchange {
	#[serde(rename = "KeyAddition", skip_serializing_if = "Option::is_none")]
	pub key_addition: Option<KeyAddition>,
	#[serde(rename = "KeyRevocation", skip_serializing_if = "Option::is_none")]
	pub key_revocation: Option<String>,
}


// FedNowCustomerMessageSignatureKeyOperationResponse ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowCustomerMessageSignatureKeyOperationResponse {
	#[serde(rename = "FedNowKeyID")]
	pub fed_now_key_id: String,
	#[serde(rename = "Status")]
	pub status: String,
	#[serde(rename = "ErrorCode", skip_serializing_if = "Option::is_none")]
	pub error_code: Option<String>,
}



// GetAllFedNowActivePublicKeys ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GetAllFedNowActivePublicKeys {
}


// GetAllCustomerPublicKeys ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GetAllCustomerPublicKeys {
}


// FedNowPublicKeyResponse ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowPublicKeyResponse {
	#[serde(rename = "FedNowMessageSignatureKeyStatus")]
	pub fed_now_message_signature_key_status: FedNowMessageSignatureKeyStatus,
	#[serde(rename = "FedNowMessageSignatureKey")]
	pub fed_now_message_signature_key: FedNowMessageSignatureKey,
}


// FedNowPublicKeyResponses ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowPublicKeyResponses {
	#[serde(rename = "PublicKeys")]
	pub public_keys: Vec<FedNowPublicKeyResponse>,
}
