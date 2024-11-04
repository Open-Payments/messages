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


// Contact9 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Contact9 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PhneNb") )]
	pub phne_nb: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr") )]
	pub email_adr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Fctn", skip_serializing_if = "Option::is_none") )]
	pub fctn: Option<String>,
}

impl Contact9 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 140 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
		}
		let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
		if !pattern.is_match(&self.phne_nb) {
			return Err(ValidationError::new(1005, "phne_nb does not match the required pattern".to_string()));
		}
		if self.email_adr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "email_adr is shorter than the minimum length of 1".to_string()));
		}
		if self.email_adr.chars().count() > 256 {
			return Err(ValidationError::new(1002, "email_adr exceeds the maximum length of 256".to_string()));
		}
		if let Some(ref val) = self.fctn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "fctn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "fctn exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// DatePeriod2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// IdentificationSource3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct IdentificationSource3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl IdentificationSource3Choice {
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


// OtherIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OtherIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sfx", skip_serializing_if = "Option::is_none") )]
	pub sfx: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: IdentificationSource3Choice,
}

impl OtherIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.sfx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sfx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "sfx exceeds the maximum length of 16".to_string()));
			}
		}
		self.tp.validate()?;
		Ok(())
	}
}


// ReportPeriodActivity1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ReportPeriodActivity1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOTX") )]
	CodeNOTX,
}

impl ReportPeriodActivity1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecuritiesSettlementSystemIdentification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesSettlementSystemIdentification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SysId") )]
	pub sys_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SysNm", skip_serializing_if = "Option::is_none") )]
	pub sys_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfJursdctn", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_jursdctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSDLglNm", skip_serializing_if = "Option::is_none") )]
	pub csd_lgl_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RspnsblPty", skip_serializing_if = "Option::is_none") )]
	pub rspnsbl_pty: Option<Vec<Contact9>>,
}

impl SecuritiesSettlementSystemIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.sys_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "sys_id is shorter than the minimum length of 1".to_string()));
		}
		if self.sys_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "sys_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.sys_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sys_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "sys_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.ctry_of_jursdctn {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_jursdctn does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.csd_lgl_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "csd_lgl_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "csd_lgl_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.rspnsbl_pty { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecurityIdentification19 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityIdentification19 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
}

impl SecurityIdentification19 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.isin {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.othr_id { for item in vec { item.validate()? } }
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


// SettlementDailyFailureReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementDailyFailureReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Data", skip_serializing_if = "Option::is_none") )]
	pub data: Option<SettlementDailyFailureReason3>,
}

impl SettlementDailyFailureReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.data_set_actn { val.validate()? }
		if let Some(ref val) = self.data { val.validate()? }
		Ok(())
	}
}


// SettlementDailyFailureReason3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementDailyFailureReason3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaildScties") )]
	pub faild_scties: SettlementTotalData1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaildCsh") )]
	pub faild_csh: SettlementTotalData1Choice,
}

impl SettlementDailyFailureReason3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.faild_scties.validate()?;
		self.faild_csh.validate()?;
		Ok(())
	}
}


// SettlementDataRate2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementDataRate2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Vol") )]
	pub vol: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: f64,
}

impl SettlementDataRate2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SettlementDataVolume2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementDataVolume2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Vol") )]
	pub vol: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: f64,
}

impl SettlementDataVolume2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SettlementFailsCurrency2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsCurrency2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Data") )]
	pub data: SettlementTotalData1,
}

impl SettlementFailsCurrency2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.ccy) {
			return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
		}
		self.data.validate()?;
		Ok(())
	}
}


// SettlementFailsDailyCSD1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsDailyCSD1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Data", skip_serializing_if = "Option::is_none") )]
	pub data: Option<SettlementFailsDailyCSD3>,
}

impl SettlementFailsDailyCSD1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.data_set_actn { val.validate()? }
		if let Some(ref val) = self.data { val.validate()? }
		Ok(())
	}
}


// SettlementFailsDailyCSD3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsDailyCSD3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraCSD") )]
	pub intra_csd: SettlementFailsDailyInstructionType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CrossCSD") )]
	pub cross_csd: SettlementFailsDailyInstructionType1Choice,
}

impl SettlementFailsDailyCSD3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.intra_csd.validate()?;
		self.cross_csd.validate()?;
		Ok(())
	}
}


