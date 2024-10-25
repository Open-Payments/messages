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


// BenchmarkCancellation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BenchmarkCancellation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
	pub tech_rcrd_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: SecurityIdentification19,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl BenchmarkCancellation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tech_rcrd_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// BenchmarkCreate1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BenchmarkCreate1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
	pub tech_rcrd_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: SecurityIdentification19,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr") )]
	pub othr: BenchmarkDetail1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Admstr") )]
	pub admstr: PartyIdentification136,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NdrsngPty", skip_serializing_if = "Option::is_none") )]
	pub ndrsng_pty: Option<PartyIdentification136>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
	pub sts: Option<StatusDetail1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechVldtyPrd", skip_serializing_if = "Option::is_none") )]
	pub tech_vldty_prd: Option<Period4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl BenchmarkCreate1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tech_rcrd_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Err(e) = self.othr.validate() { return Err(e); }
		if let Err(e) = self.admstr.validate() { return Err(e); }
		if let Some(ref val) = self.ndrsng_pty { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.sts { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tech_vldty_prd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// BenchmarkCurveName2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum BenchmarkCurveName2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "WIBO") )]
	CodeWIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TREA") )]
	CodeTREA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TIBO") )]
	CodeTIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TLBO") )]
	CodeTLBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SWAP") )]
	CodeSWAP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STBO") )]
	CodeSTBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRBO") )]
	CodePRBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PFAN") )]
	CodePFAN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NIBO") )]
	CodeNIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MAAA") )]
	CodeMAAA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MOSP") )]
	CodeMOSP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIBO") )]
	CodeLIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIBI") )]
	CodeLIBI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "JIBA") )]
	CodeJIBA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISDA") )]
	CodeISDA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GCFR") )]
	CodeGCFR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FUSW") )]
	CodeFUSW,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUCH") )]
	CodeEUCH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUUS") )]
	CodeEUUS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EURI") )]
	CodeEURI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EONS") )]
	CodeEONS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EONA") )]
	CodeEONA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CIBO") )]
	CodeCIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CDOR") )]
	CodeCDOR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BUBO") )]
	CodeBUBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BBSW") )]
	CodeBBSW,
}

impl BenchmarkCurveName2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// BenchmarkDetail1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BenchmarkDetail1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FullNm") )]
	pub full_nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Indx", skip_serializing_if = "Option::is_none") )]
	pub indx: Option<BenchmarkCurveName2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cmnt", skip_serializing_if = "Option::is_none") )]
	pub cmnt: Option<String>,
}

impl BenchmarkDetail1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.full_nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "full_nm is shorter than the minimum length of 1".to_string()));
		}
		if self.full_nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "full_nm exceeds the maximum length of 350".to_string()));
		}
		if let Some(ref val) = self.indx { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.cmnt {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cmnt is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 20000 {
				return Err(ValidationError::new(1002, "cmnt exceeds the maximum length of 20000".to_string()));
			}
		}
		Ok(())
	}
}


// BenchmarkReport1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BenchmarkReport1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cret", skip_serializing_if = "Option::is_none") )]
	pub cret: Option<BenchmarkCreate1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Upd", skip_serializing_if = "Option::is_none") )]
	pub upd: Option<BenchmarkUpdate1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cxl", skip_serializing_if = "Option::is_none") )]
	pub cxl: Option<BenchmarkCancellation1>,
}

impl BenchmarkReport1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cret { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.upd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.cxl { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// BenchmarkUpdate1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BenchmarkUpdate1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
	pub tech_rcrd_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: SecurityIdentification19,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsId", skip_serializing_if = "Option::is_none") )]
	pub prvs_id: Option<SecurityIdentification19>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr") )]
	pub othr: BenchmarkDetail1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Admstr") )]
	pub admstr: PartyIdentification136,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NdrsngPty", skip_serializing_if = "Option::is_none") )]
	pub ndrsng_pty: Option<PartyIdentification136>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
	pub sts: Option<StatusDetail1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechVldtyPrd", skip_serializing_if = "Option::is_none") )]
	pub tech_vldty_prd: Option<Period4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl BenchmarkUpdate1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tech_rcrd_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref val) = self.prvs_id { if let Err(e) = val.validate() { return Err(e); } }
		if let Err(e) = self.othr.validate() { return Err(e); }
		if let Err(e) = self.admstr.validate() { return Err(e); }
		if let Some(ref val) = self.ndrsng_pty { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.sts { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tech_vldty_prd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// FinancialBenchmarkReportV01 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialBenchmarkReportV01 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BchmkData") )]
	pub bchmk_data: Vec<BenchmarkReport1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl FinancialBenchmarkReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.bchmk_data { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// GenericIdentification36 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// NameAndAddress5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
		if let Some(ref val) = self.adr { if let Err(e) = val.validate() { return Err(e); } }
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
		if let Err(e) = self.tp.validate() { return Err(e); }
		Ok(())
	}
}


// PartyIdentification120Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification120Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
	pub prtry_id: Option<GenericIdentification36>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
	pub nm_and_adr: Option<NameAndAddress5>,
}

impl PartyIdentification120Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry_id { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.nm_and_adr { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyIdentification136 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification136 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: PartyIdentification120Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
}

impl PartyIdentification136 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// Period2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Period2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt") )]
	pub fr_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt") )]
	pub to_dt: String,
}

impl Period2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Period4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Period4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt", skip_serializing_if = "Option::is_none") )]
	pub fr_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt", skip_serializing_if = "Option::is_none") )]
	pub to_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none") )]
	pub fr_dt_to_dt: Option<Period2>,
}

impl Period4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fr_dt_to_dt { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// PostalAddress1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
		if let Some(ref val) = self.adr_tp { if let Err(e) = val.validate() { return Err(e); } }
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
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.othr_id { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// StatusDetail1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct StatusDetail1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CmptntAuthrty") )]
	pub cmptnt_authrty: SupervisingAuthorityIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
	pub sts: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsRsn") )]
	pub sts_rsn: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ActvtyPrd", skip_serializing_if = "Option::is_none") )]
	pub actvty_prd: Option<Period4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cmnt", skip_serializing_if = "Option::is_none") )]
	pub cmnt: Option<String>,
}

impl StatusDetail1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		if let Err(e) = self.cmptnt_authrty.validate() { return Err(e); }
		if let Some(ref val) = self.sts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 10 {
				return Err(ValidationError::new(1002, "sts exceeds the maximum length of 10".to_string()));
			}
		}
		if self.sts_rsn.chars().count() < 1 {
			return Err(ValidationError::new(1001, "sts_rsn is shorter than the minimum length of 1".to_string()));
		}
		if self.sts_rsn.chars().count() > 10 {
			return Err(ValidationError::new(1002, "sts_rsn exceeds the maximum length of 10".to_string()));
		}
		if let Some(ref val) = self.actvty_prd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.cmnt {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cmnt is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 20000 {
				return Err(ValidationError::new(1002, "cmnt exceeds the maximum length of 20000".to_string()));
			}
		}
		Ok(())
	}
}


// SupervisingAuthorityIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SupervisingAuthorityIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<SupervisingAuthorityIdentification1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
}

impl SupervisingAuthorityIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// SupervisingAuthorityIdentification1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SupervisingAuthorityIdentification1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
	pub prtry_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FullNm", skip_serializing_if = "Option::is_none") )]
	pub full_nm: Option<String>,
}

impl SupervisingAuthorityIdentification1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.prtry_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "prtry_id exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.full_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "full_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "full_nm exceeds the maximum length of 350".to_string()));
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
