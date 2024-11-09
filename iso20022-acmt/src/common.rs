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

#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};
use open_payments_common::ValidationError;

#[cfg(feature = "derive_samplify")]
use samplify_rs::Sampleable;


// AcceptedStatusReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AcceptedStatusReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AcceptedStatusReason1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl AcceptedStatusReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// AcceptedStatusReason1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum AcceptedStatusReason1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PLAC") )]
	CodePLAC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SECT") )]
	CodeSECT,
}

impl AcceptedStatusReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Account23 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Account23 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctId") )]
	pub acct_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdAcctDtls", skip_serializing_if = "Option::is_none") )]
	pub rltd_acct_dtls: Option<GenericIdentification1>,
}

impl Account23 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.acct_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "acct_id is shorter than the minimum length of 1".to_string()));
		}
		if self.acct_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "acct_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.rltd_acct_dtls { val.validate()? }
		Ok(())
	}
}


// Account32 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Account32 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcr") )]
	pub acct_svcr: PartyIdentification125Choice,
}

impl Account32 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
		}
		self.acct_svcr.validate()?;
		Ok(())
	}
}


// AccountContract2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountContract2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrgtGoLiveDt", skip_serializing_if = "Option::is_none") )]
	pub trgt_go_live_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrgtClsgDt", skip_serializing_if = "Option::is_none") )]
	pub trgt_clsg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UrgcyFlg", skip_serializing_if = "Option::is_none") )]
	pub urgcy_flg: Option<bool>,
}

impl AccountContract2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AccountContract3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountContract3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrgtGoLiveDt", skip_serializing_if = "Option::is_none") )]
	pub trgt_go_live_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrgtClsgDt", skip_serializing_if = "Option::is_none") )]
	pub trgt_clsg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GoLiveDt", skip_serializing_if = "Option::is_none") )]
	pub go_live_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
	pub clsg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UrgcyFlg", skip_serializing_if = "Option::is_none") )]
	pub urgcy_flg: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmvlInd", skip_serializing_if = "Option::is_none") )]
	pub rmvl_ind: Option<bool>,
}

impl AccountContract3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AccountContract4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountContract4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrgtClsgDt", skip_serializing_if = "Option::is_none") )]
	pub trgt_clsg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UrgcyFlg", skip_serializing_if = "Option::is_none") )]
	pub urgcy_flg: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmvlInd", skip_serializing_if = "Option::is_none") )]
	pub rmvl_ind: Option<bool>,
}

impl AccountContract4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AccountDesignation1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountDesignation1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Rank1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl AccountDesignation1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// AccountForAction1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountForAction1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: AccountIdentification4Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
}

impl AccountForAction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.ccy) {
			return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// AccountForAction2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountForAction2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: AccountIdentification4Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
}

impl AccountForAction2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 70".to_string()));
			}
		}
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.ccy) {
			return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// AccountIdentification4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountIdentification4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IBAN", skip_serializing_if = "Option::is_none") )]
	pub iban: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<GenericAccountIdentification1>,
}

impl AccountIdentification4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.iban {
			let pattern = Regex::new("[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "iban does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.othr { val.validate()? }
		Ok(())
	}
}


// AccountIdentificationAndName5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountIdentificationAndName5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: AccountIdentification4Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
}

impl AccountIdentificationAndName5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// AccountManagementConfirmation5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountManagementConfirmation5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConfTp") )]
	pub conf_tp: ConfirmationType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctApplId", skip_serializing_if = "Option::is_none") )]
	pub acct_appl_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClntRef", skip_serializing_if = "Option::is_none") )]
	pub clnt_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyRef", skip_serializing_if = "Option::is_none") )]
	pub ctr_pty_ref: Option<AdditionalReference13>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExstgAcctId", skip_serializing_if = "Option::is_none") )]
	pub exstg_acct_id: Option<Vec<Account23>>,
}

impl AccountManagementConfirmation5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.conf_tp.validate()?;
		if let Some(ref val) = self.acct_appl_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_appl_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_appl_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.clnt_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clnt_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clnt_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctr_pty_ref { val.validate()? }
		if let Some(ref vec) = self.exstg_acct_id { for item in vec { item.validate()? } }
		Ok(())
	}
}


// AccountManagementMessageReference5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountManagementMessageReference5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "LkdRef", skip_serializing_if = "Option::is_none") )]
	pub lkd_ref: Option<LinkedMessage5Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsReqTp") )]
	pub sts_req_tp: AccountManagementType3Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctApplId", skip_serializing_if = "Option::is_none") )]
	pub acct_appl_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExstgAcctId", skip_serializing_if = "Option::is_none") )]
	pub exstg_acct_id: Option<Account23>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtAcct", skip_serializing_if = "Option::is_none") )]
	pub invstmt_acct: Option<InvestmentAccount77>,
}

impl AccountManagementMessageReference5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.lkd_ref { val.validate()? }
		self.sts_req_tp.validate()?;
		if let Some(ref val) = self.acct_appl_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_appl_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_appl_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.exstg_acct_id { val.validate()? }
		if let Some(ref val) = self.invstmt_acct { val.validate()? }
		Ok(())
	}
}


// AccountManagementStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum AccountManagementStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RECE") )]
	CodeRECE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACCP") )]
	CodeACCP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EXEC") )]
	CodeEXEC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STNP") )]
	CodeSTNP,
}

impl AccountManagementStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AccountManagementStatusAndReason5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountManagementStatusAndReason5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
	pub sts: Status25Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsRsn", skip_serializing_if = "Option::is_none") )]
	pub sts_rsn: Option<Vec<AcceptedStatusReason1Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctApplId", skip_serializing_if = "Option::is_none") )]
	pub acct_appl_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExstgAcctId", skip_serializing_if = "Option::is_none") )]
	pub exstg_acct_id: Option<Vec<Account23>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctId", skip_serializing_if = "Option::is_none") )]
	pub acct_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSts", skip_serializing_if = "Option::is_none") )]
	pub acct_sts: Option<AccountStatus2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BlckdSts", skip_serializing_if = "Option::is_none") )]
	pub blckd_sts: Option<BlockedStatusReason2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FATCARptgDt", skip_serializing_if = "Option::is_none") )]
	pub fatca_rptg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRSRptgDt", skip_serializing_if = "Option::is_none") )]
	pub crs_rptg_dt: Option<String>,
}

impl AccountManagementStatusAndReason5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.sts.validate()?;
		if let Some(ref vec) = self.sts_rsn { for item in vec { item.validate()? } }
		if let Some(ref val) = self.acct_appl_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_appl_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_appl_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.exstg_acct_id { for item in vec { item.validate()? } }
		if let Some(ref val) = self.acct_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.acct_sts { val.validate()? }
		if let Some(ref val) = self.blckd_sts { val.validate()? }
		Ok(())
	}
}


// AccountManagementType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum AccountManagementType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACCO") )]
	CodeACCO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACCM") )]
	CodeACCM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GACC") )]
	CodeGACC,
}

impl AccountManagementType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AccountManagementType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum AccountManagementType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACCM") )]
	CodeACCM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACCO") )]
	CodeACCO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GACC") )]
	CodeGACC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACST") )]
	CodeACST,
}

impl AccountManagementType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AccountOpeningType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountOpeningType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AccountOpeningType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl AccountOpeningType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// AccountOpeningType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum AccountOpeningType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEWA") )]
	CodeNEWA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SUPA") )]
	CodeSUPA,
}

impl AccountOpeningType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AccountOwner3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountOwner3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IndvOwnrId", skip_serializing_if = "Option::is_none") )]
	pub indv_ownr_id: Option<IndividualPersonIdentification3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgOwnrId", skip_serializing_if = "Option::is_none") )]
	pub org_ownr_id: Option<PartyIdentification220>,
}

impl AccountOwner3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.indv_ownr_id { val.validate()? }
		if let Some(ref val) = self.org_ownr_id { val.validate()? }
		Ok(())
	}
}


// AccountOwnershipType4Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum AccountOwnershipType4Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "UNCO") )]
	CodeUNCO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIPA") )]
	CodeLIPA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ENTR") )]
	CodeENTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CORP") )]
	CodeCORP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CUST") )]
	CodeCUST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EURE") )]
	CodeEURE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PART") )]
	CodePART,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRUS") )]
	CodeTRUS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GOVO") )]
	CodeGOVO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "JOIT") )]
	CodeJOIT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COMO") )]
	CodeCOMO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "JOIN") )]
	CodeJOIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LLCO") )]
	CodeLLCO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOMI") )]
	CodeNOMI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NFPO") )]
	CodeNFPO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ONIS") )]
	CodeONIS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RGIC") )]
	CodeRGIC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SIGL") )]
	CodeSIGL,
}

impl AccountOwnershipType4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AccountParties12Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountParties12Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmryOwnr", skip_serializing_if = "Option::is_none") )]
	pub pmry_ownr: Option<InvestmentAccountOwnershipInformation16>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Trstee", skip_serializing_if = "Option::is_none") )]
	pub trstee: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nmnee", skip_serializing_if = "Option::is_none") )]
	pub nmnee: Option<InvestmentAccountOwnershipInformation16>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "JntOwnr", skip_serializing_if = "Option::is_none") )]
	pub jnt_ownr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
}

impl AccountParties12Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pmry_ownr { val.validate()? }
		if let Some(ref vec) = self.trstee { for item in vec { item.validate()? } }
		if let Some(ref val) = self.nmnee { val.validate()? }
		if let Some(ref vec) = self.jnt_ownr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// AccountParties13Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountParties13Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmryOwnr", skip_serializing_if = "Option::is_none") )]
	pub pmry_ownr: Option<InvestmentAccountOwnershipInformation17>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Trstee", skip_serializing_if = "Option::is_none") )]
	pub trstee: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nmnee", skip_serializing_if = "Option::is_none") )]
	pub nmnee: Option<InvestmentAccountOwnershipInformation17>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "JntOwnr", skip_serializing_if = "Option::is_none") )]
	pub jnt_ownr: Option<Vec<InvestmentAccountOwnershipInformation17>>,
}

impl AccountParties13Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pmry_ownr { val.validate()? }
		if let Some(ref vec) = self.trstee { for item in vec { item.validate()? } }
		if let Some(ref val) = self.nmnee { val.validate()? }
		if let Some(ref vec) = self.jnt_ownr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// AccountParties17 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountParties17 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrncplAcctPty") )]
	pub prncpl_acct_pty: AccountParties12Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ScndryOwnr", skip_serializing_if = "Option::is_none") )]
	pub scndry_ownr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none") )]
	pub bnfcry: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PwrOfAttny", skip_serializing_if = "Option::is_none") )]
	pub pwr_of_attny: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglGuardn", skip_serializing_if = "Option::is_none") )]
	pub lgl_guardn: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtdnForMnr", skip_serializing_if = "Option::is_none") )]
	pub ctdn_for_mnr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SucssrOnDth", skip_serializing_if = "Option::is_none") )]
	pub sucssr_on_dth: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Admstr", skip_serializing_if = "Option::is_none") )]
	pub admstr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPty", skip_serializing_if = "Option::is_none") )]
	pub othr_pty: Option<Vec<ExtendedParty14>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Grntr", skip_serializing_if = "Option::is_none") )]
	pub grntr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sttlr", skip_serializing_if = "Option::is_none") )]
	pub sttlr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SnrMggOffcl", skip_serializing_if = "Option::is_none") )]
	pub snr_mgg_offcl: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtctr", skip_serializing_if = "Option::is_none") )]
	pub prtctr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegdShrhldrNm", skip_serializing_if = "Option::is_none") )]
	pub regd_shrhldr_nm: Option<RegisteredShareholderName1Choice>,
}

impl AccountParties17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.prncpl_acct_pty.validate()?;
		if let Some(ref vec) = self.scndry_ownr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.bnfcry { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.pwr_of_attny { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.lgl_guardn { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ctdn_for_mnr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.sucssr_on_dth { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.admstr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.othr_pty { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.grntr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.sttlr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.snr_mgg_offcl { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.prtctr { for item in vec { item.validate()? } }
		if let Some(ref val) = self.regd_shrhldr_nm { val.validate()? }
		Ok(())
	}
}


// AccountParties18 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountParties18 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModScpIndctn") )]
	pub mod_scp_indctn: DataModification1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrncplAcctPty", skip_serializing_if = "Option::is_none") )]
	pub prncpl_acct_pty: Option<AccountParties13Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ScndryOwnr", skip_serializing_if = "Option::is_none") )]
	pub scndry_ownr: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none") )]
	pub bnfcry: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PwrOfAttny", skip_serializing_if = "Option::is_none") )]
	pub pwr_of_attny: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglGuardn", skip_serializing_if = "Option::is_none") )]
	pub lgl_guardn: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtdnForMnr", skip_serializing_if = "Option::is_none") )]
	pub ctdn_for_mnr: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SucssrOnDth", skip_serializing_if = "Option::is_none") )]
	pub sucssr_on_dth: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Admstr", skip_serializing_if = "Option::is_none") )]
	pub admstr: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPty", skip_serializing_if = "Option::is_none") )]
	pub othr_pty: Option<Vec<ExtendedParty15>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Grntr", skip_serializing_if = "Option::is_none") )]
	pub grntr: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sttlr", skip_serializing_if = "Option::is_none") )]
	pub sttlr: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SnrMggOffcl", skip_serializing_if = "Option::is_none") )]
	pub snr_mgg_offcl: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtctr", skip_serializing_if = "Option::is_none") )]
	pub prtctr: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegdShrhldrNm", skip_serializing_if = "Option::is_none") )]
	pub regd_shrhldr_nm: Option<RegisteredShareholderName1Choice>,
}

impl AccountParties18 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.mod_scp_indctn.validate()?;
		if let Some(ref val) = self.prncpl_acct_pty { val.validate()? }
		if let Some(ref vec) = self.scndry_ownr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.bnfcry { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.pwr_of_attny { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.lgl_guardn { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ctdn_for_mnr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.sucssr_on_dth { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.admstr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.othr_pty { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.grntr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.sttlr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.snr_mgg_offcl { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.prtctr { for item in vec { item.validate()? } }
		if let Some(ref val) = self.regd_shrhldr_nm { val.validate()? }
		Ok(())
	}
}


// AccountReport36 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountReport36 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Acct") )]
	pub acct: CustomerAccount5,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygMstrAgrmt", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_mstr_agrmt: Option<ContractDocument1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctDts", skip_serializing_if = "Option::is_none") )]
	pub ctrct_dts: Option<AccountContract3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mndt", skip_serializing_if = "Option::is_none") )]
	pub mndt: Option<Vec<OperationMandate7>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Grp", skip_serializing_if = "Option::is_none") )]
	pub grp: Option<Vec<Group6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefAcct", skip_serializing_if = "Option::is_none") )]
	pub ref_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BalTrfAcct", skip_serializing_if = "Option::is_none") )]
	pub bal_trf_acct: Option<AccountForAction1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrfAcctSvcrId", skip_serializing_if = "Option::is_none") )]
	pub trf_acct_svcr_id: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl AccountReport36 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.acct.validate()?;
		if let Some(ref val) = self.undrlyg_mstr_agrmt { val.validate()? }
		if let Some(ref val) = self.ctrct_dts { val.validate()? }
		if let Some(ref vec) = self.mndt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.grp { for item in vec { item.validate()? } }
		if let Some(ref val) = self.ref_acct { val.validate()? }
		if let Some(ref val) = self.bal_trf_acct { val.validate()? }
		if let Some(ref val) = self.trf_acct_svcr_id { val.validate()? }
		Ok(())
	}
}


// AccountSchemeName1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountSchemeName1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl AccountSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// AccountSelection3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountSelection3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctId", skip_serializing_if = "Option::is_none") )]
	pub acct_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrAcctSelctnData", skip_serializing_if = "Option::is_none") )]
	pub othr_acct_selctn_data: Option<InvestmentAccount76>,
}

impl AccountSelection3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.acct_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.othr_acct_selctn_data { val.validate()? }
		Ok(())
	}
}


// AccountStatus2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountStatus2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nbld", skip_serializing_if = "Option::is_none") )]
	pub nbld: Option<EnabledStatusReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dsbld", skip_serializing_if = "Option::is_none") )]
	pub dsbld: Option<DisabledStatusReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pdg", skip_serializing_if = "Option::is_none") )]
	pub pdg: Option<PendingStatusReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdgOpng", skip_serializing_if = "Option::is_none") )]
	pub pdg_opng: Option<PendingOpeningStatusReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Profrm", skip_serializing_if = "Option::is_none") )]
	pub profrm: Option<ProformaStatusReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Clsd", skip_serializing_if = "Option::is_none") )]
	pub clsd: Option<ClosedStatusReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsrPdg", skip_serializing_if = "Option::is_none") )]
	pub clsr_pdg: Option<ClosurePendingStatusReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<OtherAccountStatus1>>,
}

impl AccountStatus2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nbld { val.validate()? }
		if let Some(ref val) = self.dsbld { val.validate()? }
		if let Some(ref val) = self.pdg { val.validate()? }
		if let Some(ref val) = self.pdg_opng { val.validate()? }
		if let Some(ref val) = self.profrm { val.validate()? }
		if let Some(ref val) = self.clsd { val.validate()? }
		if let Some(ref val) = self.clsr_pdg { val.validate()? }
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// AccountStatus3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum AccountStatus3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ENAB") )]
	CodeENAB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DISA") )]
	CodeDISA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DELE") )]
	CodeDELE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FORM") )]
	CodeFORM,
}

impl AccountStatus3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AccountStatusModification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountStatusModification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModCd", skip_serializing_if = "Option::is_none") )]
	pub mod_cd: Option<Modification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
	pub sts: AccountStatus3Code,
}

impl AccountStatusModification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mod_cd { val.validate()? }
		self.sts.validate()?;
		Ok(())
	}
}


// AccountStatusUpdateInstruction1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountStatusUpdateInstruction1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UpdInstr") )]
	pub upd_instr: AccountStatusUpdateInstruction1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UpdInstrRsn", skip_serializing_if = "Option::is_none") )]
	pub upd_instr_rsn: Option<AccountStatusUpdateInstructionReason1Choice>,
}

impl AccountStatusUpdateInstruction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.upd_instr.validate()?;
		if let Some(ref val) = self.upd_instr_rsn { val.validate()? }
		Ok(())
	}
}


// AccountStatusUpdateInstruction1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountStatusUpdateInstruction1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AccountStatusUpdateInstruction1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl AccountStatusUpdateInstruction1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// AccountStatusUpdateInstruction1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum AccountStatusUpdateInstruction1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLOS") )]
	CodeCLOS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REAC") )]
	CodeREAC,
}

impl AccountStatusUpdateInstruction1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AccountStatusUpdateInstructionReason1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountStatusUpdateInstructionReason1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AccountStatusUpdateInstructionReason2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl AccountStatusUpdateInstructionReason1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}


// AccountStatusUpdateInstructionReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountStatusUpdateInstructionReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<Vec<AccountStatusUpdateInstructionReason1>>,
}

impl AccountStatusUpdateInstructionReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.no_spcfd_rsn { val.validate()? }
		if let Some(ref vec) = self.rsn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// AccountStatusUpdateInstructionReason2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountStatusUpdateInstructionReason2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AccountStatusUpdateRequestReason1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl AccountStatusUpdateInstructionReason2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// AccountStatusUpdateRequestReason1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum AccountStatusUpdateRequestReason1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLOE") )]
	CodeCLOE,
}

impl AccountStatusUpdateRequestReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AccountSwitchDetails1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
		self.swtch_tp.validate()?;
		if let Some(ref val) = self.swtch_sts { val.validate()? }
		if let Some(ref val) = self.bal_trf_wndw { val.validate()? }
		if let Some(ref vec) = self.rspn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// AccountType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<FundCashAccount4Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl AccountType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// AccountUsageType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountUsageType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AccountUsageType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl AccountUsageType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// AccountUsageType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum AccountUsageType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "INVE") )]
	CodeINVE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISSP") )]
	CodeISSP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SETP") )]
	CodeSETP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRDP") )]
	CodeTRDP,
}

impl AccountUsageType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AccountingStatus1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountingStatus1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AccountingStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl AccountingStatus1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// AccountingStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum AccountingStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "YDOM") )]
	CodeYDOM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NDOM") )]
	CodeNDOM,
}

impl AccountingStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ActiveCurrencyAnd13DecimalAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ActiveCurrencyAnd13DecimalAmount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl ActiveCurrencyAnd13DecimalAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ActiveCurrencyAndAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ActiveCurrencyAndAmount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl ActiveCurrencyAndAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ActiveOrHistoricCurrencyAndAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAndAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AdditionalInformation5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AdditionalInformation5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Inf") )]
	pub inf: Vec<String>,
}

impl AdditionalInformation5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.inf {
			if item.chars().count() < 1 {
				return Err(ValidationError::new(1001, "inf is shorter than the minimum length of 1".to_string()));
			}
			if item.chars().count() > 256 {
				return Err(ValidationError::new(1002, "inf exceeds the maximum length of 256".to_string()));
			}
		}
		Ok(())
	}
}


// AdditionalReference13 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AdditionalReference13 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
	pub ref_attr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefIssr", skip_serializing_if = "Option::is_none") )]
	pub ref_issr: Option<PartyIdentification125Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNm", skip_serializing_if = "Option::is_none") )]
	pub msg_nm: Option<String>,
}

impl AdditionalReference13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.ref_attr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "ref_attr is shorter than the minimum length of 1".to_string()));
		}
		if self.ref_attr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "ref_attr exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.ref_issr { val.validate()? }
		if let Some(ref val) = self.msg_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// AdditiononalInformation13 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AdditiononalInformation13 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Lmttn", skip_serializing_if = "Option::is_none") )]
	pub lmttn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctVldtn", skip_serializing_if = "Option::is_none") )]
	pub acct_vldtn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rgltr", skip_serializing_if = "Option::is_none") )]
	pub rgltr: Option<PartyIdentification125Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
	pub sts: Option<RestrictionStatus1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
	pub prd: Option<DateTimePeriod2>,
}

impl AdditiononalInformation13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.lmttn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "lmttn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "lmttn exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.acct_vldtn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_vldtn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "acct_vldtn exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.rgltr { val.validate()? }
		if let Some(ref val) = self.sts { val.validate()? }
		if let Some(ref val) = self.prd { val.validate()? }
		Ok(())
	}
}


// AddressModification3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AddressModification3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModCd", skip_serializing_if = "Option::is_none") )]
	pub mod_cd: Option<Modification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adr") )]
	pub adr: PostalAddress27,
}

impl AddressModification3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mod_cd { val.validate()? }
		self.adr.validate()?;
		Ok(())
	}
}


// AddressType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AddressType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AddressType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl AddressType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// AddressType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum AddressType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "HOME") )]
	CodeHOME,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BIZZ") )]
	CodeBIZZ,
}

impl AddressType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AddressType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AddressType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AddressType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl AddressType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// AddressType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum AddressType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ADDR") )]
	CodeADDR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PBOX") )]
	CodePBOX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HOME") )]
	CodeHOME,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BIZZ") )]
	CodeBIZZ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MLTO") )]
	CodeMLTO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DLVY") )]
	CodeDLVY,
}

impl AddressType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AddressType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AddressType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AddressType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl AddressType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// AlternateSecurityIdentification7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AlternateSecurityIdentification7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IdSrc") )]
	pub id_src: IdentificationSource1Choice,
}

impl AlternateSecurityIdentification7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		self.id_src.validate()?;
		Ok(())
	}
}


// AmountAndDirection5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AmountAndDirection5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none") )]
	pub cdt_dbt: Option<CreditDebitCode>,
}

impl AmountAndDirection5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		if let Some(ref val) = self.cdt_dbt { val.validate()? }
		Ok(())
	}
}


// AmountModification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AmountModification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModCd", skip_serializing_if = "Option::is_none") )]
	pub mod_cd: Option<Modification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: f64,
}

impl AmountModification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mod_cd { val.validate()? }
		if self.amt < 0.000000 {
			return Err(ValidationError::new(1003, "amt is less than the minimum value of 0.000000".to_string()));
		}
		Ok(())
	}
}


// Authorisation2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Authorisation2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MaxAmtByTx", skip_serializing_if = "Option::is_none") )]
	pub max_amt_by_tx: Option<FixedAmountOrUnlimited1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MaxAmtByPrd", skip_serializing_if = "Option::is_none") )]
	pub max_amt_by_prd: Option<Vec<MaximumAmountByPeriod1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MaxAmtByBlkSubmissn", skip_serializing_if = "Option::is_none") )]
	pub max_amt_by_blk_submissn: Option<FixedAmountOrUnlimited1Choice>,
}

impl Authorisation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.max_amt_by_tx { val.validate()? }
		if let Some(ref vec) = self.max_amt_by_prd { for item in vec { item.validate()? } }
		if let Some(ref val) = self.max_amt_by_blk_submissn { val.validate()? }
		Ok(())
	}
}


// BalanceTransfer5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct BalanceTransfer5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BalTrfRef", skip_serializing_if = "Option::is_none") )]
	pub bal_trf_ref: Option<BalanceTransferReference1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BalTrfMtd", skip_serializing_if = "Option::is_none") )]
	pub bal_trf_mtd: Option<SettlementMethod5Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BalTrfFndgLmt", skip_serializing_if = "Option::is_none") )]
	pub bal_trf_fndg_lmt: Option<BalanceTransferFundingLimit1>,
}

impl BalanceTransfer5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.bal_trf_ref { val.validate()? }
		if let Some(ref val) = self.bal_trf_mtd { val.validate()? }
		if let Some(ref val) = self.bal_trf_fndg_lmt { val.validate()? }
		Ok(())
	}
}


// BalanceTransferFundingLimit1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct BalanceTransferFundingLimit1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CcyAmt") )]
	pub ccy_amt: ActiveCurrencyAndAmount,
}

impl BalanceTransferFundingLimit1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.ccy_amt.validate()?;
		Ok(())
	}
}


// BalanceTransferReference1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct BalanceTransferReference1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BalTrfRef") )]
	pub bal_trf_ref: String,
}

impl BalanceTransferReference1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.bal_trf_ref.chars().count() < 1 {
			return Err(ValidationError::new(1001, "bal_trf_ref is shorter than the minimum length of 1".to_string()));
		}
		if self.bal_trf_ref.chars().count() > 35 {
			return Err(ValidationError::new(1002, "bal_trf_ref exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// BalanceTransferWindow1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// BankTransactionCodeStructure4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct BankTransactionCodeStructure4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Domn", skip_serializing_if = "Option::is_none") )]
	pub domn: Option<BankTransactionCodeStructure5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<ProprietaryBankTransactionCodeStructure1>,
}

impl BankTransactionCodeStructure4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.domn { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// BankTransactionCodeStructure5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct BankTransactionCodeStructure5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
	pub cd: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Fmly") )]
	pub fmly: BankTransactionCodeStructure6,
}

impl BankTransactionCodeStructure5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.cd.chars().count() < 1 {
			return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
		}
		if self.cd.chars().count() > 4 {
			return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
		}
		self.fmly.validate()?;
		Ok(())
	}
}


// BankTransactionCodeStructure6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct BankTransactionCodeStructure6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
	pub cd: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubFmlyCd") )]
	pub sub_fmly_cd: String,
}

impl BankTransactionCodeStructure6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.cd.chars().count() < 1 {
			return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
		}
		if self.cd.chars().count() > 4 {
			return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
		}
		if self.sub_fmly_cd.chars().count() < 1 {
			return Err(ValidationError::new(1001, "sub_fmly_cd is shorter than the minimum length of 1".to_string()));
		}
		if self.sub_fmly_cd.chars().count() > 4 {
			return Err(ValidationError::new(1002, "sub_fmly_cd exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// BlockedHoldingDetails2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct BlockedHoldingDetails2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BlckdHldg") )]
	pub blckd_hldg: Holding1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtlHldgUnits", skip_serializing_if = "Option::is_none") )]
	pub prtl_hldg_units: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HldgCertNb", skip_serializing_if = "Option::is_none") )]
	pub hldg_cert_nb: Option<String>,
}

impl BlockedHoldingDetails2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.blckd_hldg.validate()?;
		if let Some(ref val) = self.hldg_cert_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "hldg_cert_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "hldg_cert_nb exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// BlockedReason2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct BlockedReason2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<BlockedReason2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl BlockedReason2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// BlockedReason2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum BlockedReason2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BKRP") )]
	CodeBKRP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CMMT") )]
	CodeCMMT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CNFS") )]
	CodeCNFS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MORT") )]
	CodeMORT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PCOM") )]
	CodePCOM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PLDG") )]
	CodePLDG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRPE") )]
	CodeTRPE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SANC") )]
	CodeSANC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRAN") )]
	CodeTRAN,
}

impl BlockedReason2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// BlockedStatusReason2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct BlockedStatusReason2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxTp") )]
	pub tx_tp: TransactionType5Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Blckd") )]
	pub blckd: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<Vec<BlockedReason2Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf") )]
	pub addtl_inf: String,
}

impl BlockedStatusReason2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tx_tp.validate()?;
		if let Some(ref vec) = self.rsn { for item in vec { item.validate()? } }
		if self.addtl_inf.chars().count() < 1 {
			return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
		}
		if self.addtl_inf.chars().count() > 350 {
			return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
		}
		Ok(())
	}
}


// BlockedStatusReason2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct BlockedStatusReason2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<Vec<BlockedStatusReason2>>,
}

impl BlockedStatusReason2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.no_spcfd_rsn { val.validate()? }
		if let Some(ref vec) = self.rsn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// BranchAndFinancialInstitutionIdentification8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct BranchAndFinancialInstitutionIdentification8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstnId") )]
	pub fin_instn_id: FinancialInstitutionIdentification23,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BrnchId", skip_serializing_if = "Option::is_none") )]
	pub brnch_id: Option<BranchData5>,
}

impl BranchAndFinancialInstitutionIdentification8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.fin_instn_id.validate()?;
		if let Some(ref val) = self.brnch_id { val.validate()? }
		Ok(())
	}
}


// BranchData4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct BranchData4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<PostalAddress1>,
}

impl BranchData4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		Ok(())
	}
}


// BranchData5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct BranchData5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<PostalAddress27>,
}

impl BranchData5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		Ok(())
	}
}


// BusinessDayConvention1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum BusinessDayConvention1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FWNG") )]
	CodeFWNG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PREC") )]
	CodePREC,
}

impl BusinessDayConvention1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CRSForm1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CRSForm1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<CRSFormType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl CRSForm1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// CRSFormType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum CRSFormType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CER4") )]
	CodeCER4,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CER3") )]
	CodeCER3,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CER5") )]
	CodeCER5,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CER6") )]
	CodeCER6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CER8") )]
	CodeCER8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CER1") )]
	CodeCER1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CER2") )]
	CodeCER2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CER7") )]
	CodeCER7,
}

