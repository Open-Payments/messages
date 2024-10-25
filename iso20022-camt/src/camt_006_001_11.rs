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


// AccountIdentification4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "iban does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.othr { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// AccountSchemeName1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// ActiveCurrencyAndAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// AddressType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
pub struct AddressType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AddressType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl AddressType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// Amount2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Amount2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtWthtCcy", skip_serializing_if = "Option::is_none") )]
	pub amt_wtht_ccy: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtWthCcy", skip_serializing_if = "Option::is_none") )]
	pub amt_wth_ccy: Option<ActiveCurrencyAndAmount>,
}

impl Amount2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt_wtht_ccy {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "amt_wtht_ccy is less than the minimum value of 0.000000".to_string()));
			}
		}
		if let Some(ref val) = self.amt_wth_ccy { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// Amount3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Amount3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtWthCcy", skip_serializing_if = "Option::is_none") )]
	pub amt_wth_ccy: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtWthtCcy", skip_serializing_if = "Option::is_none") )]
	pub amt_wtht_ccy: Option<f64>,
}

impl Amount3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt_wth_ccy { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.amt_wtht_ccy {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "amt_wtht_ccy is less than the minimum value of 0.000000".to_string()));
			}
		}
		Ok(())
	}
}


// BranchAndFinancialInstitutionIdentification8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BranchAndFinancialInstitutionIdentification8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstnId") )]
	pub fin_instn_id: FinancialInstitutionIdentification23,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BrnchId", skip_serializing_if = "Option::is_none") )]
	pub brnch_id: Option<BranchData5>,
}

impl BranchAndFinancialInstitutionIdentification8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.fin_instn_id.validate() { return Err(e); }
		if let Some(ref val) = self.brnch_id { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// BranchData5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if !pattern.is_match(&val) {
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
		if let Some(ref val) = self.pstl_adr { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// CancelledStatusReason1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum CancelledStatusReason1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CANI") )]
	CodeCANI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CANS") )]
	CodeCANS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSUB") )]
	CodeCSUB,
}

impl CancelledStatusReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CashAccount43 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
		if let Some(ref val) = self.id { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&val) {
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
		if let Some(ref val) = self.prxy { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ownr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.svcr { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// CashAccountAndEntry5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CashAccountAndEntry5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Acct") )]
	pub acct: CashAccount43,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ntry", skip_serializing_if = "Option::is_none") )]
	pub ntry: Option<CashEntry2>,
}

impl CashAccountAndEntry5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.acct.validate() { return Err(e); }
		if let Some(ref val) = self.ntry { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// CashAccountType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// CashEntry2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CashEntry2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
	pub sts: Option<EntryStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StmtId", skip_serializing_if = "Option::is_none") )]
	pub stmt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr_ref: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlNtryInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_ntry_inf: Option<Vec<String>>,
}

impl CashEntry2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.dt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.sts { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.stmt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "stmt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "stmt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.addtl_ntry_inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_ntry_inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 140 {
					return Err(ValidationError::new(1002, "addtl_ntry_inf exceeds the maximum length of 140".to_string()));
				}
			}
		}
		Ok(())
	}
}


// ClearingSystemIdentification2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
pub struct ClearingSystemMemberIdentification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none") )]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MmbId") )]
	pub mmb_id: String,
}

impl ClearingSystemMemberIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.clr_sys_id { if let Err(e) = val.validate() { return Err(e); } }
		if self.mmb_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "mmb_id is shorter than the minimum length of 1".to_string()));
		}
		if self.mmb_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "mmb_id exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// Contact13 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
		if let Some(ref val) = self.nm_prfx { if let Err(e) = val.validate() { return Err(e); } }
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
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "phne_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.mob_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "mob_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.fax_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(&val) {
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
		if let Some(ref vec) = self.othr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.prefrd_mtd { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// CreditDebitCode ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// DateAndDateTime2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DateAndDateTime2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtTm", skip_serializing_if = "Option::is_none") )]
	pub dt_tm: Option<String>,
}

impl DateAndDateTime2Choice {
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


// DateTimePeriod1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DateTimePeriod1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtTm") )]
	pub fr_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDtTm") )]
	pub to_dt_tm: String,
}

impl DateTimePeriod1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateTimePeriod1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DateTimePeriod1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtTm", skip_serializing_if = "Option::is_none") )]
	pub fr_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none") )]
	pub to_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtTmRg", skip_serializing_if = "Option::is_none") )]
	pub dt_tm_rg: Option<DateTimePeriod1>,
}