// SettlementFailsDailyData3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsDailyData3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgDt") )]
	pub rptg_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DalyRcrd") )]
	pub daly_rcrd: SettlementFailsDailyInstrument3,
}

impl SettlementFailsDailyData3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.daly_rcrd.validate()?;
		Ok(())
	}
}


// SettlementFailsDailyInstructionType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsDailyInstructionType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Data", skip_serializing_if = "Option::is_none") )]
	pub data: Option<SettlementFailsDailyInstructionType3>,
}

impl SettlementFailsDailyInstructionType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.data_set_actn { val.validate()? }
		if let Some(ref val) = self.data { val.validate()? }
		Ok(())
	}
}


// SettlementFailsDailyInstructionType3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsDailyInstructionType3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryVrssPmt") )]
	pub dlvry_vrss_pmt: SettlementDailyFailureReason1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryWthPmt") )]
	pub dlvry_wth_pmt: SettlementDailyFailureReason1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtFreeOfDlvry") )]
	pub pmt_free_of_dlvry: SettlementDailyFailureReason1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FreeOfPmt") )]
	pub free_of_pmt: SettlementDailyFailureReason1Choice,
}

impl SettlementFailsDailyInstructionType3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.dlvry_vrss_pmt.validate()?;
		self.dlvry_wth_pmt.validate()?;
		self.pmt_free_of_dlvry.validate()?;
		self.free_of_pmt.validate()?;
		Ok(())
	}
}


// SettlementFailsDailyInstrument3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsDailyInstrument3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Eqty") )]
	pub eqty: SettlementFailsDailyTransactionType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvrgnDebt") )]
	pub svrgn_debt: SettlementFailsDailyTransactionType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Bd") )]
	pub bd: SettlementFailsDailyTransactionType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrTrfblScties") )]
	pub othr_trfbl_scties: SettlementFailsDailyTransactionType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XchgTraddFnds") )]
	pub xchg_tradd_fnds: SettlementFailsDailyTransactionType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CllctvInvstmtUdrtkgs") )]
	pub cllctv_invstmt_udrtkgs: SettlementFailsDailyTransactionType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MnyMktInstrm") )]
	pub mny_mkt_instrm: SettlementFailsDailyTransactionType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmssnAllwnc") )]
	pub emssn_allwnc: SettlementFailsDailyTransactionType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr") )]
	pub othr: SettlementFailsDailyTransactionType1Choice,
}

impl SettlementFailsDailyInstrument3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.eqty.validate()?;
		self.svrgn_debt.validate()?;
		self.bd.validate()?;
		self.othr_trfbl_scties.validate()?;
		self.xchg_tradd_fnds.validate()?;
		self.cllctv_invstmt_udrtkgs.validate()?;
		self.mny_mkt_instrm.validate()?;
		self.emssn_allwnc.validate()?;
		self.othr.validate()?;
		Ok(())
	}
}


// SettlementFailsDailyTransactionType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsDailyTransactionType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Data", skip_serializing_if = "Option::is_none") )]
	pub data: Option<SettlementFailsDailyTransactionType3>,
}

impl SettlementFailsDailyTransactionType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.data_set_actn { val.validate()? }
		if let Some(ref val) = self.data { val.validate()? }
		Ok(())
	}
}


// SettlementFailsDailyTransactionType3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsDailyTransactionType3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesBuyOrSell") )]
	pub scties_buy_or_sell: SettlementFailsDailyCSD1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollMgmtOpr") )]
	pub coll_mgmt_opr: SettlementFailsDailyCSD1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesLndgOrBrrwg") )]
	pub scties_lndg_or_brrwg: SettlementFailsDailyCSD1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RpAgrmt") )]
	pub rp_agrmt: SettlementFailsDailyCSD1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr") )]
	pub othr: SettlementFailsDailyCSD1Choice,
}

impl SettlementFailsDailyTransactionType3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.scties_buy_or_sell.validate()?;
		self.coll_mgmt_opr.validate()?;
		self.scties_lndg_or_brrwg.validate()?;
		self.rp_agrmt.validate()?;
		self.othr.validate()?;
		Ok(())
	}
}