impl CRSFormType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CRSSource1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CRSSource1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<CRSSourceStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl CRSSource1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// CRSSourceStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum CRSSourceStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CALC") )]
	CodeCALC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DECL") )]
	CodeDECL,
}

impl CRSSourceStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CRSStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum CRSStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "C101") )]
	CodeC101,
	#[cfg_attr( feature = "derive_serde", serde(rename = "C102") )]
	CodeC102,
	#[cfg_attr( feature = "derive_serde", serde(rename = "C103") )]
	CodeC103,
	#[cfg_attr( feature = "derive_serde", serde(rename = "C104") )]
	CodeC104,
	#[cfg_attr( feature = "derive_serde", serde(rename = "C105") )]
	CodeC105,
	#[cfg_attr( feature = "derive_serde", serde(rename = "C106") )]
	CodeC106,
	#[cfg_attr( feature = "derive_serde", serde(rename = "C107") )]
	CodeC107,
	#[cfg_attr( feature = "derive_serde", serde(rename = "C108") )]
	CodeC108,
	#[cfg_attr( feature = "derive_serde", serde(rename = "C109") )]
	CodeC109,
	#[cfg_attr( feature = "derive_serde", serde(rename = "C110") )]
	CodeC110,
	#[cfg_attr( feature = "derive_serde", serde(rename = "C111") )]
	CodeC111,
	#[cfg_attr( feature = "derive_serde", serde(rename = "C112") )]
	CodeC112,
	#[cfg_attr( feature = "derive_serde", serde(rename = "C113") )]
	CodeC113,
	#[cfg_attr( feature = "derive_serde", serde(rename = "C114") )]
	CodeC114,
}

impl CRSStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CRSStatus3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CRSStatus3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<CRSStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl CRSStatus3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// CRSStatus4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CRSStatus4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: CRSStatus3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Src", skip_serializing_if = "Option::is_none") )]
	pub src: Option<CRSSource1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XcptnlRptgCtry", skip_serializing_if = "Option::is_none") )]
	pub xcptnl_rptg_ctry: Option<String>,
}

impl CRSStatus4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		if let Some(ref val) = self.src { val.validate()? }
		if let Some(ref val) = self.xcptnl_rptg_ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "xcptnl_rptg_ctry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// CardType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum CardType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRDT") )]
	CodeCRDT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DBIT") )]
	CodeDBIT,
}

impl CardType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CashAccount204 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CashAccount204 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCcy") )]
	pub sttlm_ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: AccountIdentificationAndName5,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none") )]
	pub acct_ownr: Option<PartyIdentification125Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr: Option<FinancialInstitutionIdentification11Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrBrnch", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr_brnch: Option<BranchData4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOwnrOthrId", skip_serializing_if = "Option::is_none") )]
	pub acct_ownr_othr_id: Option<Vec<GenericIdentification82>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtAcctTp", skip_serializing_if = "Option::is_none") )]
	pub invstmt_acct_tp: Option<AccountType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none") )]
	pub cdt_dbt: Option<CreditDebit3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmInstrRsn", skip_serializing_if = "Option::is_none") )]
	pub sttlm_instr_rsn: Option<SettlementInstructionReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctPurp", skip_serializing_if = "Option::is_none") )]
	pub csh_acct_purp: Option<CashAccountType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctDsgnt", skip_serializing_if = "Option::is_none") )]
	pub csh_acct_dsgnt: Option<AccountDesignation1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DvddPctg", skip_serializing_if = "Option::is_none") )]
	pub dvdd_pctg: Option<f64>,
}

impl CashAccount204 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.sttlm_ccy) {
			return Err(ValidationError::new(1005, "sttlm_ccy does not match the required pattern".to_string()));
		}
		self.id.validate()?;
		if let Some(ref val) = self.acct_ownr { val.validate()? }
		if let Some(ref val) = self.acct_svcr { val.validate()? }
		if let Some(ref val) = self.acct_svcr_brnch { val.validate()? }
		if let Some(ref vec) = self.acct_ownr_othr_id { for item in vec { item.validate()? } }
		if let Some(ref val) = self.invstmt_acct_tp { val.validate()? }
		if let Some(ref val) = self.cdt_dbt { val.validate()? }
		if let Some(ref val) = self.sttlm_instr_rsn { val.validate()? }
		if let Some(ref val) = self.csh_acct_purp { val.validate()? }
		if let Some(ref val) = self.csh_acct_dsgnt { val.validate()? }
		if let Some(ref val) = self.dvdd_pctg {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "dvdd_pctg is less than the minimum value of 0.000000".to_string()));
			}
			if *val > 100.000000 {
				return Err(ValidationError::new(1004, "dvdd_pctg exceeds the maximum value of 100.000000".to_string()));
			}
		}
		Ok(())
	}
}


// CashAccount40 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CashAccount40 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<AccountIdentification4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<CashAccountType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
	pub ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prxy", skip_serializing_if = "Option::is_none") )]
	pub prxy: Option<ProxyAccountIdentification1>,
}

impl CashAccount40 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.prxy { val.validate()? }
		Ok(())
	}
}


// CashAccount43 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CashAccount43 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<AccountIdentification4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<CashAccountType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
	pub ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prxy", skip_serializing_if = "Option::is_none") )]
	pub prxy: Option<ProxyAccountIdentification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ownr", skip_serializing_if = "Option::is_none") )]
	pub ownr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Svcr", skip_serializing_if = "Option::is_none") )]
	pub svcr: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl CashAccount43 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.prxy { val.validate()? }
		if let Some(ref val) = self.ownr { val.validate()? }
		if let Some(ref val) = self.svcr { val.validate()? }
		Ok(())
	}
}


// CashAccountType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CashAccountType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl CashAccountType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CashAccountType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CashAccountType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<CashAccountType5Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl CashAccountType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// CashAccountType5Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum CashAccountType5Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEND") )]
	CodeLEND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COLL") )]
	CodeCOLL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SETT") )]
	CodeSETT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MARR") )]
	CodeMARR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SEGT") )]
	CodeSEGT,
}

impl CashAccountType5Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CashSettlement3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CashSettlement3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctDtls", skip_serializing_if = "Option::is_none") )]
	pub csh_acct_dtls: Option<Vec<CashAccount204>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCshSttlmDtls", skip_serializing_if = "Option::is_none") )]
	pub othr_csh_sttlm_dtls: Option<Vec<PaymentInstrument17>>,
}

impl CashSettlement3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.csh_acct_dtls { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.othr_csh_sttlm_dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CashSettlement4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CashSettlement4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModScpIndctn") )]
	pub mod_scp_indctn: DataModification2Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctDtls", skip_serializing_if = "Option::is_none") )]
	pub csh_acct_dtls: Option<Vec<CashAccount204>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCshSttlmDtls", skip_serializing_if = "Option::is_none") )]
	pub othr_csh_sttlm_dtls: Option<Vec<PaymentInstrument17>>,
}

impl CashSettlement4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.mod_scp_indctn.validate()?;
		if let Some(ref vec) = self.csh_acct_dtls { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.othr_csh_sttlm_dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CategoryPurpose1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CategoryPurpose1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl CategoryPurpose1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CertificateType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum CertificateType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AMLC") )]
	CodeAMLC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DVLC") )]
	CodeDVLC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DFOR") )]
	CodeDFOR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GOST") )]
	CodeGOST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IDEN") )]
	CodeIDEN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INCU") )]
	CodeINCU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LREF") )]
	CodeLREF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PASS") )]
	CodePASS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRAD") )]
	CodePRAD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PKIC") )]
	CodePKIC,
}

impl CertificateType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CertificationType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CertificationType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<CertificateType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl CertificationType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// Channel2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Channel2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<CommunicationMethod3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl Channel2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ChargeBearerType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum ChargeBearerType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DEBT") )]
	CodeDEBT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRED") )]
	CodeCRED,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SHAR") )]
	CodeSHAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SLEV") )]
	CodeSLEV,
}

impl ChargeBearerType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Cheque19 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Cheque19 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqTp", skip_serializing_if = "Option::is_none") )]
	pub chq_tp: Option<ChequeType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqNb", skip_serializing_if = "Option::is_none") )]
	pub chq_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqFr", skip_serializing_if = "Option::is_none") )]
	pub chq_fr: Option<NameAndAddress18>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryMtd", skip_serializing_if = "Option::is_none") )]
	pub dlvry_mtd: Option<ChequeDeliveryMethod1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DlvrTo", skip_serializing_if = "Option::is_none") )]
	pub dlvr_to: Option<NameAndAddress18>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none") )]
	pub instr_prty: Option<Priority2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqMtrtyDt", skip_serializing_if = "Option::is_none") )]
	pub chq_mtrty_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none") )]
	pub frms_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MemoFld", skip_serializing_if = "Option::is_none") )]
	pub memo_fld: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RgnlClrZone", skip_serializing_if = "Option::is_none") )]
	pub rgnl_clr_zone: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtLctn", skip_serializing_if = "Option::is_none") )]
	pub prt_lctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sgntr", skip_serializing_if = "Option::is_none") )]
	pub sgntr: Option<Vec<String>>,
}

impl Cheque19 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.chq_tp { val.validate()? }
		if let Some(ref val) = self.chq_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "chq_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "chq_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.chq_fr { val.validate()? }
		if let Some(ref val) = self.dlvry_mtd { val.validate()? }
		if let Some(ref val) = self.dlvr_to { val.validate()? }
		if let Some(ref val) = self.instr_prty { val.validate()? }
		if let Some(ref val) = self.frms_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "frms_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "frms_cd exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.memo_fld {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "memo_fld is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "memo_fld exceeds the maximum length of 35".to_string()));
				}
			}
		}
		if let Some(ref val) = self.rgnl_clr_zone {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rgnl_clr_zone is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rgnl_clr_zone exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.prt_lctn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prt_lctn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prt_lctn exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.sgntr {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "sgntr is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 70 {
					return Err(ValidationError::new(1002, "sgntr exceeds the maximum length of 70".to_string()));
				}
			}
		}
		Ok(())
	}
}


// Cheque4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Cheque4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PyeeId") )]
	pub pyee_id: NameAndAddress5,
}

impl Cheque4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pyee_id.validate()?;
		Ok(())
	}
}


// ChequeDelivery1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum ChequeDelivery1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MLDB") )]
	CodeMLDB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MLCD") )]
	CodeMLCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MLFA") )]
	CodeMLFA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRDB") )]
	CodeCRDB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRCD") )]
	CodeCRCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRFA") )]
	CodeCRFA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUDB") )]
	CodePUDB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUCD") )]
	CodePUCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUFA") )]
	CodePUFA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RGDB") )]
	CodeRGDB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RGCD") )]
	CodeRGCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RGFA") )]
	CodeRGFA,
}

impl ChequeDelivery1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ChequeDeliveryMethod1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ChequeDeliveryMethod1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<ChequeDelivery1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ChequeDeliveryMethod1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ChequeType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum ChequeType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CCHQ") )]
	CodeCCHQ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CCCH") )]
	CodeCCCH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BCHQ") )]
	CodeBCHQ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DRFT") )]
	CodeDRFT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ELDR") )]
	CodeELDR,
}

impl ChequeType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CitizenshipInformation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CitizenshipInformation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ntlty") )]
	pub ntlty: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MnrInd", skip_serializing_if = "Option::is_none") )]
	pub mnr_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
	pub start_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt", skip_serializing_if = "Option::is_none") )]
	pub end_dt: Option<String>,
}

impl CitizenshipInformation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CitizenshipInformation2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CitizenshipInformation2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ntlty") )]
	pub ntlty: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MnrInd") )]
	pub mnr_ind: bool,
}

impl CitizenshipInformation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CivilStatus1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CivilStatus1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<CivilStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl CivilStatus1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// CivilStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum CivilStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DIVO") )]
	CodeDIVO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LDIV") )]
	CodeLDIV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MARR") )]
	CodeMARR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SEPA") )]
	CodeSEPA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SING") )]
	CodeSING,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UNIO") )]
	CodeUNIO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WIDO") )]
	CodeWIDO,
}

impl CivilStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ClearingSystemIdentification2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ClearingSystemIdentification2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ClearingSystemIdentification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 5 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 5".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ClearingSystemMemberIdentification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ClearingSystemMemberIdentification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none") )]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MmbId") )]
	pub mmb_id: String,
}

impl ClearingSystemMemberIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.clr_sys_id { val.validate()? }
		if self.mmb_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "mmb_id is shorter than the minimum length of 1".to_string()));
		}
		if self.mmb_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "mmb_id exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// ClearingSystemMemberIdentification4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ClearingSystemMemberIdentification4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "USCHU", skip_serializing_if = "Option::is_none") )]
	pub uschu: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NZNCC", skip_serializing_if = "Option::is_none") )]
	pub nzncc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IENSC", skip_serializing_if = "Option::is_none") )]
	pub iensc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBSC", skip_serializing_if = "Option::is_none") )]
	pub gbsc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USCH", skip_serializing_if = "Option::is_none") )]
	pub usch: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHBC", skip_serializing_if = "Option::is_none") )]
	pub chbc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USFW", skip_serializing_if = "Option::is_none") )]
	pub usfw: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PTNCC", skip_serializing_if = "Option::is_none") )]
	pub ptncc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RUCB", skip_serializing_if = "Option::is_none") )]
	pub rucb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ITNCC", skip_serializing_if = "Option::is_none") )]
	pub itncc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ATBLZ", skip_serializing_if = "Option::is_none") )]
	pub atblz: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CACPA", skip_serializing_if = "Option::is_none") )]
	pub cacpa: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHSIC", skip_serializing_if = "Option::is_none") )]
	pub chsic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DEBLZ", skip_serializing_if = "Option::is_none") )]
	pub deblz: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ESNCC", skip_serializing_if = "Option::is_none") )]
	pub esncc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ZANCC", skip_serializing_if = "Option::is_none") )]
	pub zancc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HKNCC", skip_serializing_if = "Option::is_none") )]
	pub hkncc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AUBSBx", skip_serializing_if = "Option::is_none") )]
	pub aubs_bx: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AUBSBs", skip_serializing_if = "Option::is_none") )]
	pub aubs_bs: Option<String>,
}

impl ClearingSystemMemberIdentification4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.uschu {
			let pattern = Regex::new("CH[0-9]{6,6}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "uschu does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nzncc {
			let pattern = Regex::new("NZ[0-9]{6,6}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nzncc does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.iensc {
			let pattern = Regex::new("IE[0-9]{6,6}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "iensc does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.gbsc {
			let pattern = Regex::new("SC[0-9]{6,6}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "gbsc does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.usch {
			let pattern = Regex::new("CP[0-9]{4,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "usch does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.chbc {
			let pattern = Regex::new("SW[0-9]{3,5}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "chbc does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.usfw {
			let pattern = Regex::new("FW[0-9]{9,9}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "usfw does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ptncc {
			let pattern = Regex::new("PT[0-9]{8,8}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ptncc does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.rucb {
			let pattern = Regex::new("RU[0-9]{9,9}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "rucb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.itncc {
			let pattern = Regex::new("IT[0-9]{10,10}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "itncc does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.atblz {
			let pattern = Regex::new("AT[0-9]{5,5}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "atblz does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.cacpa {
			let pattern = Regex::new("CA[0-9]{9,9}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "cacpa does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.chsic {
			let pattern = Regex::new("SW[0-9]{6,6}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "chsic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.deblz {
			let pattern = Regex::new("BL[0-9]{8,8}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "deblz does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.esncc {
			let pattern = Regex::new("ES[0-9]{8,9}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "esncc does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.zancc {
			let pattern = Regex::new("ZA[0-9]{6,6}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "zancc does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.hkncc {
			let pattern = Regex::new("HK[0-9]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "hkncc does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.aubs_bx {
			let pattern = Regex::new("AU[0-9]{6,6}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "aubs_bx does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.aubs_bs {
			let pattern = Regex::new("AU[0-9]{6,6}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "aubs_bs does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// ClosedStatusReason1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ClosedStatusReason1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
	pub cd: ClosedStatusReason2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl ClosedStatusReason1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd.validate()?;
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}


// ClosedStatusReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ClosedStatusReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<Vec<ClosedStatusReason1>>,
}

impl ClosedStatusReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.no_spcfd_rsn { val.validate()? }
		if let Some(ref vec) = self.rsn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ClosedStatusReason1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum ClosedStatusReason1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ASIN") )]
	CodeASIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLIN") )]
	CodeCLIN,
}

impl ClosedStatusReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ClosedStatusReason2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ClosedStatusReason2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<ClosedStatusReason1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl ClosedStatusReason2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// ClosurePendingStatusReason1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ClosurePendingStatusReason1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
	pub cd: ClosurePendingStatusReason2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl ClosurePendingStatusReason1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd.validate()?;
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}


// ClosurePendingStatusReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ClosurePendingStatusReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<Vec<ClosurePendingStatusReason1>>,
}

impl ClosurePendingStatusReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.no_spcfd_rsn { val.validate()? }
		if let Some(ref vec) = self.rsn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ClosurePendingStatusReason1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum ClosurePendingStatusReason1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLOS") )]
	CodeCLOS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PEND") )]
	CodePEND,
}

impl ClosurePendingStatusReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ClosurePendingStatusReason2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ClosurePendingStatusReason2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<ClosurePendingStatusReason1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl ClosurePendingStatusReason2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// CodeOrProprietary1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CodeOrProprietary1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification13>,
}

impl CodeOrProprietary1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// Collateral1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Collateral1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "COLL") )]
	CodeCOLL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NCOL") )]
	CodeNCOL,
}

impl Collateral1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CommunicationAddress3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CommunicationAddress3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Email", skip_serializing_if = "Option::is_none") )]
	pub email: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Phne", skip_serializing_if = "Option::is_none") )]
	pub phne: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mob", skip_serializing_if = "Option::is_none") )]
	pub mob: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaxNb", skip_serializing_if = "Option::is_none") )]
	pub fax_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TlxAdr", skip_serializing_if = "Option::is_none") )]
	pub tlx_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "URLAdr", skip_serializing_if = "Option::is_none") )]
	pub url_adr: Option<String>,
}

impl CommunicationAddress3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.email {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "email is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "email exceeds the maximum length of 256".to_string()));
			}
		}
		if let Some(ref val) = self.phne {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "phne does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.mob {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "mob does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.fax_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "fax_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tlx_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tlx_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tlx_adr exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.url_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "url_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "url_adr exceeds the maximum length of 256".to_string()));
			}
		}
		Ok(())
	}
}


// CommunicationAddress6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CommunicationAddress6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
	pub adr_tp: Option<AddressType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Email", skip_serializing_if = "Option::is_none") )]
	pub email: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Phne", skip_serializing_if = "Option::is_none") )]
	pub phne: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mob", skip_serializing_if = "Option::is_none") )]
	pub mob: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaxNb", skip_serializing_if = "Option::is_none") )]
	pub fax_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TlxAdr", skip_serializing_if = "Option::is_none") )]
	pub tlx_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "URLAdr", skip_serializing_if = "Option::is_none") )]
	pub url_adr: Option<String>,
}

impl CommunicationAddress6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.adr_tp { val.validate()? }
		if let Some(ref val) = self.email {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "email is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "email exceeds the maximum length of 256".to_string()));
			}
		}
		if let Some(ref val) = self.phne {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "phne does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.mob {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "mob does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.fax_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "fax_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tlx_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tlx_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tlx_adr exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.url_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "url_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "url_adr exceeds the maximum length of 256".to_string()));
			}
		}
		Ok(())
	}
}


// CommunicationFormat1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CommunicationFormat1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl CommunicationFormat1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CommunicationMethod1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum CommunicationMethod1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SWMT") )]
	CodeSWMT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SWMX") )]
	CodeSWMX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FAXI") )]
	CodeFAXI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EMAL") )]
	CodeEMAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PROP") )]
	CodePROP,
}

impl CommunicationMethod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CommunicationMethod2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CommunicationMethod2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<CommunicationMethod2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl CommunicationMethod2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CommunicationMethod2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum CommunicationMethod2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "EMAL") )]
	CodeEMAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FAXI") )]
	CodeFAXI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FILE") )]
	CodeFILE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ONLI") )]
	CodeONLI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "POST") )]
	CodePOST,
}

impl CommunicationMethod2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CommunicationMethod3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CommunicationMethod3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<CommunicationMethod1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl CommunicationMethod3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// CommunicationMethod3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum CommunicationMethod3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "EMAL") )]
	CodeEMAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FAXI") )]
	CodeFAXI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "POST") )]
	CodePOST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PHON") )]
	CodePHON,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FILE") )]
	CodeFILE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ONLI") )]
	CodeONLI,
}

impl CommunicationMethod3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CompanyLink1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CompanyLink1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<CompanyLink1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl CompanyLink1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// CompanyLink1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum CompanyLink1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AGEN") )]
	CodeAGEN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BROK") )]
	CodeBROK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PART") )]
	CodePART,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MEMB") )]
	CodeMEMB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PCOM") )]
	CodePCOM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RELA") )]
	CodeRELA,
}

impl CompanyLink1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ConductClassification1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum ConductClassification1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NSTA") )]
	CodeNSTA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RCLT") )]
	CodeRCLT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STAN") )]
	CodeSTAN,
}

impl ConductClassification1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ConfirmationType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ConfirmationType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AccountManagementType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl ConfirmationType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// ConsolidationType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ConsolidationType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<ConsolidationType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl ConsolidationType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// ConsolidationType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum ConsolidationType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "GENL") )]
	CodeGENL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PART") )]
	CodePART,
}

impl ConsolidationType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Contact13 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Contact13 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none") )]
	pub nm_prfx: Option<NamePrefix2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PhneNb", skip_serializing_if = "Option::is_none") )]
	pub phne_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MobNb", skip_serializing_if = "Option::is_none") )]
	pub mob_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaxNb", skip_serializing_if = "Option::is_none") )]
	pub fax_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "URLAdr", skip_serializing_if = "Option::is_none") )]
	pub url_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none") )]
	pub email_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmailPurp", skip_serializing_if = "Option::is_none") )]
	pub email_purp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "JobTitl", skip_serializing_if = "Option::is_none") )]
	pub job_titl: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rspnsblty", skip_serializing_if = "Option::is_none") )]
	pub rspnsblty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dept", skip_serializing_if = "Option::is_none") )]
	pub dept: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<OtherContact1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrefrdMtd", skip_serializing_if = "Option::is_none") )]
	pub prefrd_mtd: Option<PreferredContactMethod2Code>,
}

impl Contact13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm_prfx { val.validate()? }
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.phne_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "phne_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.mob_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "mob_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.fax_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "fax_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.url_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "url_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "url_adr exceeds the maximum length of 2048".to_string()));
			}
		}
		if let Some(ref val) = self.email_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "email_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "email_adr exceeds the maximum length of 256".to_string()));
			}
		}
		if let Some(ref val) = self.email_purp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "email_purp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "email_purp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.job_titl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "job_titl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "job_titl exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.rspnsblty {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rspnsblty is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rspnsblty exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.dept {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dept is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "dept exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		if let Some(ref val) = self.prefrd_mtd { val.validate()? }
		Ok(())
	}
}


// ContractDocument1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ContractDocument1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
	pub ref_attr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SgnOffDt", skip_serializing_if = "Option::is_none") )]
	pub sgn_off_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Vrsn", skip_serializing_if = "Option::is_none") )]
	pub vrsn: Option<String>,
}

impl ContractDocument1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.ref_attr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "ref_attr is shorter than the minimum length of 1".to_string()));
		}
		if self.ref_attr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "ref_attr exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.vrsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "vrsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 6 {
				return Err(ValidationError::new(1002, "vrsn exceeds the maximum length of 6".to_string()));
			}
		}
		Ok(())
	}
}


// CountryAndResidentialStatusType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CountryAndResidentialStatusType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
	pub ctry: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ResdtlSts") )]
	pub resdtl_sts: ResidentialStatus1Code,
}

impl CountryAndResidentialStatusType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry) {
			return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
		}
		self.resdtl_sts.validate()?;
		Ok(())
	}
}


// CountryAndResidentialStatusType2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CountryAndResidentialStatusType2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
	pub ctry: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ResdtlSts") )]
	pub resdtl_sts: ResidentialStatus1Code,
}

impl CountryAndResidentialStatusType2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry) {
			return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
		}
		self.resdtl_sts.validate()?;
		Ok(())
	}
}


// CreditDebit3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum CreditDebit3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRDT") )]
	CodeCRDT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DBIT") )]
	CodeDBIT,
}

impl CreditDebit3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CreditDebitCode ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum CreditDebitCode {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRDT") )]
	CodeCRDT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DBIT") )]
	CodeDBIT,
}

impl CreditDebitCode {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CreditTransferTransaction59 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditTransferTransaction59 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtId") )]
	pub pmt_id: PaymentIdentification6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation26>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRateMrkr", skip_serializing_if = "Option::is_none") )]
	pub tax_rate_mrkr: Option<TaxRateMarker1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none") )]
	pub chrg_br: Option<ChargeBearerType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqInstr", skip_serializing_if = "Option::is_none") )]
	pub chq_instr: Option<Cheque19>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy", skip_serializing_if = "Option::is_none") )]
	pub frqcy: Option<Frequency1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrfInstr", skip_serializing_if = "Option::is_none") )]
	pub trf_instr: Option<TransferInstruction1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt") )]
	pub cdtr_agt: BranchAndFinancialInstitutionIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
	pub cdtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub instr_for_cdtr_agt: Option<Vec<InstructionForCreditorAgent3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<Purpose2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RgltryRptg", skip_serializing_if = "Option::is_none") )]
	pub rgltry_rptg: Option<Vec<RegulatoryReporting3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
	pub tax: Option<TaxData1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdRmtInf", skip_serializing_if = "Option::is_none") )]
	pub rltd_rmt_inf: Option<Vec<RemittanceLocation9>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
	pub rmt_inf: Option<RemittanceInformation22>,
}

impl CreditTransferTransaction59 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pmt_id.validate()?;
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		if let Some(ref val) = self.tax_rate_mrkr { val.validate()? }
		self.amt.validate()?;
		if let Some(ref val) = self.chrg_br { val.validate()? }
		if let Some(ref val) = self.chq_instr { val.validate()? }
		if let Some(ref val) = self.frqcy { val.validate()? }
		if let Some(ref val) = self.trf_instr { val.validate()? }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.intrmy_agt1 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3 { val.validate()? }
		self.cdtr_agt.validate()?;
		if let Some(ref val) = self.cdtr { val.validate()? }
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref vec) = self.instr_for_cdtr_agt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.purp { val.validate()? }
		if let Some(ref vec) = self.rgltry_rptg { for item in vec { item.validate()? } }
		if let Some(ref val) = self.tax { val.validate()? }
		if let Some(ref vec) = self.rltd_rmt_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rmt_inf { val.validate()? }
		Ok(())
	}
}


// CreditorReferenceInformation3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorReferenceInformation3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<CreditorReferenceType3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ref", skip_serializing_if = "Option::is_none") )]
	pub ref_attr: Option<String>,
}

impl CreditorReferenceInformation3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.ref_attr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ref_attr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ref_attr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CreditorReferenceType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorReferenceType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl CreditorReferenceType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CreditorReferenceType3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorReferenceType3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
	pub cd_or_prtry: CreditorReferenceType2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl CreditorReferenceType3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd_or_prtry.validate()?;
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CustomerAccount4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CustomerAccount4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<AccountIdentification4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
	pub sts: Option<AccountStatus3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<CashAccountType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MnthlyPmtVal", skip_serializing_if = "Option::is_none") )]
	pub mnthly_pmt_val: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MnthlyRcvdVal", skip_serializing_if = "Option::is_none") )]
	pub mnthly_rcvd_val: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MnthlyTxNb", skip_serializing_if = "Option::is_none") )]
	pub mnthly_tx_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AvrgBal", skip_serializing_if = "Option::is_none") )]
	pub avrg_bal: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctPurp", skip_serializing_if = "Option::is_none") )]
	pub acct_purp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FlrNtfctnAmt", skip_serializing_if = "Option::is_none") )]
	pub flr_ntfctn_amt: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClngNtfctnAmt", skip_serializing_if = "Option::is_none") )]
	pub clng_ntfctn_amt: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StmtFrqcyAndFrmt", skip_serializing_if = "Option::is_none") )]
	pub stmt_frqcy_and_frmt: Option<Vec<StatementFrequencyAndForm1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
	pub clsg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none") )]
	pub rstrctn: Option<Vec<Restriction1>>,
}

impl CustomerAccount4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.sts { val.validate()? }
		if let Some(ref val) = self.tp { val.validate()? }
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.ccy) {
			return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.mnthly_pmt_val {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "mnthly_pmt_val is less than the minimum value of 0.000000".to_string()));
			}
		}
		if let Some(ref val) = self.mnthly_rcvd_val {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "mnthly_rcvd_val is less than the minimum value of 0.000000".to_string()));
			}
		}
		if let Some(ref val) = self.mnthly_tx_nb {
			let pattern = Regex::new("[0-9]{1,5}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "mnthly_tx_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.avrg_bal {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "avrg_bal is less than the minimum value of 0.000000".to_string()));
			}
		}
		if let Some(ref val) = self.acct_purp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_purp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "acct_purp exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.flr_ntfctn_amt {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "flr_ntfctn_amt is less than the minimum value of 0.000000".to_string()));
			}
		}
		if let Some(ref val) = self.clng_ntfctn_amt {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "clng_ntfctn_amt is less than the minimum value of 0.000000".to_string()));
			}
		}
		if let Some(ref vec) = self.stmt_frqcy_and_frmt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.rstrctn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CustomerAccount5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CustomerAccount5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: Vec<AccountIdentification4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
	pub sts: Option<AccountStatus3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<CashAccountType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MnthlyPmtVal", skip_serializing_if = "Option::is_none") )]
	pub mnthly_pmt_val: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MnthlyRcvdVal", skip_serializing_if = "Option::is_none") )]
	pub mnthly_rcvd_val: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MnthlyTxNb", skip_serializing_if = "Option::is_none") )]
	pub mnthly_tx_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AvrgBal", skip_serializing_if = "Option::is_none") )]
	pub avrg_bal: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctPurp", skip_serializing_if = "Option::is_none") )]
	pub acct_purp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FlrNtfctnAmt", skip_serializing_if = "Option::is_none") )]
	pub flr_ntfctn_amt: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClngNtfctnAmt", skip_serializing_if = "Option::is_none") )]
	pub clng_ntfctn_amt: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StmtFrqcyAndFrmt", skip_serializing_if = "Option::is_none") )]
	pub stmt_frqcy_and_frmt: Option<Vec<StatementFrequencyAndForm1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
	pub clsg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none") )]
	pub rstrctn: Option<Vec<Restriction1>>,
}

