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
			if !pattern.is_match(&val) {
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
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.rspnsbl_pty { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SettlementDataRate1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementDataRate1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfInstrs", skip_serializing_if = "Option::is_none") )]
	pub nb_of_instrs: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValOfInstrs", skip_serializing_if = "Option::is_none") )]
	pub val_of_instrs: Option<f64>,
}

impl SettlementDataRate1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
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


// SettlementFailsAnnualReportV01 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsAnnualReportV01 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptHdr") )]
	pub rpt_hdr: SettlementFailsReportHeader2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnlAggt") )]
	pub anl_aggt: SettlementFailsData4,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SettlementFailsAnnualReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.rpt_hdr.validate() { return Err(e); }
		if let Err(e) = self.anl_aggt.validate() { return Err(e); }
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SettlementFailsData4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsData4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ttl") )]
	pub ttl: SettlementTotalData1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FailrRsn") )]
	pub failr_rsn: SettlementFailureReason3,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ElgblForDrgtn") )]
	pub elgbl_for_drgtn: SettlementFailsDerogation1,
}

impl SettlementFailsData4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ttl.validate() { return Err(e); }
		if let Err(e) = self.failr_rsn.validate() { return Err(e); }
		if let Err(e) = self.elgbl_for_drgtn.validate() { return Err(e); }
		Ok(())
	}
}


// SettlementFailsDerogation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsDerogation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ElgbltyInd") )]
	pub elgblty_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Justfn", skip_serializing_if = "Option::is_none") )]
	pub justfn: Option<SettlementFailsJustification1>,
}

impl SettlementFailsDerogation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.justfn { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// SettlementFailsJustification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementFailsJustification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate") )]
	pub rate: SettlementDataRate1Choice,
}

impl SettlementFailsJustification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.rate.validate() { return Err(e); }
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
		if let Err(e) = self.rptg_prd.validate() { return Err(e); }
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.ccy) {
			return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
		}
		if let Err(e) = self.rpt_sts.validate() { return Err(e); }
		if let Err(e) = self.scties_sttlm_sys.validate() { return Err(e); }
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
		for item in &self.desc { if let Err(e) = item.validate() { return Err(e); } }
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
		if let Err(e) = self.sttld.validate() { return Err(e); }
		if let Err(e) = self.faild.validate() { return Err(e); }
		if let Err(e) = self.ttl.validate() { return Err(e); }
		if let Err(e) = self.faild_rate.validate() { return Err(e); }
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