// SettlementFailsData3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsData3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ttl") )]
	pub ttl: SettlementTotalData1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtcptInFail", skip_serializing_if = "Option::is_none") )]
	pub ptcpt_in_fail: Option<SettlementFailsParticipantRange1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FlsPerCcy", skip_serializing_if = "Option::is_none") )]
	pub fls_per_ccy: Option<Vec<SettlementFailsCurrency2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FlsPerFinInstrmTp", skip_serializing_if = "Option::is_none") )]
	pub fls_per_fin_instrm_tp: Option<SettlementFailsInstrument2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesInFail", skip_serializing_if = "Option::is_none") )]
	pub scties_in_fail: Option<SettlementFailsSecuritiesRange1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FlsPerTxTp", skip_serializing_if = "Option::is_none") )]
	pub fls_per_tx_tp: Option<SettlementFailsTransactionType2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlSttlmPnlties", skip_serializing_if = "Option::is_none") )]
	pub ttl_sttlm_pnlties: Option<SettlementDataVolume2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FailrRsn") )]
	pub failr_rsn: SettlementFailureReason3,
}

impl SettlementFailsData3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.ttl.validate()?;
		if let Some(ref val) = self.ptcpt_in_fail { val.validate()? }
		if let Some(ref vec) = self.fls_per_ccy { for item in vec { item.validate()? } }
		if let Some(ref val) = self.fls_per_fin_instrm_tp { val.validate()? }
		if let Some(ref val) = self.scties_in_fail { val.validate()? }
		if let Some(ref val) = self.fls_per_tx_tp { val.validate()? }
		if let Some(ref val) = self.ttl_sttlm_pnlties { val.validate()? }
		self.failr_rsn.validate()?;
		Ok(())
	}
}


// SettlementFailsInstrument2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsInstrument2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Eqty") )]
	pub eqty: SettlementTotalData1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvrgnDebt") )]
	pub svrgn_debt: SettlementTotalData1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Bd") )]
	pub bd: SettlementTotalData1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrTrfblScties") )]
	pub othr_trfbl_scties: SettlementTotalData1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XchgTraddFnds") )]
	pub xchg_tradd_fnds: SettlementTotalData1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CllctvInvstmtUdrtkgs") )]
	pub cllctv_invstmt_udrtkgs: SettlementTotalData1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MnyMktInstrm") )]
	pub mny_mkt_instrm: SettlementTotalData1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmssnAllwnc") )]
	pub emssn_allwnc: SettlementTotalData1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr") )]
	pub othr: SettlementTotalData1Choice,
}

impl SettlementFailsInstrument2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.eqty.validate()?;
		self.svrgn_debt.validate()?;
		self.bd.validate()?;
		self.othr_trfbl_scties.validate()?;
		self.xchg_tradd_fnds.validate()?;
		self.cllctv_invstmt_udrtkgs.validate()?;
		self.mny_mkt_instrm.validate()?;
		self.emssn_allwnc.validate()?;
		self.othr.validate()?;
		Ok(())
	}
}


// SettlementFailsMonthlyReportV01 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsMonthlyReportV01 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptHdr") )]
	pub rpt_hdr: SettlementFailsReportHeader2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MnthlyAggt") )]
	pub mnthly_aggt: SettlementFailsData3,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DalyData") )]
	pub daly_data: Vec<SettlementFailsDailyData3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SettlementFailsMonthlyReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rpt_hdr.validate()?;
		self.mnthly_aggt.validate()?;
		for item in &self.daly_data { item.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SettlementFailsParticipant1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsParticipant1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI") )]
	pub lei: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rank") )]
	pub rank: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Aggt") )]
	pub aggt: SettlementTotalData1,
}

impl SettlementFailsParticipant1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.lei) {
			return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
		}
		let pattern = Regex::new("[0-9]{1,2}").unwrap();
		if !pattern.is_match(&self.rank) {
			return Err(ValidationError::new(1005, "rank does not match the required pattern".to_string()));
		}
		self.aggt.validate()?;
		Ok(())
	}
}


// SettlementFailsParticipantRange1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsParticipantRange1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "HghstInVol") )]
	pub hghst_in_vol: Vec<SettlementFailsParticipant1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HghstInVal") )]
	pub hghst_in_val: Vec<SettlementFailsParticipant1>,
}

impl SettlementFailsParticipantRange1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.hghst_in_vol { item.validate()? }
		for item in &self.hghst_in_val { item.validate()? }
		Ok(())
	}
}