impl CustomerAccount5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.id { item.validate()? }
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.sts { val.validate()? }
		if let Some(ref val) = self.tp { val.validate()? }
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.ccy) {
			return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.mnthly_pmt_val {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "mnthly_pmt_val is less than the minimum value of 0.000000".to_string()));
			}
		}
		if let Some(ref val) = self.mnthly_rcvd_val {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "mnthly_rcvd_val is less than the minimum value of 0.000000".to_string()));
			}
		}
		if let Some(ref val) = self.mnthly_tx_nb {
			let pattern = Regex::new("[0-9]{1,5}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "mnthly_tx_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.avrg_bal {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "avrg_bal is less than the minimum value of 0.000000".to_string()));
			}
		}
		if let Some(ref val) = self.acct_purp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_purp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "acct_purp exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.flr_ntfctn_amt {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "flr_ntfctn_amt is less than the minimum value of 0.000000".to_string()));
			}
		}
		if let Some(ref val) = self.clng_ntfctn_amt {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "clng_ntfctn_amt is less than the minimum value of 0.000000".to_string()));
			}
		}
		if let Some(ref vec) = self.stmt_frqcy_and_frmt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.rstrctn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CustomerAccountModification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CustomerAccountModification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: Vec<AccountIdentification4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<NameModification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
	pub sts: Option<AccountStatusModification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<TypeModification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MnthlyPmtVal", skip_serializing_if = "Option::is_none") )]
	pub mnthly_pmt_val: Option<AmountModification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MnthlyRcvdVal", skip_serializing_if = "Option::is_none") )]
	pub mnthly_rcvd_val: Option<AmountModification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MnthlyTxNb", skip_serializing_if = "Option::is_none") )]
	pub mnthly_tx_nb: Option<NumberModification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AvrgBal", skip_serializing_if = "Option::is_none") )]
	pub avrg_bal: Option<AmountModification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctPurp", skip_serializing_if = "Option::is_none") )]
	pub acct_purp: Option<PurposeModification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FlrNtfctnAmt", skip_serializing_if = "Option::is_none") )]
	pub flr_ntfctn_amt: Option<AmountModification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClngNtfctnAmt", skip_serializing_if = "Option::is_none") )]
	pub clng_ntfctn_amt: Option<AmountModification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StmtFrqcyAndFrmt", skip_serializing_if = "Option::is_none") )]
	pub stmt_frqcy_and_frmt: Option<Vec<StatementFrequencyAndFormModification1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
	pub clsg_dt: Option<DateModification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none") )]
	pub rstrctn: Option<Vec<RestrictionModification1>>,
}

impl CustomerAccountModification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.id { item.validate()? }
		if let Some(ref val) = self.nm { val.validate()? }
		if let Some(ref val) = self.sts { val.validate()? }
		if let Some(ref val) = self.tp { val.validate()? }
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.ccy) {
			return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.mnthly_pmt_val { val.validate()? }
		if let Some(ref val) = self.mnthly_rcvd_val { val.validate()? }
		if let Some(ref val) = self.mnthly_tx_nb { val.validate()? }
		if let Some(ref val) = self.avrg_bal { val.validate()? }
		if let Some(ref val) = self.acct_purp { val.validate()? }
		if let Some(ref val) = self.flr_ntfctn_amt { val.validate()? }
		if let Some(ref val) = self.clng_ntfctn_amt { val.validate()? }
		if let Some(ref vec) = self.stmt_frqcy_and_frmt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.clsg_dt { val.validate()? }
		if let Some(ref vec) = self.rstrctn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CustomerConductClassification1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CustomerConductClassification1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<ConductClassification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl CustomerConductClassification1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// DataBaseCheck1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DataBaseCheck1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DBChck") )]
	pub db_chck: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
}

impl DataBaseCheck1 {
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


// DataModification1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum DataModification1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "INSE") )]
	CodeINSE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UPDT") )]
	CodeUPDT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DELT") )]
	CodeDELT,
}

impl DataModification1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DataModification2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum DataModification2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "INSE") )]
	CodeINSE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DELT") )]
	CodeDELT,
}

impl DataModification2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateAndAmount1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DateAndAmount1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt") )]
	pub dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveCurrencyAndAmount,
}

impl DateAndAmount1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		Ok(())
	}
}


// DateAndDateTime1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DateAndDateTime1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtTm", skip_serializing_if = "Option::is_none") )]
	pub dt_tm: Option<String>,
}

impl DateAndDateTime1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateAndPlaceOfBirth1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DateAndPlaceOfBirth1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BirthDt") )]
	pub birth_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none") )]
	pub prvc_of_birth: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CityOfBirth") )]
	pub city_of_birth: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfBirth") )]
	pub ctry_of_birth: String,
}

impl DateAndPlaceOfBirth1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.prvc_of_birth {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prvc_of_birth is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prvc_of_birth exceeds the maximum length of 35".to_string()));
			}
		}
		if self.city_of_birth.chars().count() < 1 {
			return Err(ValidationError::new(1001, "city_of_birth is shorter than the minimum length of 1".to_string()));
		}
		if self.city_of_birth.chars().count() > 35 {
			return Err(ValidationError::new(1002, "city_of_birth exceeds the maximum length of 35".to_string()));
		}
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry_of_birth) {
			return Err(ValidationError::new(1005, "ctry_of_birth does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// DateAndType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DateAndType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: DateType2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt") )]
	pub dt: String,
}

impl DateAndType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		Ok(())
	}
}


// DateModification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DateModification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModCd", skip_serializing_if = "Option::is_none") )]
	pub mod_cd: Option<Modification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt") )]
	pub dt: String,
}

impl DateModification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mod_cd { val.validate()? }
		Ok(())
	}
}


// DatePeriod2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DatePeriod2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt") )]
	pub fr_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt") )]
	pub to_dt: String,
}

impl DatePeriod2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateTimePeriod2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DateTimePeriod2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtTm") )]
	pub fr_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none") )]
	pub to_dt_tm: Option<String>,
}

impl DateTimePeriod2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DateType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl DateType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// DeMinimus1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DeMinimus1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DeMnmsAplbl", skip_serializing_if = "Option::is_none") )]
	pub de_mnms_aplbl: Option<DeMinimusApplicable1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DeMnmsNotAplbl", skip_serializing_if = "Option::is_none") )]
	pub de_mnms_not_aplbl: Option<DeMinimusNotApplicable1>,
}

impl DeMinimus1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.de_mnms_aplbl { val.validate()? }
		if let Some(ref val) = self.de_mnms_not_aplbl { val.validate()? }
		Ok(())
	}
}


// DeMinimusApplicable1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DeMinimusApplicable1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NewIssePrmssn") )]
	pub new_isse_prmssn: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pctg", skip_serializing_if = "Option::is_none") )]
	pub pctg: Option<f64>,
}

impl DeMinimusApplicable1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DeMinimusNotApplicable1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DeMinimusNotApplicable1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RstrctdPrsnRsn") )]
	pub rstrctd_prsn_rsn: String,
}

impl DeMinimusNotApplicable1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.rstrctd_prsn_rsn.chars().count() < 1 {
			return Err(ValidationError::new(1001, "rstrctd_prsn_rsn is shorter than the minimum length of 1".to_string()));
		}
		if self.rstrctd_prsn_rsn.chars().count() > 350 {
			return Err(ValidationError::new(1002, "rstrctd_prsn_rsn exceeds the maximum length of 350".to_string()));
		}
		Ok(())
	}
}


// DirectDebitInstructionDetails3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DirectDebitInstructionDetails3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtId") )]
	pub mndt_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AutomtdDrctDbtInstrInd", skip_serializing_if = "Option::is_none") )]
	pub automtd_drct_dbt_instr_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DrctDbtTrfblInd", skip_serializing_if = "Option::is_none") )]
	pub drct_dbt_trfbl_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
	pub cdtr: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LastColltnCcyAmt", skip_serializing_if = "Option::is_none") )]
	pub last_colltn_ccy_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LastColltnDt", skip_serializing_if = "Option::is_none") )]
	pub last_colltn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrDtls", skip_serializing_if = "Option::is_none") )]
	pub othr_dtls: Option<Vec<TransferInstruction1>>,
}

impl DirectDebitInstructionDetails3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.mndt_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "mndt_id is shorter than the minimum length of 1".to_string()));
		}
		if self.mndt_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "mndt_id exceeds the maximum length of 35".to_string()));
		}
		self.cdtr.validate()?;
		if let Some(ref val) = self.last_colltn_ccy_amt { val.validate()? }
		if let Some(ref vec) = self.othr_dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// DirectDebitMandate7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DirectDebitMandate7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct") )]
	pub dbtr_acct: AccountIdentificationAndName5,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
	pub dbtr: Option<PartyIdentification125Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrTaxIdNb", skip_serializing_if = "Option::is_none") )]
	pub dbtr_tax_id_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrNtlRegnNb", skip_serializing_if = "Option::is_none") )]
	pub dbtr_ntl_regn_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
	pub cdtr: Option<PartyIdentification125Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt") )]
	pub dbtr_agt: FinancialInstitutionIdentification11Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtBrnch", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt_brnch: Option<BranchData4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<FinancialInstitutionIdentification11Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtBrnch", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt_brnch: Option<BranchData4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnId", skip_serializing_if = "Option::is_none") )]
	pub regn_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtId", skip_serializing_if = "Option::is_none") )]
	pub mndt_id: Option<String>,
}

impl DirectDebitMandate7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.dbtr_acct.validate()?;
		if let Some(ref val) = self.dbtr { val.validate()? }
		if let Some(ref val) = self.dbtr_tax_id_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dbtr_tax_id_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "dbtr_tax_id_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.dbtr_ntl_regn_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dbtr_ntl_regn_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "dbtr_ntl_regn_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.cdtr { val.validate()? }
		self.dbtr_agt.validate()?;
		if let Some(ref val) = self.dbtr_agt_brnch { val.validate()? }
		if let Some(ref val) = self.cdtr_agt { val.validate()? }
		if let Some(ref val) = self.cdtr_agt_brnch { val.validate()? }
		if let Some(ref val) = self.regn_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "regn_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "regn_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.mndt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mndt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mndt_id exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// DisabledReason2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum DisabledReason2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLOS") )]
	CodeCLOS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BKRP") )]
	CodeBKRP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CMMT") )]
	CodeCMMT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CNFS") )]
	CodeCNFS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MORT") )]
	CodeMORT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PCOM") )]
	CodePCOM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PLDG") )]
	CodePLDG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRPE") )]
	CodeTRPE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SANC") )]
	CodeSANC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRAN") )]
	CodeTRAN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REJT") )]
	CodeREJT,
}

impl DisabledReason2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DisabledStatusReason1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DisabledStatusReason1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
	pub cd: DisabledStatusReason2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl DisabledStatusReason1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd.validate()?;
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}


// DisabledStatusReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DisabledStatusReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<Vec<DisabledStatusReason1>>,
}

impl DisabledStatusReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.no_spcfd_rsn { val.validate()? }
		if let Some(ref vec) = self.rsn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// DisabledStatusReason2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DisabledStatusReason2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<DisabledReason2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl DisabledStatusReason2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// DistributionPolicy1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum DistributionPolicy1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DIST") )]
	CodeDIST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACCU") )]
	CodeACCU,
}

impl DistributionPolicy1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DocumentAdjustment1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentAdjustment1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl DocumentAdjustment1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		if let Some(ref val) = self.cdt_dbt_ind { val.validate()? }
		if let Some(ref val) = self.rsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "rsn exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// DocumentAmount1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentAmount1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: DocumentAmountType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl DocumentAmount1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		self.amt.validate()?;
		Ok(())
	}
}


// DocumentAmountType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentAmountType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl DocumentAmountType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// DocumentLineIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentLineIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<DocumentLineType1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nb", skip_serializing_if = "Option::is_none") )]
	pub nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdDt", skip_serializing_if = "Option::is_none") )]
	pub rltd_dt: Option<String>,
}

impl DocumentLineIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nb exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// DocumentLineInformation2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentLineInformation2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: Vec<DocumentLineIdentification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<RemittanceAmount4>,
}

impl DocumentLineInformation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.id { item.validate()? }
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 2048".to_string()));
			}
		}
		if let Some(ref val) = self.amt { val.validate()? }
		Ok(())
	}
}


// DocumentLineType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentLineType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
	pub cd_or_prtry: DocumentLineType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl DocumentLineType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd_or_prtry.validate()?;
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// DocumentLineType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentLineType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl DocumentLineType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// DocumentToSend4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentToSend4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rcpt") )]
	pub rcpt: PartyIdentification125Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtdOfTrnsmssn") )]
	pub mtd_of_trnsmssn: CommunicationMethod3Choice,
}

impl DocumentToSend4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
		}
		if self.tp.chars().count() > 140 {
			return Err(ValidationError::new(1002, "tp exceeds the maximum length of 140".to_string()));
		}
		self.rcpt.validate()?;
		self.mtd_of_trnsmssn.validate()?;
		Ok(())
	}
}


// DocumentType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
	pub cd_or_prtry: DocumentType2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl DocumentType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd_or_prtry.validate()?;
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// DocumentType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl DocumentType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// Eligible1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Eligible1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ELIG") )]
	CodeELIG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NELI") )]
	CodeNELI,
}

impl Eligible1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// EnabledStatusReason1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct EnabledStatusReason1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
	pub cd: EnabledStatusReason2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl EnabledStatusReason1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd.validate()?;
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}


// EnabledStatusReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct EnabledStatusReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<Vec<EnabledStatusReason1>>,
}

impl EnabledStatusReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.no_spcfd_rsn { val.validate()? }
		if let Some(ref vec) = self.rsn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// EnabledStatusReason1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum EnabledStatusReason1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MODI") )]
	CodeMODI,
}

impl EnabledStatusReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// EnabledStatusReason2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct EnabledStatusReason2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<EnabledStatusReason1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl EnabledStatusReason2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// EndPoint1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct EndPoint1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfPmts", skip_serializing_if = "Option::is_none") )]
	pub nb_of_pmts: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LastPmtDt", skip_serializing_if = "Option::is_none") )]
	pub last_pmt_dt: Option<String>,
}

impl EndPoint1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nb_of_pmts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nb_of_pmts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nb_of_pmts exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// EventFrequency10Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum EventFrequency10Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
	CodeDAIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ADHO") )]
	CodeADHO,
}

impl EventFrequency10Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// EventFrequency1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum EventFrequency1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
	CodeYEAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SEMI") )]
	CodeSEMI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QUTR") )]
	CodeQUTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TOMN") )]
	CodeTOMN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
	CodeMNTH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TWMN") )]
	CodeTWMN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TOWK") )]
	CodeTOWK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
	CodeWEEK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
	CodeDAIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ADHO") )]
	CodeADHO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INDA") )]
	CodeINDA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OVNG") )]
	CodeOVNG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ONDE") )]
	CodeONDE,
}

impl EventFrequency1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// EventFrequency8Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum EventFrequency8Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ADHO") )]
	CodeADHO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
	CodeYEAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
	CodeDAIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FOMN") )]
	CodeFOMN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TOMN") )]
	CodeTOMN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TOWK") )]
	CodeTOWK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TYEA") )]
	CodeTYEA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INDA") )]
	CodeINDA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
	CodeMNTH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ONDE") )]
	CodeONDE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OVNG") )]
	CodeOVNG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QUTR") )]
	CodeQUTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SEMI") )]
	CodeSEMI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TWMN") )]
	CodeTWMN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
	CodeWEEK,
}

impl EventFrequency8Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// EventFrequency9Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum EventFrequency9Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
	CodeYEAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SEMI") )]
	CodeSEMI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QUTR") )]
	CodeQUTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TOMN") )]
	CodeTOMN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
	CodeMNTH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TWMN") )]
	CodeTWMN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TOWK") )]
	CodeTOWK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
	CodeWEEK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
	CodeDAIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ADHO") )]
	CodeADHO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INDA") )]
	CodeINDA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OVNG") )]
	CodeOVNG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ONDE") )]
	CodeONDE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NONE") )]
	CodeNONE,
}

impl EventFrequency9Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ExtendedParty14 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ExtendedParty14 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "XtndedPtyRole") )]
	pub xtnded_pty_role: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPtyDtls") )]
	pub othr_pty_dtls: InvestmentAccountOwnershipInformation16,
}

impl ExtendedParty14 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.xtnded_pty_role.chars().count() < 1 {
			return Err(ValidationError::new(1001, "xtnded_pty_role is shorter than the minimum length of 1".to_string()));
		}
		if self.xtnded_pty_role.chars().count() > 350 {
			return Err(ValidationError::new(1002, "xtnded_pty_role exceeds the maximum length of 350".to_string()));
		}
		self.othr_pty_dtls.validate()?;
		Ok(())
	}
}


// ExtendedParty15 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ExtendedParty15 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "XtndedPtyRole") )]
	pub xtnded_pty_role: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPtyDtls") )]
	pub othr_pty_dtls: InvestmentAccountOwnershipInformation17,
}

impl ExtendedParty15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.xtnded_pty_role.chars().count() < 1 {
			return Err(ValidationError::new(1001, "xtnded_pty_role is shorter than the minimum length of 1".to_string()));
		}
		if self.xtnded_pty_role.chars().count() > 350 {
			return Err(ValidationError::new(1002, "xtnded_pty_role exceeds the maximum length of 350".to_string()));
		}
		self.othr_pty_dtls.validate()?;
		Ok(())
	}
}


// Extension1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Extension1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlcAndNm") )]
	pub plc_and_nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Txt") )]
	pub txt: String,
}

impl Extension1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.plc_and_nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "plc_and_nm is shorter than the minimum length of 1".to_string()));
		}
		if self.plc_and_nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "plc_and_nm exceeds the maximum length of 350".to_string()));
		}
		if self.txt.chars().count() < 1 {
			return Err(ValidationError::new(1001, "txt is shorter than the minimum length of 1".to_string()));
		}
		if self.txt.chars().count() > 350 {
			return Err(ValidationError::new(1002, "txt exceeds the maximum length of 350".to_string()));
		}
		Ok(())
	}
}


// FATCAForm1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FATCAForm1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<FATCAFormType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl FATCAForm1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// FATCAFormType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum FATCAFormType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CER5") )]
	CodeCER5,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CER7") )]
	CodeCER7,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CER1") )]
	CodeCER1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CER2") )]
	CodeCER2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CER3") )]
	CodeCER3,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CER4") )]
	CodeCER4,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CER6") )]
	CodeCER6,
}

impl FATCAFormType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FATCASource1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FATCASource1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<FATCASourceStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl FATCASource1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// FATCASourceStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum FATCASourceStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CALC") )]
	CodeCALC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DECL") )]
	CodeDECL,
}

impl FATCASourceStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FATCAStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum FATCAStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "F101") )]
	CodeF101,
	#[cfg_attr( feature = "derive_serde", serde(rename = "F102") )]
	CodeF102,
	#[cfg_attr( feature = "derive_serde", serde(rename = "F103") )]
	CodeF103,
	#[cfg_attr( feature = "derive_serde", serde(rename = "F104") )]
	CodeF104,
	#[cfg_attr( feature = "derive_serde", serde(rename = "F105") )]
	CodeF105,
	#[cfg_attr( feature = "derive_serde", serde(rename = "F201") )]
	CodeF201,
	#[cfg_attr( feature = "derive_serde", serde(rename = "F202") )]
	CodeF202,
	#[cfg_attr( feature = "derive_serde", serde(rename = "F203") )]
	CodeF203,
	#[cfg_attr( feature = "derive_serde", serde(rename = "F204") )]
	CodeF204,
	#[cfg_attr( feature = "derive_serde", serde(rename = "F205") )]
	CodeF205,
	#[cfg_attr( feature = "derive_serde", serde(rename = "F206") )]
	CodeF206,
}

impl FATCAStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FATCAStatus2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FATCAStatus2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: FATCAStatus2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Src", skip_serializing_if = "Option::is_none") )]
	pub src: Option<FATCASource1Choice>,
}

impl FATCAStatus2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		if let Some(ref val) = self.src { val.validate()? }
		Ok(())
	}
}


// FATCAStatus2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FATCAStatus2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<FATCAStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl FATCAStatus2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// FinancialIdentificationSchemeName1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FinancialIdentificationSchemeName1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl FinancialIdentificationSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// FinancialInstitutionIdentification11Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FinancialInstitutionIdentification11Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
	pub nm_and_adr: Option<NameAndAddress5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BICFI", skip_serializing_if = "Option::is_none") )]
	pub bicfi: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none") )]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
	pub prtry_id: Option<SimpleIdentificationInformation4>,
}

impl FinancialInstitutionIdentification11Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm_and_adr { val.validate()? }
		if let Some(ref val) = self.bicfi {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "bicfi does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.clr_sys_mmb_id { val.validate()? }
		if let Some(ref val) = self.prtry_id { val.validate()? }
		Ok(())
	}
}


// FinancialInstitutionIdentification23 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FinancialInstitutionIdentification23 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BICFI", skip_serializing_if = "Option::is_none") )]
	pub bicfi: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none") )]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<PostalAddress27>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<GenericFinancialIdentification1>,
}

impl FinancialInstitutionIdentification23 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.bicfi {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "bicfi does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.clr_sys_mmb_id { val.validate()? }
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
		Ok(())
	}
}


// FinancialInstrument55 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FinancialInstrument55 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: SecurityIdentification25Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
	pub shrt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryId", skip_serializing_if = "Option::is_none") )]
	pub splmtry_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssTp", skip_serializing_if = "Option::is_none") )]
	pub clss_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesForm", skip_serializing_if = "Option::is_none") )]
	pub scties_form: Option<FormOfSecurity1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none") )]
	pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdctGrp", skip_serializing_if = "Option::is_none") )]
	pub pdct_grp: Option<String>,
}

impl FinancialInstrument55 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.shrt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "shrt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "shrt_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.splmtry_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "splmtry_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "splmtry_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.clss_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clss_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clss_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.scties_form { val.validate()? }
		if let Some(ref val) = self.dstrbtn_plcy { val.validate()? }
		if let Some(ref val) = self.pdct_grp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pdct_grp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "pdct_grp exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// FinancialInstrument87 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FinancialInstrument87 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: SecurityIdentification25Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
	pub shrt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryId", skip_serializing_if = "Option::is_none") )]
	pub splmtry_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssTp", skip_serializing_if = "Option::is_none") )]
	pub clss_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesForm", skip_serializing_if = "Option::is_none") )]
	pub scties_form: Option<FormOfSecurity1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none") )]
	pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdctGrp", skip_serializing_if = "Option::is_none") )]
	pub pdct_grp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BlckdHldgDtls", skip_serializing_if = "Option::is_none") )]
	pub blckd_hldg_dtls: Option<BlockedHoldingDetails2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pldgg", skip_serializing_if = "Option::is_none") )]
	pub pldgg: Option<Eligible1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Coll", skip_serializing_if = "Option::is_none") )]
	pub coll: Option<Collateral1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ThrdPtyRghts", skip_serializing_if = "Option::is_none") )]
	pub thrd_pty_rghts: Option<ThirdPartyRights2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FndOwnrsh", skip_serializing_if = "Option::is_none") )]
	pub fnd_ownrsh: Option<FundOwnership1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FndIntntn", skip_serializing_if = "Option::is_none") )]
	pub fnd_intntn: Option<FundIntention1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OprlSts", skip_serializing_if = "Option::is_none") )]
	pub oprl_sts: Option<OperationalStatus1Code>,
}

impl FinancialInstrument87 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.shrt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "shrt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "shrt_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.splmtry_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "splmtry_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "splmtry_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.clss_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clss_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clss_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.scties_form { val.validate()? }
		if let Some(ref val) = self.dstrbtn_plcy { val.validate()? }
		if let Some(ref val) = self.pdct_grp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pdct_grp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "pdct_grp exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.blckd_hldg_dtls { val.validate()? }
		if let Some(ref val) = self.pldgg { val.validate()? }
		if let Some(ref val) = self.coll { val.validate()? }
		if let Some(ref val) = self.thrd_pty_rghts { val.validate()? }
		if let Some(ref val) = self.fnd_ownrsh { val.validate()? }
		if let Some(ref val) = self.fnd_intntn { val.validate()? }
		if let Some(ref val) = self.oprl_sts { val.validate()? }
		Ok(())
	}
}


// FiscalYear1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FiscalYear1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
	pub start_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt", skip_serializing_if = "Option::is_none") )]
	pub end_dt: Option<String>,
}

impl FiscalYear1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FixedAmountOrUnlimited1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FixedAmountOrUnlimited1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NotLtd", skip_serializing_if = "Option::is_none") )]
	pub not_ltd: Option<String>,
}

impl FixedAmountOrUnlimited1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref val) = self.not_ltd {
			if val.chars().count() < 9 {
				return Err(ValidationError::new(1001, "not_ltd is shorter than the minimum length of 9".to_string()));
			}
			if val.chars().count() > 9 {
				return Err(ValidationError::new(1002, "not_ltd exceeds the maximum length of 9".to_string()));
			}
			let pattern = Regex::new("UNLIMITED").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "not_ltd does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// FormOfSecurity1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum FormOfSecurity1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BEAR") )]
	CodeBEAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REGD") )]
	CodeREGD,
}

impl FormOfSecurity1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Frequency1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Frequency1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Seq", skip_serializing_if = "Option::is_none") )]
	pub seq: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt") )]
	pub start_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndPtChc") )]
	pub end_pt_chc: EndPoint1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdFrqcyPttrn", skip_serializing_if = "Option::is_none") )]
	pub reqd_frqcy_pttrn: Option<Frequency37Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NonWorkgDayAdjstmnt", skip_serializing_if = "Option::is_none") )]
	pub non_workg_day_adjstmnt: Option<BusinessDayConvention1Code>,
}

impl Frequency1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.seq {
			let pattern = Regex::new("[0-9]{1,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "seq does not match the required pattern".to_string()));
			}
		}
		self.end_pt_chc.validate()?;
		if let Some(ref val) = self.reqd_frqcy_pttrn { val.validate()? }
		if let Some(ref val) = self.non_workg_day_adjstmnt { val.validate()? }
		Ok(())
	}
}


// Frequency10Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Frequency10Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEVR") )]
	CodeNEVR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
	CodeYEAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RATE") )]
	CodeRATE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIAN") )]
	CodeMIAN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QURT") )]
	CodeQURT,
}

impl Frequency10Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Frequency20Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Frequency20Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<EventFrequency8Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl Frequency20Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// Frequency37Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Frequency37Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Frequency10Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl Frequency37Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// Frequency7Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Frequency7Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
	CodeYEAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
	CodeDAIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
	CodeMNTH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QURT") )]
	CodeQURT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIAN") )]
	CodeMIAN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TEND") )]
	CodeTEND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MOVE") )]
	CodeMOVE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
	CodeWEEK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INDA") )]
	CodeINDA,
}

impl Frequency7Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FullLegalNameModification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FullLegalNameModification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModCd", skip_serializing_if = "Option::is_none") )]
	pub mod_cd: Option<Modification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FullLglNm") )]
	pub full_lgl_nm: String,
}

impl FullLegalNameModification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mod_cd { val.validate()? }
		if self.full_lgl_nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "full_lgl_nm is shorter than the minimum length of 1".to_string()));
		}
		if self.full_lgl_nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "full_lgl_nm exceeds the maximum length of 350".to_string()));
		}
		Ok(())
	}
}


// FundCashAccount4Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum FundCashAccount4Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "HEDG") )]
	CodeHEDG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CPFO") )]
	CodeCPFO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CPFS") )]
	CodeCPFS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SRSA") )]
	CodeSRSA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSDO") )]
	CodeCSDO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TOFF") )]
	CodeTOFF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ICSA") )]
	CodeICSA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSDM") )]
	CodeCSDM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSDP") )]
	CodeCSDP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PPEN") )]
	CodePPEN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CPEN") )]
	CodeCPEN,
}

impl FundCashAccount4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FundIntention1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum FundIntention1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "YQUA") )]
	CodeYQUA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NQUA") )]
	CodeNQUA,
}

impl FundIntention1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FundOwnership1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum FundOwnership1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "YALL") )]
	CodeYALL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NALL") )]
	CodeNALL,
}

impl FundOwnership1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// GDPRData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GDPRData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CnsntTp") )]
	pub cnsnt_tp: GDPRDataConsent1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CnsntInd") )]
	pub cnsnt_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CnsntDt") )]
	pub cnsnt_dt: String,
}

impl GDPRData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cnsnt_tp.validate()?;
		Ok(())
	}
}


// GDPRDataConsent1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GDPRDataConsent1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<GDPRDataConsent1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl GDPRDataConsent1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// GDPRDataConsent1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum GDPRDataConsent1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DP00") )]
	CodeDP00,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DP03") )]
	CodeDP03,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DP01") )]
	CodeDP01,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DP02") )]
	CodeDP02,
}

impl GDPRDataConsent1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Garnishment4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Garnishment4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: GarnishmentType1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Grnshee", skip_serializing_if = "Option::is_none") )]
	pub grnshee: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrnshmtAdmstr", skip_serializing_if = "Option::is_none") )]
	pub grnshmt_admstr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefNb", skip_serializing_if = "Option::is_none") )]
	pub ref_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none") )]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FmlyMdclInsrncInd", skip_serializing_if = "Option::is_none") )]
	pub fmly_mdcl_insrnc_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MplyeeTermntnInd", skip_serializing_if = "Option::is_none") )]
	pub mplyee_termntn_ind: Option<bool>,
}

impl Garnishment4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		if let Some(ref val) = self.grnshee { val.validate()? }
		if let Some(ref val) = self.grnshmt_admstr { val.validate()? }
		if let Some(ref val) = self.ref_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ref_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "ref_nb exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.rmtd_amt { val.validate()? }
		Ok(())
	}
}


// GarnishmentType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GarnishmentType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
	pub cd_or_prtry: GarnishmentType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GarnishmentType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd_or_prtry.validate()?;
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GarnishmentType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GarnishmentType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl GarnishmentType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// Gender1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Gender1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FEMA") )]
	CodeFEMA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MALE") )]
	CodeMALE,
}

impl Gender1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// GenderCode ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum GenderCode {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MALE") )]
	CodeMALE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FEMA") )]
	CodeFEMA,
}

impl GenderCode {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// GenericAccountIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericAccountIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<AccountSchemeName1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericAccountIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 34 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 34".to_string()));
		}
		if let Some(ref val) = self.schme_nm { val.validate()? }
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GenericFinancialIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericFinancialIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericFinancialIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.schme_nm { val.validate()? }
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GenericIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GenericIdentification13 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericIdentification13 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
	pub issr: String,
}

impl GenericIdentification13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 4 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 4".to_string()));
		}
		let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if self.issr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
		}
		if self.issr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// GenericIdentification30 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericIdentification30 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
	pub issr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
}

impl GenericIdentification30 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		if self.issr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
		}
		if self.issr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GenericIdentification36 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericIdentification36 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
	pub issr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
}

impl GenericIdentification36 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if self.issr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
		}
		if self.issr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GenericIdentification44 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericIdentification44 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: OtherIdentification1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsseDt", skip_serializing_if = "Option::is_none") )]
	pub isse_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt", skip_serializing_if = "Option::is_none") )]
	pub xpry_dt: Option<String>,
}

impl GenericIdentification44 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		self.tp.validate()?;
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GenericIdentification47 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericIdentification47 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
	pub issr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
}