impl DateTimePeriod1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dt_tm_rg { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// EntryStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum EntryStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOOK") )]
	CodeBOOK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PDNG") )]
	CodePDNG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FUTR") )]
	CodeFUTR,
}

impl EntryStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ErrorHandling3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ErrorHandling3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ErrorHandling3Choice {
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


// ErrorHandling5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ErrorHandling5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Err") )]
	pub err: ErrorHandling3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
}

impl ErrorHandling5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.err.validate() { return Err(e); }
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// FinalStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum FinalStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "STLD") )]
	CodeSTLD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RJTD") )]
	CodeRJTD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CAND") )]
	CodeCAND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FNLD") )]
	CodeFNLD,
}

impl FinalStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FinancialIdentificationSchemeName1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// FinancialInstitutionIdentification23 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "bicfi does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.clr_sys_mmb_id { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&val) {
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
		if let Some(ref val) = self.pstl_adr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.othr { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// GenericAccountIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
		if let Some(ref val) = self.schme_nm { if let Err(e) = val.validate() { return Err(e); } }
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
		if let Some(ref val) = self.schme_nm { if let Err(e) = val.validate() { return Err(e); } }
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


// GenericIdentification30 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// GenericOrganisationIdentification3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
		if let Some(ref val) = self.schme_nm { if let Err(e) = val.validate() { return Err(e); } }
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
		if let Some(ref val) = self.schme_nm { if let Err(e) = val.validate() { return Err(e); } }
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


// LongPaymentIdentification4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct LongPaymentIdentification4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
	pub tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
	pub uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmAmt") )]
	pub intr_bk_sttlm_amt: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt") )]
	pub intr_bk_sttlm_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none") )]
	pub pmt_mtd: Option<PaymentOrigin1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt") )]
	pub instg_agt: BranchAndFinancialInstitutionIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAgt") )]
	pub instd_agt: BranchAndFinancialInstitutionIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtryTp", skip_serializing_if = "Option::is_none") )]
	pub ntry_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none") )]
	pub end_to_end_id: Option<String>,
}

impl LongPaymentIdentification4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
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
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pmt_mtd { if let Err(e) = val.validate() { return Err(e); } }
		if let Err(e) = self.instg_agt.validate() { return Err(e); }
		if let Err(e) = self.instd_agt.validate() { return Err(e); }
		if let Some(ref val) = self.ntry_tp {
			let pattern = Regex::new("[BEOVW]{1,1}[0-9]{2,2}|DUM").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "ntry_tp does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// MarketInfrastructureIdentification1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MarketInfrastructureIdentification1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl MarketInfrastructureIdentification1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 3 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 3".to_string()));
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


// MessageHeader8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MessageHeader8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none") )]
	pub msg_pgntn: Option<Pagination1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBizQry", skip_serializing_if = "Option::is_none") )]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqTp", skip_serializing_if = "Option::is_none") )]
	pub req_tp: Option<RequestType4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QryNm", skip_serializing_if = "Option::is_none") )]
	pub qry_nm: Option<String>,
}

impl MessageHeader8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.msg_pgntn { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.orgnl_biz_qry { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.req_tp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.qry_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "qry_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "qry_nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// NamePrefix2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// NumberAndSumOfTransactions2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NumberAndSumOfTransactions2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfNtries", skip_serializing_if = "Option::is_none") )]
	pub nb_of_ntries: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sum", skip_serializing_if = "Option::is_none") )]
	pub sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNetNtryAmt", skip_serializing_if = "Option::is_none") )]
	pub ttl_net_ntry_amt: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
}

impl NumberAndSumOfTransactions2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nb_of_ntries {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "nb_of_ntries does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.cdt_dbt_ind { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// OrganisationIdentification39 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.othr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// OrganisationIdentificationSchemeName1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// OriginalBusinessQuery1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OriginalBusinessQuery1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none") )]
	pub msg_nm_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
}

impl OriginalBusinessQuery1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.msg_nm_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_nm_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_nm_id exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// OtherContact1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// Pagination1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Pagination1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PgNb") )]
	pub pg_nb: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LastPgInd") )]
	pub last_pg_ind: bool,
}

