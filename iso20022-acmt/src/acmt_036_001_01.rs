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
use regex::Regex;


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

impl AccountSwitchDetails1 {
	pub fn validate(&self) -> bool {
		if !self.unq_ref_nb.validate() { return false }
		if !self.rtg_unq_ref_nb.validate() { return false }
		if !self.swtch_tp.validate() { return false }
		if let Some(ref swtch_sts_value) = self.swtch_sts { if !swtch_sts_value.validate() { return false; } }
		if let Some(ref bal_trf_wndw_value) = self.bal_trf_wndw { if !bal_trf_wndw_value.validate() { return false; } }
		if let Some(ref rspn_vec) = self.rspn { for item in rspn_vec { if !item.validate() { return false; } } }
		return true
	}
}


// AccountSwitchTerminationSwitchV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountSwitchTerminationSwitchV01 {
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "AcctSwtchDtls")]
	pub acct_swtch_dtls: AccountSwitchDetails1,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl AccountSwitchTerminationSwitchV01 {
	pub fn validate(&self) -> bool {
		if !self.msg_id.validate() { return false }
		if !self.acct_swtch_dtls.validate() { return false }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
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

impl BalanceTransferWindow1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}

impl ISODate {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}

impl ISODateTime {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}

impl Max350Text {
	pub fn validate(&self) -> bool {
		if self.max350_text.chars().count() < 1 {
			return false
		}
		if self.max350_text.chars().count() > 350 {
			return false
		}
		return true
	}
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}

impl Max35Text {
	pub fn validate(&self) -> bool {
		if self.max35_text.chars().count() < 1 {
			return false
		}
		if self.max35_text.chars().count() > 35 {
			return false
		}
		return true
	}
}


// MessageIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
}

impl MessageIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		return true
	}
}


// ResponseDetails1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ResponseDetails1 {
	#[serde(rename = "RspnCd")]
	pub rspn_cd: Max35Text,
	#[serde(rename = "AddtlDtls", skip_serializing_if = "Option::is_none")]
	pub addtl_dtls: Option<Max350Text>,
}

impl ResponseDetails1 {
	pub fn validate(&self) -> bool {
		if !self.rspn_cd.validate() { return false }
		if let Some(ref addtl_dtls_value) = self.addtl_dtls { if !addtl_dtls_value.validate() { return false; } }
		return true
	}
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}

impl SupplementaryData1 {
	pub fn validate(&self) -> bool {
		if let Some(ref plc_and_nm_value) = self.plc_and_nm { if !plc_and_nm_value.validate() { return false; } }
		if !self.envlp.validate() { return false }
		return true
	}
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> bool {
		return true
	}
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

impl SwitchStatus1Code {
	pub fn validate(&self) -> bool {
		return true
	}
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

impl SwitchType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}
