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


// AccountSwitchDetails1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AccountSwitchDetails1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnqRefNb") )]
	pub unq_ref_nb: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RtgUnqRefNb") )]
	pub rtg_unq_ref_nb: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SwtchRcvdDtTm", skip_serializing_if = "Option::is_none") )]
	pub swtch_rcvd_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SwtchDt", skip_serializing_if = "Option::is_none") )]
	pub swtch_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SwtchTp") )]
	pub swtch_tp: SwitchType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SwtchSts", skip_serializing_if = "Option::is_none") )]
	pub swtch_sts: Option<SwitchStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BalTrfWndw", skip_serializing_if = "Option::is_none") )]
	pub bal_trf_wndw: Option<BalanceTransferWindow1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rspn", skip_serializing_if = "Option::is_none") )]
	pub rspn: Option<Vec<ResponseDetails1>>,
}

impl AccountSwitchDetails1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.unq_ref_nb.chars().count() < 1 {
			return Err(ValidationError::new(1001, "unq_ref_nb is shorter than the minimum length of 1".to_string()));
		}
		if self.unq_ref_nb.chars().count() > 35 {
			return Err(ValidationError::new(1002, "unq_ref_nb exceeds the maximum length of 35".to_string()));
		}
		if self.rtg_unq_ref_nb.chars().count() < 1 {
			return Err(ValidationError::new(1001, "rtg_unq_ref_nb is shorter than the minimum length of 1".to_string()));
		}
		if self.rtg_unq_ref_nb.chars().count() > 35 {
			return Err(ValidationError::new(1002, "rtg_unq_ref_nb exceeds the maximum length of 35".to_string()));
		}
		if let Err(e) = self.swtch_tp.validate() { return Err(e); }
		if let Some(ref val) = self.swtch_sts { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.bal_trf_wndw { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.rspn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// AccountSwitchTerminationSwitchV01 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AccountSwitchTerminationSwitchV01 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSwtchDtls") )]
	pub acct_swtch_dtls: AccountSwitchDetails1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl AccountSwitchTerminationSwitchV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_id.validate() { return Err(e); }
		if let Err(e) = self.acct_swtch_dtls.validate() { return Err(e); }
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// BalanceTransferWindow1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum BalanceTransferWindow1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DAYH") )]
	CodeDAYH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EARL") )]
	CodeEARL,
}

impl BalanceTransferWindow1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// MessageIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MessageIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
}

impl MessageIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// ResponseDetails1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ResponseDetails1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RspnCd") )]
	pub rspn_cd: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlDtls", skip_serializing_if = "Option::is_none") )]
	pub addtl_dtls: Option<String>,
}

impl ResponseDetails1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.rspn_cd.chars().count() < 1 {
			return Err(ValidationError::new(1001, "rspn_cd is shorter than the minimum length of 1".to_string()));
		}
		if self.rspn_cd.chars().count() > 35 {
			return Err(ValidationError::new(1002, "rspn_cd exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.addtl_dtls {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_dtls is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "addtl_dtls exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}


// SupplementaryData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SupplementaryData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none") )]
	pub plc_and_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Envlp") )]
	pub envlp: SupplementaryDataEnvelope1,
}

impl SupplementaryData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.plc_and_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "plc_and_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "plc_and_nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Err(e) = self.envlp.validate() { return Err(e); }
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


// SwitchStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum SwitchStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACPT") )]
	CodeACPT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BTRQ") )]
	CodeBTRQ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BTRS") )]
	CodeBTRS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COMP") )]
	CodeCOMP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REDT") )]
	CodeREDT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REDE") )]
	CodeREDE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REJT") )]
	CodeREJT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REQU") )]
	CodeREQU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TMTN") )]
	CodeTMTN,
}

impl SwitchStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SwitchType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum SwitchType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FULL") )]
	CodeFULL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PART") )]
	CodePART,
}

impl SwitchType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