impl Pagination1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,5}").unwrap();
		if !pattern.is_match(&self.pg_nb) {
			return Err(ValidationError::new(1005, "pg_nb does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// Party50Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Party50Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty", skip_serializing_if = "Option::is_none") )]
	pub pty: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Agt", skip_serializing_if = "Option::is_none") )]
	pub agt: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl Party50Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pty { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.agt { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// Party52Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Party52Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgId", skip_serializing_if = "Option::is_none") )]
	pub org_id: Option<OrganisationIdentification39>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvtId", skip_serializing_if = "Option::is_none") )]
	pub prvt_id: Option<PersonIdentification18>,
}

impl Party52Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.org_id { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.prvt_id { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyIdentification272 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
		if let Some(ref val) = self.pstl_adr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.id { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ctry_of_res {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "ctry_of_res does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctct_dtls { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// PaymentCommon6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentCommon6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtFr", skip_serializing_if = "Option::is_none") )]
	pub pmt_fr: Option<System3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTo", skip_serializing_if = "Option::is_none") )]
	pub pmt_to: Option<System3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CmonSts", skip_serializing_if = "Option::is_none") )]
	pub cmon_sts: Option<Vec<PaymentStatus6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtryDt", skip_serializing_if = "Option::is_none") )]
	pub ntry_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none") )]
	pub pmt_mtd: Option<PaymentOrigin1Choice>,
}

impl PaymentCommon6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pmt_fr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.pmt_to { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.cmon_sts { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.reqd_exctn_dt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ntry_dt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.cdt_dbt_ind { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.pmt_mtd { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// PaymentIdentification8Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentIdentification8Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
	pub tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
	pub uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QId", skip_serializing_if = "Option::is_none") )]
	pub q_id: Option<QueueTransactionIdentification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LngBizId", skip_serializing_if = "Option::is_none") )]
	pub lng_biz_id: Option<LongPaymentIdentification4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtBizId", skip_serializing_if = "Option::is_none") )]
	pub shrt_biz_id: Option<ShortPaymentIdentification4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
	pub prtry_id: Option<String>,
}

impl PaymentIdentification8Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
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
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.q_id { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.lng_biz_id { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.shrt_biz_id { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.prtry_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "prtry_id exceeds the maximum length of 70".to_string()));
			}
		}
		Ok(())
	}
}


// PaymentInstruction47 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentInstruction47 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId", skip_serializing_if = "Option::is_none") )]
	pub msg_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
	pub sts: Option<Vec<PaymentStatus6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none") )]
	pub instd_amt: Option<Amount3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmAmt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_amt: Option<Amount2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none") )]
	pub pmt_mtd: Option<PaymentOrigin1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prty", skip_serializing_if = "Option::is_none") )]
	pub prty: Option<Priority1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgVldtyTm", skip_serializing_if = "Option::is_none") )]
	pub prcg_vldty_tm: Option<DateTimePeriod1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrCpy", skip_serializing_if = "Option::is_none") )]
	pub instr_cpy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<PaymentType4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GnrtdOrdr", skip_serializing_if = "Option::is_none") )]
	pub gnrtd_ordr: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
	pub tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none") )]
	pub end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pties", skip_serializing_if = "Option::is_none") )]
	pub pties: Option<PaymentTransactionParty4>,
}