// SettlementFailsReportHeader2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsReportHeader2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgPrd") )]
	pub rptg_prd: DatePeriod2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptSts") )]
	pub rpt_sts: TransactionOperationType4Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesSttlmSys") )]
	pub scties_sttlm_sys: SecuritiesSettlementSystemIdentification2,
}

impl SettlementFailsReportHeader2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rptg_prd.validate()?;
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.ccy) {
			return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
		}
		self.rpt_sts.validate()?;
		self.scties_sttlm_sys.validate()?;
		Ok(())
	}
}


// SettlementFailsSecurities1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsSecurities1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId") )]
	pub fin_instrm_id: SecurityIdentification19,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rank") )]
	pub rank: String,
}

impl SettlementFailsSecurities1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.fin_instrm_id.validate()?;
		let pattern = Regex::new("[0-9]{1,2}").unwrap();
		if !pattern.is_match(&self.rank) {
			return Err(ValidationError::new(1005, "rank does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// SettlementFailsSecuritiesRange1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsSecuritiesRange1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "HghstInVol") )]
	pub hghst_in_vol: Vec<SettlementFailsSecurities1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HghstInVal") )]
	pub hghst_in_val: Vec<SettlementFailsSecurities1>,
}

impl SettlementFailsSecuritiesRange1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.hghst_in_vol { item.validate()? }
		for item in &self.hghst_in_val { item.validate()? }
		Ok(())
	}
}


// SettlementFailsTransactionType2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsTransactionType2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesBuyOrSell") )]
	pub scties_buy_or_sell: SettlementTotalData1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollMgmtOpr") )]
	pub coll_mgmt_opr: SettlementTotalData1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesLndgOrBrrwg") )]
	pub scties_lndg_or_brrwg: SettlementTotalData1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RpAgrmt") )]
	pub rp_agrmt: SettlementTotalData1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr") )]
	pub othr: SettlementTotalData1Choice,
}

impl SettlementFailsTransactionType2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.scties_buy_or_sell.validate()?;
		self.coll_mgmt_opr.validate()?;
		self.scties_lndg_or_brrwg.validate()?;
		self.rp_agrmt.validate()?;
		self.othr.validate()?;
		Ok(())
	}
}


// SettlementFailureReason2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailureReason2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MainRsns") )]
	pub main_rsns: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EffcncyImprvmt") )]
	pub effcncy_imprvmt: String,
}

impl SettlementFailureReason2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.main_rsns.chars().count() < 1 {
			return Err(ValidationError::new(1001, "main_rsns is shorter than the minimum length of 1".to_string()));
		}
		if self.main_rsns.chars().count() > 2048 {
			return Err(ValidationError::new(1002, "main_rsns exceeds the maximum length of 2048".to_string()));
		}
		if self.effcncy_imprvmt.chars().count() < 1 {
			return Err(ValidationError::new(1001, "effcncy_imprvmt is shorter than the minimum length of 1".to_string()));
		}
		if self.effcncy_imprvmt.chars().count() > 2048 {
			return Err(ValidationError::new(1002, "effcncy_imprvmt exceeds the maximum length of 2048".to_string()));
		}
		Ok(())
	}
}


// SettlementFailureReason3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailureReason3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AvrgDrtn", skip_serializing_if = "Option::is_none") )]
	pub avrg_drtn: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc") )]
	pub desc: Vec<SettlementFailureReason2>,
}

impl SettlementFailureReason3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.desc { item.validate()? }
		Ok(())
	}
}


// SettlementTotalData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementTotalData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sttld") )]
	pub sttld: SettlementDataVolume2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Faild") )]
	pub faild: SettlementDataVolume2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ttl") )]
	pub ttl: SettlementDataVolume2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaildRate") )]
	pub faild_rate: SettlementDataRate2,
}

impl SettlementTotalData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.sttld.validate()?;
		self.faild.validate()?;
		self.ttl.validate()?;
		self.faild_rate.validate()?;
		Ok(())
	}
}


// SettlementTotalData1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementTotalData1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Data", skip_serializing_if = "Option::is_none") )]
	pub data: Option<SettlementTotalData1>,
}

impl SettlementTotalData1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.data_set_actn { val.validate()? }
		if let Some(ref val) = self.data { val.validate()? }
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
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TransactionOperationType4Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TransactionOperationType4Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEWT") )]
	CodeNEWT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AMND") )]
	CodeAMND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CANC") )]
	CodeCANC,
}

impl TransactionOperationType4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