impl GenericIdentification47 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		if self.issr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
		}
		if self.issr.chars().count() > 4 {
			return Err(ValidationError::new(1002, "issr exceeds the maximum length of 4".to_string()));
		}
		let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
		if !pattern.is_match(&self.issr) {
			return Err(ValidationError::new(1005, "issr does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 4".to_string()));
			}
			let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "schme_nm does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// GenericIdentification81 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericIdentification81 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IdTp") )]
	pub id_tp: OtherIdentification3Choice,
}

impl GenericIdentification81 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		self.id_tp.validate()?;
		Ok(())
	}
}


// GenericIdentification82 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericIdentification82 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: OtherIdentification3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsseDt", skip_serializing_if = "Option::is_none") )]
	pub isse_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt", skip_serializing_if = "Option::is_none") )]
	pub xpry_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Stat", skip_serializing_if = "Option::is_none") )]
	pub stat: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssrCtry", skip_serializing_if = "Option::is_none") )]
	pub issr_ctry: Option<String>,
}

impl GenericIdentification82 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		self.tp.validate()?;
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.stat {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "stat is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "stat exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.issr_ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "issr_ctry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// GenericOrganisationIdentification3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericOrganisationIdentification3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericOrganisationIdentification3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 256 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 256".to_string()));
		}
		if let Some(ref val) = self.schme_nm { val.validate()? }
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GenericPersonIdentification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericPersonIdentification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericPersonIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 256 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 256".to_string()));
		}
		if let Some(ref val) = self.schme_nm { val.validate()? }
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// Group5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Group5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModCd", skip_serializing_if = "Option::is_none") )]
	pub mod_cd: Option<Modification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrpId") )]
	pub grp_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty") )]
	pub pty: Vec<PartyAndCertificate7>,
}

impl Group5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mod_cd { val.validate()? }
		if self.grp_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "grp_id is shorter than the minimum length of 1".to_string()));
		}
		if self.grp_id.chars().count() > 4 {
			return Err(ValidationError::new(1002, "grp_id exceeds the maximum length of 4".to_string()));
		}
		let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
		if !pattern.is_match(&self.grp_id) {
			return Err(ValidationError::new(1005, "grp_id does not match the required pattern".to_string()));
		}
		for item in &self.pty { item.validate()? }
		Ok(())
	}
}


// Group6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Group6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrpId") )]
	pub grp_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty") )]
	pub pty: Vec<PartyAndCertificate6>,
}

impl Group6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.grp_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "grp_id is shorter than the minimum length of 1".to_string()));
		}
		if self.grp_id.chars().count() > 4 {
			return Err(ValidationError::new(1002, "grp_id exceeds the maximum length of 4".to_string()));
		}
		let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
		if !pattern.is_match(&self.grp_id) {
			return Err(ValidationError::new(1005, "grp_id does not match the required pattern".to_string()));
		}
		for item in &self.pty { item.validate()? }
		Ok(())
	}
}


// HighFrequencyTradingProfile1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct HighFrequencyTradingProfile1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmFrqcy", skip_serializing_if = "Option::is_none") )]
	pub sttlm_frqcy: Option<SettlementFrequency1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CnsldtnTp", skip_serializing_if = "Option::is_none") )]
	pub cnsldtn_tp: Option<ConsolidationType1Choice>,
}

impl HighFrequencyTradingProfile1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sttlm_frqcy { val.validate()? }
		if let Some(ref val) = self.cnsldtn_tp { val.validate()? }
		Ok(())
	}
}


// Holding1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Holding1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CERT") )]
	CodeCERT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NPRH") )]
	CodeNPRH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRTH") )]
	CodePRTH,
}

impl Holding1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// IdentificationAssignment4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct IdentificationAssignment4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cretr", skip_serializing_if = "Option::is_none") )]
	pub cretr: Option<Party50Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrstAgt", skip_serializing_if = "Option::is_none") )]
	pub frst_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Assgnr") )]
	pub assgnr: Party50Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Assgne") )]
	pub assgne: Party50Choice,
}

impl IdentificationAssignment4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.cretr { val.validate()? }
		if let Some(ref val) = self.frst_agt { val.validate()? }
		self.assgnr.validate()?;
		self.assgne.validate()?;
		Ok(())
	}
}


// IdentificationInformation5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct IdentificationInformation5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty", skip_serializing_if = "Option::is_none") )]
	pub pty: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Acct", skip_serializing_if = "Option::is_none") )]
	pub acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Agt", skip_serializing_if = "Option::is_none") )]
	pub agt: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl IdentificationInformation5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pty { val.validate()? }
		if let Some(ref val) = self.acct { val.validate()? }
		if let Some(ref val) = self.agt { val.validate()? }
		Ok(())
	}
}


// IdentificationModification5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct IdentificationModification5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlPtyAndAcctId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_pty_and_acct_id: Option<IdentificationInformation5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UpdtdPtyAndAcctId") )]
	pub updtd_pty_and_acct_id: IdentificationInformation5,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl IdentificationModification5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.orgnl_pty_and_acct_id { val.validate()? }
		self.updtd_pty_and_acct_id.validate()?;
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// IdentificationSource1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct IdentificationSource1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dmst", skip_serializing_if = "Option::is_none") )]
	pub dmst: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl IdentificationSource1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dmst {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "dmst does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// IdentificationVerification5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct IdentificationVerification5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyAndAcctId") )]
	pub pty_and_acct_id: IdentificationInformation5,
}

impl IdentificationVerification5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		self.pty_and_acct_id.validate()?;
		Ok(())
	}
}


// IncomePreference2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum IncomePreference2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CASH") )]
	CodeCASH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SECU") )]
	CodeSECU,
}

impl IncomePreference2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// IndividualPerson29 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct IndividualPerson29 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none") )]
	pub nm_prfx: Option<NamePrefix1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GvnNm", skip_serializing_if = "Option::is_none") )]
	pub gvn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MddlNm", skip_serializing_if = "Option::is_none") )]
	pub mddl_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr") )]
	pub pstl_adr: Vec<PostalAddress21>,
}

impl IndividualPerson29 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm_prfx { val.validate()? }
		if let Some(ref val) = self.gvn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "gvn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "gvn_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.mddl_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mddl_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mddl_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
		}
		for item in &self.pstl_adr { item.validate()? }
		Ok(())
	}
}


// IndividualPerson30 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct IndividualPerson30 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "GvnNm", skip_serializing_if = "Option::is_none") )]
	pub gvn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MddlNm", skip_serializing_if = "Option::is_none") )]
	pub mddl_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Gndr", skip_serializing_if = "Option::is_none") )]
	pub gndr: Option<GenderCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BirthDt", skip_serializing_if = "Option::is_none") )]
	pub birth_dt: Option<String>,
}

impl IndividualPerson30 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.gvn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "gvn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "gvn_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.mddl_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mddl_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mddl_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
		}
		if let Some(ref val) = self.gndr { val.validate()? }
		Ok(())
	}
}


// IndividualPerson35 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct IndividualPerson35 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "GvnNm", skip_serializing_if = "Option::is_none") )]
	pub gvn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MddlNm", skip_serializing_if = "Option::is_none") )]
	pub mddl_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Gndr", skip_serializing_if = "Option::is_none") )]
	pub gndr: Option<Gender1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BirthDt", skip_serializing_if = "Option::is_none") )]
	pub birth_dt: Option<String>,
}

impl IndividualPerson35 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.gvn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "gvn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "gvn_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.mddl_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mddl_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mddl_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
		}
		if let Some(ref val) = self.gndr { val.validate()? }
		Ok(())
	}
}


// IndividualPerson37 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct IndividualPerson37 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none") )]
	pub nm_prfx: Option<NamePrefix1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GvnNm", skip_serializing_if = "Option::is_none") )]
	pub gvn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MddlNm", skip_serializing_if = "Option::is_none") )]
	pub mddl_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmSfx", skip_serializing_if = "Option::is_none") )]
	pub nm_sfx: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Gndr", skip_serializing_if = "Option::is_none") )]
	pub gndr: Option<Gender1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BirthDt", skip_serializing_if = "Option::is_none") )]
	pub birth_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfBirth", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_birth: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none") )]
	pub prvc_of_birth: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CityOfBirth", skip_serializing_if = "Option::is_none") )]
	pub city_of_birth: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prfssn", skip_serializing_if = "Option::is_none") )]
	pub prfssn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr") )]
	pub pstl_adr: Vec<PostalAddress21>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctznsh", skip_serializing_if = "Option::is_none") )]
	pub ctznsh: Option<Vec<CitizenshipInformation2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmplngCpny", skip_serializing_if = "Option::is_none") )]
	pub emplng_cpny: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizFctn", skip_serializing_if = "Option::is_none") )]
	pub biz_fctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PltclyXpsdPrsn", skip_serializing_if = "Option::is_none") )]
	pub pltcly_xpsd_prsn: Option<PoliticallyExposedPerson1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DthDt", skip_serializing_if = "Option::is_none") )]
	pub dth_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CvlSts", skip_serializing_if = "Option::is_none") )]
	pub cvl_sts: Option<CivilStatus1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EdctnLvl", skip_serializing_if = "Option::is_none") )]
	pub edctn_lvl: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FmlyInf", skip_serializing_if = "Option::is_none") )]
	pub fmly_inf: Option<PersonalInformation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GDPRData", skip_serializing_if = "Option::is_none") )]
	pub gdpr_data: Option<Vec<GDPRData1>>,
}

impl IndividualPerson37 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm_prfx { val.validate()? }
		if let Some(ref val) = self.gvn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "gvn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "gvn_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.mddl_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mddl_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mddl_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
		}
		if let Some(ref val) = self.nm_sfx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm_sfx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nm_sfx exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.gndr { val.validate()? }
		if let Some(ref val) = self.ctry_of_birth {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_birth does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prvc_of_birth {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prvc_of_birth is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prvc_of_birth exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.city_of_birth {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "city_of_birth is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "city_of_birth exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.prfssn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prfssn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prfssn exceeds the maximum length of 35".to_string()));
			}
		}
		for item in &self.pstl_adr { item.validate()? }
		if let Some(ref vec) = self.ctznsh { for item in vec { item.validate()? } }
		if let Some(ref val) = self.emplng_cpny {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "emplng_cpny is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "emplng_cpny exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.biz_fctn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "biz_fctn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "biz_fctn exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.pltcly_xpsd_prsn { val.validate()? }
		if let Some(ref val) = self.cvl_sts { val.validate()? }
		if let Some(ref val) = self.edctn_lvl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "edctn_lvl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "edctn_lvl exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.fmly_inf { val.validate()? }
		if let Some(ref vec) = self.gdpr_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// IndividualPerson38 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct IndividualPerson38 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none") )]
	pub nm_prfx: Option<NamePrefix1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GvnNm", skip_serializing_if = "Option::is_none") )]
	pub gvn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MddlNm", skip_serializing_if = "Option::is_none") )]
	pub mddl_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmSfx", skip_serializing_if = "Option::is_none") )]
	pub nm_sfx: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Gndr", skip_serializing_if = "Option::is_none") )]
	pub gndr: Option<Gender1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BirthDt", skip_serializing_if = "Option::is_none") )]
	pub birth_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfBirth", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_birth: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none") )]
	pub prvc_of_birth: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CityOfBirth", skip_serializing_if = "Option::is_none") )]
	pub city_of_birth: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prfssn", skip_serializing_if = "Option::is_none") )]
	pub prfssn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfdPstlAdr", skip_serializing_if = "Option::is_none") )]
	pub modfd_pstl_adr: Option<Vec<ModificationScope34>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfdCtznsh", skip_serializing_if = "Option::is_none") )]
	pub modfd_ctznsh: Option<Vec<ModificationScope39>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmplngCpny", skip_serializing_if = "Option::is_none") )]
	pub emplng_cpny: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizFctn", skip_serializing_if = "Option::is_none") )]
	pub biz_fctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PltclyXpsdPrsn", skip_serializing_if = "Option::is_none") )]
	pub pltcly_xpsd_prsn: Option<PoliticallyExposedPerson1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DthDt", skip_serializing_if = "Option::is_none") )]
	pub dth_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CvlSts", skip_serializing_if = "Option::is_none") )]
	pub cvl_sts: Option<CivilStatus1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EdctnLvl", skip_serializing_if = "Option::is_none") )]
	pub edctn_lvl: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FmlyInf", skip_serializing_if = "Option::is_none") )]
	pub fmly_inf: Option<PersonalInformation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GDPRData", skip_serializing_if = "Option::is_none") )]
	pub gdpr_data: Option<Vec<GDPRData1>>,
}

impl IndividualPerson38 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm_prfx { val.validate()? }
		if let Some(ref val) = self.gvn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "gvn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "gvn_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.mddl_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mddl_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mddl_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
		}
		if let Some(ref val) = self.nm_sfx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm_sfx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nm_sfx exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.gndr { val.validate()? }
		if let Some(ref val) = self.ctry_of_birth {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_birth does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prvc_of_birth {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prvc_of_birth is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prvc_of_birth exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.city_of_birth {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "city_of_birth is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "city_of_birth exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.prfssn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prfssn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prfssn exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.modfd_pstl_adr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.modfd_ctznsh { for item in vec { item.validate()? } }
		if let Some(ref val) = self.emplng_cpny {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "emplng_cpny is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "emplng_cpny exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.biz_fctn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "biz_fctn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "biz_fctn exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.pltcly_xpsd_prsn { val.validate()? }
		if let Some(ref val) = self.cvl_sts { val.validate()? }
		if let Some(ref val) = self.edctn_lvl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "edctn_lvl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "edctn_lvl exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.fmly_inf { val.validate()? }
		if let Some(ref vec) = self.gdpr_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// IndividualPerson44 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct IndividualPerson44 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CurNm") )]
	pub cur_nm: IndividualPersonNameLong2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsNm", skip_serializing_if = "Option::is_none") )]
	pub prvs_nm: Option<Vec<IndividualPersonNameLong2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Gndr", skip_serializing_if = "Option::is_none") )]
	pub gndr: Option<Gender1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Lang", skip_serializing_if = "Option::is_none") )]
	pub lang: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BirthDt", skip_serializing_if = "Option::is_none") )]
	pub birth_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfBirth", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_birth: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none") )]
	pub prvc_of_birth: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CityOfBirth", skip_serializing_if = "Option::is_none") )]
	pub city_of_birth: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxtnCtry", skip_serializing_if = "Option::is_none") )]
	pub taxtn_ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryAndResdtlSts", skip_serializing_if = "Option::is_none") )]
	pub ctry_and_resdtl_sts: Option<CountryAndResidentialStatusType1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SclSctyNb", skip_serializing_if = "Option::is_none") )]
	pub scl_scty_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<Vec<PostalAddress27>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtznshInf", skip_serializing_if = "Option::is_none") )]
	pub ctznsh_inf: Option<Vec<CitizenshipInformation1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmryComAdr", skip_serializing_if = "Option::is_none") )]
	pub pmry_com_adr: Option<CommunicationAddress3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ScndryComAdr", skip_serializing_if = "Option::is_none") )]
	pub scndry_com_adr: Option<CommunicationAddress3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
	pub othr_id: Option<Vec<GenericIdentification44>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrDtls", skip_serializing_if = "Option::is_none") )]
	pub othr_dtls: Option<Vec<TransferInstruction1>>,
}

impl IndividualPerson44 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cur_nm.validate()?;
		if let Some(ref vec) = self.prvs_nm { for item in vec { item.validate()? } }
		if let Some(ref val) = self.gndr { val.validate()? }
		if let Some(ref val) = self.ctry_of_birth {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_birth does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prvc_of_birth {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prvc_of_birth is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prvc_of_birth exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.city_of_birth {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "city_of_birth is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "city_of_birth exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.taxtn_ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "taxtn_ctry does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctry_and_resdtl_sts { val.validate()? }
		if let Some(ref val) = self.scl_scty_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "scl_scty_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "scl_scty_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.pstl_adr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ctznsh_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.pmry_com_adr { val.validate()? }
		if let Some(ref val) = self.scndry_com_adr { val.validate()? }
		if let Some(ref vec) = self.othr_id { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.othr_dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// IndividualPersonIdentification2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct IndividualPersonIdentification2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IdNb", skip_serializing_if = "Option::is_none") )]
	pub id_nb: Option<GenericIdentification81>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrsnNm", skip_serializing_if = "Option::is_none") )]
	pub prsn_nm: Option<IndividualPerson30>,
}

impl IndividualPersonIdentification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id_nb { val.validate()? }
		if let Some(ref val) = self.prsn_nm { val.validate()? }
		Ok(())
	}
}


// IndividualPersonIdentification3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct IndividualPersonIdentification3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IdNb", skip_serializing_if = "Option::is_none") )]
	pub id_nb: Option<GenericIdentification81>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrsnNm", skip_serializing_if = "Option::is_none") )]
	pub prsn_nm: Option<IndividualPerson35>,
}

impl IndividualPersonIdentification3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id_nb { val.validate()? }
		if let Some(ref val) = self.prsn_nm { val.validate()? }
		Ok(())
	}
}


// IndividualPersonNameLong2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct IndividualPersonNameLong2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none") )]
	pub nm_prfx: Option<NamePrefix2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Srnm") )]
	pub srnm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GvnNm", skip_serializing_if = "Option::is_none") )]
	pub gvn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MddlNm", skip_serializing_if = "Option::is_none") )]
	pub mddl_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Initls", skip_serializing_if = "Option::is_none") )]
	pub initls: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmSfx", skip_serializing_if = "Option::is_none") )]
	pub nm_sfx: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
	pub start_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt", skip_serializing_if = "Option::is_none") )]
	pub end_dt: Option<String>,
}

impl IndividualPersonNameLong2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm_prfx { val.validate()? }
		if self.srnm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "srnm is shorter than the minimum length of 1".to_string()));
		}
		if self.srnm.chars().count() > 35 {
			return Err(ValidationError::new(1002, "srnm exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.gvn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "gvn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "gvn_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.mddl_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mddl_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mddl_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.initls {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "initls is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 6 {
				return Err(ValidationError::new(1002, "initls exceeds the maximum length of 6".to_string()));
			}
		}
		if let Some(ref val) = self.nm_sfx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm_sfx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "nm_sfx exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}


// InformationDistribution1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InformationDistribution1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InformationDistribution2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl InformationDistribution1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// InformationDistribution2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum InformationDistribution2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ELEC") )]
	CodeELEC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NONE") )]
	CodeNONE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PAPR") )]
	CodePAPR,
}

impl InformationDistribution2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InitialAmount1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InitialAmount1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlNbOfInstlmts", skip_serializing_if = "Option::is_none") )]
	pub initl_nb_of_instlmts: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveCurrencyAndAmount>,
}

impl InitialAmount1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt { val.validate()? }
		Ok(())
	}
}


// InstructionForCreditorAgent3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InstructionForCreditorAgent3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrInf", skip_serializing_if = "Option::is_none") )]
	pub instr_inf: Option<String>,
}

impl InstructionForCreditorAgent3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.instr_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "instr_inf exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// Insurance1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Insurance1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIFE") )]
	CodeLIFE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PDIS") )]
	CodePDIS,
}

impl Insurance1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InsuranceType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InsuranceType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Insurance1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl InsuranceType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// Intermediary46 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Intermediary46 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: PartyIdentification177Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none") )]
	pub lgl_ntty_idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Acct", skip_serializing_if = "Option::is_none") )]
	pub acct: Option<Account32>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WvdTrlrComssnInd", skip_serializing_if = "Option::is_none") )]
	pub wvd_trlr_comssn_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Role", skip_serializing_if = "Option::is_none") )]
	pub role: Option<PartyRole2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmryComAdr", skip_serializing_if = "Option::is_none") )]
	pub pmry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ScndryComAdr", skip_serializing_if = "Option::is_none") )]
	pub scndry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
	pub nm_and_adr: Option<NameAndAddress4>,
}

impl Intermediary46 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.lgl_ntty_idr {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lgl_ntty_idr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.acct { val.validate()? }
		if let Some(ref val) = self.role { val.validate()? }
		if let Some(ref vec) = self.pmry_com_adr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.scndry_com_adr { for item in vec { item.validate()? } }
		if let Some(ref val) = self.nm_and_adr { val.validate()? }
		Ok(())
	}
}


// Intermediary47 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Intermediary47 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: PartyIdentification125Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none") )]
	pub lgl_ntty_idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Acct", skip_serializing_if = "Option::is_none") )]
	pub acct: Option<Account32>,
}

impl Intermediary47 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.lgl_ntty_idr {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lgl_ntty_idr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.acct { val.validate()? }
		Ok(())
	}
}


// InvestmentAccount73 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InvestmentAccount73 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dsgnt", skip_serializing_if = "Option::is_none") )]
	pub dsgnt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<AccountType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OwnrshTp") )]
	pub ownrsh_tp: OwnershipType2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxXmptn", skip_serializing_if = "Option::is_none") )]
	pub tax_xmptn: Option<TaxExemptionReason2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StmtFrqcy", skip_serializing_if = "Option::is_none") )]
	pub stmt_frqcy: Option<StatementFrequencyReason2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefCcy", skip_serializing_if = "Option::is_none") )]
	pub ref_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Lang", skip_serializing_if = "Option::is_none") )]
	pub lang: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IncmPref", skip_serializing_if = "Option::is_none") )]
	pub incm_pref: Option<IncomePreference2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RinvstmtDtls", skip_serializing_if = "Option::is_none") )]
	pub rinvstmt_dtls: Option<Vec<Reinvestment4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxWhldgMtd", skip_serializing_if = "Option::is_none") )]
	pub tax_whldg_mtd: Option<TaxWithholdingMethod3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRptg", skip_serializing_if = "Option::is_none") )]
	pub tax_rptg: Option<Vec<TaxReporting3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LttrInttDtls", skip_serializing_if = "Option::is_none") )]
	pub lttr_intt_dtls: Option<LetterIntent1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcmltnRghtRef", skip_serializing_if = "Option::is_none") )]
	pub acmltn_rght_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqrdSgntriesNb", skip_serializing_if = "Option::is_none") )]
	pub reqrd_sgntries_nb: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FndFmlyNm", skip_serializing_if = "Option::is_none") )]
	pub fnd_fmly_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmDtls", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_dtls: Option<Vec<FinancialInstrument87>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RndgDtls", skip_serializing_if = "Option::is_none") )]
	pub rndg_dtls: Option<RoundingParameters1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr: Option<PartyIdentification125Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BlckdSts", skip_serializing_if = "Option::is_none") )]
	pub blckd_sts: Option<BlockedStatusReason2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctUsgTp", skip_serializing_if = "Option::is_none") )]
	pub acct_usg_tp: Option<AccountUsageType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrgnStsCertfctn", skip_serializing_if = "Option::is_none") )]
	pub frgn_sts_certfctn: Option<Provided1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSgntrDtTm", skip_serializing_if = "Option::is_none") )]
	pub acct_sgntr_dt_tm: Option<DateAndDateTime1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxChanlTp", skip_serializing_if = "Option::is_none") )]
	pub tx_chanl_tp: Option<TransactionChannelType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtAcctCtgy", skip_serializing_if = "Option::is_none") )]
	pub invstmt_acct_ctgy: Option<InvestmentAccountCategory1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pldgg", skip_serializing_if = "Option::is_none") )]
	pub pldgg: Option<Eligible1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Coll", skip_serializing_if = "Option::is_none") )]
	pub coll: Option<Collateral1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ThrdPtyRghts", skip_serializing_if = "Option::is_none") )]
	pub thrd_pty_rghts: Option<ThirdPartyRights2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PwrOfAttnyLvlOfCtrl", skip_serializing_if = "Option::is_none") )]
	pub pwr_of_attny_lvl_of_ctrl: Option<LevelOfControl1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctgSts", skip_serializing_if = "Option::is_none") )]
	pub acctg_sts: Option<AccountingStatus1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OpngDt", skip_serializing_if = "Option::is_none") )]
	pub opng_dt: Option<DateAndDateTime1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
	pub clsg_dt: Option<DateAndDateTime1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NegInd", skip_serializing_if = "Option::is_none") )]
	pub neg_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgOrdr", skip_serializing_if = "Option::is_none") )]
	pub prcg_ordr: Option<PositionEffect3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Lblty", skip_serializing_if = "Option::is_none") )]
	pub lblty: Option<Liability1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrPrfl", skip_serializing_if = "Option::is_none") )]
	pub invstr_prfl: Option<Vec<InvestorProfile2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FsclYr", skip_serializing_if = "Option::is_none") )]
	pub fscl_yr: Option<FiscalYear1Choice>,
}

impl InvestmentAccount73 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.dsgnt {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dsgnt is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "dsgnt exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tp { val.validate()? }
		self.ownrsh_tp.validate()?;
		if let Some(ref val) = self.tax_xmptn { val.validate()? }
		if let Some(ref val) = self.stmt_frqcy { val.validate()? }
		if let Some(ref val) = self.ref_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ref_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.incm_pref { val.validate()? }
		if let Some(ref vec) = self.rinvstmt_dtls { for item in vec { item.validate()? } }
		if let Some(ref val) = self.tax_whldg_mtd { val.validate()? }
		if let Some(ref vec) = self.tax_rptg { for item in vec { item.validate()? } }
		if let Some(ref val) = self.lttr_intt_dtls { val.validate()? }
		if let Some(ref val) = self.acmltn_rght_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acmltn_rght_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acmltn_rght_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.fnd_fmly_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "fnd_fmly_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "fnd_fmly_nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref vec) = self.fin_instrm_dtls { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rndg_dtls { val.validate()? }
		if let Some(ref val) = self.acct_svcr { val.validate()? }
		if let Some(ref val) = self.blckd_sts { val.validate()? }
		if let Some(ref val) = self.acct_usg_tp { val.validate()? }
		if let Some(ref val) = self.frgn_sts_certfctn { val.validate()? }
		if let Some(ref val) = self.acct_sgntr_dt_tm { val.validate()? }
		if let Some(ref val) = self.tx_chanl_tp { val.validate()? }
		if let Some(ref val) = self.invstmt_acct_ctgy { val.validate()? }
		if let Some(ref val) = self.pldgg { val.validate()? }
		if let Some(ref val) = self.coll { val.validate()? }
		if let Some(ref val) = self.thrd_pty_rghts { val.validate()? }
		if let Some(ref val) = self.pwr_of_attny_lvl_of_ctrl { val.validate()? }
		if let Some(ref val) = self.acctg_sts { val.validate()? }
		if let Some(ref val) = self.opng_dt { val.validate()? }
		if let Some(ref val) = self.clsg_dt { val.validate()? }
		if let Some(ref val) = self.prcg_ordr { val.validate()? }
		if let Some(ref val) = self.lblty { val.validate()? }
		if let Some(ref vec) = self.invstr_prfl { for item in vec { item.validate()? } }
		if let Some(ref val) = self.fscl_yr { val.validate()? }
		Ok(())
	}
}


// InvestmentAccount74 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InvestmentAccount74 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSts", skip_serializing_if = "Option::is_none") )]
	pub acct_sts: Option<AccountStatus2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BlckdSts", skip_serializing_if = "Option::is_none") )]
	pub blckd_sts: Option<BlockedStatusReason2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsDt", skip_serializing_if = "Option::is_none") )]
	pub sts_dt: Option<DateAndDateTime1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dsgnt", skip_serializing_if = "Option::is_none") )]
	pub dsgnt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<AccountType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OwnrshTp", skip_serializing_if = "Option::is_none") )]
	pub ownrsh_tp: Option<OwnershipType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxXmptn", skip_serializing_if = "Option::is_none") )]
	pub tax_xmptn: Option<TaxExemptionReason2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StmtFrqcy", skip_serializing_if = "Option::is_none") )]
	pub stmt_frqcy: Option<StatementFrequencyReason2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefCcy", skip_serializing_if = "Option::is_none") )]
	pub ref_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Lang", skip_serializing_if = "Option::is_none") )]
	pub lang: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IncmPref", skip_serializing_if = "Option::is_none") )]
	pub incm_pref: Option<IncomePreference2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RinvstmtDtls", skip_serializing_if = "Option::is_none") )]
	pub rinvstmt_dtls: Option<Vec<Reinvestment4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxWhldgMtd", skip_serializing_if = "Option::is_none") )]
	pub tax_whldg_mtd: Option<TaxWithholdingMethod3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRptg", skip_serializing_if = "Option::is_none") )]
	pub tax_rptg: Option<Vec<TaxReporting3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LttrInttDtls", skip_serializing_if = "Option::is_none") )]
	pub lttr_intt_dtls: Option<LetterIntent1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcmltnRghtRef", skip_serializing_if = "Option::is_none") )]
	pub acmltn_rght_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqrdSgntriesNb", skip_serializing_if = "Option::is_none") )]
	pub reqrd_sgntries_nb: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FndFmlyNm", skip_serializing_if = "Option::is_none") )]
	pub fnd_fmly_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmDtls", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_dtls: Option<Vec<FinancialInstrument87>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RndgDtls", skip_serializing_if = "Option::is_none") )]
	pub rndg_dtls: Option<RoundingParameters1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr: Option<PartyIdentification125Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctUsgTp", skip_serializing_if = "Option::is_none") )]
	pub acct_usg_tp: Option<AccountUsageType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrgnStsCertfctn", skip_serializing_if = "Option::is_none") )]
	pub frgn_sts_certfctn: Option<Provided1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSgntrDtTm", skip_serializing_if = "Option::is_none") )]
	pub acct_sgntr_dt_tm: Option<DateAndDateTime1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxChanlTp", skip_serializing_if = "Option::is_none") )]
	pub tx_chanl_tp: Option<TransactionChannelType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtAcctCtgy", skip_serializing_if = "Option::is_none") )]
	pub invstmt_acct_ctgy: Option<InvestmentAccountCategory1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pldgg", skip_serializing_if = "Option::is_none") )]
	pub pldgg: Option<Eligible1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Coll", skip_serializing_if = "Option::is_none") )]
	pub coll: Option<Collateral1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ThrdPtyRghts", skip_serializing_if = "Option::is_none") )]
	pub thrd_pty_rghts: Option<ThirdPartyRights2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PwrOfAttnyLvlOfCtrl", skip_serializing_if = "Option::is_none") )]
	pub pwr_of_attny_lvl_of_ctrl: Option<LevelOfControl1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctgSts", skip_serializing_if = "Option::is_none") )]
	pub acctg_sts: Option<AccountingStatus1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OpngDt", skip_serializing_if = "Option::is_none") )]
	pub opng_dt: Option<DateAndDateTime1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
	pub clsg_dt: Option<DateAndDateTime1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NegInd", skip_serializing_if = "Option::is_none") )]
	pub neg_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgOrdr", skip_serializing_if = "Option::is_none") )]
	pub prcg_ordr: Option<PositionEffect3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Lblty", skip_serializing_if = "Option::is_none") )]
	pub lblty: Option<Liability1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrPrfl", skip_serializing_if = "Option::is_none") )]
	pub invstr_prfl: Option<Vec<InvestorProfile2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FsclYr", skip_serializing_if = "Option::is_none") )]
	pub fscl_yr: Option<FiscalYear1Choice>,
}