impl PaymentInstruction47 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.msg_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.reqd_exctn_dt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.sts { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.instd_amt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.intr_bk_sttlm_amt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.purp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "purp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 10 {
				return Err(ValidationError::new(1002, "purp exceeds the maximum length of 10".to_string()));
			}
		}
		if let Some(ref val) = self.pmt_mtd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.prty { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.prcg_vldty_tm { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.instr_cpy {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_cpy is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 20000 {
				return Err(ValidationError::new(1002, "instr_cpy exceeds the maximum length of 20000".to_string()));
			}
		}
		if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.pties { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// PaymentInstrument1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PaymentInstrument1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BDT") )]
	CodeBDT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BCT") )]
	CodeBCT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CDT") )]
	CodeCDT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CCT") )]
	CodeCCT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHK") )]
	CodeCHK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BKT") )]
	CodeBKT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DCP") )]
	CodeDCP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CCP") )]
	CodeCCP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RTI") )]
	CodeRTI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CAN") )]
	CodeCAN,
}

impl PaymentInstrument1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PaymentOrigin1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentOrigin1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FINMT", skip_serializing_if = "Option::is_none") )]
	pub finmt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XMLMsgNm", skip_serializing_if = "Option::is_none") )]
	pub xml_msg_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Instrm", skip_serializing_if = "Option::is_none") )]
	pub instrm: Option<PaymentInstrument1Code>,
}

impl PaymentOrigin1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.finmt {
			let pattern = Regex::new("[0-9]{1,3}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "finmt does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.xml_msg_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "xml_msg_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "xml_msg_nm exceeds the maximum length of 35".to_string()));
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
		if let Some(ref val) = self.instrm { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// PaymentStatus6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentStatus6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<PaymentStatusCode6Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtTm", skip_serializing_if = "Option::is_none") )]
	pub dt_tm: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<Vec<PaymentStatusReason1Choice>>,
}

impl PaymentStatus6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.dt_tm { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// PaymentStatusCode6Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentStatusCode6Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pdg", skip_serializing_if = "Option::is_none") )]
	pub pdg: Option<PendingStatus4Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Fnl", skip_serializing_if = "Option::is_none") )]
	pub fnl: Option<FinalStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RTGS", skip_serializing_if = "Option::is_none") )]
	pub rtgs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sttlm", skip_serializing_if = "Option::is_none") )]
	pub sttlm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl PaymentStatusCode6Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pdg { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.fnl { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.rtgs {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rtgs is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "rtgs exceeds the maximum length of 4".to_string()));
			}
			let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "rtgs does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.sttlm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sttlm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "sttlm exceeds the maximum length of 4".to_string()));
			}
			let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "sttlm does not match the required pattern".to_string()));
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


// PaymentStatusReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentStatusReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Umtchd", skip_serializing_if = "Option::is_none") )]
	pub umtchd: Option<UnmatchedStatusReason1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Canc", skip_serializing_if = "Option::is_none") )]
	pub canc: Option<CancelledStatusReason1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sspd", skip_serializing_if = "Option::is_none") )]
	pub sspd: Option<SuspendedStatusReason1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdgFlngSttlm", skip_serializing_if = "Option::is_none") )]
	pub pdg_flng_sttlm: Option<PendingFailingSettlement1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdgSttlm", skip_serializing_if = "Option::is_none") )]
	pub pdg_sttlm: Option<PendingSettlement2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryRjctn", skip_serializing_if = "Option::is_none") )]
	pub prtry_rjctn: Option<ProprietaryStatusJustification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl PaymentStatusReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.umtchd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.canc { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.sspd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.pdg_flng_sttlm { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.pdg_sttlm { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.prtry_rjctn { if let Err(e) = val.validate() { return Err(e); } }
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


// PaymentTransactionParty4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentTransactionParty4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none") )]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none") )]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<Party50Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
	pub dbtr: Option<Party50Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgRmbrsmntAgt", skip_serializing_if = "Option::is_none") )]
	pub instg_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdRmbrsmntAgt", skip_serializing_if = "Option::is_none") )]
	pub instd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
	pub cdtr: Option<Party50Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<Party50Choice>,
}

impl PaymentTransactionParty4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instg_agt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.instd_agt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ultmt_dbtr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.dbtr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.dbtr_agt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.instg_rmbrsmnt_agt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.instd_rmbrsmnt_agt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.intrmy_agt1 { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.intrmy_agt2 { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.intrmy_agt3 { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.cdtr_agt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.cdtr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ultmt_cdtr { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// PaymentType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PaymentType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CBS") )]
	CodeCBS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BCK") )]
	CodeBCK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BAL") )]
	CodeBAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLS") )]
	CodeCLS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CTR") )]
	CodeCTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CBH") )]
	CodeCBH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CBP") )]
	CodeCBP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DPG") )]
	CodeDPG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DPN") )]
	CodeDPN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EXP") )]
	CodeEXP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TCH") )]
	CodeTCH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LMT") )]
	CodeLMT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIQ") )]
	CodeLIQ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DPP") )]
	CodeDPP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DPH") )]
	CodeDPH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DPS") )]
	CodeDPS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STF") )]
	CodeSTF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRP") )]
	CodeTRP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TCS") )]
	CodeTCS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LOA") )]
	CodeLOA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LOR") )]
	CodeLOR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TCP") )]
	CodeTCP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OND") )]
	CodeOND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MGL") )]
	CodeMGL,
}

impl PaymentType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PaymentType4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentType4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<PaymentType3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl PaymentType4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
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


// PendingFailingSettlement1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PendingFailingSettlement1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AWMO") )]
	CodeAWMO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AWSH") )]
	CodeAWSH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LAAW") )]
	CodeLAAW,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DOCY") )]
	CodeDOCY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLAT") )]
	CodeCLAT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CERT") )]
	CodeCERT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MINO") )]
	CodeMINO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PHSE") )]
	CodePHSE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SBLO") )]
	CodeSBLO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DKNY") )]
	CodeDKNY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STCD") )]
	CodeSTCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BENO") )]
	CodeBENO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LACK") )]
	CodeLACK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LATE") )]
	CodeLATE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CANR") )]
	CodeCANR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MLAT") )]
	CodeMLAT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OBJT") )]
	CodeOBJT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DOCC") )]
	CodeDOCC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BLOC") )]
	CodeBLOC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHAS") )]
	CodeCHAS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEWI") )]
	CodeNEWI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLAC") )]
	CodeCLAC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PART") )]
	CodePART,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CMON") )]
	CodeCMON,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COLL") )]
	CodeCOLL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DEPO") )]
	CodeDEPO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FLIM") )]
	CodeFLIM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOFX") )]
	CodeNOFX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INCA") )]
	CodeINCA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LINK") )]
	CodeLINK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BYIY") )]
	CodeBYIY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CAIS") )]
	CodeCAIS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LALO") )]
	CodeLALO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MONY") )]
	CodeMONY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NCON") )]
	CodeNCON,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YCOL") )]
	CodeYCOL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REFS") )]
	CodeREFS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SDUT") )]
	CodeSDUT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CYCL") )]
	CodeCYCL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BATC") )]
	CodeBATC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GUAD") )]
	CodeGUAD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PREA") )]
	CodePREA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GLOB") )]
	CodeGLOB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CPEC") )]
	CodeCPEC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MUNO") )]
	CodeMUNO,
}

impl PendingFailingSettlement1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PendingSettlement2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PendingSettlement2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AWMO") )]
	CodeAWMO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CAIS") )]
	CodeCAIS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REFU") )]
	CodeREFU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AWSH") )]
	CodeAWSH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PHSE") )]
	CodePHSE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TAMM") )]
	CodeTAMM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DOCY") )]
	CodeDOCY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DOCC") )]
	CodeDOCC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BLOC") )]
	CodeBLOC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHAS") )]
	CodeCHAS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEWI") )]
	CodeNEWI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLAC") )]
	CodeCLAC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MUNO") )]
	CodeMUNO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GLOB") )]
	CodeGLOB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PREA") )]
	CodePREA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GUAD") )]
	CodeGUAD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PART") )]
	CodePART,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NMAS") )]
	CodeNMAS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CMON") )]
	CodeCMON,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YCOL") )]
	CodeYCOL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COLL") )]
	CodeCOLL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DEPO") )]
	CodeDEPO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FLIM") )]
	CodeFLIM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOFX") )]
	CodeNOFX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INCA") )]
	CodeINCA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LINK") )]
	CodeLINK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FUTU") )]
	CodeFUTU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LACK") )]
	CodeLACK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LALO") )]
	CodeLALO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MONY") )]
	CodeMONY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NCON") )]
	CodeNCON,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REFS") )]
	CodeREFS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SDUT") )]
	CodeSDUT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BATC") )]
	CodeBATC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CYCL") )]
	CodeCYCL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SBLO") )]
	CodeSBLO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CPEC") )]
	CodeCPEC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MINO") )]
	CodeMINO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PCAP") )]
	CodePCAP,
}

impl PendingSettlement2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PendingStatus4Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PendingStatus4Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACPD") )]
	CodeACPD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VALD") )]
	CodeVALD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MATD") )]
	CodeMATD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AUTD") )]
	CodeAUTD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INVD") )]
	CodeINVD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UMAC") )]
	CodeUMAC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STLE") )]
	CodeSTLE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STLM") )]
	CodeSTLM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SSPD") )]
	CodeSSPD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PCAN") )]
	CodePCAN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PSTL") )]
	CodePSTL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PFST") )]
	CodePFST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SMLR") )]
	CodeSMLR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RMLR") )]
	CodeRMLR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SRBL") )]
	CodeSRBL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AVLB") )]
	CodeAVLB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SRML") )]
	CodeSRML,
}

impl PendingStatus4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PersonIdentification18 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PersonIdentification18 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none") )]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<GenericPersonIdentification2>>,
}

impl PersonIdentification18 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dt_and_plc_of_birth { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.othr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// PersonIdentificationSchemeName1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// PostalAddress27 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
		if let Some(ref val) = self.adr_tp { if let Err(e) = val.validate() { return Err(e); } }
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
			if !pattern.is_match(&val) {
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


// Priority1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Priority1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Priority5Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl Priority1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
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


// Priority5Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum Priority5Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "HIGH") )]
	CodeHIGH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LOWW") )]
	CodeLOWW,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NORM") )]
	CodeNORM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "URGT") )]
	CodeURGT,
}

