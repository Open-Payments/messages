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


// FedNowParticipantFile1 is This is the participant profile of the FedNow participant and contains the participant's identification, name and the FedNow services the participant has enrolled for.
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct FedNowParticipantFile1 {
	#[serde(rename = "BizDay")]
	pub biz_day: String,
	#[validate]
	#[serde(rename = "PtcptPrfl")]
	pub ptcpt_prfl: Vec<FedNowParticipantProfile1>,
}


// FedNowParticipantProfile1 is This specifies the FedNow services the FedNow participant has enrolled for.
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct FedNowParticipantProfile1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Svcs")]
	pub svcs: Vec<String>,
}


// ISODate is A particular point in the progression of time in a calendar year expressed in the YYYY-MM-DD format. This representation is defined in "XML Schema Part 2: Datatypes Second Edition - W3C Recommendation 28 October 2004" which is aligned with ISO 8601.
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// Max140Text is Specifies a character string with a maximum length of 140 characters.
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct Max140Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 140)]
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
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


// ServicesFedNow1 is This indicates a FedNow participant is enabled to receive request for payment messages.
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct ServicesFedNow1 {
	#[validate(enumerate = ["CTSR", "CTRO", "RFPR"])]
	#[serde(rename = "Services_FedNow_1")]
	pub services_fed_now_1: String,
}


// Admi998SuplDataV01 is This is the FedNow participant file and contains the FedNow Service funds-transfer business day and the FedNow participants with their FedNow Service profile.
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct Admi998SuplDataV01 {
	#[validate]
	#[serde(rename = "PtcptFile")]
	pub ptcpt_file: FedNowParticipantFile1,
}
