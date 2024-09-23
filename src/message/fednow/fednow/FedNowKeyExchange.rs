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


// Max300AlphaNumericString ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max300AlphaNumericString {
	#[serde(rename = "Max300AlphaNumericString")]
	pub max300_alpha_numeric_string: String,
}


// Max50AlphaNumericString ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max50AlphaNumericString {
	#[serde(rename = "Max50AlphaNumericString")]
	pub max50_alpha_numeric_string: String,
}


// Max300Text ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max300Text {
	#[serde(rename = "Max300Text")]
	pub max300_text: String,
}


// RoutingNumberFRS1 is This is a routing number used by the Service participant in connection with the message.

//                 

//                 Note: This may be a master account routing number or a subaccount routing number.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RoutingNumberFRS1 {
	#[serde(rename = "RoutingNumber_FRS_1")]
	pub routing_number_frs_1: String,
}


// fed_now_key_id is The key finger print used as the key id.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct fed_now_key_id {
	#[serde(rename = "FedNowKeyID")]
	pub fed_now_key_id: String,
}


// fed_now_status_description is A description for the status update intended for the key owner.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct fed_now_status_description {
	#[serde(rename = "FedNowStatusDescription")]
	pub fed_now_status_description: String,
}


// FedNowMessageSignatureKeyStatus ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FedNowMessageSignatureKeyStatus {
	#[serde(rename = "KeyStatus")]
	pub key_status: String,
	#[serde(rename = "StatusDateTime")]
	pub status_date_time: String,
}


// status_date_time ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct status_date_time {
	#[serde(rename = "StatusDateTime")]
	pub status_date_time: String,
}


// key_status ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct key_status {
	#[serde(rename = "KeyStatus")]
	pub key_status: String,
}


// FedNowMessageSignatureKey ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
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


// key_creation_date_time is The creation datetime for the key. This value is overwritten on submission based on the
//                             System timezone for FedNow
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct key_creation_date_time {
	#[serde(rename = "KeyCreationDateTime")]
	pub key_creation_date_time: String,
}


// XsDateTime ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct XsDateTime {
	#[serde(rename = "xs:dateTime")]
	pub xs_date_time: String,
}


// key_expiration_date_time is The Expiration datetime. This is the date and time the customer intends for their keys to be
//                         automatically expired.
//                         This value should be no more than 365 days from the submission time and will be used as a point
//                         in time to the nearest system date.
//                         If the expiration time is 3:00 AM Wednesday, Coordinated Universal Time (UTC)
//                         this translates to 10:00 PM Tuesday, Eastern Time (ET)
// 
//                         For this time, we will take expire this at midnight on the given date.
//                         In the example above, the key will not be valid at 12:01 AM Wednesday, Eastern Time (ET).
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct key_expiration_date_time {
	#[serde(rename = "KeyExpirationDateTime")]
	pub key_expiration_date_time: String,
}


// target_rtn is Identifier of the RTN to associate this key
//                         with. If providing the ETI, the scope of this key is all RTNs managed by that connection party.
//                         If providing a single RTN, FedNow will check and only accept the key for operations against that RTN only
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct target_rtn {
	#[serde(rename = "TargetRTN")]
	pub target_rtn: String,
}


// KeyAddition ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct KeyAddition {
	#[serde(rename = "Key")]
	pub key: Option<FedNowMessageSignatureKey>,
}


// KeyRevocation ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct KeyRevocation {
	#[serde(rename = "KeyRevocation")]
	pub key_revocation: Option<String>,
	#[serde(rename = "FedNowStatusDescription")]
	pub fed_now_status_description: Option<String>,
	#[serde(rename = "FedNowKeyID")]
	pub fed_now_key_id: Option<String>,
}


// FedNowMessageSignatureKeyExchange ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FedNowMessageSignatureKeyExchange {
	#[serde(rename = "KeyAddition")]
	pub key_addition: Option<KeyAddition>,
	#[serde(rename = "KeyRevocation")]
	pub key_revocation: Option<String>,
}


// FedNowCustomerMessageSignatureKeyOperationResponse ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FedNowCustomerMessageSignatureKeyOperationResponse {
	#[serde(rename = "FedNowKeyID")]
	pub fed_now_key_id: String,
	#[serde(rename = "Status")]
	pub status: String,
	#[serde(rename = "ErrorCode")]
	pub error_code: Option<String>,
}


// error_code ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct error_code {
	#[serde(rename = "ErrorCode")]
	pub error_code: String,
}


// GetAllFedNowActivePublicKeys ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GetAllFedNowActivePublicKeys {
}


// GetAllCustomerPublicKeys ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GetAllCustomerPublicKeys {
}


// FedNowPublicKeyResponse ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FedNowPublicKeyResponse {
	#[serde(rename = "FedNowMessageSignatureKeyStatus")]
	pub fed_now_message_signature_key_status: FedNowMessageSignatureKeyStatus,
	#[serde(rename = "FedNowMessageSignatureKey")]
	pub fed_now_message_signature_key: FedNowMessageSignatureKey,
}


// FedNowPublicKeyResponses ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FedNowPublicKeyResponses {
	#[serde(rename = "PublicKeys")]
	pub public_keys: Vec<FedNowPublicKeyResponse>,
}