impl Priority5Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ProprietaryStatusJustification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProprietaryStatusJustification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryStsRsn") )]
	pub prtry_sts_rsn: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: String,
}

impl ProprietaryStatusJustification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.prtry_sts_rsn.chars().count() < 1 {
			return Err(ValidationError::new(1001, "prtry_sts_rsn is shorter than the minimum length of 1".to_string()));
		}
		if self.prtry_sts_rsn.chars().count() > 4 {
			return Err(ValidationError::new(1002, "prtry_sts_rsn exceeds the maximum length of 4".to_string()));
		}
		let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
		if !pattern.is_match(&self.prtry_sts_rsn) {
			return Err(ValidationError::new(1005, "prtry_sts_rsn does not match the required pattern".to_string()));
		}
		if self.rsn.chars().count() < 1 {
			return Err(ValidationError::new(1001, "rsn is shorter than the minimum length of 1".to_string()));
		}
		if self.rsn.chars().count() > 256 {
			return Err(ValidationError::new(1002, "rsn exceeds the maximum length of 256".to_string()));
		}
		Ok(())
	}
}


// ProxyAccountIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProxyAccountIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<ProxyAccountType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
}

impl ProxyAccountIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
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


// QueueTransactionIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct QueueTransactionIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "QId") )]
	pub q_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PosInQ") )]
	pub pos_in_q: String,
}