impl InvestmentAccount74 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.acct_sts { val.validate()? }
		if let Some(ref val) = self.blckd_sts { val.validate()? }
		if let Some(ref val) = self.sts_dt { val.validate()? }
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.dsgnt {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dsgnt is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "dsgnt exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.ownrsh_tp { val.validate()? }
		if let Some(ref val) = self.tax_xmptn { val.validate()? }
		if let Some(ref val) = self.stmt_frqcy { val.validate()? }
		if let Some(ref val) = self.ref_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ref_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.incm_pref { val.validate()? }
		if let Some(ref vec) = self.rinvstmt_dtls { for item in vec { item.validate()? } }
		if let Some(ref val) = self.tax_whldg_mtd { val.validate()? }
		if let Some(ref vec) = self.tax_rptg { for item in vec { item.validate()? } }
		if let Some(ref val) = self.lttr_intt_dtls { val.validate()? }
		if let Some(ref val) = self.acmltn_rght_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acmltn_rght_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acmltn_rght_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.fnd_fmly_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "fnd_fmly_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "fnd_fmly_nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref vec) = self.fin_instrm_dtls { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rndg_dtls { val.validate()? }
		if let Some(ref val) = self.acct_svcr { val.validate()? }
		if let Some(ref val) = self.acct_usg_tp { val.validate()? }
		if let Some(ref val) = self.frgn_sts_certfctn { val.validate()? }
		if let Some(ref val) = self.acct_sgntr_dt_tm { val.validate()? }
		if let Some(ref val) = self.tx_chanl_tp { val.validate()? }
		if let Some(ref val) = self.invstmt_acct_ctgy { val.validate()? }
		if let Some(ref val) = self.pldgg { val.validate()? }
		if let Some(ref val) = self.coll { val.validate()? }
		if let Some(ref val) = self.thrd_pty_rghts { val.validate()? }
		if let Some(ref val) = self.pwr_of_attny_lvl_of_ctrl { val.validate()? }
		if let Some(ref val) = self.acctg_sts { val.validate()? }
		if let Some(ref val) = self.opng_dt { val.validate()? }
		if let Some(ref val) = self.clsg_dt { val.validate()? }
		if let Some(ref val) = self.prcg_ordr { val.validate()? }
		if let Some(ref val) = self.lblty { val.validate()? }
		if let Some(ref vec) = self.invstr_prfl { for item in vec { item.validate()? } }
		if let Some(ref val) = self.fscl_yr { val.validate()? }
		Ok(())
	}
}


// InvestmentAccount75 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InvestmentAccount75 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctStsUpdInstr", skip_serializing_if = "Option::is_none") )]
	pub acct_sts_upd_instr: Option<AccountStatusUpdateInstruction1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dsgnt", skip_serializing_if = "Option::is_none") )]
	pub dsgnt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<AccountType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OwnrshTp", skip_serializing_if = "Option::is_none") )]
	pub ownrsh_tp: Option<OwnershipType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxXmptn", skip_serializing_if = "Option::is_none") )]
	pub tax_xmptn: Option<TaxExemptionReason2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StmtFrqcy", skip_serializing_if = "Option::is_none") )]
	pub stmt_frqcy: Option<StatementFrequencyReason2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefCcy", skip_serializing_if = "Option::is_none") )]
	pub ref_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Lang", skip_serializing_if = "Option::is_none") )]
	pub lang: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IncmPref", skip_serializing_if = "Option::is_none") )]
	pub incm_pref: Option<IncomePreference2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RinvstmtDtls", skip_serializing_if = "Option::is_none") )]
	pub rinvstmt_dtls: Option<Vec<Reinvestment4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxWhldgMtd", skip_serializing_if = "Option::is_none") )]
	pub tax_whldg_mtd: Option<TaxWithholdingMethod3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRptg", skip_serializing_if = "Option::is_none") )]
	pub tax_rptg: Option<Vec<TaxReporting3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LttrInttDtls", skip_serializing_if = "Option::is_none") )]
	pub lttr_intt_dtls: Option<LetterIntent1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcmltnRghtRef", skip_serializing_if = "Option::is_none") )]
	pub acmltn_rght_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqrdSgntriesNb", skip_serializing_if = "Option::is_none") )]
	pub reqrd_sgntries_nb: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FndFmlyNm", skip_serializing_if = "Option::is_none") )]
	pub fnd_fmly_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfdFinInstrmDtls", skip_serializing_if = "Option::is_none") )]
	pub modfd_fin_instrm_dtls: Option<Vec<ModificationScope42>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RndgDtls", skip_serializing_if = "Option::is_none") )]
	pub rndg_dtls: Option<RoundingParameters1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr: Option<PartyIdentification125Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BlckdSts", skip_serializing_if = "Option::is_none") )]
	pub blckd_sts: Option<BlockedStatusReason2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctUsgTp", skip_serializing_if = "Option::is_none") )]
	pub acct_usg_tp: Option<AccountUsageType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrgnStsCertfctn", skip_serializing_if = "Option::is_none") )]
	pub frgn_sts_certfctn: Option<Provided1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSgntrDtTm", skip_serializing_if = "Option::is_none") )]
	pub acct_sgntr_dt_tm: Option<DateAndDateTime1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxChanlTp", skip_serializing_if = "Option::is_none") )]
	pub tx_chanl_tp: Option<TransactionChannelType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtAcctCtgy", skip_serializing_if = "Option::is_none") )]
	pub invstmt_acct_ctgy: Option<InvestmentAccountCategory1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pldgg", skip_serializing_if = "Option::is_none") )]
	pub pldgg: Option<Eligible1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Coll", skip_serializing_if = "Option::is_none") )]
	pub coll: Option<Collateral1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ThrdPtyRghts", skip_serializing_if = "Option::is_none") )]
	pub thrd_pty_rghts: Option<ThirdPartyRights2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PwrOfAttnyLvlOfCtrl", skip_serializing_if = "Option::is_none") )]
	pub pwr_of_attny_lvl_of_ctrl: Option<LevelOfControl1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctgSts", skip_serializing_if = "Option::is_none") )]
	pub acctg_sts: Option<AccountingStatus1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OpngDt", skip_serializing_if = "Option::is_none") )]
	pub opng_dt: Option<DateAndDateTime1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
	pub clsg_dt: Option<DateAndDateTime1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NegInd", skip_serializing_if = "Option::is_none") )]
	pub neg_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgOrdr", skip_serializing_if = "Option::is_none") )]
	pub prcg_ordr: Option<PositionEffect3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Lblty", skip_serializing_if = "Option::is_none") )]
	pub lblty: Option<Liability1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfdInvstrPrfl", skip_serializing_if = "Option::is_none") )]
	pub modfd_invstr_prfl: Option<Vec<ModificationScope46>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FsclYr", skip_serializing_if = "Option::is_none") )]
	pub fscl_yr: Option<FiscalYear1Choice>,
}

impl InvestmentAccount75 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.acct_sts_upd_instr { val.validate()? }
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.dsgnt {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dsgnt is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "dsgnt exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.ownrsh_tp { val.validate()? }
		if let Some(ref val) = self.tax_xmptn { val.validate()? }
		if let Some(ref val) = self.stmt_frqcy { val.validate()? }
		if let Some(ref val) = self.ref_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ref_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.incm_pref { val.validate()? }
		if let Some(ref vec) = self.rinvstmt_dtls { for item in vec { item.validate()? } }
		if let Some(ref val) = self.tax_whldg_mtd { val.validate()? }
		if let Some(ref vec) = self.tax_rptg { for item in vec { item.validate()? } }
		if let Some(ref val) = self.lttr_intt_dtls { val.validate()? }
		if let Some(ref val) = self.acmltn_rght_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acmltn_rght_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acmltn_rght_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.fnd_fmly_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "fnd_fmly_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "fnd_fmly_nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref vec) = self.modfd_fin_instrm_dtls { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rndg_dtls { val.validate()? }
		if let Some(ref val) = self.acct_svcr { val.validate()? }
		if let Some(ref val) = self.blckd_sts { val.validate()? }
		if let Some(ref val) = self.acct_usg_tp { val.validate()? }
		if let Some(ref val) = self.frgn_sts_certfctn { val.validate()? }
		if let Some(ref val) = self.acct_sgntr_dt_tm { val.validate()? }
		if let Some(ref val) = self.tx_chanl_tp { val.validate()? }
		if let Some(ref val) = self.invstmt_acct_ctgy { val.validate()? }
		if let Some(ref val) = self.pldgg { val.validate()? }
		if let Some(ref val) = self.coll { val.validate()? }
		if let Some(ref val) = self.thrd_pty_rghts { val.validate()? }
		if let Some(ref val) = self.pwr_of_attny_lvl_of_ctrl { val.validate()? }
		if let Some(ref val) = self.acctg_sts { val.validate()? }
		if let Some(ref val) = self.opng_dt { val.validate()? }
		if let Some(ref val) = self.clsg_dt { val.validate()? }
		if let Some(ref val) = self.prcg_ordr { val.validate()? }
		if let Some(ref val) = self.lblty { val.validate()? }
		if let Some(ref vec) = self.modfd_invstr_prfl { for item in vec { item.validate()? } }
		if let Some(ref val) = self.fscl_yr { val.validate()? }
		Ok(())
	}
}


// InvestmentAccount76 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InvestmentAccount76 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dsgnt", skip_serializing_if = "Option::is_none") )]
	pub dsgnt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FndTp", skip_serializing_if = "Option::is_none") )]
	pub fnd_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FndFmlyNm", skip_serializing_if = "Option::is_none") )]
	pub fnd_fmly_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctyDtls", skip_serializing_if = "Option::is_none") )]
	pub scty_dtls: Option<FinancialInstrument55>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none") )]
	pub acct_ownr: Option<AccountOwner3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Intrmy", skip_serializing_if = "Option::is_none") )]
	pub intrmy: Option<Vec<Intermediary47>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr: Option<PartyIdentification125Choice>,
}

impl InvestmentAccount76 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.dsgnt {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dsgnt is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "dsgnt exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.fnd_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "fnd_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "fnd_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.fnd_fmly_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "fnd_fmly_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "fnd_fmly_nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.scty_dtls { val.validate()? }
		if let Some(ref val) = self.acct_ownr { val.validate()? }
		if let Some(ref vec) = self.intrmy { for item in vec { item.validate()? } }
		if let Some(ref val) = self.acct_svcr { val.validate()? }
		Ok(())
	}
}


// InvestmentAccount77 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InvestmentAccount77 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctId") )]
	pub acct_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctNm", skip_serializing_if = "Option::is_none") )]
	pub acct_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctDsgnt", skip_serializing_if = "Option::is_none") )]
	pub acct_dsgnt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OwnrId", skip_serializing_if = "Option::is_none") )]
	pub ownr_id: Option<OwnerIdentification3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr: Option<PartyIdentification125Choice>,
}

impl InvestmentAccount77 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.acct_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "acct_id is shorter than the minimum length of 1".to_string()));
		}
		if self.acct_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "acct_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.acct_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.acct_dsgnt {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_dsgnt is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_dsgnt exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ownr_id { val.validate()? }
		if let Some(ref val) = self.acct_svcr { val.validate()? }
		Ok(())
	}
}


// InvestmentAccountCategory1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InvestmentAccountCategory1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InvestmentAccountCategory1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl InvestmentAccountCategory1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// InvestmentAccountCategory1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum InvestmentAccountCategory1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MAND") )]
	CodeMAND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RETA") )]
	CodeRETA,
}

impl InvestmentAccountCategory1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestmentAccountModification4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InvestmentAccountModification4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModRsn", skip_serializing_if = "Option::is_none") )]
	pub mod_rsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctApplId", skip_serializing_if = "Option::is_none") )]
	pub acct_appl_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClntRef", skip_serializing_if = "Option::is_none") )]
	pub clnt_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyRef", skip_serializing_if = "Option::is_none") )]
	pub ctr_pty_ref: Option<AdditionalReference13>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExstgAcctId", skip_serializing_if = "Option::is_none") )]
	pub exstg_acct_id: Option<Vec<Account23>>,
}

impl InvestmentAccountModification4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mod_rsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mod_rsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "mod_rsn exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.acct_appl_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_appl_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_appl_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.clnt_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clnt_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clnt_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctr_pty_ref { val.validate()? }
		if let Some(ref vec) = self.exstg_acct_id { for item in vec { item.validate()? } }
		Ok(())
	}
}


// InvestmentAccountOpening4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InvestmentAccountOpening4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OpngTp") )]
	pub opng_tp: AccountOpeningType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctApplId", skip_serializing_if = "Option::is_none") )]
	pub acct_appl_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClntRef", skip_serializing_if = "Option::is_none") )]
	pub clnt_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyRef", skip_serializing_if = "Option::is_none") )]
	pub ctr_pty_ref: Option<AdditionalReference13>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExstgAcctId", skip_serializing_if = "Option::is_none") )]
	pub exstg_acct_id: Option<Vec<Account23>>,
}

impl InvestmentAccountOpening4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.opng_tp.validate()?;
		if let Some(ref val) = self.acct_appl_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_appl_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_appl_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.clnt_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clnt_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clnt_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctr_pty_ref { val.validate()? }
		if let Some(ref vec) = self.exstg_acct_id { for item in vec { item.validate()? } }
		Ok(())
	}
}


// InvestmentAccountOwnershipInformation16 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InvestmentAccountOwnershipInformation16 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty") )]
	pub pty: Party47Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MnyLndrgChck", skip_serializing_if = "Option::is_none") )]
	pub mny_lndrg_chck: Option<MoneyLaunderingCheck1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrPrflVldtn", skip_serializing_if = "Option::is_none") )]
	pub invstr_prfl_vldtn: Option<Vec<PartyProfileInformation5>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OwnrshBnfcryRate", skip_serializing_if = "Option::is_none") )]
	pub ownrsh_bnfcry_rate: Option<OwnershipBeneficiaryRate1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClntId", skip_serializing_if = "Option::is_none") )]
	pub clnt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FsclXmptn", skip_serializing_if = "Option::is_none") )]
	pub fscl_xmptn: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SgntryRghtInd", skip_serializing_if = "Option::is_none") )]
	pub sgntry_rght_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MiFIDClssfctn", skip_serializing_if = "Option::is_none") )]
	pub mi_fid_clssfctn: Option<MiFIDClassification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ntfctn", skip_serializing_if = "Option::is_none") )]
	pub ntfctn: Option<Vec<Notification2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FATCAFormTp", skip_serializing_if = "Option::is_none") )]
	pub fatca_form_tp: Option<Vec<FATCAForm1Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FATCASts", skip_serializing_if = "Option::is_none") )]
	pub fatca_sts: Option<Vec<FATCAStatus2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FATCARptgDt", skip_serializing_if = "Option::is_none") )]
	pub fatca_rptg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRSFormTp", skip_serializing_if = "Option::is_none") )]
	pub crs_form_tp: Option<Vec<CRSForm1Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRSSts", skip_serializing_if = "Option::is_none") )]
	pub crs_sts: Option<Vec<CRSStatus4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRSRptgDt", skip_serializing_if = "Option::is_none") )]
	pub crs_rptg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
	pub othr_id: Option<Vec<GenericIdentification82>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxXmptn", skip_serializing_if = "Option::is_none") )]
	pub tax_xmptn: Option<TaxExemptionReason2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRptg", skip_serializing_if = "Option::is_none") )]
	pub tax_rptg: Option<Vec<TaxReporting3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Lang", skip_serializing_if = "Option::is_none") )]
	pub lang: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MailTp", skip_serializing_if = "Option::is_none") )]
	pub mail_tp: Option<MailType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryAndResdtlSts", skip_serializing_if = "Option::is_none") )]
	pub ctry_and_resdtl_sts: Option<CountryAndResidentialStatusType2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MntryWlth", skip_serializing_if = "Option::is_none") )]
	pub mntry_wlth: Option<DateAndAmount1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EqtyVal", skip_serializing_if = "Option::is_none") )]
	pub eqty_val: Option<DateAndAmount1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WorkgCptl", skip_serializing_if = "Option::is_none") )]
	pub workg_cptl: Option<DateAndAmount1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CpnyLk", skip_serializing_if = "Option::is_none") )]
	pub cpny_lk: Option<CompanyLink1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ElctrncMlngSvcRef", skip_serializing_if = "Option::is_none") )]
	pub elctrnc_mlng_svc_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmryComAdr", skip_serializing_if = "Option::is_none") )]
	pub pmry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ScndryComAdr", skip_serializing_if = "Option::is_none") )]
	pub scndry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRgltryInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_rgltry_inf: Option<RegulatoryInformation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctgSts", skip_serializing_if = "Option::is_none") )]
	pub acctg_sts: Option<AccountingStatus1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<AdditiononalInformation13>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrlgPty", skip_serializing_if = "Option::is_none") )]
	pub ctrlg_pty: Option<bool>,
}

impl InvestmentAccountOwnershipInformation16 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty.validate()?;
		if let Some(ref val) = self.mny_lndrg_chck { val.validate()? }
		if let Some(ref vec) = self.invstr_prfl_vldtn { for item in vec { item.validate()? } }
		if let Some(ref val) = self.ownrsh_bnfcry_rate { val.validate()? }
		if let Some(ref val) = self.clnt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clnt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clnt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.mi_fid_clssfctn { val.validate()? }
		if let Some(ref vec) = self.ntfctn { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.fatca_form_tp { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.fatca_sts { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.crs_form_tp { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.crs_sts { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.othr_id { for item in vec { item.validate()? } }
		if let Some(ref val) = self.tax_xmptn { val.validate()? }
		if let Some(ref vec) = self.tax_rptg { for item in vec { item.validate()? } }
		if let Some(ref val) = self.mail_tp { val.validate()? }
		if let Some(ref val) = self.ctry_and_resdtl_sts { val.validate()? }
		if let Some(ref val) = self.mntry_wlth { val.validate()? }
		if let Some(ref val) = self.eqty_val { val.validate()? }
		if let Some(ref val) = self.workg_cptl { val.validate()? }
		if let Some(ref val) = self.cpny_lk { val.validate()? }
		if let Some(ref val) = self.elctrnc_mlng_svc_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "elctrnc_mlng_svc_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "elctrnc_mlng_svc_ref exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref vec) = self.pmry_com_adr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.scndry_com_adr { for item in vec { item.validate()? } }
		if let Some(ref val) = self.addtl_rgltry_inf { val.validate()? }
		if let Some(ref val) = self.acctg_sts { val.validate()? }
		if let Some(ref vec) = self.addtl_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// InvestmentAccountOwnershipInformation17 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InvestmentAccountOwnershipInformation17 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty") )]
	pub pty: Party48Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MnyLndrgChck", skip_serializing_if = "Option::is_none") )]
	pub mny_lndrg_chck: Option<MoneyLaunderingCheck1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfdInvstrPrflVldtn", skip_serializing_if = "Option::is_none") )]
	pub modfd_invstr_prfl_vldtn: Option<Vec<ModificationScope27>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OwnrshBnfcryRate", skip_serializing_if = "Option::is_none") )]
	pub ownrsh_bnfcry_rate: Option<OwnershipBeneficiaryRate1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClntId", skip_serializing_if = "Option::is_none") )]
	pub clnt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FsclXmptn", skip_serializing_if = "Option::is_none") )]
	pub fscl_xmptn: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SgntryRghtInd", skip_serializing_if = "Option::is_none") )]
	pub sgntry_rght_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MiFIDClssfctn", skip_serializing_if = "Option::is_none") )]
	pub mi_fid_clssfctn: Option<MiFIDClassification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ntfctn", skip_serializing_if = "Option::is_none") )]
	pub ntfctn: Option<Vec<Notification2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FATCAFormTp", skip_serializing_if = "Option::is_none") )]
	pub fatca_form_tp: Option<Vec<FATCAForm1Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FATCASts", skip_serializing_if = "Option::is_none") )]
	pub fatca_sts: Option<Vec<FATCAStatus2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FATCARptgDt", skip_serializing_if = "Option::is_none") )]
	pub fatca_rptg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRSFormTp", skip_serializing_if = "Option::is_none") )]
	pub crs_form_tp: Option<Vec<CRSForm1Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRSSts", skip_serializing_if = "Option::is_none") )]
	pub crs_sts: Option<Vec<CRSStatus4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRSRptgDt", skip_serializing_if = "Option::is_none") )]
	pub crs_rptg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
	pub othr_id: Option<Vec<GenericIdentification82>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxXmptn", skip_serializing_if = "Option::is_none") )]
	pub tax_xmptn: Option<TaxExemptionReason2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRptg", skip_serializing_if = "Option::is_none") )]
	pub tax_rptg: Option<Vec<TaxReporting3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Lang", skip_serializing_if = "Option::is_none") )]
	pub lang: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MailTp", skip_serializing_if = "Option::is_none") )]
	pub mail_tp: Option<MailType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryAndResdtlSts", skip_serializing_if = "Option::is_none") )]
	pub ctry_and_resdtl_sts: Option<CountryAndResidentialStatusType2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MntryWlth", skip_serializing_if = "Option::is_none") )]
	pub mntry_wlth: Option<DateAndAmount1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EqtyVal", skip_serializing_if = "Option::is_none") )]
	pub eqty_val: Option<DateAndAmount1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WorkgCptl", skip_serializing_if = "Option::is_none") )]
	pub workg_cptl: Option<DateAndAmount1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CpnyLk", skip_serializing_if = "Option::is_none") )]
	pub cpny_lk: Option<CompanyLink1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ElctrncMlngSvcRef", skip_serializing_if = "Option::is_none") )]
	pub elctrnc_mlng_svc_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmryComAdr", skip_serializing_if = "Option::is_none") )]
	pub pmry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ScndryComAdr", skip_serializing_if = "Option::is_none") )]
	pub scndry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRgltryInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_rgltry_inf: Option<RegulatoryInformation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctgSts", skip_serializing_if = "Option::is_none") )]
	pub acctg_sts: Option<AccountingStatus1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<AdditiononalInformation13>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrlgPty", skip_serializing_if = "Option::is_none") )]
	pub ctrlg_pty: Option<bool>,
}

impl InvestmentAccountOwnershipInformation17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty.validate()?;
		if let Some(ref val) = self.mny_lndrg_chck { val.validate()? }
		if let Some(ref vec) = self.modfd_invstr_prfl_vldtn { for item in vec { item.validate()? } }
		if let Some(ref val) = self.ownrsh_bnfcry_rate { val.validate()? }
		if let Some(ref val) = self.clnt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clnt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clnt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.mi_fid_clssfctn { val.validate()? }
		if let Some(ref vec) = self.ntfctn { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.fatca_form_tp { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.fatca_sts { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.crs_form_tp { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.crs_sts { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.othr_id { for item in vec { item.validate()? } }
		if let Some(ref val) = self.tax_xmptn { val.validate()? }
		if let Some(ref vec) = self.tax_rptg { for item in vec { item.validate()? } }
		if let Some(ref val) = self.mail_tp { val.validate()? }
		if let Some(ref val) = self.ctry_and_resdtl_sts { val.validate()? }
		if let Some(ref val) = self.mntry_wlth { val.validate()? }
		if let Some(ref val) = self.eqty_val { val.validate()? }
		if let Some(ref val) = self.workg_cptl { val.validate()? }
		if let Some(ref val) = self.cpny_lk { val.validate()? }
		if let Some(ref val) = self.elctrnc_mlng_svc_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "elctrnc_mlng_svc_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "elctrnc_mlng_svc_ref exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref vec) = self.pmry_com_adr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.scndry_com_adr { for item in vec { item.validate()? } }
		if let Some(ref val) = self.addtl_rgltry_inf { val.validate()? }
		if let Some(ref val) = self.acctg_sts { val.validate()? }
		if let Some(ref vec) = self.addtl_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// InvestmentFundOrder4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InvestmentFundOrder4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrRef", skip_serializing_if = "Option::is_none") )]
	pub ordr_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MstrRef", skip_serializing_if = "Option::is_none") )]
	pub mstr_ref: Option<String>,
}

impl InvestmentFundOrder4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ordr_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ordr_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ordr_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.mstr_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mstr_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mstr_ref exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// InvestmentFundRole6Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum InvestmentFundRole6Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CACO") )]
	CodeCACO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CONC") )]
	CodeCONC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CUST") )]
	CodeCUST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DATP") )]
	CodeDATP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DIST") )]
	CodeDIST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FACT") )]
	CodeFACT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FIAD") )]
	CodeFIAD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FIAG") )]
	CodeFIAG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FMCO") )]
	CodeFMCO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FNBR") )]
	CodeFNBR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FTAG") )]
	CodeFTAG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INTR") )]
	CodeINTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INVE") )]
	CodeINVE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INVS") )]
	CodeINVS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PAYI") )]
	CodePAYI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REGI") )]
	CodeREGI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRAG") )]
	CodeTRAG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRAN") )]
	CodeTRAN,
}

impl InvestmentFundRole6Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestmentFundRole7Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum InvestmentFundRole7Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CONC") )]
	CodeCONC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DIST") )]
	CodeDIST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FMCO") )]
	CodeFMCO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INTR") )]
	CodeINTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PAYI") )]
	CodePAYI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRAG") )]
	CodeTRAG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CUST") )]
	CodeCUST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CACO") )]
	CodeCACO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FACT") )]
	CodeFACT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INVE") )]
	CodeINVE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INVS") )]
	CodeINVS,
}

impl InvestmentFundRole7Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestmentFundTransactionType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum InvestmentFundTransactionType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ALLL") )]
	CodeALLL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SELL") )]
	CodeSELL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BUYI") )]
	CodeBUYI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SWIO") )]
	CodeSWIO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRIN") )]
	CodeTRIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TOUT") )]
	CodeTOUT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SUBS") )]
	CodeSUBS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REDM") )]
	CodeREDM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CDEP") )]
	CodeCDEP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CWIT") )]
	CodeCWIT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DIVP") )]
	CodeDIVP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CAEV") )]
	CodeCAEV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CROI") )]
	CodeCROI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CROO") )]
	CodeCROO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DIVI") )]
	CodeDIVI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INSP") )]
	CodeINSP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REAA") )]
	CodeREAA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RWPL") )]
	CodeRWPL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RDIV") )]
	CodeRDIV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SSPL") )]
	CodeSSPL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SUAA") )]
	CodeSUAA,
}

impl InvestmentFundTransactionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestmentPlan16 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InvestmentPlan16 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy") )]
	pub frqcy: Frequency20Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
	pub start_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt", skip_serializing_if = "Option::is_none") )]
	pub end_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Qty") )]
	pub qty: UnitsOrAmount1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrssAmtInd", skip_serializing_if = "Option::is_none") )]
	pub grss_amt_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IncmPref", skip_serializing_if = "Option::is_none") )]
	pub incm_pref: Option<IncomePreference2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlAmt", skip_serializing_if = "Option::is_none") )]
	pub initl_amt: Option<InitialAmount1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfInstlmts", skip_serializing_if = "Option::is_none") )]
	pub ttl_nb_of_instlmts: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RndgDrctn", skip_serializing_if = "Option::is_none") )]
	pub rndg_drctn: Option<RoundingDirection1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctyDtls") )]
	pub scty_dtls: Vec<Repartition6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfdCshSttlm", skip_serializing_if = "Option::is_none") )]
	pub modfd_csh_sttlm: Option<Vec<CashSettlement4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctRef", skip_serializing_if = "Option::is_none") )]
	pub ctrct_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdCtrctRef", skip_serializing_if = "Option::is_none") )]
	pub rltd_ctrct_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdctId", skip_serializing_if = "Option::is_none") )]
	pub pdct_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SLAChrgAndComssnRef", skip_serializing_if = "Option::is_none") )]
	pub sla_chrg_and_comssn_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InsrncCover", skip_serializing_if = "Option::is_none") )]
	pub insrnc_cover: Option<InsuranceType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlanSts", skip_serializing_if = "Option::is_none") )]
	pub plan_sts: Option<PlanStatus2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstlmtMgrRole", skip_serializing_if = "Option::is_none") )]
	pub instlmt_mgr_role: Option<PartyRole4Choice>,
}

impl InvestmentPlan16 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.frqcy.validate()?;
		self.qty.validate()?;
		if let Some(ref val) = self.incm_pref { val.validate()? }
		if let Some(ref val) = self.initl_amt { val.validate()? }
		if let Some(ref val) = self.rndg_drctn { val.validate()? }
		for item in &self.scty_dtls { item.validate()? }
		if let Some(ref vec) = self.modfd_csh_sttlm { for item in vec { item.validate()? } }
		if let Some(ref val) = self.ctrct_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctrct_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctrct_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.rltd_ctrct_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rltd_ctrct_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rltd_ctrct_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.pdct_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pdct_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "pdct_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.sla_chrg_and_comssn_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sla_chrg_and_comssn_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sla_chrg_and_comssn_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.insrnc_cover { val.validate()? }
		if let Some(ref val) = self.plan_sts { val.validate()? }
		if let Some(ref val) = self.instlmt_mgr_role { val.validate()? }
		Ok(())
	}
}


// InvestmentPlan17 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InvestmentPlan17 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy") )]
	pub frqcy: Frequency20Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
	pub start_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt", skip_serializing_if = "Option::is_none") )]
	pub end_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Qty") )]
	pub qty: UnitsOrAmount1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrssAmtInd", skip_serializing_if = "Option::is_none") )]
	pub grss_amt_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IncmPref", skip_serializing_if = "Option::is_none") )]
	pub incm_pref: Option<IncomePreference2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlAmt", skip_serializing_if = "Option::is_none") )]
	pub initl_amt: Option<InitialAmount1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfInstlmts", skip_serializing_if = "Option::is_none") )]
	pub ttl_nb_of_instlmts: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RndgDrctn", skip_serializing_if = "Option::is_none") )]
	pub rndg_drctn: Option<RoundingDirection1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctyDtls") )]
	pub scty_dtls: Vec<Repartition6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshSttlm", skip_serializing_if = "Option::is_none") )]
	pub csh_sttlm: Option<Vec<CashSettlement3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctRef", skip_serializing_if = "Option::is_none") )]
	pub ctrct_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdCtrctRef", skip_serializing_if = "Option::is_none") )]
	pub rltd_ctrct_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdctId", skip_serializing_if = "Option::is_none") )]
	pub pdct_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SLAChrgAndComssnRef", skip_serializing_if = "Option::is_none") )]
	pub sla_chrg_and_comssn_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InsrncCover", skip_serializing_if = "Option::is_none") )]
	pub insrnc_cover: Option<InsuranceType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlanSts", skip_serializing_if = "Option::is_none") )]
	pub plan_sts: Option<PlanStatus2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstlmtMgrRole", skip_serializing_if = "Option::is_none") )]
	pub instlmt_mgr_role: Option<PartyRole4Choice>,
}

