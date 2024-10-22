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

pub mod iso20022 {
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
	
	
	// AnyMIC1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum AnyMIC1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ANYM") )]
		CodeANYM,
	}
	
	impl AnyMIC1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CollateralType6Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CollateralType6Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "GBBK") )]
		CodeGBBK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BOND") )]
		CodeBOND,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CASH") )]
		CodeCASH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COMM") )]
		CodeCOMM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INSU") )]
		CodeINSU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LCRE") )]
		CodeLCRE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PHYS") )]
		CodePHYS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SECU") )]
		CodeSECU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "STCF") )]
		CodeSTCF,
	}
	
	impl CollateralType6Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CorporateSectorCriteria5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CorporateSectorCriteria5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FISctr", skip_serializing_if = "Option::is_none") )]
		pub fi_sctr: Option<Vec<FinancialPartySectorType2Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NFISctr", skip_serializing_if = "Option::is_none") )]
		pub nfi_sctr: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
		pub not_rptd: Option<NotReported1Code>,
	}
	
	impl CorporateSectorCriteria5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref vec) = self.fi_sctr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.nfi_sctr {
				for item in vec {
					let pattern = Regex::new("[A-U]{1,1}").unwrap();
					if !pattern.is_match(&item) {
						return Err(ValidationError::new(1005, "nfi_sctr does not match the required pattern".to_string()));
					}
				}
			}
			if let Some(ref val) = self.not_rptd { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DateOrBlankQuery2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DateOrBlankQuery2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rg", skip_serializing_if = "Option::is_none") )]
		pub rg: Option<DatePeriod1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
		pub not_rptd: Option<NotReported1Code>,
	}
	
	impl DateOrBlankQuery2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.rg { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.not_rptd { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DatePeriod1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DatePeriod1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt", skip_serializing_if = "Option::is_none") )]
		pub fr_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt") )]
		pub to_dt: String,
	}
	
	impl DatePeriod1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// ExposureType10Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ExposureType10Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "SBSC") )]
		CodeSBSC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MGLD") )]
		CodeMGLD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SLEB") )]
		CodeSLEB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REPO") )]
		CodeREPO,
	}
	
	impl ExposureType10Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// FinancialPartySectorType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum FinancialPartySectorType2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "AIFD") )]
		CodeAIFD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CSDS") )]
		CodeCSDS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CCPS") )]
		CodeCCPS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CDTI") )]
		CodeCDTI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INUN") )]
		CodeINUN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ORPI") )]
		CodeORPI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVF") )]
		CodeINVF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REIN") )]
		CodeREIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UCIT") )]
		CodeUCIT,
	}
	
	impl FinancialPartySectorType2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Frequency14Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Frequency14Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
		CodeDAIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
		CodeWEEK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
		CodeMNTH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADHO") )]
		CodeADHO,
	}
	
	impl Frequency14Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// NotReported1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum NotReported1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NORP") )]
		CodeNORP,
	}
	
	impl NotReported1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Operation3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Operation3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ANDD") )]
		CodeANDD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ORRR") )]
		CodeORRR,
	}
	
	impl Operation3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PartyIdentification121Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PartyIdentification121Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none") )]
		pub lgl_ntty_idr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
		pub nm_and_adr: Option<NameAndAddress5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<GenericIdentification1>,
	}
	
	impl PartyIdentification121Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.any_bic {
				let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.lgl_ntty_idr {
				let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "lgl_ntty_idr does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.nm_and_adr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry_id { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyNatureType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PartyNatureType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NFIN") )]
		CodeNFIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FIIN") )]
		CodeFIIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CCPS") )]
		CodeCCPS,
	}
	
	impl PartyNatureType1Code {
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
	
	
	// SecuritiesFinancingReportingTransactionQueryV02 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecuritiesFinancingReportingTransactionQueryV02 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RqstngAuthrty") )]
		pub rqstng_authrty: PartyIdentification121Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradQryData") )]
		pub trad_qry_data: TradeReportQuery13Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl SecuritiesFinancingReportingTransactionQueryV02 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rqstng_authrty.validate() { return Err(e); }
			if let Err(e) = self.trad_qry_data.validate() { return Err(e); }
			if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// SecuritiesTradeVenueCriteria1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecuritiesTradeVenueCriteria1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MIC", skip_serializing_if = "Option::is_none") )]
		pub mic: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyMIC", skip_serializing_if = "Option::is_none") )]
		pub any_mic: Option<AnyMIC1Code>,
	}
	
	impl SecuritiesTradeVenueCriteria1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref vec) = self.mic {
				for item in vec {
					let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
					if !pattern.is_match(&item) {
						return Err(ValidationError::new(1005, "mic does not match the required pattern".to_string()));
					}
				}
			}
			if let Some(ref val) = self.any_mic { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// TradeAdditionalQueryCriteria7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeAdditionalQueryCriteria7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ActnTp", skip_serializing_if = "Option::is_none") )]
		pub actn_tp: Option<Vec<TransactionOperationType6Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ExctnVn", skip_serializing_if = "Option::is_none") )]
		pub exctn_vn: Option<SecuritiesTradeVenueCriteria1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtrOfCtrPty", skip_serializing_if = "Option::is_none") )]
		pub ntr_of_ctr_pty: Option<Vec<PartyNatureType1Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CorpSctr", skip_serializing_if = "Option::is_none") )]
		pub corp_sctr: Option<Vec<CorporateSectorCriteria5>>,
	}
	
	impl TradeAdditionalQueryCriteria7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref vec) = self.actn_tp { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.exctn_vn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.ntr_of_ctr_pty { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.corp_sctr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TradeDateTimeQueryCriteria2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeDateTimeQueryCriteria2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgDtTm", skip_serializing_if = "Option::is_none") )]
		pub rptg_dt_tm: Option<DateTimePeriod1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none") )]
		pub exctn_dt_tm: Option<DateTimePeriod1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none") )]
		pub mtrty_dt: Option<DateOrBlankQuery2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none") )]
		pub termntn_dt: Option<DateOrBlankQuery2Choice>,
	}
	
	impl TradeDateTimeQueryCriteria2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.rptg_dt_tm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.exctn_dt_tm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.mtrty_dt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.termntn_dt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradePartyIdentificationQuery8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradePartyIdentificationQuery8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClntId", skip_serializing_if = "Option::is_none") )]
		pub clnt_id: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
		pub not_rptd: Option<NotReported1Code>,
	}
	
	impl TradePartyIdentificationQuery8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref vec) = self.lei {
				for item in vec {
					let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
					if !pattern.is_match(&item) {
						return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
					}
				}
			}
			if let Some(ref vec) = self.any_bic {
				for item in vec {
					let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
					if !pattern.is_match(&item) {
						return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
					}
				}
			}
			if let Some(ref vec) = self.clnt_id {
				for item in vec {
					if item.chars().count() < 1 {
						return Err(ValidationError::new(1001, "clnt_id is shorter than the minimum length of 1".to_string()));
					}
					if item.chars().count() > 50 {
						return Err(ValidationError::new(1002, "clnt_id exceeds the maximum length of 50".to_string()));
					}
				}
			}
			if let Some(ref val) = self.not_rptd { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradePartyIdentificationQuery9 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradePartyIdentificationQuery9 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryCd", skip_serializing_if = "Option::is_none") )]
		pub ctry_cd: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClntId", skip_serializing_if = "Option::is_none") )]
		pub clnt_id: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
		pub not_rptd: Option<NotReported1Code>,
	}
	
	impl TradePartyIdentificationQuery9 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref vec) = self.lei {
				for item in vec {
					let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
					if !pattern.is_match(&item) {
						return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
					}
				}
			}
			if let Some(ref vec) = self.ctry_cd {
				for item in vec {
					let pattern = Regex::new("[A-Z]{2,2}").unwrap();
					if !pattern.is_match(&item) {
						return Err(ValidationError::new(1005, "ctry_cd does not match the required pattern".to_string()));
					}
				}
			}
			if let Some(ref vec) = self.any_bic {
				for item in vec {
					let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
					if !pattern.is_match(&item) {
						return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
					}
				}
			}
			if let Some(ref vec) = self.clnt_id {
				for item in vec {
					if item.chars().count() < 1 {
						return Err(ValidationError::new(1001, "clnt_id is shorter than the minimum length of 1".to_string()));
					}
					if item.chars().count() > 50 {
						return Err(ValidationError::new(1002, "clnt_id exceeds the maximum length of 50".to_string()));
					}
				}
			}
			if let Some(ref val) = self.not_rptd { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradePartyQueryCriteria5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradePartyQueryCriteria5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Oprtr") )]
		pub oprtr: Operation3Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none") )]
		pub rptg_ctr_pty: Option<TradePartyIdentificationQuery8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPtyBrnch", skip_serializing_if = "Option::is_none") )]
		pub rptg_ctr_pty_brnch: Option<TradePartyIdentificationQuery9>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none") )]
		pub othr_ctr_pty: Option<TradePartyIdentificationQuery8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPtyBrnch", skip_serializing_if = "Option::is_none") )]
		pub othr_ctr_pty_brnch: Option<TradePartyIdentificationQuery9>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none") )]
		pub bnfcry: Option<TradePartyIdentificationQuery8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubmitgAgt", skip_serializing_if = "Option::is_none") )]
		pub submitg_agt: Option<TradePartyIdentificationQuery8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Brkr", skip_serializing_if = "Option::is_none") )]
		pub brkr: Option<TradePartyIdentificationQuery8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CCP", skip_serializing_if = "Option::is_none") )]
		pub ccp: Option<TradePartyIdentificationQuery8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AgtLndr", skip_serializing_if = "Option::is_none") )]
		pub agt_lndr: Option<TradePartyIdentificationQuery8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none") )]
		pub trpty_agt: Option<TradePartyIdentificationQuery8>,
	}
	
	impl TradePartyQueryCriteria5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.oprtr.validate() { return Err(e); }
			if let Some(ref val) = self.rptg_ctr_pty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rptg_ctr_pty_brnch { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.othr_ctr_pty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.othr_ctr_pty_brnch { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.bnfcry { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.submitg_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.brkr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ccp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.agt_lndr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.trpty_agt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradeQueryCriteria10 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeQueryCriteria10 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradLifeCyclHstry") )]
		pub trad_life_cycl_hstry: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OutsdngTradInd") )]
		pub outsdng_trad_ind: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradPtyCrit", skip_serializing_if = "Option::is_none") )]
		pub trad_pty_crit: Option<TradePartyQueryCriteria5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradTpCrit", skip_serializing_if = "Option::is_none") )]
		pub trad_tp_crit: Option<TradeTypeQueryCriteria2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TmCrit", skip_serializing_if = "Option::is_none") )]
		pub tm_crit: Option<TradeDateTimeQueryCriteria2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCrit", skip_serializing_if = "Option::is_none") )]
		pub othr_crit: Option<TradeAdditionalQueryCriteria7>,
	}
	
	impl TradeQueryCriteria10 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.trad_pty_crit { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.trad_tp_crit { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.tm_crit { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.othr_crit { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradeQueryExecutionFrequency3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeQueryExecutionFrequency3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrqcyTp") )]
		pub frqcy_tp: Frequency14Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryDay", skip_serializing_if = "Option::is_none") )]
		pub dlvry_day: Option<Vec<WeekDay3Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DayOfMnth", skip_serializing_if = "Option::is_none") )]
		pub day_of_mnth: Option<Vec<f64>>,
	}
	
	impl TradeQueryExecutionFrequency3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.frqcy_tp.validate() { return Err(e); }
			if let Some(ref vec) = self.dlvry_day { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TradeRecurrentQuery5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeRecurrentQuery5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "QryTp") )]
		pub qry_tp: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy") )]
		pub frqcy: TradeQueryExecutionFrequency3,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VldUntil") )]
		pub vld_until: String,
	}
	
	impl TradeRecurrentQuery5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.qry_tp.chars().count() < 1 {
				return Err(ValidationError::new(1001, "qry_tp is shorter than the minimum length of 1".to_string()));
			}
			if self.qry_tp.chars().count() > 1000 {
				return Err(ValidationError::new(1002, "qry_tp exceeds the maximum length of 1000".to_string()));
			}
			if let Err(e) = self.frqcy.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// TradeReportQuery13Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeReportQuery13Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdHocQry", skip_serializing_if = "Option::is_none") )]
		pub ad_hoc_qry: Option<TradeQueryCriteria10>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RcrntQry", skip_serializing_if = "Option::is_none") )]
		pub rcrnt_qry: Option<TradeRecurrentQuery5>,
	}
	
	impl TradeReportQuery13Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.ad_hoc_qry { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rcrnt_qry { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradeTypeQueryCriteria2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeTypeQueryCriteria2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Oprtr") )]
		pub oprtr: Operation3Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesFincgTxTp", skip_serializing_if = "Option::is_none") )]
		pub scties_fincg_tx_tp: Option<Vec<ExposureType10Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CollCmpntTp", skip_serializing_if = "Option::is_none") )]
		pub coll_cmpnt_tp: Option<Vec<CollateralType6Code>>,
	}
	
	impl TradeTypeQueryCriteria2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.oprtr.validate() { return Err(e); }
			if let Some(ref vec) = self.scties_fincg_tx_tp { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.coll_cmpnt_tp { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TransactionOperationType6Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum TransactionOperationType6Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "REUU") )]
		CodeREUU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COLU") )]
		CodeCOLU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CORR") )]
		CodeCORR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ETRM") )]
		CodeETRM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VALU") )]
		CodeVALU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "POSC") )]
		CodePOSC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NEWT") )]
		CodeNEWT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MODI") )]
		CodeMODI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MARU") )]
		CodeMARU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EROR") )]
		CodeEROR,
	}
	
	impl TransactionOperationType6Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// WeekDay3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum WeekDay3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ALLD") )]
		CodeALLD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XBHL") )]
		CodeXBHL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IBHL") )]
		CodeIBHL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FRID") )]
		CodeFRID,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MOND") )]
		CodeMOND,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SATD") )]
		CodeSATD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SUND") )]
		CodeSUND,
		#[cfg_attr( feature = "derive_serde", serde(rename = "THUD") )]
		CodeTHUD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TUED") )]
		CodeTUED,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WEDD") )]
		CodeWEDD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WDAY") )]
		CodeWDAY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WEND") )]
		CodeWEND,
	}
	
	impl WeekDay3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
}