impl QueueTransactionIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.q_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "q_id is shorter than the minimum length of 1".to_string()));
		}
		if self.q_id.chars().count() > 16 {
			return Err(ValidationError::new(1002, "q_id exceeds the maximum length of 16".to_string()));
		}
		if self.pos_in_q.chars().count() < 1 {
			return Err(ValidationError::new(1001, "pos_in_q is shorter than the minimum length of 1".to_string()));
		}
		if self.pos_in_q.chars().count() > 16 {
			return Err(ValidationError::new(1002, "pos_in_q exceeds the maximum length of 16".to_string()));
		}
		Ok(())
	}
}


// RequestType4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RequestType4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCtrl", skip_serializing_if = "Option::is_none") )]
	pub pmt_ctrl: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Enqry", skip_serializing_if = "Option::is_none") )]
	pub enqry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification1>,
}

impl RequestType4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pmt_ctrl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pmt_ctrl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "pmt_ctrl exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.enqry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "enqry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "enqry exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// ReturnTransactionV11 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReturnTransactionV11 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgHdr") )]
	pub msg_hdr: MessageHeader8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptOrErr") )]
	pub rpt_or_err: TransactionReportOrError7Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl ReturnTransactionV11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_hdr.validate() { return Err(e); }
		if let Err(e) = self.rpt_or_err.validate() { return Err(e); }
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SecuritiesTransactionReferences1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionReferences1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOwnrTxId", skip_serializing_if = "Option::is_none") )]
	pub acct_ownr_tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr_tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none") )]
	pub mkt_infrstrctr_tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgId", skip_serializing_if = "Option::is_none") )]
	pub prcg_id: Option<String>,
}

impl SecuritiesTransactionReferences1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.acct_ownr_tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_ownr_tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_ownr_tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.acct_svcr_tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_svcr_tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_svcr_tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.mkt_infrstrctr_tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mkt_infrstrctr_tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mkt_infrstrctr_tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.prcg_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prcg_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prcg_id exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ShortPaymentIdentification4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ShortPaymentIdentification4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
	pub tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
	pub uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt") )]
	pub intr_bk_sttlm_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt") )]
	pub instg_agt: BranchAndFinancialInstitutionIdentification8,
}

impl ShortPaymentIdentification4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
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
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
			}
		}
		if let Err(e) = self.instg_agt.validate() { return Err(e); }
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


// SuspendedStatusReason1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum SuspendedStatusReason1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SUBY") )]
	CodeSUBY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SUBS") )]
	CodeSUBS,
}