impl InvestmentPlan17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.frqcy.validate()?;
		self.qty.validate()?;
		if let Some(ref val) = self.incm_pref { val.validate()? }
		if let Some(ref val) = self.initl_amt { val.validate()? }
		if let Some(ref val) = self.rndg_drctn { val.validate()? }
		for item in &self.scty_dtls { item.validate()? }
		if let Some(ref vec) = self.csh_sttlm { for item in vec { item.validate()? } }
		if let Some(ref val) = self.ctrct_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctrct_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctrct_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.rltd_ctrct_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rltd_ctrct_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rltd_ctrct_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.pdct_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pdct_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "pdct_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.sla_chrg_and_comssn_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sla_chrg_and_comssn_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sla_chrg_and_comssn_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.insrnc_cover { val.validate()? }
		if let Some(ref val) = self.plan_sts { val.validate()? }
		if let Some(ref val) = self.instlmt_mgr_role { val.validate()? }
		Ok(())
	}
}


// InvestorProfile2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InvestorProfile2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<ProfileType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
	pub sts: Option<InvestorProfileStatus1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Trsr", skip_serializing_if = "Option::is_none") )]
	pub trsr: Option<TreasuryProfile1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HghFrqcyTradg", skip_serializing_if = "Option::is_none") )]
	pub hgh_frqcy_tradg: Option<HighFrequencyTradingProfile1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktMakr", skip_serializing_if = "Option::is_none") )]
	pub mkt_makr: Option<MarketMakerProfile2>,
}

impl InvestorProfile2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.sts { val.validate()? }
		if let Some(ref val) = self.trsr { val.validate()? }
		if let Some(ref val) = self.hgh_frqcy_tradg { val.validate()? }
		if let Some(ref val) = self.mkt_makr { val.validate()? }
		Ok(())
	}
}


// InvestorProfileStatus1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InvestorProfileStatus1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InvestorProfileStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl InvestorProfileStatus1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// InvestorProfileStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum InvestorProfileStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DISA") )]
	CodeDISA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DISG") )]
	CodeDISG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ENAB") )]
	CodeENAB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ENBG") )]
	CodeENBG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ADMI") )]
	CodeADMI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ANLY") )]
	CodeANLY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NAPP") )]
	CodeNAPP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PSUS") )]
	CodePSUS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PEND") )]
	CodePEND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SUPS") )]
	CodeSUPS,
}

impl InvestorProfileStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// KYCCheckType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct KYCCheckType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<KnowYourCustomerCheckType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl KYCCheckType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// KnowYourCustomerCheckType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum KnowYourCustomerCheckType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ENHA") )]
	CodeENHA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ORDN") )]
	CodeORDN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SIMP") )]
	CodeSIMP,
}

impl KnowYourCustomerCheckType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// LetterIntent1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct LetterIntent1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "LttrInttRef") )]
	pub lttr_intt_ref: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
	pub start_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt", skip_serializing_if = "Option::is_none") )]
	pub end_dt: Option<String>,
}

impl LetterIntent1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.lttr_intt_ref.chars().count() < 1 {
			return Err(ValidationError::new(1001, "lttr_intt_ref is shorter than the minimum length of 1".to_string()));
		}
		if self.lttr_intt_ref.chars().count() > 35 {
			return Err(ValidationError::new(1002, "lttr_intt_ref exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.amt { val.validate()? }
		Ok(())
	}
}


// LevelOfControl1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct LevelOfControl1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<LevelOfControl1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl LevelOfControl1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// LevelOfControl1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum LevelOfControl1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRAN") )]
	CodeTRAN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VIEW") )]
	CodeVIEW,
}

impl LevelOfControl1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Liability1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Liability1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Liability1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl Liability1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// Liability1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Liability1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "INVE") )]
	CodeINVE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BROK") )]
	CodeBROK,
}

impl Liability1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// LinkedMessage5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct LinkedMessage5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none") )]
	pub prvs_ref: Option<AdditionalReference13>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrRef", skip_serializing_if = "Option::is_none") )]
	pub othr_ref: Option<AdditionalReference13>,
}

impl LinkedMessage5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.prvs_ref { val.validate()? }
		if let Some(ref val) = self.othr_ref { val.validate()? }
		Ok(())
	}
}


// LocalInstrument2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct LocalInstrument2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl LocalInstrument2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// MailType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MailType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<MailType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl MailType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// MailType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum MailType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AIRM") )]
	CodeAIRM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ORDM") )]
	CodeORDM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REGM") )]
	CodeREGM,
}

impl MailType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// MarketMakerProfile2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MarketMakerProfile2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctPrd", skip_serializing_if = "Option::is_none") )]
	pub ctrct_prd: Option<DateTimePeriod2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cmplc", skip_serializing_if = "Option::is_none") )]
	pub cmplc: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MaxSprd", skip_serializing_if = "Option::is_none") )]
	pub max_sprd: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dscnt", skip_serializing_if = "Option::is_none") )]
	pub dscnt: Option<f64>,
}

impl MarketMakerProfile2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ctrct_prd { val.validate()? }
		Ok(())
	}
}


// MarketPracticeVersion1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MarketPracticeVersion1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nb", skip_serializing_if = "Option::is_none") )]
	pub nb: Option<String>,
}

impl MarketPracticeVersion1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 35 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nb exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// MaximumAmountByPeriod1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MaximumAmountByPeriod1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MaxAmt") )]
	pub max_amt: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfDays") )]
	pub nb_of_days: String,
}

impl MaximumAmountByPeriod1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.max_amt.validate()?;
		let pattern = Regex::new("[0-9]{1,3}").unwrap();
		if !pattern.is_match(&self.nb_of_days) {
			return Err(ValidationError::new(1005, "nb_of_days does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// MessageIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// MessageIdentification8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MessageIdentification8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId", skip_serializing_if = "Option::is_none") )]
	pub msg_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrstAgt", skip_serializing_if = "Option::is_none") )]
	pub frst_agt: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl MessageIdentification8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.msg_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.frst_agt { val.validate()? }
		Ok(())
	}
}


// MiFIDClassification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MiFIDClassification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Clssfctn") )]
	pub clssfctn: OrderOriginatorEligibility1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nrrtv", skip_serializing_if = "Option::is_none") )]
	pub nrrtv: Option<String>,
}

impl MiFIDClassification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.clssfctn.validate()?;
		if let Some(ref val) = self.nrrtv {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nrrtv is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "nrrtv exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}


// Modification1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Modification1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOCH") )]
	CodeNOCH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MODI") )]
	CodeMODI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DELE") )]
	CodeDELE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ADDD") )]
	CodeADDD,
}

impl Modification1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ModificationScope21 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ModificationScope21 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModScpIndctn") )]
	pub mod_scp_indctn: DataModification1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsseAllcn") )]
	pub isse_allcn: NewIssueAllocation2,
}

impl ModificationScope21 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.mod_scp_indctn.validate()?;
		self.isse_allcn.validate()?;
		Ok(())
	}
}


// ModificationScope27 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ModificationScope27 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModScpIndctn") )]
	pub mod_scp_indctn: DataModification2Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrPrflVldtn") )]
	pub invstr_prfl_vldtn: PartyProfileInformation5,
}

impl ModificationScope27 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.mod_scp_indctn.validate()?;
		self.invstr_prfl_vldtn.validate()?;
		Ok(())
	}
}


// ModificationScope34 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ModificationScope34 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModScpIndctn") )]
	pub mod_scp_indctn: DataModification1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr") )]
	pub pstl_adr: PostalAddress21,
}

impl ModificationScope34 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.mod_scp_indctn.validate()?;
		self.pstl_adr.validate()?;
		Ok(())
	}
}


// ModificationScope39 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ModificationScope39 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModScpIndctn") )]
	pub mod_scp_indctn: DataModification2Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctznsh") )]
	pub ctznsh: CitizenshipInformation2,
}

impl ModificationScope39 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.mod_scp_indctn.validate()?;
		self.ctznsh.validate()?;
		Ok(())
	}
}


// ModificationScope40 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ModificationScope40 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModScpIndctn") )]
	pub mod_scp_indctn: DataModification1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Intrmy") )]
	pub intrmy: Intermediary46,
}

impl ModificationScope40 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.mod_scp_indctn.validate()?;
		self.intrmy.validate()?;
		Ok(())
	}
}


// ModificationScope41 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ModificationScope41 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModScpIndctn") )]
	pub mod_scp_indctn: DataModification1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtPlan") )]
	pub invstmt_plan: InvestmentPlan16,
}

impl ModificationScope41 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.mod_scp_indctn.validate()?;
		self.invstmt_plan.validate()?;
		Ok(())
	}
}


// ModificationScope42 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ModificationScope42 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModScpIndctn") )]
	pub mod_scp_indctn: DataModification2Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmDtls") )]
	pub fin_instrm_dtls: FinancialInstrument87,
}

impl ModificationScope42 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.mod_scp_indctn.validate()?;
		self.fin_instrm_dtls.validate()?;
		Ok(())
	}
}


// ModificationScope43 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ModificationScope43 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModScpIndctn") )]
	pub mod_scp_indctn: DataModification1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Plcmnt") )]
	pub plcmnt: ReferredAgent3,
}

impl ModificationScope43 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.mod_scp_indctn.validate()?;
		self.plcmnt.validate()?;
		Ok(())
	}
}


// ModificationScope44 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ModificationScope44 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModScpIndctn") )]
	pub mod_scp_indctn: DataModification1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvcLvlAgrmt") )]
	pub svc_lvl_agrmt: DocumentToSend4,
}

impl ModificationScope44 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.mod_scp_indctn.validate()?;
		self.svc_lvl_agrmt.validate()?;
		Ok(())
	}
}


// ModificationScope45 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ModificationScope45 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModScpIndctn") )]
	pub mod_scp_indctn: DataModification1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf") )]
	pub addtl_inf: Vec<AdditiononalInformation13>,
}

impl ModificationScope45 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.mod_scp_indctn.validate()?;
		for item in &self.addtl_inf { item.validate()? }
		Ok(())
	}
}


// ModificationScope46 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ModificationScope46 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModScpIndctn") )]
	pub mod_scp_indctn: DataModification1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrPrfl") )]
	pub invstr_prfl: InvestorProfile2,
}

impl ModificationScope46 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.mod_scp_indctn.validate()?;
		self.invstr_prfl.validate()?;
		Ok(())
	}
}


// MoneyLaunderingCheck1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MoneyLaunderingCheck1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<MoneyLaunderingCheck1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl MoneyLaunderingCheck1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// MoneyLaunderingCheck1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum MoneyLaunderingCheck1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PASS") )]
	CodePASS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOTC") )]
	CodeNOTC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EXEM") )]
	CodeEXEM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLMO") )]
	CodeCLMO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AUTH") )]
	CodeAUTH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "POEP") )]
	CodePOEP,
}

impl MoneyLaunderingCheck1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// NameAndAddress15 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct NameAndAddress15 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<PostalAddress21>,
}

impl NameAndAddress15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		Ok(())
	}
}


// NameAndAddress18 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct NameAndAddress18 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adr") )]
	pub adr: PostalAddress27,
}

impl NameAndAddress18 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 140 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
		}
		self.adr.validate()?;
		Ok(())
	}
}


// NameAndAddress4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct NameAndAddress4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adr") )]
	pub adr: PostalAddress1,
}

impl NameAndAddress4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
			}
		}
		self.adr.validate()?;
		Ok(())
	}
}


// NameAndAddress5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct NameAndAddress5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adr", skip_serializing_if = "Option::is_none") )]
	pub adr: Option<PostalAddress1>,
}

impl NameAndAddress5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
		}
		if let Some(ref val) = self.adr { val.validate()? }
		Ok(())
	}
}


// NameModification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct NameModification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModCd", skip_serializing_if = "Option::is_none") )]
	pub mod_cd: Option<Modification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
}

impl NameModification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mod_cd { val.validate()? }
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 70 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 70".to_string()));
		}
		Ok(())
	}
}


// NamePrefix1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct NamePrefix1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<NamePrefix1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl NamePrefix1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// NamePrefix1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum NamePrefix1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DOCT") )]
	CodeDOCT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIST") )]
	CodeMIST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MISS") )]
	CodeMISS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MADM") )]
	CodeMADM,
}

impl NamePrefix1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// NamePrefix2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum NamePrefix2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DOCT") )]
	CodeDOCT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MADM") )]
	CodeMADM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MISS") )]
	CodeMISS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIST") )]
	CodeMIST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIKS") )]
	CodeMIKS,
}

impl NamePrefix2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// NewAccount4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct NewAccount4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Acct") )]
	pub acct: CashAccount43,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctPty") )]
	pub acct_pty: Vec<IndividualPerson44>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Org", skip_serializing_if = "Option::is_none") )]
	pub org: Option<Organisation43>,
}

impl NewAccount4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.acct.validate()?;
		for item in &self.acct_pty { item.validate()? }
		if let Some(ref val) = self.org { val.validate()? }
		Ok(())
	}
}


// NewIssueAllocation2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct NewIssueAllocation2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rstrctd") )]
	pub rstrctd: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XmptPrsnRsn", skip_serializing_if = "Option::is_none") )]
	pub xmpt_prsn_rsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DeMnms", skip_serializing_if = "Option::is_none") )]
	pub de_mnms: Option<DeMinimus1Choice>,
}

impl NewIssueAllocation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.xmpt_prsn_rsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "xmpt_prsn_rsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "xmpt_prsn_rsn exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.de_mnms { val.validate()? }
		Ok(())
	}
}


// NoReasonCode ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum NoReasonCode {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NORE") )]
	CodeNORE,
}

impl NoReasonCode {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Notification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Notification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtfctnTp") )]
	pub ntfctn_tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Reqrd") )]
	pub reqrd: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DstrbtnTp", skip_serializing_if = "Option::is_none") )]
	pub dstrbtn_tp: Option<InformationDistribution1Choice>,
}

impl Notification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.ntfctn_tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "ntfctn_tp is shorter than the minimum length of 1".to_string()));
		}
		if self.ntfctn_tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "ntfctn_tp exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.dstrbtn_tp { val.validate()? }
		Ok(())
	}
}


// NumberModification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct NumberModification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModCd", skip_serializing_if = "Option::is_none") )]
	pub mod_cd: Option<Modification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nb") )]
	pub nb: String,
}

impl NumberModification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mod_cd { val.validate()? }
		let pattern = Regex::new("[0-9]{1,5}").unwrap();
		if !pattern.is_match(&self.nb) {
			return Err(ValidationError::new(1005, "nb does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// OperationMandate6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OperationMandate6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModCd", skip_serializing_if = "Option::is_none") )]
	pub mod_cd: Option<Modification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AplblChanl") )]
	pub aplbl_chanl: Vec<Channel2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqrdSgntrNb") )]
	pub reqrd_sgntr_nb: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SgntrOrdrInd") )]
	pub sgntr_ordr_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtHldr", skip_serializing_if = "Option::is_none") )]
	pub mndt_hldr: Option<Vec<PartyAndAuthorisation6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BkOpr") )]
	pub bk_opr: Vec<BankTransactionCodeStructure4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
	pub start_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt", skip_serializing_if = "Option::is_none") )]
	pub end_dt: Option<String>,
}

impl OperationMandate6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mod_cd { val.validate()? }
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		for item in &self.aplbl_chanl { item.validate()? }
		let pattern = Regex::new("[\\+]{0,1}[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.reqrd_sgntr_nb) {
			return Err(ValidationError::new(1005, "reqrd_sgntr_nb does not match the required pattern".to_string()));
		}
		if let Some(ref vec) = self.mndt_hldr { for item in vec { item.validate()? } }
		for item in &self.bk_opr { item.validate()? }
		Ok(())
	}
}


// OperationMandate7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OperationMandate7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AplblChanl") )]
	pub aplbl_chanl: Vec<Channel2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqrdSgntrNb") )]
	pub reqrd_sgntr_nb: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SgntrOrdrInd") )]
	pub sgntr_ordr_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtHldr", skip_serializing_if = "Option::is_none") )]
	pub mndt_hldr: Option<Vec<PartyAndAuthorisation7>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BkOpr") )]
	pub bk_opr: Vec<BankTransactionCodeStructure4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
	pub start_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt", skip_serializing_if = "Option::is_none") )]
	pub end_dt: Option<String>,
}

impl OperationMandate7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		for item in &self.aplbl_chanl { item.validate()? }
		let pattern = Regex::new("[\\+]{0,1}[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.reqrd_sgntr_nb) {
			return Err(ValidationError::new(1005, "reqrd_sgntr_nb does not match the required pattern".to_string()));
		}
		if let Some(ref vec) = self.mndt_hldr { for item in vec { item.validate()? } }
		for item in &self.bk_opr { item.validate()? }
		Ok(())
	}
}


// OperationalStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum OperationalStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ENAB") )]
	CodeENAB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPEC") )]
	CodeSPEC,
}

impl OperationalStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OrderOriginatorEligibility1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum OrderOriginatorEligibility1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ELIG") )]
	CodeELIG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RETL") )]
	CodeRETL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PROF") )]
	CodePROF,
}

impl OrderOriginatorEligibility1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Organisation23 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Organisation23 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
	pub shrt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr") )]
	pub pstl_adr: Vec<PostalAddress21>,
}

impl Organisation23 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
		}
		if let Some(ref val) = self.shrt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "shrt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "shrt_nm exceeds the maximum length of 35".to_string()));
			}
		}
		for item in &self.pstl_adr { item.validate()? }
		Ok(())
	}
}


// Organisation39 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Organisation39 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
	pub shrt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<PartyIdentification177Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none") )]
	pub lgl_ntty_idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnCtry", skip_serializing_if = "Option::is_none") )]
	pub regn_ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnDt", skip_serializing_if = "Option::is_none") )]
	pub regn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<Vec<PostalAddress21>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TpOfOrg", skip_serializing_if = "Option::is_none") )]
	pub tp_of_org: Option<OrganisationType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlcOfListg", skip_serializing_if = "Option::is_none") )]
	pub plc_of_listg: Option<Vec<String>>,
}

impl Organisation39 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.shrt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "shrt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "shrt_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.lgl_ntty_idr {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lgl_ntty_idr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.purp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "purp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "purp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.regn_ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "regn_ctry does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.pstl_adr { for item in vec { item.validate()? } }
		if let Some(ref val) = self.tp_of_org { val.validate()? }
		if let Some(ref vec) = self.plc_of_listg {
			for item in vec {
				let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "plc_of_listg does not match the required pattern".to_string()));
				}
			}
		}
		Ok(())
	}
}


// Organisation40 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Organisation40 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
	pub shrt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<PartyIdentification177Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none") )]
	pub lgl_ntty_idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnCtry", skip_serializing_if = "Option::is_none") )]
	pub regn_ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnDt", skip_serializing_if = "Option::is_none") )]
	pub regn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfdPstlAdr", skip_serializing_if = "Option::is_none") )]
	pub modfd_pstl_adr: Option<Vec<ModificationScope34>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TpOfOrg", skip_serializing_if = "Option::is_none") )]
	pub tp_of_org: Option<OrganisationType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlcOfListg", skip_serializing_if = "Option::is_none") )]
	pub plc_of_listg: Option<Vec<String>>,
}

impl Organisation40 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.shrt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "shrt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "shrt_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.lgl_ntty_idr {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lgl_ntty_idr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.purp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "purp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "purp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.regn_ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "regn_ctry does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.modfd_pstl_adr { for item in vec { item.validate()? } }
		if let Some(ref val) = self.tp_of_org { val.validate()? }
		if let Some(ref vec) = self.plc_of_listg {
			for item in vec {
				let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "plc_of_listg does not match the required pattern".to_string()));
				}
			}
		}
		Ok(())
	}
}


// Organisation42 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Organisation42 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FullLglNm") )]
	pub full_lgl_nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgNm", skip_serializing_if = "Option::is_none") )]
	pub tradg_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfOpr") )]
	pub ctry_of_opr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnDt", skip_serializing_if = "Option::is_none") )]
	pub regn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OprlAdr", skip_serializing_if = "Option::is_none") )]
	pub oprl_adr: Option<PostalAddress27>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizAdr", skip_serializing_if = "Option::is_none") )]
	pub biz_adr: Option<PostalAddress27>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglAdr") )]
	pub lgl_adr: PostalAddress27,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BllgAdr", skip_serializing_if = "Option::is_none") )]
	pub bllg_adr: Option<PostalAddress27>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgId") )]
	pub org_id: OrganisationIdentification39,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RprtvOffcr", skip_serializing_if = "Option::is_none") )]
	pub rprtv_offcr: Option<Vec<PartyIdentification274>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrsrMgr", skip_serializing_if = "Option::is_none") )]
	pub trsr_mgr: Option<PartyIdentification274>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MainMndtHldr", skip_serializing_if = "Option::is_none") )]
	pub main_mndt_hldr: Option<Vec<PartyIdentification274>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sndr", skip_serializing_if = "Option::is_none") )]
	pub sndr: Option<Vec<PartyIdentification274>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglRprtv", skip_serializing_if = "Option::is_none") )]
	pub lgl_rprtv: Option<Vec<PartyIdentification274>>,
}

impl Organisation42 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.full_lgl_nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "full_lgl_nm is shorter than the minimum length of 1".to_string()));
		}
		if self.full_lgl_nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "full_lgl_nm exceeds the maximum length of 350".to_string()));
		}
		if let Some(ref val) = self.tradg_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tradg_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "tradg_nm exceeds the maximum length of 350".to_string()));
			}
		}
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry_of_opr) {
			return Err(ValidationError::new(1005, "ctry_of_opr does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.oprl_adr { val.validate()? }
		if let Some(ref val) = self.biz_adr { val.validate()? }
		self.lgl_adr.validate()?;
		if let Some(ref val) = self.bllg_adr { val.validate()? }
		self.org_id.validate()?;
		if let Some(ref vec) = self.rprtv_offcr { for item in vec { item.validate()? } }
		if let Some(ref val) = self.trsr_mgr { val.validate()? }
		if let Some(ref vec) = self.main_mndt_hldr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.sndr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.lgl_rprtv { for item in vec { item.validate()? } }
		Ok(())
	}
}


// Organisation43 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Organisation43 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FullLglNm") )]
	pub full_lgl_nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgNm", skip_serializing_if = "Option::is_none") )]
	pub tradg_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgLglSts", skip_serializing_if = "Option::is_none") )]
	pub org_lgl_sts: Option<OrganisationLegalStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EstblishdDt", skip_serializing_if = "Option::is_none") )]
	pub estblishd_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnNb", skip_serializing_if = "Option::is_none") )]
	pub regn_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnCtry", skip_serializing_if = "Option::is_none") )]
	pub regn_ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnDt", skip_serializing_if = "Option::is_none") )]
	pub regn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxtnIdNb", skip_serializing_if = "Option::is_none") )]
	pub taxtn_id_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxtnCtry", skip_serializing_if = "Option::is_none") )]
	pub taxtn_ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfOpr", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_opr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BrdRsltnInd", skip_serializing_if = "Option::is_none") )]
	pub brd_rsltn_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizAdr", skip_serializing_if = "Option::is_none") )]
	pub biz_adr: Option<PostalAddress27>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OprlAdr", skip_serializing_if = "Option::is_none") )]
	pub oprl_adr: Option<PostalAddress27>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglAdr", skip_serializing_if = "Option::is_none") )]
	pub lgl_adr: Option<PostalAddress27>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RprtvOffcr", skip_serializing_if = "Option::is_none") )]
	pub rprtv_offcr: Option<Vec<PartyIdentification272>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrsrMgr", skip_serializing_if = "Option::is_none") )]
	pub trsr_mgr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MainMndtHldr", skip_serializing_if = "Option::is_none") )]
	pub main_mndt_hldr: Option<Vec<PartyIdentification272>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sndr", skip_serializing_if = "Option::is_none") )]
	pub sndr: Option<Vec<PartyIdentification272>>,
}

impl Organisation43 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.full_lgl_nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "full_lgl_nm is shorter than the minimum length of 1".to_string()));
		}
		if self.full_lgl_nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "full_lgl_nm exceeds the maximum length of 350".to_string()));
		}
		if let Some(ref val) = self.tradg_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tradg_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "tradg_nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.org_lgl_sts { val.validate()? }
		if let Some(ref val) = self.regn_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "regn_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "regn_nb exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.regn_ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "regn_ctry does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.taxtn_id_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "taxtn_id_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "taxtn_id_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.taxtn_ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "taxtn_ctry does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctry_of_opr {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_opr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.biz_adr { val.validate()? }
		if let Some(ref val) = self.oprl_adr { val.validate()? }
		if let Some(ref val) = self.lgl_adr { val.validate()? }
		if let Some(ref vec) = self.rprtv_offcr { for item in vec { item.validate()? } }
		if let Some(ref val) = self.trsr_mgr { val.validate()? }
		if let Some(ref vec) = self.main_mndt_hldr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.sndr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// Organisation44 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Organisation44 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FullLglNm", skip_serializing_if = "Option::is_none") )]
	pub full_lgl_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgId") )]
	pub org_id: OrganisationIdentification39,
}

impl Organisation44 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.full_lgl_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "full_lgl_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "full_lgl_nm exceeds the maximum length of 350".to_string()));
			}
		}
		self.org_id.validate()?;
		Ok(())
	}
}


// OrganisationIdentification39 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OrganisationIdentification39 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<GenericOrganisationIdentification3>>,
}

impl OrganisationIdentification39 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OrganisationIdentificationSchemeName1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl OrganisationIdentificationSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// OrganisationLegalStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum OrganisationLegalStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CIOC") )]
	CodeCIOC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHAR") )]
	CodeCHAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CICC") )]
	CodeCICC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GENP") )]
	CodeGENP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IAPS") )]
	CodeIAPS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LLPP") )]
	CodeLLPP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PCLG") )]
	CodePCLG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIMP") )]
	CodeLIMP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PCLS") )]
	CodePCLS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PCLC") )]
	CodePCLC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SOLE") )]
	CodeSOLE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UNLC") )]
	CodeUNLC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UNLT") )]
	CodeUNLT,
}

impl OrganisationLegalStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OrganisationModification3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OrganisationModification3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FullLglNm") )]
	pub full_lgl_nm: FullLegalNameModification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgNm", skip_serializing_if = "Option::is_none") )]
	pub tradg_nm: Option<TradingNameModification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfOpr") )]
	pub ctry_of_opr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnDt", skip_serializing_if = "Option::is_none") )]
	pub regn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OprlAdr", skip_serializing_if = "Option::is_none") )]
	pub oprl_adr: Option<AddressModification3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizAdr", skip_serializing_if = "Option::is_none") )]
	pub biz_adr: Option<AddressModification3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglAdr") )]
	pub lgl_adr: AddressModification3,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BllgAdr", skip_serializing_if = "Option::is_none") )]
	pub bllg_adr: Option<AddressModification3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgId") )]
	pub org_id: OrganisationIdentification39,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RprtvOffcr", skip_serializing_if = "Option::is_none") )]
	pub rprtv_offcr: Option<Vec<PartyModification3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrsrMgr", skip_serializing_if = "Option::is_none") )]
	pub trsr_mgr: Option<PartyModification3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MainMndtHldr", skip_serializing_if = "Option::is_none") )]
	pub main_mndt_hldr: Option<Vec<PartyModification3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sndr", skip_serializing_if = "Option::is_none") )]
	pub sndr: Option<Vec<PartyModification3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglRprtv", skip_serializing_if = "Option::is_none") )]
	pub lgl_rprtv: Option<Vec<PartyModification3>>,
}

impl OrganisationModification3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.full_lgl_nm.validate()?;
		if let Some(ref val) = self.tradg_nm { val.validate()? }
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry_of_opr) {
			return Err(ValidationError::new(1005, "ctry_of_opr does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.oprl_adr { val.validate()? }
		if let Some(ref val) = self.biz_adr { val.validate()? }
		self.lgl_adr.validate()?;
		if let Some(ref val) = self.bllg_adr { val.validate()? }
		self.org_id.validate()?;
		if let Some(ref vec) = self.rprtv_offcr { for item in vec { item.validate()? } }
		if let Some(ref val) = self.trsr_mgr { val.validate()? }
		if let Some(ref vec) = self.main_mndt_hldr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.sndr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.lgl_rprtv { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OrganisationType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OrganisationType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<OrganisationType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl OrganisationType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// OrganisationType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum OrganisationType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "IFUN") )]
	CodeIFUN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRIV") )]
	CodePRIV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUBL") )]
	CodePUBL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PFUN") )]
	CodePFUN,
}

impl OrganisationType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OriginalTransactionReference43 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OriginalTransactionReference43 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId", skip_serializing_if = "Option::is_none") )]
	pub msg_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none") )]
	pub msg_nm_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTx", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx: Option<Vec<PaymentIdentification15>>,
}

impl OriginalTransactionReference43 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.msg_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.msg_nm_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_nm_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_nm_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.orgnl_tx { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OtherAccountStatus1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OtherAccountStatus1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
	pub sts: GenericIdentification36,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<GenericIdentification36>,
}

impl OtherAccountStatus1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.sts.validate()?;
		if let Some(ref val) = self.rsn { val.validate()? }
		Ok(())
	}
}


// OtherContact1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OtherContact1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChanlTp") )]
	pub chanl_tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
}

impl OtherContact1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.chanl_tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "chanl_tp is shorter than the minimum length of 1".to_string()));
		}
		if self.chanl_tp.chars().count() > 4 {
			return Err(ValidationError::new(1002, "chanl_tp exceeds the maximum length of 4".to_string()));
		}
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 128 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 128".to_string()));
			}
		}
		Ok(())
	}
}


// OtherIdentification1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OtherIdentification1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<PersonIdentificationType5Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl OtherIdentification1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// OtherIdentification3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OtherIdentification3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<PartyIdentificationType7Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl OtherIdentification3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// OwnerIdentification3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OwnerIdentification3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IndvOwnrId", skip_serializing_if = "Option::is_none") )]
	pub indv_ownr_id: Option<IndividualPersonIdentification2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgOwnrId", skip_serializing_if = "Option::is_none") )]
	pub org_ownr_id: Option<PartyIdentification139>,
}

impl OwnerIdentification3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.indv_ownr_id { val.validate()? }
		if let Some(ref val) = self.org_ownr_id { val.validate()? }
		Ok(())
	}
}


// OwnershipBeneficiaryRate1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OwnershipBeneficiaryRate1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Frctn", skip_serializing_if = "Option::is_none") )]
	pub frctn: Option<String>,
}

impl OwnershipBeneficiaryRate1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.frctn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "frctn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "frctn exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// OwnershipType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OwnershipType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AccountOwnershipType4Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl OwnershipType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// Party47Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Party47Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Org", skip_serializing_if = "Option::is_none") )]
	pub org: Option<Organisation39>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IndvPrsn", skip_serializing_if = "Option::is_none") )]
	pub indv_prsn: Option<IndividualPerson37>,
}

impl Party47Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.org { val.validate()? }
		if let Some(ref val) = self.indv_prsn { val.validate()? }
		Ok(())
	}
}


// Party48Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Party48Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Org", skip_serializing_if = "Option::is_none") )]
	pub org: Option<Organisation40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IndvPrsn", skip_serializing_if = "Option::is_none") )]
	pub indv_prsn: Option<IndividualPerson38>,
}

