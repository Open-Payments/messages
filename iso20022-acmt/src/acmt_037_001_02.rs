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


// AccountSwitchDetails1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountSwitchDetails1 {
	#[serde(rename = "UnqRefNb")]
	pub unq_ref_nb: Max35Text,
	#[serde(rename = "RtgUnqRefNb")]
	pub rtg_unq_ref_nb: Max35Text,
	#[serde(rename = "SwtchRcvdDtTm", skip_serializing_if = "Option::is_none")]
	pub swtch_rcvd_dt_tm: Option<String>,
	#[serde(rename = "SwtchDt", skip_serializing_if = "Option::is_none")]
	pub swtch_dt: Option<String>,
	#[serde(rename = "SwtchTp")]
	pub swtch_tp: SwitchType1Code,
	#[serde(rename = "SwtchSts", skip_serializing_if = "Option::is_none")]
	pub swtch_sts: Option<SwitchStatus1Code>,
	#[serde(rename = "BalTrfWndw", skip_serializing_if = "Option::is_none")]
	pub bal_trf_wndw: Option<BalanceTransferWindow1Code>,
	#[serde(rename = "Rspn", skip_serializing_if = "Option::is_none")]
	pub rspn: Option<Vec<ResponseDetails1>>,
}


// AccountSwitchTechnicalRejectionV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountSwitchTechnicalRejectionV02 {
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "AcctSwtchDtls")]
	pub acct_swtch_dtls: AccountSwitchDetails1,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// BalanceTransferWindow1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BalanceTransferWindow1Code {
	#[default]
	#[serde(rename = "DAYH")]
	CodeDAYH,
	#[serde(rename = "EARL")]
	CodeEARL,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}


// MessageIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
}


// ResponseDetails1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ResponseDetails1 {
	#[serde(rename = "RspnCd")]
	pub rspn_cd: Max35Text,
	#[serde(rename = "AddtlDtls", skip_serializing_if = "Option::is_none")]
	pub addtl_dtls: Option<Max350Text>,
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// SwitchStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SwitchStatus1Code {
	#[default]
	#[serde(rename = "ACPT")]
	CodeACPT,
	#[serde(rename = "BTRQ")]
	CodeBTRQ,
	#[serde(rename = "BTRS")]
	CodeBTRS,
	#[serde(rename = "COMP")]
	CodeCOMP,
	#[serde(rename = "REDT")]
	CodeREDT,
	#[serde(rename = "REDE")]
	CodeREDE,
	#[serde(rename = "REJT")]
	CodeREJT,
	#[serde(rename = "REQU")]
	CodeREQU,
	#[serde(rename = "TMTN")]
	CodeTMTN,
}


// SwitchType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SwitchType1Code {
	#[default]
	#[serde(rename = "FULL")]
	CodeFULL,
	#[serde(rename = "PART")]
	CodePART,
}