impl SuspendedStatusReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// System3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct System3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SysId", skip_serializing_if = "Option::is_none") )]
	pub sys_id: Option<MarketInfrastructureIdentification1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MmbId", skip_serializing_if = "Option::is_none") )]
	pub mmb_id: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctId", skip_serializing_if = "Option::is_none") )]
	pub acct_id: Option<AccountIdentification4Choice>,
}

impl System3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sys_id { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.mmb_id { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.acct_id { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// Transaction159 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Transaction159 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTo", skip_serializing_if = "Option::is_none") )]
	pub pmt_to: Option<System3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtFr", skip_serializing_if = "Option::is_none") )]
	pub pmt_fr: Option<System3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pmt", skip_serializing_if = "Option::is_none") )]
	pub pmt: Option<PaymentInstruction47>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctNtry", skip_serializing_if = "Option::is_none") )]
	pub acct_ntry: Option<CashAccountAndEntry5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesTxRefs", skip_serializing_if = "Option::is_none") )]
	pub scties_tx_refs: Option<SecuritiesTransactionReferences1>,
}

impl Transaction159 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pmt_to { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.pmt_fr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.cdt_dbt_ind { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.pmt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.acct_ntry { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.scties_tx_refs { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// TransactionOrError6Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransactionOrError6Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tx", skip_serializing_if = "Option::is_none") )]
	pub tx: Option<Transaction159>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizErr", skip_serializing_if = "Option::is_none") )]
	pub biz_err: Option<Vec<ErrorHandling5>>,
}

impl TransactionOrError6Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tx { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.biz_err { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TransactionReport8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransactionReport8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtId") )]
	pub pmt_id: PaymentIdentification8Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxOrErr") )]
	pub tx_or_err: TransactionOrError6Choice,
}

impl TransactionReport8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pmt_id.validate() { return Err(e); }
		if let Err(e) = self.tx_or_err.validate() { return Err(e); }
		Ok(())
	}
}


// TransactionReportOrError7Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransactionReportOrError7Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizRpt", skip_serializing_if = "Option::is_none") )]
	pub biz_rpt: Option<Transactions11>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OprlErr", skip_serializing_if = "Option::is_none") )]
	pub oprl_err: Option<Vec<ErrorHandling5>>,
}

impl TransactionReportOrError7Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.biz_rpt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.oprl_err { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// Transactions11 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Transactions11 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCmonInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_cmon_inf: Option<PaymentCommon6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxsSummry", skip_serializing_if = "Option::is_none") )]
	pub txs_summry: Option<NumberAndSumOfTransactions2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxRpt") )]
	pub tx_rpt: Vec<TransactionReport8>,
}

impl Transactions11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pmt_cmon_inf { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.txs_summry { if let Err(e) = val.validate() { return Err(e); } }
		for item in &self.tx_rpt { if let Err(e) = item.validate() { return Err(e); } }
		Ok(())
	}
}


// UnmatchedStatusReason1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum UnmatchedStatusReason1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CMIS") )]
	CodeCMIS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DDAT") )]
	CodeDDAT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DELN") )]
	CodeDELN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DEPT") )]
	CodeDEPT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DMON") )]
	CodeDMON,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DDEA") )]
	CodeDDEA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DQUA") )]
	CodeDQUA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CADE") )]
	CodeCADE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SETR") )]
	CodeSETR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DSEC") )]
	CodeDSEC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VASU") )]
	CodeVASU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DTRA") )]
	CodeDTRA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RSPR") )]
	CodeRSPR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REPO") )]
	CodeREPO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLAT") )]
	CodeCLAT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RERT") )]
	CodeRERT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REPA") )]
	CodeREPA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REPP") )]
	CodeREPP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PHYS") )]
	CodePHYS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IIND") )]
	CodeIIND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FRAP") )]
	CodeFRAP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PLCE") )]
	CodePLCE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PODU") )]
	CodePODU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FORF") )]
	CodeFORF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REGD") )]
	CodeREGD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RTGS") )]
	CodeRTGS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ICAG") )]
	CodeICAG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CPCA") )]
	CodeCPCA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHAR") )]
	CodeCHAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IEXE") )]
	CodeIEXE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NCRR") )]
	CodeNCRR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NMAS") )]
	CodeNMAS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SAFE") )]
	CodeSAFE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DTRD") )]
	CodeDTRD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LATE") )]
	CodeLATE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TERM") )]
	CodeTERM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ICUS") )]
	CodeICUS,
}

impl UnmatchedStatusReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