impl Party48Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.org { val.validate()? }
		if let Some(ref val) = self.indv_prsn { val.validate()? }
		Ok(())
	}
}


// Party50Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Party50Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty", skip_serializing_if = "Option::is_none") )]
	pub pty: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Agt", skip_serializing_if = "Option::is_none") )]
	pub agt: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl Party50Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pty { val.validate()? }
		if let Some(ref val) = self.agt { val.validate()? }
		Ok(())
	}
}


// Party52Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Party52Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgId", skip_serializing_if = "Option::is_none") )]
	pub org_id: Option<OrganisationIdentification39>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvtId", skip_serializing_if = "Option::is_none") )]
	pub prvt_id: Option<PersonIdentification18>,
}

impl Party52Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.org_id { val.validate()? }
		if let Some(ref val) = self.prvt_id { val.validate()? }
		Ok(())
	}
}


// PartyAndAuthorisation6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyAndAuthorisation6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModCd", skip_serializing_if = "Option::is_none") )]
	pub mod_cd: Option<Modification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyOrGrp") )]
	pub pty_or_grp: PartyOrGroup3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SgntrOrdr", skip_serializing_if = "Option::is_none") )]
	pub sgntr_ordr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Authstn") )]
	pub authstn: Authorisation2,
}

impl PartyAndAuthorisation6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mod_cd { val.validate()? }
		self.pty_or_grp.validate()?;
		if let Some(ref val) = self.sgntr_ordr {
			let pattern = Regex::new("[\\+]{0,1}[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "sgntr_ordr does not match the required pattern".to_string()));
			}
		}
		self.authstn.validate()?;
		Ok(())
	}
}


// PartyAndAuthorisation7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyAndAuthorisation7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyOrGrp") )]
	pub pty_or_grp: PartyOrGroup3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SgntrOrdr", skip_serializing_if = "Option::is_none") )]
	pub sgntr_ordr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Authstn") )]
	pub authstn: Authorisation2,
}

impl PartyAndAuthorisation7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty_or_grp.validate()?;
		if let Some(ref val) = self.sgntr_ordr {
			let pattern = Regex::new("[\\+]{0,1}[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "sgntr_ordr does not match the required pattern".to_string()));
			}
		}
		self.authstn.validate()?;
		Ok(())
	}
}


// PartyAndCertificate6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyAndCertificate6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty") )]
	pub pty: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cert", skip_serializing_if = "Option::is_none") )]
	pub cert: Option<String>,
}

impl PartyAndCertificate6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty.validate()?;
		if let Some(ref val) = self.cert {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cert is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 10240 {
				return Err(ValidationError::new(1002, "cert exceeds the maximum length of 10240".to_string()));
			}
		}
		Ok(())
	}
}


// PartyAndCertificate7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyAndCertificate7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModCd", skip_serializing_if = "Option::is_none") )]
	pub mod_cd: Option<Modification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty") )]
	pub pty: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cert", skip_serializing_if = "Option::is_none") )]
	pub cert: Option<String>,
}

impl PartyAndCertificate7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mod_cd { val.validate()? }
		self.pty.validate()?;
		if let Some(ref val) = self.cert {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cert is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 10240 {
				return Err(ValidationError::new(1002, "cert exceeds the maximum length of 10240".to_string()));
			}
		}
		Ok(())
	}
}


// PartyAndSignature4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyAndSignature4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty") )]
	pub pty: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sgntr") )]
	pub sgntr: SkipPayload,
}

impl PartyAndSignature4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty.validate()?;
		self.sgntr.validate()?;
		Ok(())
	}
}


// PartyIdentification125Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentification125Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
	pub prtry_id: Option<GenericIdentification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
	pub nm_and_adr: Option<NameAndAddress5>,
}

impl PartyIdentification125Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry_id { val.validate()? }
		if let Some(ref val) = self.nm_and_adr { val.validate()? }
		Ok(())
	}
}


// PartyIdentification139 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentification139 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty") )]
	pub pty: PartyIdentification125Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
}

impl PartyIdentification139 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty.validate()?;
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PartyIdentification177Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentification177Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
	pub prtry_id: Option<GenericIdentification1>,
}

impl PartyIdentification177Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry_id { val.validate()? }
		Ok(())
	}
}


// PartyIdentification182Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentification182Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
	pub prtry_id: Option<GenericIdentification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
	pub nm_and_adr: Option<NameAndAddress15>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxIdNb", skip_serializing_if = "Option::is_none") )]
	pub tax_id_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtlRegnNb", skip_serializing_if = "Option::is_none") )]
	pub ntl_regn_nb: Option<String>,
}

impl PartyIdentification182Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry_id { val.validate()? }
		if let Some(ref val) = self.nm_and_adr { val.validate()? }
		if let Some(ref val) = self.tax_id_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_id_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_id_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ntl_regn_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ntl_regn_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ntl_regn_nb exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// PartyIdentification220 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentification220 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<PartyIdentification182Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none") )]
	pub lgl_ntty_idr: Option<String>,
}

impl PartyIdentification220 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.lgl_ntty_idr {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lgl_ntty_idr does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PartyIdentification272 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentification272 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<PostalAddress27>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<Party52Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_res: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none") )]
	pub ctct_dtls: Option<Contact13>,
}

impl PartyIdentification272 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.ctry_of_res {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_res does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctct_dtls { val.validate()? }
		Ok(())
	}
}


// PartyIdentification274 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentification274 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<PostalAddress27>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<PersonIdentification18>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_res: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none") )]
	pub ctct_dtls: Option<Contact13>,
}

impl PartyIdentification274 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.ctry_of_res {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_res does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctct_dtls { val.validate()? }
		Ok(())
	}
}


// PartyIdentificationType7Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PartyIdentificationType7Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ATIN") )]
	CodeATIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IDCD") )]
	CodeIDCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NRIN") )]
	CodeNRIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PASS") )]
	CodePASS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "POCD") )]
	CodePOCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SOCS") )]
	CodeSOCS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SRSA") )]
	CodeSRSA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GUNL") )]
	CodeGUNL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GTIN") )]
	CodeGTIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ITIN") )]
	CodeITIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CPFA") )]
	CodeCPFA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AREG") )]
	CodeAREG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DRLC") )]
	CodeDRLC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EMID") )]
	CodeEMID,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NINV") )]
	CodeNINV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INCL") )]
	CodeINCL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GIIN") )]
	CodeGIIN,
}

impl PartyIdentificationType7Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PartyModification3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyModification3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModCd", skip_serializing_if = "Option::is_none") )]
	pub mod_cd: Option<Modification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId") )]
	pub pty_id: PartyIdentification274,
}

impl PartyModification3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mod_cd { val.validate()? }
		self.pty_id.validate()?;
		Ok(())
	}
}


// PartyOrGroup3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyOrGroup3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrpId", skip_serializing_if = "Option::is_none") )]
	pub grp_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty", skip_serializing_if = "Option::is_none") )]
	pub pty: Option<PartyAndCertificate6>,
}

impl PartyOrGroup3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.grp_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "grp_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "grp_id exceeds the maximum length of 4".to_string()));
			}
			let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "grp_id does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pty { val.validate()? }
		Ok(())
	}
}


// PartyProfileInformation5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyProfileInformation5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CertfctnInd", skip_serializing_if = "Option::is_none") )]
	pub certfctn_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldtngPty", skip_serializing_if = "Option::is_none") )]
	pub vldtng_pty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChckngPty", skip_serializing_if = "Option::is_none") )]
	pub chckng_pty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RspnsblPty", skip_serializing_if = "Option::is_none") )]
	pub rspnsbl_pty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CertTp", skip_serializing_if = "Option::is_none") )]
	pub cert_tp: Option<CertificationType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChckngDt", skip_serializing_if = "Option::is_none") )]
	pub chckng_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChckngFrqcy", skip_serializing_if = "Option::is_none") )]
	pub chckng_frqcy: Option<EventFrequency1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NxtRvsnDt", skip_serializing_if = "Option::is_none") )]
	pub nxt_rvsn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SlryRg", skip_serializing_if = "Option::is_none") )]
	pub slry_rg: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SrcOfWlth", skip_serializing_if = "Option::is_none") )]
	pub src_of_wlth: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CstmrCndctClssfctn", skip_serializing_if = "Option::is_none") )]
	pub cstmr_cndct_clssfctn: Option<CustomerConductClassification1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RskLvl", skip_serializing_if = "Option::is_none") )]
	pub rsk_lvl: Option<RiskLevel2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KnowYourCstmrChckTp", skip_serializing_if = "Option::is_none") )]
	pub know_your_cstmr_chck_tp: Option<KYCCheckType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KnowYourCstmrDBChck", skip_serializing_if = "Option::is_none") )]
	pub know_your_cstmr_db_chck: Option<DataBaseCheck1>,
}

impl PartyProfileInformation5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.vldtng_pty {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "vldtng_pty is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "vldtng_pty exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.chckng_pty {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "chckng_pty is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "chckng_pty exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.rspnsbl_pty {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rspnsbl_pty is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "rspnsbl_pty exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.cert_tp { val.validate()? }
		if let Some(ref val) = self.chckng_frqcy { val.validate()? }
		if let Some(ref val) = self.slry_rg {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "slry_rg is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "slry_rg exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.src_of_wlth {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "src_of_wlth is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "src_of_wlth exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.cstmr_cndct_clssfctn { val.validate()? }
		if let Some(ref val) = self.rsk_lvl { val.validate()? }
		if let Some(ref val) = self.know_your_cstmr_chck_tp { val.validate()? }
		if let Some(ref val) = self.know_your_cstmr_db_chck { val.validate()? }
		Ok(())
	}
}


// PartyRole1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PartyRole1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CUST") )]
	CodeCUST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INVS") )]
	CodeINVS,
}

impl PartyRole1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PartyRole2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyRole2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InvestmentFundRole6Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl PartyRole2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// PartyRole4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyRole4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InvestmentFundRole7Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl PartyRole4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// PartyRole5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyRole5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<PartyRole1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl PartyRole5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// PaymentCard29 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentCard29 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: CardType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nb") )]
	pub nb: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HldrNm") )]
	pub hldr_nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
	pub start_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt") )]
	pub xpry_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CardIssrNm", skip_serializing_if = "Option::is_none") )]
	pub card_issr_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CardIssrId", skip_serializing_if = "Option::is_none") )]
	pub card_issr_id: Option<PartyIdentification125Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctyCd", skip_serializing_if = "Option::is_none") )]
	pub scty_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SeqNb", skip_serializing_if = "Option::is_none") )]
	pub seq_nb: Option<String>,
}

impl PaymentCard29 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		if self.nb.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nb is shorter than the minimum length of 1".to_string()));
		}
		if self.nb.chars().count() > 35 {
			return Err(ValidationError::new(1002, "nb exceeds the maximum length of 35".to_string()));
		}
		if self.hldr_nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "hldr_nm is shorter than the minimum length of 1".to_string()));
		}
		if self.hldr_nm.chars().count() > 35 {
			return Err(ValidationError::new(1002, "hldr_nm exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.card_issr_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "card_issr_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "card_issr_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.card_issr_id { val.validate()? }
		if let Some(ref val) = self.scty_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "scty_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "scty_cd exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.seq_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "seq_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 3 {
				return Err(ValidationError::new(1002, "seq_nb exceeds the maximum length of 3".to_string()));
			}
		}
		Ok(())
	}
}


// PaymentIdentification15 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentIdentification15 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrId", skip_serializing_if = "Option::is_none") )]
	pub instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndToEndId") )]
	pub end_to_end_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
	pub tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
	pub uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none") )]
	pub clr_sys_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrstAgt", skip_serializing_if = "Option::is_none") )]
	pub frst_agt: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl PaymentIdentification15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if self.end_to_end_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "end_to_end_id is shorter than the minimum length of 1".to_string()));
		}
		if self.end_to_end_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "end_to_end_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.clr_sys_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clr_sys_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clr_sys_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.frst_agt { val.validate()? }
		Ok(())
	}
}


// PaymentIdentification6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentIdentification6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrId", skip_serializing_if = "Option::is_none") )]
	pub instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndToEndId") )]
	pub end_to_end_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
	pub uetr: Option<String>,
}

impl PaymentIdentification6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if self.end_to_end_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "end_to_end_id is shorter than the minimum length of 1".to_string()));
		}
		if self.end_to_end_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "end_to_end_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PaymentInstruction43 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentInstruction43 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInfId") )]
	pub pmt_inf_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtd") )]
	pub pmt_mtd: PaymentMethod3Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BtchBookg", skip_serializing_if = "Option::is_none") )]
	pub btch_bookg: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxs", skip_serializing_if = "Option::is_none") )]
	pub nb_of_txs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none") )]
	pub ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation26>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdExctnDt") )]
	pub reqd_exctn_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PoolgAdjstmntDt", skip_serializing_if = "Option::is_none") )]
	pub poolg_adjstmnt_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr") )]
	pub dbtr: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct") )]
	pub dbtr_acct: CashAccount40,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt") )]
	pub dbtr_agt: BranchAndFinancialInstitutionIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForDbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub instr_for_dbtr_agt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none") )]
	pub chrg_br: Option<ChargeBearerType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgsAcct", skip_serializing_if = "Option::is_none") )]
	pub chrgs_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgsAcctAgt", skip_serializing_if = "Option::is_none") )]
	pub chrgs_acct_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtTrfTxInf") )]
	pub cdt_trf_tx_inf: Vec<CreditTransferTransaction59>,
}

impl PaymentInstruction43 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.pmt_inf_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "pmt_inf_id is shorter than the minimum length of 1".to_string()));
		}
		if self.pmt_inf_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "pmt_inf_id exceeds the maximum length of 35".to_string()));
		}
		self.pmt_mtd.validate()?;
		if let Some(ref val) = self.nb_of_txs {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nb_of_txs does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		self.dbtr.validate()?;
		self.dbtr_acct.validate()?;
		self.dbtr_agt.validate()?;
		if let Some(ref val) = self.dbtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.instr_for_dbtr_agt {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_for_dbtr_agt is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "instr_for_dbtr_agt exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.chrg_br { val.validate()? }
		if let Some(ref val) = self.chrgs_acct { val.validate()? }
		if let Some(ref val) = self.chrgs_acct_agt { val.validate()? }
		for item in &self.cdt_trf_tx_inf { item.validate()? }
		Ok(())
	}
}


// PaymentInstrument17 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentInstrument17 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCcy") )]
	pub sttlm_ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DvddPctg", skip_serializing_if = "Option::is_none") )]
	pub dvdd_pctg: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SbcptPmtInstrm", skip_serializing_if = "Option::is_none") )]
	pub sbcpt_pmt_instrm: Option<PaymentInstrument24Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RedPmtInstrm", skip_serializing_if = "Option::is_none") )]
	pub red_pmt_instrm: Option<PaymentInstrument19Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DvddPmtInstrm", skip_serializing_if = "Option::is_none") )]
	pub dvdd_pmt_instrm: Option<PaymentInstrument19Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvgsPlanPmtInstrm", skip_serializing_if = "Option::is_none") )]
	pub svgs_plan_pmt_instrm: Option<PaymentInstrument24Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstPmtInstrm", skip_serializing_if = "Option::is_none") )]
	pub intrst_pmt_instrm: Option<PaymentInstrument19Choice>,
}

impl PaymentInstrument17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.sttlm_ccy) {
			return Err(ValidationError::new(1005, "sttlm_ccy does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.dvdd_pctg {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "dvdd_pctg is less than the minimum value of 0.000000".to_string()));
			}
			if *val > 100.000000 {
				return Err(ValidationError::new(1004, "dvdd_pctg exceeds the maximum value of 100.000000".to_string()));
			}
		}
		if let Some(ref val) = self.sbcpt_pmt_instrm { val.validate()? }
		if let Some(ref val) = self.red_pmt_instrm { val.validate()? }
		if let Some(ref val) = self.dvdd_pmt_instrm { val.validate()? }
		if let Some(ref val) = self.svgs_plan_pmt_instrm { val.validate()? }
		if let Some(ref val) = self.intrst_pmt_instrm { val.validate()? }
		Ok(())
	}
}


// PaymentInstrument19Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentInstrument19Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqDtls", skip_serializing_if = "Option::is_none") )]
	pub chq_dtls: Option<Cheque4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BkrsDrftDtls", skip_serializing_if = "Option::is_none") )]
	pub bkrs_drft_dtls: Option<Cheque4>,
}

impl PaymentInstrument19Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.chq_dtls { val.validate()? }
		if let Some(ref val) = self.bkrs_drft_dtls { val.validate()? }
		Ok(())
	}
}


// PaymentInstrument24Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentInstrument24Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCardDtls", skip_serializing_if = "Option::is_none") )]
	pub pmt_card_dtls: Option<PaymentCard29>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DrctDbtDtls", skip_serializing_if = "Option::is_none") )]
	pub drct_dbt_dtls: Option<DirectDebitMandate7>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Chq", skip_serializing_if = "Option::is_none") )]
	pub chq: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BkrsDrft", skip_serializing_if = "Option::is_none") )]
	pub bkrs_drft: Option<bool>,
}

impl PaymentInstrument24Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pmt_card_dtls { val.validate()? }
		if let Some(ref val) = self.drct_dbt_dtls { val.validate()? }
		Ok(())
	}
}


// PaymentMethod3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PaymentMethod3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHK") )]
	CodeCHK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRF") )]
	CodeTRF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRA") )]
	CodeTRA,
}

impl PaymentMethod3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PaymentTypeInformation26 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentTypeInformation26 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none") )]
	pub instr_prty: Option<Priority2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none") )]
	pub svc_lvl: Option<Vec<ServiceLevel8Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none") )]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none") )]
	pub ctgy_purp: Option<CategoryPurpose1Choice>,
}

impl PaymentTypeInformation26 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instr_prty { val.validate()? }
		if let Some(ref vec) = self.svc_lvl { for item in vec { item.validate()? } }
		if let Some(ref val) = self.lcl_instrm { val.validate()? }
		if let Some(ref val) = self.ctgy_purp { val.validate()? }
		Ok(())
	}
}


// PendingOpeningStatusReason1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PendingOpeningStatusReason1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
	pub cd: PendingOpeningStatusReason2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl PendingOpeningStatusReason1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd.validate()?;
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}


// PendingOpeningStatusReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PendingOpeningStatusReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<Vec<PendingOpeningStatusReason1>>,
}

impl PendingOpeningStatusReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.no_spcfd_rsn { val.validate()? }
		if let Some(ref vec) = self.rsn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PendingOpeningStatusReason1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PendingOpeningStatusReason1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ATHR") )]
	CodeATHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ATHP") )]
	CodeATHP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FRDM") )]
	CodeFRDM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KYCM") )]
	CodeKYCM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOTO") )]
	CodeNOTO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REST") )]
	CodeREST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RIGH") )]
	CodeRIGH,
}

impl PendingOpeningStatusReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PendingOpeningStatusReason2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PendingOpeningStatusReason2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<PendingOpeningStatusReason1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl PendingOpeningStatusReason2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// PendingStatusReason14 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PendingStatusReason14 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
	pub cd: PendingStatusReason2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl PendingStatusReason14 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd.validate()?;
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}


// PendingStatusReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PendingStatusReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<Vec<PendingStatusReason14>>,
}

impl PendingStatusReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.no_spcfd_rsn { val.validate()? }
		if let Some(ref vec) = self.rsn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PendingStatusReason1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PendingStatusReason1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "KYCM") )]
	CodeKYCM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FRDM") )]
	CodeFRDM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RIGH") )]
	CodeRIGH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ATHR") )]
	CodeATHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ATHP") )]
	CodeATHP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MODI") )]
	CodeMODI,
}

impl PendingStatusReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PendingStatusReason2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PendingStatusReason2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<PendingStatusReason1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl PendingStatusReason2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// PersonIdentification18 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PersonIdentification18 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none") )]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<GenericPersonIdentification2>>,
}

impl PersonIdentification18 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dt_and_plc_of_birth { val.validate()? }
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PersonIdentificationSchemeName1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PersonIdentificationSchemeName1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl PersonIdentificationSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// PersonIdentificationType5Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PersonIdentificationType5Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AREG") )]
	CodeAREG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CPFA") )]
	CodeCPFA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DRLC") )]
	CodeDRLC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EMID") )]
	CodeEMID,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IDCD") )]
	CodeIDCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NRIN") )]
	CodeNRIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PASS") )]
	CodePASS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "POCD") )]
	CodePOCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SOCS") )]
	CodeSOCS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SRSA") )]
	CodeSRSA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GUNL") )]
	CodeGUNL,
}

impl PersonIdentificationType5Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PersonalInformation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PersonalInformation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmOfFthr", skip_serializing_if = "Option::is_none") )]
	pub nm_of_fthr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MdnNmOfMthr", skip_serializing_if = "Option::is_none") )]
	pub mdn_nm_of_mthr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmOfPrtnr", skip_serializing_if = "Option::is_none") )]
	pub nm_of_prtnr: Option<String>,
}

impl PersonalInformation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm_of_fthr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm_of_fthr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nm_of_fthr exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.mdn_nm_of_mthr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mdn_nm_of_mthr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mdn_nm_of_mthr exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.nm_of_prtnr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm_of_prtnr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nm_of_prtnr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// PlanStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PlanStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACTV") )]
	CodeACTV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLOS") )]
	CodeCLOS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SUSP") )]
	CodeSUSP,
}

impl PlanStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PlanStatus2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PlanStatus2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<PlanStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl PlanStatus2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// PoliticalExposureType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PoliticalExposureType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<PoliticalExposureType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl PoliticalExposureType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// PoliticalExposureType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PoliticalExposureType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NPEX") )]
	CodeNPEX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YPEX") )]
	CodeYPEX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PEXD") )]
	CodePEXD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PEXF") )]
	CodePEXF,
}

impl PoliticalExposureType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PoliticallyExposedPerson1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PoliticallyExposedPerson1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PltclyXpsdPrsnTp") )]
	pub pltcly_xpsd_prsn_tp: PoliticalExposureType2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PltclyXpsdPrsnSts", skip_serializing_if = "Option::is_none") )]
	pub pltcly_xpsd_prsn_sts: Option<PoliticallyExposedPersonStatus1Choice>,
}

impl PoliticallyExposedPerson1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pltcly_xpsd_prsn_tp.validate()?;
		if let Some(ref val) = self.pltcly_xpsd_prsn_sts { val.validate()? }
		Ok(())
	}
}


// PoliticallyExposedPersonStatus1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PoliticallyExposedPersonStatus1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<PoliticallyExposedPersonStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl PoliticallyExposedPersonStatus1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// PoliticallyExposedPersonStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PoliticallyExposedPersonStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PE03") )]
	CodePE03,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PE01") )]
	CodePE01,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PE02") )]
	CodePE02,
}

impl PoliticallyExposedPersonStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PositionEffect3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PositionEffect3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FIFO") )]
	CodeFIFO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIFO") )]
	CodeLIFO,
}

impl PositionEffect3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PostalAddress1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PostalAddress1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
	pub adr_tp: Option<AddressType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
	pub adr_line: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
	pub strt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
	pub bldg_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
	pub pst_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
	pub twn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
	pub ctry_sub_dvsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
	pub ctry: String,
}

impl PostalAddress1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.adr_tp { val.validate()? }
		if let Some(ref vec) = self.adr_line {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "adr_line is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 70 {
					return Err(ValidationError::new(1002, "adr_line exceeds the maximum length of 70".to_string()));
				}
			}
		}
		if let Some(ref val) = self.strt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "strt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "strt_nm exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.bldg_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bldg_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "bldg_nb exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.pst_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pst_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "pst_cd exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.twn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "twn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "twn_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctry_sub_dvsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctry_sub_dvsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctry_sub_dvsn exceeds the maximum length of 35".to_string()));
			}
		}
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry) {
			return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// PostalAddress21 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PostalAddress21 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
	pub adr_tp: Option<AddressType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MlngInd", skip_serializing_if = "Option::is_none") )]
	pub mlng_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnAdrInd", skip_serializing_if = "Option::is_none") )]
	pub regn_adr_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CareOf", skip_serializing_if = "Option::is_none") )]
	pub care_of: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
	pub adr_line: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
	pub strt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
	pub bldg_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNm", skip_serializing_if = "Option::is_none") )]
	pub bldg_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstBx", skip_serializing_if = "Option::is_none") )]
	pub pst_bx: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SdInBldg", skip_serializing_if = "Option::is_none") )]
	pub sd_in_bldg: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Flr", skip_serializing_if = "Option::is_none") )]
	pub flr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SuiteId", skip_serializing_if = "Option::is_none") )]
	pub suite_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
	pub pst_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none") )]
	pub dstrct_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Vllg", skip_serializing_if = "Option::is_none") )]
	pub vllg: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
	pub twn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Stat", skip_serializing_if = "Option::is_none") )]
	pub stat: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
	pub ctry: String,
}

impl PostalAddress21 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.adr_tp { val.validate()? }
		if let Some(ref val) = self.care_of {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "care_of is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "care_of exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref vec) = self.adr_line {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "adr_line is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 70 {
					return Err(ValidationError::new(1002, "adr_line exceeds the maximum length of 70".to_string()));
				}
			}
		}
		if let Some(ref val) = self.strt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "strt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "strt_nm exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.bldg_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bldg_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "bldg_nb exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.bldg_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bldg_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "bldg_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.pst_bx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pst_bx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 10 {
				return Err(ValidationError::new(1002, "pst_bx exceeds the maximum length of 10".to_string()));
			}
		}
		if let Some(ref val) = self.sd_in_bldg {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sd_in_bldg is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sd_in_bldg exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.flr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "flr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "flr exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.suite_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "suite_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 10 {
				return Err(ValidationError::new(1002, "suite_id exceeds the maximum length of 10".to_string()));
			}
		}
		if let Some(ref val) = self.pst_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pst_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "pst_cd exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.dstrct_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dstrct_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "dstrct_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.vllg {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "vllg is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "vllg exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.twn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "twn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "twn_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.stat {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "stat is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "stat exceeds the maximum length of 70".to_string()));
			}
		}
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry) {
			return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// PostalAddress27 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PostalAddress27 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
	pub adr_tp: Option<AddressType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CareOf", skip_serializing_if = "Option::is_none") )]
	pub care_of: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dept", skip_serializing_if = "Option::is_none") )]
	pub dept: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubDept", skip_serializing_if = "Option::is_none") )]
	pub sub_dept: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
	pub strt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
	pub bldg_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNm", skip_serializing_if = "Option::is_none") )]
	pub bldg_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Flr", skip_serializing_if = "Option::is_none") )]
	pub flr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitNb", skip_serializing_if = "Option::is_none") )]
	pub unit_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstBx", skip_serializing_if = "Option::is_none") )]
	pub pst_bx: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Room", skip_serializing_if = "Option::is_none") )]
	pub room: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
	pub pst_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
	pub twn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none") )]
	pub twn_lctn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none") )]
	pub dstrct_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
	pub ctry_sub_dvsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
	pub adr_line: Option<Vec<String>>,
}

impl PostalAddress27 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.adr_tp { val.validate()? }
		if let Some(ref val) = self.care_of {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "care_of is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "care_of exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.dept {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dept is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "dept exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.sub_dept {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sub_dept is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "sub_dept exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.strt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "strt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "strt_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.bldg_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bldg_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "bldg_nb exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.bldg_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bldg_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "bldg_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.flr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "flr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "flr exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.unit_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "unit_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "unit_nb exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.pst_bx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pst_bx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "pst_bx exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.room {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "room is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "room exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.pst_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pst_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "pst_cd exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.twn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "twn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "twn_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.twn_lctn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "twn_lctn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "twn_lctn_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.dstrct_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dstrct_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "dstrct_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.ctry_sub_dvsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctry_sub_dvsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctry_sub_dvsn exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.adr_line {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "adr_line is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 70 {
					return Err(ValidationError::new(1002, "adr_line exceeds the maximum length of 70".to_string()));
				}
			}
		}
		Ok(())
	}
}


// PreferredContactMethod2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PreferredContactMethod2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MAIL") )]
	CodeMAIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FAXX") )]
	CodeFAXX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LETT") )]
	CodeLETT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CELL") )]
	CodeCELL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ONLI") )]
	CodeONLI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PHON") )]
	CodePHON,
}

impl PreferredContactMethod2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Priority2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Priority2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "HIGH") )]
	CodeHIGH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NORM") )]
	CodeNORM,
}

impl Priority2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ProfileType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ProfileType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<ProfileType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl ProfileType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// ProfileType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum ProfileType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "HEDG") )]
	CodeHEDG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HFTR") )]
	CodeHFTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MAKE") )]
	CodeMAKE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TREA") )]
	CodeTREA,
}

impl ProfileType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ProformaStatusReason1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ProformaStatusReason1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
	pub cd: ProformaStatusReason2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl ProformaStatusReason1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd.validate()?;
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}


// ProformaStatusReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ProformaStatusReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<Vec<ProformaStatusReason1>>,
}

impl ProformaStatusReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.no_spcfd_rsn { val.validate()? }
		if let Some(ref vec) = self.rsn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ProformaStatusReason1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum ProformaStatusReason1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MODI") )]
	CodeMODI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RIGH") )]
	CodeRIGH,
}

impl ProformaStatusReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ProformaStatusReason2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ProformaStatusReason2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<ProformaStatusReason1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl ProformaStatusReason2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// ProprietaryBankTransactionCodeStructure1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ProprietaryBankTransactionCodeStructure1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
	pub cd: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl ProprietaryBankTransactionCodeStructure1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.cd.chars().count() < 1 {
			return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
		}
		if self.cd.chars().count() > 35 {
			return Err(ValidationError::new(1002, "cd exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// Provided1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Provided1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NPRO") )]
	CodeNPRO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PROV") )]
	CodePROV,
}

impl Provided1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ProxyAccountIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ProxyAccountIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<ProxyAccountType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
}

impl ProxyAccountIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 2048 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 2048".to_string()));
		}
		Ok(())
	}
}


// ProxyAccountType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ProxyAccountType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ProxyAccountType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// Purpose2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Purpose2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl Purpose2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// PurposeModification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PurposeModification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModCd", skip_serializing_if = "Option::is_none") )]
	pub mod_cd: Option<Modification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp") )]
	pub purp: String,
}

impl PurposeModification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mod_cd { val.validate()? }
		if self.purp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "purp is shorter than the minimum length of 1".to_string()));
		}
		if self.purp.chars().count() > 140 {
			return Err(ValidationError::new(1002, "purp exceeds the maximum length of 140".to_string()));
		}
		Ok(())
	}
}


// Rank1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Rank1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRIM") )]
	CodePRIM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SECO") )]
	CodeSECO,
}

impl Rank1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// References3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct References3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqToBeCmpltdId") )]
	pub req_to_be_cmpltd_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrcId") )]
	pub prc_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqRsn") )]
	pub req_rsn: Vec<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AttchdDocNm", skip_serializing_if = "Option::is_none") )]
	pub attchd_doc_nm: Option<Vec<String>>,
}

impl References3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.msg_id.validate()?;
		self.req_to_be_cmpltd_id.validate()?;
		self.prc_id.validate()?;
		for item in &self.req_rsn {
			if item.chars().count() < 1 {
				return Err(ValidationError::new(1001, "req_rsn is shorter than the minimum length of 1".to_string()));
			}
			if item.chars().count() > 35 {
				return Err(ValidationError::new(1002, "req_rsn exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.attchd_doc_nm {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "attchd_doc_nm is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 70 {
					return Err(ValidationError::new(1002, "attchd_doc_nm exceeds the maximum length of 70".to_string()));
				}
			}
		}
		Ok(())
	}
}


// References4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct References4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrcId") )]
	pub prc_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AttchdDocNm", skip_serializing_if = "Option::is_none") )]
	pub attchd_doc_nm: Option<Vec<String>>,
}

impl References4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.msg_id.validate()?;
		self.prc_id.validate()?;
		if let Some(ref vec) = self.attchd_doc_nm {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "attchd_doc_nm is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 70 {
					return Err(ValidationError::new(1002, "attchd_doc_nm exceeds the maximum length of 70".to_string()));
				}
			}
		}
		Ok(())
	}
}


// References5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct References5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqTp") )]
	pub req_tp: UseCases1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrcId") )]
	pub prc_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AckdMsgId", skip_serializing_if = "Option::is_none") )]
	pub ackd_msg_id: Option<Vec<MessageIdentification1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
	pub sts: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AttchdDocNm", skip_serializing_if = "Option::is_none") )]
	pub attchd_doc_nm: Option<Vec<String>>,
}

impl References5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.req_tp.validate()?;
		self.msg_id.validate()?;
		self.prc_id.validate()?;
		if let Some(ref vec) = self.ackd_msg_id { for item in vec { item.validate()? } }
		if let Some(ref val) = self.sts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sts exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.attchd_doc_nm {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "attchd_doc_nm is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 70 {
					return Err(ValidationError::new(1002, "attchd_doc_nm exceeds the maximum length of 70".to_string()));
				}
			}
		}
		Ok(())
	}
}


// References6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct References6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RjctdReqTp") )]
	pub rjctd_req_tp: UseCases1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RjctnRsn") )]
	pub rjctn_rsn: Vec<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RjctdReqId") )]
	pub rjctd_req_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrcId") )]
	pub prc_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AttchdDocNm", skip_serializing_if = "Option::is_none") )]
	pub attchd_doc_nm: Option<Vec<String>>,
}

impl References6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rjctd_req_tp.validate()?;
		for item in &self.rjctn_rsn {
			if item.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rjctn_rsn is shorter than the minimum length of 1".to_string()));
			}
			if item.chars().count() > 350 {
				return Err(ValidationError::new(1002, "rjctn_rsn exceeds the maximum length of 350".to_string()));
			}
		}
		self.rjctd_req_id.validate()?;
		self.msg_id.validate()?;
		self.prc_id.validate()?;
		if let Some(ref vec) = self.attchd_doc_nm {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "attchd_doc_nm is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 70 {
					return Err(ValidationError::new(1002, "attchd_doc_nm exceeds the maximum length of 70".to_string()));
				}
			}
		}
		Ok(())
	}
}


// Referred1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Referred1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "REFR") )]
	CodeREFR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NRFR") )]
	CodeNRFR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UKNW") )]
	CodeUKNW,
}

impl Referred1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ReferredAgent3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ReferredAgent3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rfrd") )]
	pub rfrd: Referred1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RfrdPlcmntAgt", skip_serializing_if = "Option::is_none") )]
	pub rfrd_plcmnt_agt: Option<PartyIdentification125Choice>,
}

impl ReferredAgent3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rfrd.validate()?;
		if let Some(ref val) = self.rfrd_plcmnt_agt { val.validate()? }
		Ok(())
	}
}


// ReferredDocumentInformation8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ReferredDocumentInformation8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<DocumentType1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nb", skip_serializing_if = "Option::is_none") )]
	pub nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdDt", skip_serializing_if = "Option::is_none") )]
	pub rltd_dt: Option<DateAndType1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LineDtls", skip_serializing_if = "Option::is_none") )]
	pub line_dtls: Option<Vec<DocumentLineInformation2>>,
}

impl ReferredDocumentInformation8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.rltd_dt { val.validate()? }
		if let Some(ref vec) = self.line_dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// RegisteredShareholderName1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RegisteredShareholderName1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IndvPrsn", skip_serializing_if = "Option::is_none") )]
	pub indv_prsn: Option<IndividualPerson29>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Org", skip_serializing_if = "Option::is_none") )]
	pub org: Option<Organisation23>,
}

impl RegisteredShareholderName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.indv_prsn { val.validate()? }
		if let Some(ref val) = self.org { val.validate()? }
		Ok(())
	}
}


// RegulatoryAuthority2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RegulatoryAuthority2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
}

impl RegulatoryAuthority2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// RegulatoryInformation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RegulatoryInformation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sctr", skip_serializing_if = "Option::is_none") )]
	pub sctr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Brnch", skip_serializing_if = "Option::is_none") )]
	pub brnch: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Grp", skip_serializing_if = "Option::is_none") )]
	pub grp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<String>,
}

impl RegulatoryInformation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sctr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sctr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sctr exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.brnch {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "brnch is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "brnch exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.grp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "grp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "grp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.othr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "othr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "othr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// RegulatoryReporting3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RegulatoryReporting3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtCdtRptgInd", skip_serializing_if = "Option::is_none") )]
	pub dbt_cdt_rptg_ind: Option<RegulatoryReportingType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Authrty", skip_serializing_if = "Option::is_none") )]
	pub authrty: Option<RegulatoryAuthority2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dtls", skip_serializing_if = "Option::is_none") )]
	pub dtls: Option<Vec<StructuredRegulatoryReporting3>>,
}

impl RegulatoryReporting3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dbt_cdt_rptg_ind { val.validate()? }
		if let Some(ref val) = self.authrty { val.validate()? }
		if let Some(ref vec) = self.dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// RegulatoryReportingType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum RegulatoryReportingType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRED") )]
	CodeCRED,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DEBT") )]
	CodeDEBT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOTH") )]
	CodeBOTH,
}

impl RegulatoryReportingType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Reinvestment4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Reinvestment4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmDtls") )]
	pub fin_instrm_dtls: FinancialInstrument87,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdNAVCcy", skip_serializing_if = "Option::is_none") )]
	pub reqd_nav_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RinvstmtPctg") )]
	pub rinvstmt_pctg: f64,
}

impl Reinvestment4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.fin_instrm_dtls.validate()?;
		if let Some(ref val) = self.reqd_nav_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "reqd_nav_ccy does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// RejectedReason16Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RejectedReason16Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<RejectedStatusReason6Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl RejectedReason16Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// RejectedStatusReason6Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum RejectedStatusReason6Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SAFE") )]
	CodeSAFE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NSLA") )]
	CodeNSLA,
}

impl RejectedStatusReason6Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RejectionReason31 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RejectionReason31 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: RejectedReason16Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_rsn_inf: Option<String>,
}

impl RejectionReason31 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rsn.validate()?;
		if let Some(ref val) = self.addtl_rsn_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_rsn_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "addtl_rsn_inf exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}


// RemittanceAmount4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RemittanceAmount4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtAmtAndTp", skip_serializing_if = "Option::is_none") )]
	pub rmt_amt_and_tp: Option<Vec<DocumentAmount1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none") )]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
}

impl RemittanceAmount4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.rmt_amt_and_tp { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.adjstmnt_amt_and_rsn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// RemittanceInformation22 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RemittanceInformation22 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ustrd", skip_serializing_if = "Option::is_none") )]
	pub ustrd: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Strd", skip_serializing_if = "Option::is_none") )]
	pub strd: Option<Vec<StructuredRemittanceInformation18>>,
}

impl RemittanceInformation22 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.ustrd {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "ustrd is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 140 {
					return Err(ValidationError::new(1002, "ustrd exceeds the maximum length of 140".to_string()));
				}
			}
		}
		if let Some(ref vec) = self.strd { for item in vec { item.validate()? } }
		Ok(())
	}
}


// RemittanceLocation9 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RemittanceLocation9 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtId", skip_serializing_if = "Option::is_none") )]
	pub rmt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtLctnMtd", skip_serializing_if = "Option::is_none") )]
	pub rmt_lctn_mtd: Option<RemittanceLocationMethod2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtLctnElctrncAdr", skip_serializing_if = "Option::is_none") )]
	pub rmt_lctn_elctrnc_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtLctnPstlAdr", skip_serializing_if = "Option::is_none") )]
	pub rmt_lctn_pstl_adr: Option<NameAndAddress18>,
}

impl RemittanceLocation9 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rmt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rmt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rmt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.rmt_lctn_mtd { val.validate()? }
		if let Some(ref val) = self.rmt_lctn_elctrnc_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rmt_lctn_elctrnc_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "rmt_lctn_elctrnc_adr exceeds the maximum length of 2048".to_string()));
			}
		}
		if let Some(ref val) = self.rmt_lctn_pstl_adr { val.validate()? }
		Ok(())
	}
}


// RemittanceLocationMethod2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum RemittanceLocationMethod2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FAXI") )]
	CodeFAXI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EDIC") )]
	CodeEDIC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "URID") )]
	CodeURID,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EMAL") )]
	CodeEMAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "POST") )]
	CodePOST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SMSM") )]
	CodeSMSM,
}

impl RemittanceLocationMethod2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Repartition6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Repartition6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Qty") )]
	pub qty: UnitsOrAmountOrPercentage1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrm") )]
	pub fin_instrm: FinancialInstrument87,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CcyOfPlan", skip_serializing_if = "Option::is_none") )]
	pub ccy_of_plan: Option<String>,
}

impl Repartition6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.qty.validate()?;
		self.fin_instrm.validate()?;
		if let Some(ref val) = self.ccy_of_plan {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ccy_of_plan does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// ResidentialStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum ResidentialStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RESI") )]
	CodeRESI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRES") )]
	CodePRES,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NRES") )]
	CodeNRES,
}

impl ResidentialStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ResponseDetails1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// Restriction1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Restriction1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RstrctnTp") )]
	pub rstrctn_tp: CodeOrProprietary1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldFr") )]
	pub vld_fr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldUntil", skip_serializing_if = "Option::is_none") )]
	pub vld_until: Option<String>,
}

impl Restriction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rstrctn_tp.validate()?;
		Ok(())
	}
}


// RestrictionModification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RestrictionModification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModCd", skip_serializing_if = "Option::is_none") )]
	pub mod_cd: Option<Modification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rstrctn") )]
	pub rstrctn: Restriction1,
}

impl RestrictionModification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mod_cd { val.validate()? }
		self.rstrctn.validate()?;
		Ok(())
	}
}


// RestrictionStatus1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RestrictionStatus1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<RestrictionStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl RestrictionStatus1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// RestrictionStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum RestrictionStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACTV") )]
	CodeACTV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INAC") )]
	CodeINAC,
}

impl RestrictionStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RiskLevel1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum RiskLevel1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "HIGH") )]
	CodeHIGH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LOWW") )]
	CodeLOWW,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MEDM") )]
	CodeMEDM,
}

impl RiskLevel1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RiskLevel2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RiskLevel2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<RiskLevel1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl RiskLevel2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// RoundingDirection1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum RoundingDirection1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RDUP") )]
	CodeRDUP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RDWN") )]
	CodeRDWN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STAN") )]
	CodeSTAN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DIST") )]
	CodeDIST,
}

impl RoundingDirection1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RoundingParameters1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RoundingParameters1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RndgMdlus", skip_serializing_if = "Option::is_none") )]
	pub rndg_mdlus: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RndgDrctn") )]
	pub rndg_drctn: RoundingDirection1Code,
}

impl RoundingParameters1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rndg_drctn.validate()?;
		Ok(())
	}
}


// SecurityIdentification25Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecurityIdentification25Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SEDOL", skip_serializing_if = "Option::is_none") )]
	pub sedol: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CUSIP", skip_serializing_if = "Option::is_none") )]
	pub cusip: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RIC", skip_serializing_if = "Option::is_none") )]
	pub ric: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none") )]
	pub tckr_symb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none") )]
	pub blmbrg: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CTA", skip_serializing_if = "Option::is_none") )]
	pub cta: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QUICK", skip_serializing_if = "Option::is_none") )]
	pub quick: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Wrtppr", skip_serializing_if = "Option::is_none") )]
	pub wrtppr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dtch", skip_serializing_if = "Option::is_none") )]
	pub dtch: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Vlrn", skip_serializing_if = "Option::is_none") )]
	pub vlrn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCVM", skip_serializing_if = "Option::is_none") )]
	pub scvm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Belgn", skip_serializing_if = "Option::is_none") )]
	pub belgn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cmon", skip_serializing_if = "Option::is_none") )]
	pub cmon: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPrtryId", skip_serializing_if = "Option::is_none") )]
	pub othr_prtry_id: Option<AlternateSecurityIdentification7>,
}

impl SecurityIdentification25Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.isin {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ric {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ric is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ric exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tckr_symb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tckr_symb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tckr_symb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.blmbrg {
			let pattern = Regex::new("(BBG)[BCDFGHJKLMNPQRSTVWXYZ\\d]{8}\\d").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "blmbrg does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.cta {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cta is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cta exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.cmon {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cmon is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 12 {
				return Err(ValidationError::new(1002, "cmon exceeds the maximum length of 12".to_string()));
			}
		}
		if let Some(ref val) = self.othr_prtry_id { val.validate()? }
		Ok(())
	}
}


// ServiceLevel8Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ServiceLevel8Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ServiceLevel8Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// SettlementFrequency1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SettlementFrequency1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<EventFrequency10Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl SettlementFrequency1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// SettlementInstructionReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SettlementInstructionReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<SettlementInstructionReason1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl SettlementInstructionReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// SettlementInstructionReason1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum SettlementInstructionReason1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSHI") )]
	CodeCSHI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ALLL") )]
	CodeALLL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSHO") )]
	CodeCSHO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHAR") )]
	CodeCHAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DIVI") )]
	CodeDIVI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INTE") )]
	CodeINTE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SAVP") )]
	CodeSAVP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REDM") )]
	CodeREDM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SAVE") )]
	CodeSAVE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BUYI") )]
	CodeBUYI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SELL") )]
	CodeSELL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SUBS") )]
	CodeSUBS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WTHP") )]
	CodeWTHP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CORP") )]
	CodeCORP,
}

impl SettlementInstructionReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SettlementMethod5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SettlementMethod5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdt", skip_serializing_if = "Option::is_none") )]
	pub cdt: Option<CreditTransferTransaction59>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbt", skip_serializing_if = "Option::is_none") )]
	pub dbt: Option<CreditTransferTransaction59>,
}

impl SettlementMethod5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cdt { val.validate()? }
		if let Some(ref val) = self.dbt { val.validate()? }
		Ok(())
	}
}


// SimpleIdentificationInformation4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SimpleIdentificationInformation4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
}

impl SimpleIdentificationInformation4 {
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


// SkipPayload ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SkipPayload {
}

impl SkipPayload {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// StatementFrequencyAndForm1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct StatementFrequencyAndForm1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy") )]
	pub frqcy: Frequency7Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ComMtd") )]
	pub com_mtd: CommunicationMethod2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryAdr") )]
	pub dlvry_adr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Frmt") )]
	pub frmt: CommunicationFormat1Choice,
}

impl StatementFrequencyAndForm1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.frqcy.validate()?;
		self.com_mtd.validate()?;
		if self.dlvry_adr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "dlvry_adr is shorter than the minimum length of 1".to_string()));
		}
		if self.dlvry_adr.chars().count() > 350 {
			return Err(ValidationError::new(1002, "dlvry_adr exceeds the maximum length of 350".to_string()));
		}
		self.frmt.validate()?;
		Ok(())
	}
}


// StatementFrequencyAndFormModification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct StatementFrequencyAndFormModification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModCd", skip_serializing_if = "Option::is_none") )]
	pub mod_cd: Option<Modification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StmtFrqcyAndForm") )]
	pub stmt_frqcy_and_form: StatementFrequencyAndForm1,
}

impl StatementFrequencyAndFormModification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mod_cd { val.validate()? }
		self.stmt_frqcy_and_form.validate()?;
		Ok(())
	}
}


// StatementFrequencyReason2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct StatementFrequencyReason2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<EventFrequency9Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl StatementFrequencyReason2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// Status25Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Status25Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
	pub sts: Option<AccountManagementStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rjctd", skip_serializing_if = "Option::is_none") )]
	pub rjctd: Option<Vec<RejectionReason31>>,
}

impl Status25Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sts { val.validate()? }
		if let Some(ref vec) = self.rjctd { for item in vec { item.validate()? } }
		Ok(())
	}
}


// StructuredRegulatoryReporting3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct StructuredRegulatoryReporting3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Inf", skip_serializing_if = "Option::is_none") )]
	pub inf: Option<Vec<String>>,
}

impl StructuredRegulatoryReporting3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 10 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 10".to_string()));
			}
		}
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref vec) = self.inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "inf exceeds the maximum length of 35".to_string()));
				}
			}
		}
		Ok(())
	}
}


// StructuredRemittanceInformation18 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct StructuredRemittanceInformation18 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RfrdDocInf", skip_serializing_if = "Option::is_none") )]
	pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation8>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RfrdDocAmt", skip_serializing_if = "Option::is_none") )]
	pub rfrd_doc_amt: Option<RemittanceAmount4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrRefInf", skip_serializing_if = "Option::is_none") )]
	pub cdtr_ref_inf: Option<CreditorReferenceInformation3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Invcr", skip_serializing_if = "Option::is_none") )]
	pub invcr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Invcee", skip_serializing_if = "Option::is_none") )]
	pub invcee: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRmt", skip_serializing_if = "Option::is_none") )]
	pub tax_rmt: Option<TaxData1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrnshmtRmt", skip_serializing_if = "Option::is_none") )]
	pub grnshmt_rmt: Option<Garnishment4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRmtInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_rmt_inf: Option<Vec<String>>,
}

impl StructuredRemittanceInformation18 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.rfrd_doc_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rfrd_doc_amt { val.validate()? }
		if let Some(ref val) = self.cdtr_ref_inf { val.validate()? }
		if let Some(ref val) = self.invcr { val.validate()? }
		if let Some(ref val) = self.invcee { val.validate()? }
		if let Some(ref val) = self.tax_rmt { val.validate()? }
		if let Some(ref val) = self.grnshmt_rmt { val.validate()? }
		if let Some(ref vec) = self.addtl_rmt_inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_rmt_inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 140 {
					return Err(ValidationError::new(1002, "addtl_rmt_inf exceeds the maximum length of 140".to_string()));
				}
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
		self.envlp.validate()?;
		Ok(())
	}
}


// SupplementaryDataEnvelope1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// TaxAmount3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TaxAmount3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxblBaseAmt", skip_serializing_if = "Option::is_none") )]
	pub taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none") )]
	pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dtls", skip_serializing_if = "Option::is_none") )]
	pub dtls: Option<Vec<TaxRecordDetails3>>,
}

impl TaxAmount3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.taxbl_base_amt { val.validate()? }
		if let Some(ref val) = self.ttl_amt { val.validate()? }
		if let Some(ref vec) = self.dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TaxAuthorisation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TaxAuthorisation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Titl", skip_serializing_if = "Option::is_none") )]
	pub titl: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
}

impl TaxAuthorisation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.titl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "titl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "titl exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// TaxData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TaxData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
	pub cdtr: Option<TaxParty1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
	pub dbtr: Option<TaxParty2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<TaxParty2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdmstnZone", skip_serializing_if = "Option::is_none") )]
	pub admstn_zone: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefNb", skip_serializing_if = "Option::is_none") )]
	pub ref_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mtd", skip_serializing_if = "Option::is_none") )]
	pub mtd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlTaxblBaseAmt", skip_serializing_if = "Option::is_none") )]
	pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlTaxAmt", skip_serializing_if = "Option::is_none") )]
	pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SeqNb", skip_serializing_if = "Option::is_none") )]
	pub seq_nb: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rcrd", skip_serializing_if = "Option::is_none") )]
	pub rcrd: Option<Vec<TaxRecord3>>,
}

impl TaxData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cdtr { val.validate()? }
		if let Some(ref val) = self.dbtr { val.validate()? }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.admstn_zone {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "admstn_zone is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "admstn_zone exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ref_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ref_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "ref_nb exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.mtd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mtd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mtd exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ttl_taxbl_base_amt { val.validate()? }
		if let Some(ref val) = self.ttl_tax_amt { val.validate()? }
		if let Some(ref vec) = self.rcrd { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TaxExemptReason3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum TaxExemptReason3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NONE") )]
	CodeNONE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MASA") )]
	CodeMASA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MISA") )]
	CodeMISA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SISA") )]
	CodeSISA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IISA") )]
	CodeIISA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CUYP") )]
	CodeCUYP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRYP") )]
	CodePRYP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ASTR") )]
	CodeASTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EMPY") )]
	CodeEMPY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EMCY") )]
	CodeEMCY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EPRY") )]
	CodeEPRY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ECYE") )]
	CodeECYE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NFPI") )]
	CodeNFPI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NFQP") )]
	CodeNFQP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DECP") )]
	CodeDECP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IRAC") )]
	CodeIRAC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IRAR") )]
	CodeIRAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KEOG") )]
	CodeKEOG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PFSP") )]
	CodePFSP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "401K") )]
	Code401K,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SIRA") )]
	CodeSIRA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "403B") )]
	Code403B,
	#[cfg_attr( feature = "derive_serde", serde(rename = "457X") )]
	Code457X,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RIRA") )]
	CodeRIRA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RIAN") )]
	CodeRIAN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RCRF") )]
	CodeRCRF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RCIP") )]
	CodeRCIP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EIFP") )]
	CodeEIFP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EIOP") )]
	CodeEIOP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FORE") )]
	CodeFORE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INCA") )]
	CodeINCA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MINO") )]
	CodeMINO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ASSO") )]
	CodeASSO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DIPL") )]
	CodeDIPL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DOME") )]
	CodeDOME,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FORP") )]
	CodeFORP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ORDR") )]
	CodeORDR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PENF") )]
	CodePENF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REFU") )]
	CodeREFU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RIHO") )]
	CodeRIHO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ADMI") )]
	CodeADMI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TANR") )]
	CodeTANR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OANR") )]
	CodeOANR,
}

impl TaxExemptReason3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TaxExemptionReason2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TaxExemptionReason2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<TaxExemptReason3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl TaxExemptionReason2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// TaxParty1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TaxParty1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxId", skip_serializing_if = "Option::is_none") )]
	pub tax_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnId", skip_serializing_if = "Option::is_none") )]
	pub regn_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxTp", skip_serializing_if = "Option::is_none") )]
	pub tax_tp: Option<String>,
}

impl TaxParty1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tax_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.regn_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "regn_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "regn_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tax_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_tp exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// TaxParty2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TaxParty2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxId", skip_serializing_if = "Option::is_none") )]
	pub tax_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnId", skip_serializing_if = "Option::is_none") )]
	pub regn_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxTp", skip_serializing_if = "Option::is_none") )]
	pub tax_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Authstn", skip_serializing_if = "Option::is_none") )]
	pub authstn: Option<TaxAuthorisation1>,
}

impl TaxParty2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tax_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.regn_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "regn_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "regn_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tax_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.authstn { val.validate()? }
		Ok(())
	}
}


// TaxPeriod3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TaxPeriod3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Yr", skip_serializing_if = "Option::is_none") )]
	pub yr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<TaxRecordPeriod1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDt", skip_serializing_if = "Option::is_none") )]
	pub fr_to_dt: Option<DatePeriod2>,
}

impl TaxPeriod3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.fr_to_dt { val.validate()? }
		Ok(())
	}
}


// TaxRateMarker1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum TaxRateMarker1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ALPR") )]
	CodeALPR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ALIT") )]
	CodeALIT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GRSS") )]
	CodeGRSS,
}

impl TaxRateMarker1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TaxRecord3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TaxRecord3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctgy", skip_serializing_if = "Option::is_none") )]
	pub ctgy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtgyDtls", skip_serializing_if = "Option::is_none") )]
	pub ctgy_dtls: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrSts", skip_serializing_if = "Option::is_none") )]
	pub dbtr_sts: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CertId", skip_serializing_if = "Option::is_none") )]
	pub cert_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none") )]
	pub frms_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
	pub prd: Option<TaxPeriod3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none") )]
	pub tax_amt: Option<TaxAmount3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl TaxRecord3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctgy {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctgy is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctgy exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctgy_dtls {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctgy_dtls is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctgy_dtls exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.dbtr_sts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dbtr_sts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "dbtr_sts exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.cert_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cert_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cert_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.frms_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "frms_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "frms_cd exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.prd { val.validate()? }
		if let Some(ref val) = self.tax_amt { val.validate()? }
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// TaxRecordDetails3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TaxRecordDetails3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
	pub prd: Option<TaxPeriod3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl TaxRecordDetails3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.prd { val.validate()? }
		self.amt.validate()?;
		Ok(())
	}
}


// TaxRecordPeriod1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum TaxRecordPeriod1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM01") )]
	CodeMM01,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM02") )]
	CodeMM02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM03") )]
	CodeMM03,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM04") )]
	CodeMM04,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM05") )]
	CodeMM05,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM06") )]
	CodeMM06,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM07") )]
	CodeMM07,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM08") )]
	CodeMM08,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM09") )]
	CodeMM09,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM10") )]
	CodeMM10,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM11") )]
	CodeMM11,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM12") )]
	CodeMM12,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QTR1") )]
	CodeQTR1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QTR2") )]
	CodeQTR2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QTR3") )]
	CodeQTR3,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QTR4") )]
	CodeQTR4,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HLF1") )]
	CodeHLF1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HLF2") )]
	CodeHLF2,
}

impl TaxRecordPeriod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TaxReporting3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TaxReporting3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxtnCtry") )]
	pub taxtn_ctry: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRate", skip_serializing_if = "Option::is_none") )]
	pub tax_rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxPyer", skip_serializing_if = "Option::is_none") )]
	pub tax_pyer: Option<PartyIdentification125Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRcpt", skip_serializing_if = "Option::is_none") )]
	pub tax_rcpt: Option<PartyIdentification125Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctDtls", skip_serializing_if = "Option::is_none") )]
	pub csh_acct_dtls: Option<CashAccount204>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
}

impl TaxReporting3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.taxtn_ctry) {
			return Err(ValidationError::new(1005, "taxtn_ctry does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.tax_pyer { val.validate()? }
		if let Some(ref val) = self.tax_rcpt { val.validate()? }
		if let Some(ref val) = self.csh_acct_dtls { val.validate()? }
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}


// TaxWithholdingMethod3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum TaxWithholdingMethod3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MITX") )]
	CodeMITX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INVE") )]
	CodeINVE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACCT") )]
	CodeACCT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EXMT") )]
	CodeEXMT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REPT") )]
	CodeREPT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRTF") )]
	CodeCRTF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WHCO") )]
	CodeWHCO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WTHD") )]
	CodeWTHD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WTRE") )]
	CodeWTRE,
}

impl TaxWithholdingMethod3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ThirdPartyRights2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ThirdPartyRights2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtTm", skip_serializing_if = "Option::is_none") )]
	pub dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Hldr", skip_serializing_if = "Option::is_none") )]
	pub hldr: Option<PartyIdentification125Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none") )]
	pub lgl_ntty_idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
}

impl ThirdPartyRights2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
		}
		if self.tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.hldr { val.validate()? }
		if let Some(ref val) = self.lgl_ntty_idr {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lgl_ntty_idr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}


// TradingNameModification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TradingNameModification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModCd", skip_serializing_if = "Option::is_none") )]
	pub mod_cd: Option<Modification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgNm") )]
	pub tradg_nm: String,
}

impl TradingNameModification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mod_cd { val.validate()? }
		if self.tradg_nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tradg_nm is shorter than the minimum length of 1".to_string()));
		}
		if self.tradg_nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "tradg_nm exceeds the maximum length of 350".to_string()));
		}
		Ok(())
	}
}


// TransactionChannel2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum TransactionChannel2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FIAD") )]
	CodeFIAD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HOBA") )]
	CodeHOBA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BRAN") )]
	CodeBRAN,
}

impl TransactionChannel2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TransactionChannelType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TransactionChannelType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<TransactionChannel2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl TransactionChannelType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// TransactionType5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TransactionType5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InvestmentFundTransactionType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl TransactionType5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// TransferInstruction1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TransferInstruction1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrfInd", skip_serializing_if = "Option::is_none") )]
	pub trf_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
	pub cd: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartDtTm", skip_serializing_if = "Option::is_none") )]
	pub start_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
	pub start_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
}

impl TransferInstruction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.cd.chars().count() < 1 {
			return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
		}
		if self.cd.chars().count() > 35 {
			return Err(ValidationError::new(1002, "cd exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 256".to_string()));
			}
		}
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}


// TreasuryProfile1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TreasuryProfile1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt") )]
	pub dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradrTp") )]
	pub tradr_tp: PartyRole5Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate") )]
	pub rate: f64,
}

impl TreasuryProfile1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tradr_tp.validate()?;
		Ok(())
	}
}


// TypeModification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TypeModification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModCd", skip_serializing_if = "Option::is_none") )]
	pub mod_cd: Option<Modification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: CashAccountType2Choice,
}

impl TypeModification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mod_cd { val.validate()? }
		self.tp.validate()?;
		Ok(())
	}
}


// UnitsOrAmount1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct UnitsOrAmount1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
	pub unit: Option<f64>,
}

impl UnitsOrAmount1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt { val.validate()? }
		Ok(())
	}
}


// UnitsOrAmountOrPercentage1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct UnitsOrAmountOrPercentage1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
	pub unit: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pctg", skip_serializing_if = "Option::is_none") )]
	pub pctg: Option<f64>,
}

impl UnitsOrAmountOrPercentage1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt { val.validate()? }
		Ok(())
	}
}


// UseCases1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum UseCases1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "OPEN") )]
	CodeOPEN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNTN") )]
	CodeMNTN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLSG") )]
	CodeCLSG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VIEW") )]
	CodeVIEW,
}

impl UseCases1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// VerificationReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct VerificationReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl VerificationReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// VerificationReport5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct VerificationReport5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlId") )]
	pub orgnl_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Vrfctn") )]
	pub vrfctn: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<VerificationReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlPtyAndAcctId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_pty_and_acct_id: Option<IdentificationInformation5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UpdtdPtyAndAcctId", skip_serializing_if = "Option::is_none") )]
	pub updtd_pty_and_acct_id: Option<IdentificationInformation5>,
}

impl VerificationReport5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.orgnl_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.rsn { val.validate()? }
		if let Some(ref val) = self.orgnl_pty_and_acct_id { val.validate()? }
		if let Some(ref val) = self.updtd_pty_and_acct_id { val.validate()? }
		Ok(())
	}
}