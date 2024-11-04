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


// BasketQuery1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BasketQuery1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Strr", skip_serializing_if = "Option::is_none") )]
	pub strr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Idr", skip_serializing_if = "Option::is_none") )]
	pub idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<String>,
}

impl BasketQuery1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.strr {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "strr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.idr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "idr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "idr exceeds the maximum length of 52".to_string()));
			}
		}
		if let Some(ref val) = self.isin {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// CorporateSectorCriteria6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CorporateSectorCriteria6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FISctr", skip_serializing_if = "Option::is_none") )]
	pub fi_sctr: Option<Vec<FinancialPartySectorType2Code>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NFISctr", skip_serializing_if = "Option::is_none") )]
	pub nfi_sctr: Option<Vec<NonFinancialPartySector1Code>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
	pub not_rptd: Option<NotReported1Code>,
}

impl CorporateSectorCriteria6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.fi_sctr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.nfi_sctr { for item in vec { item.validate()? } }
		if let Some(ref val) = self.not_rptd { val.validate()? }
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
		if let Some(ref val) = self.rg { val.validate()? }
		if let Some(ref val) = self.not_rptd { val.validate()? }
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


// DateTimeOrBlankQuery1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DateTimeOrBlankQuery1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rg", skip_serializing_if = "Option::is_none") )]
	pub rg: Option<DateTimePeriod1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
	pub not_rptd: Option<NotReported1Code>,
}

impl DateTimeOrBlankQuery1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rg { val.validate()? }
		if let Some(ref val) = self.not_rptd { val.validate()? }
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


// DerivativeEventType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum DerivativeEventType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ALOC") )]
	CodeALOC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLRG") )]
	CodeCLRG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLAL") )]
	CodeCLAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COMP") )]
	CodeCOMP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CORP") )]
	CodeCORP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CREV") )]
	CodeCREV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ETRM") )]
	CodeETRM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EXER") )]
	CodeEXER,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INCP") )]
	CodeINCP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOVA") )]
	CodeNOVA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PTNG") )]
	CodePTNG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRAD") )]
	CodeTRAD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UPDT") )]
	CodeUPDT,
}

impl DerivativeEventType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DerivativesTradeReportQueryV05 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DerivativesTradeReportQueryV05 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RqstngAuthrty") )]
	pub rqstng_authrty: PartyIdentification121Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradQryData") )]
	pub trad_qry_data: TradeReportQuery18Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl DerivativesTradeReportQueryV05 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rqstng_authrty.validate()?;
		self.trad_qry_data.validate()?;
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// FinancialInstrumentContractType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum FinancialInstrumentContractType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CFDS") )]
	CodeCFDS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FRAS") )]
	CodeFRAS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FUTR") )]
	CodeFUTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FORW") )]
	CodeFORW,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OPTN") )]
	CodeOPTN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPDB") )]
	CodeSPDB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SWAP") )]
	CodeSWAP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SWPT") )]
	CodeSWPT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
}

impl FinancialInstrumentContractType2Code {
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


// GenericIdentification175 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification175 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericIdentification175 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 72 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 72".to_string()));
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


// ISINQueryCriteria1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ISINQueryCriteria1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Idr", skip_serializing_if = "Option::is_none") )]
	pub idr: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
	pub not_rptd: Option<NotReported1Code>,
}

impl ISINQueryCriteria1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.idr {
			for item in vec {
				let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "idr does not match the required pattern".to_string()));
				}
			}
		}
		if let Some(ref val) = self.not_rptd { val.validate()? }
		Ok(())
	}
}


// LegalPersonIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct LegalPersonIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: OrganisationIdentification15Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
}

impl LegalPersonIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// ModificationLevel1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ModificationLevel1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PSTN") )]
	CodePSTN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TCTN") )]
	CodeTCTN,
}

impl ModificationLevel1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
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
		if let Some(ref val) = self.adr { val.validate()? }
		Ok(())
	}
}


// NaturalPersonIdentification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NaturalPersonIdentification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: GenericIdentification175,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dmcl", skip_serializing_if = "Option::is_none") )]
	pub dmcl: Option<String>,
}

impl NaturalPersonIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 105 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 105".to_string()));
			}
		}
		if let Some(ref val) = self.dmcl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dmcl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 500 {
				return Err(ValidationError::new(1002, "dmcl exceeds the maximum length of 500".to_string()));
			}
		}
		Ok(())
	}
}


// NaturalPersonIdentification3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NaturalPersonIdentification3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: NaturalPersonIdentification2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
}

impl NaturalPersonIdentification3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// NonFinancialPartySector1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum NonFinancialPartySector1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "WTER") )]
	CodeWTER,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MING") )]
	CodeMING,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MAFG") )]
	CodeMAFG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPLY") )]
	CodeSPLY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSTR") )]
	CodeCSTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AGRI") )]
	CodeAGRI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACAF") )]
	CodeACAF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EDUC") )]
	CodeEDUC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AEAR") )]
	CodeAEAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FINA") )]
	CodeFINA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HHSW") )]
	CodeHHSW,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INCO") )]
	CodeINCO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WRRM") )]
	CodeWRRM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTSA") )]
	CodeOTSA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PSTA") )]
	CodePSTA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PADS") )]
	CodePADS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RESA") )]
	CodeRESA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRAS") )]
	CodeTRAS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ASSA") )]
	CodeASSA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AHAE") )]
	CodeAHAE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AEOB") )]
	CodeAEOB,
}

impl NonFinancialPartySector1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// NotAvailable1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum NotAvailable1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NTAV") )]
	CodeNTAV,
}

impl NotAvailable1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
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


// OrganisationIdentification15Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrganisationIdentification15Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<OrganisationIdentification38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
}

impl OrganisationIdentification15Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.othr { val.validate()? }
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// OrganisationIdentification38 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrganisationIdentification38 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: GenericIdentification175,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dmcl", skip_serializing_if = "Option::is_none") )]
	pub dmcl: Option<String>,
}

impl OrganisationIdentification38 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 105 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 105".to_string()));
			}
		}
		if let Some(ref val) = self.dmcl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dmcl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 500 {
				return Err(ValidationError::new(1002, "dmcl exceeds the maximum length of 500".to_string()));
			}
		}
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
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.lgl_ntty_idr {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lgl_ntty_idr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm_and_adr { val.validate()? }
		if let Some(ref val) = self.prtry_id { val.validate()? }
		Ok(())
	}
}


// PartyIdentification248Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification248Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Lgl", skip_serializing_if = "Option::is_none") )]
	pub lgl: Option<LegalPersonIdentification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ntrl", skip_serializing_if = "Option::is_none") )]
	pub ntrl: Option<NaturalPersonIdentification3>,
}

impl PartyIdentification248Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.lgl { val.validate()? }
		if let Some(ref val) = self.ntrl { val.validate()? }
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


// ProductClassificationCriteria1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProductClassificationCriteria1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none") )]
	pub clssfctn_fin_instrm: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none") )]
	pub unq_pdct_idr: Option<Vec<String>>,
}

impl ProductClassificationCriteria1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.clssfctn_fin_instrm {
			for item in vec {
				let pattern = Regex::new("[A-Z]{6,6}").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "clssfctn_fin_instrm does not match the required pattern".to_string()));
				}
			}
		}
		if let Some(ref vec) = self.unq_pdct_idr {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "unq_pdct_idr is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 52 {
					return Err(ValidationError::new(1002, "unq_pdct_idr exceeds the maximum length of 52".to_string()));
				}
			}
		}
		Ok(())
	}
}


// ProductType4Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ProductType4Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRDT") )]
	CodeCRDT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CURR") )]
	CodeCURR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EQUI") )]
	CodeEQUI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INTR") )]
	CodeINTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COMM") )]
	CodeCOMM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
}

impl ProductType4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
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
		if let Some(ref val) = self.any_mic { val.validate()? }
		Ok(())
	}
}


// SecurityIdentification20Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityIdentification20Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
}

impl SecurityIdentification20Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.isin {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 25 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 25".to_string()));
			}
		}
		Ok(())
	}
}


// SecurityIdentificationQuery4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityIdentificationQuery4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none") )]
	pub altrntv_instrm_id: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NotAvlbl", skip_serializing_if = "Option::is_none") )]
	pub not_avlbl: Option<NotAvailable1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none") )]
	pub unq_pdct_idr: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Indx", skip_serializing_if = "Option::is_none") )]
	pub indx: Option<Vec<SecurityIdentification20Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Bskt", skip_serializing_if = "Option::is_none") )]
	pub bskt: Option<Vec<BasketQuery1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
	pub not_rptd: Option<NotReported1Code>,
}

impl SecurityIdentificationQuery4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.isin {
			for item in vec {
				let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
				}
			}
		}
		if let Some(ref vec) = self.altrntv_instrm_id {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "altrntv_instrm_id is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 52 {
					return Err(ValidationError::new(1002, "altrntv_instrm_id exceeds the maximum length of 52".to_string()));
				}
			}
		}
		if let Some(ref val) = self.not_avlbl { val.validate()? }
		if let Some(ref vec) = self.unq_pdct_idr {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "unq_pdct_idr is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 52 {
					return Err(ValidationError::new(1002, "unq_pdct_idr exceeds the maximum length of 52".to_string()));
				}
			}
		}
		if let Some(ref vec) = self.indx { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.bskt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.not_rptd { val.validate()? }
		Ok(())
	}
}


// SecurityIdentificationQueryCriteria1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityIdentificationQueryCriteria1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none") )]
	pub altrntv_instrm_id: Option<Vec<String>>,
}

impl SecurityIdentificationQueryCriteria1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.isin {
			for item in vec {
				let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
				}
			}
		}
		if let Some(ref vec) = self.altrntv_instrm_id {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "altrntv_instrm_id is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 52 {
					return Err(ValidationError::new(1002, "altrntv_instrm_id exceeds the maximum length of 52".to_string()));
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


// TradeAdditionalQueryCriteria9 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradeAdditionalQueryCriteria9 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ActnTp", skip_serializing_if = "Option::is_none") )]
	pub actn_tp: Option<Vec<TransactionOperationType8Code>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExctnVn", skip_serializing_if = "Option::is_none") )]
	pub exctn_vn: Option<SecuritiesTradeVenueCriteria1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtrOfCtrPty", skip_serializing_if = "Option::is_none") )]
	pub ntr_of_ctr_pty: Option<PartyNatureType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CorpSctr", skip_serializing_if = "Option::is_none") )]
	pub corp_sctr: Option<CorporateSectorCriteria6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AsstClss", skip_serializing_if = "Option::is_none") )]
	pub asst_clss: Option<Vec<ProductType4Code>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdctClssfctn", skip_serializing_if = "Option::is_none") )]
	pub pdct_clssfctn: Option<ProductClassificationCriteria1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Lvl", skip_serializing_if = "Option::is_none") )]
	pub lvl: Option<ModificationLevel1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtTp", skip_serializing_if = "Option::is_none") )]
	pub evt_tp: Option<Vec<DerivativeEventType3Code>>,
}

impl TradeAdditionalQueryCriteria9 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.actn_tp { for item in vec { item.validate()? } }
		if let Some(ref val) = self.exctn_vn { val.validate()? }
		if let Some(ref val) = self.ntr_of_ctr_pty { val.validate()? }
		if let Some(ref val) = self.corp_sctr { val.validate()? }
		if let Some(ref vec) = self.asst_clss { for item in vec { item.validate()? } }
		if let Some(ref val) = self.pdct_clssfctn { val.validate()? }
		if let Some(ref val) = self.lvl { val.validate()? }
		if let Some(ref vec) = self.evt_tp { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TradeDateTimeQueryCriteria6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradeDateTimeQueryCriteria6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgDtTm", skip_serializing_if = "Option::is_none") )]
	pub rptg_dt_tm: Option<DateTimePeriod1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none") )]
	pub exctn_dt_tm: Option<DateTimePeriod1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none") )]
	pub mtrty_dt: Option<DateOrBlankQuery2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FctvDt", skip_serializing_if = "Option::is_none") )]
	pub fctv_dt: Option<DatePeriod1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValtnDtTm", skip_serializing_if = "Option::is_none") )]
	pub valtn_dt_tm: Option<DateTimePeriod1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XprtnDt", skip_serializing_if = "Option::is_none") )]
	pub xprtn_dt: Option<DateOrBlankQuery2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EarlyTermntnDt", skip_serializing_if = "Option::is_none") )]
	pub early_termntn_dt: Option<DatePeriod1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollTmStmp", skip_serializing_if = "Option::is_none") )]
	pub coll_tm_stmp: Option<DateTimeOrBlankQuery1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HstrclAsOfDt", skip_serializing_if = "Option::is_none") )]
	pub hstrcl_as_of_dt: Option<String>,
}

impl TradeDateTimeQueryCriteria6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rptg_dt_tm { val.validate()? }
		if let Some(ref val) = self.exctn_dt_tm { val.validate()? }
		if let Some(ref val) = self.mtrty_dt { val.validate()? }
		if let Some(ref val) = self.fctv_dt { val.validate()? }
		if let Some(ref val) = self.valtn_dt_tm { val.validate()? }
		if let Some(ref val) = self.xprtn_dt { val.validate()? }
		if let Some(ref val) = self.early_termntn_dt { val.validate()? }
		if let Some(ref val) = self.coll_tm_stmp { val.validate()? }
		Ok(())
	}
}


// TradePartyIdentificationQuery10Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradePartyIdentificationQuery10Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<Vec<PartyIdentification248Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
	pub not_rptd: Option<NotReported1Code>,
}

impl TradePartyIdentificationQuery10Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.id { for item in vec { item.validate()? } }
		if let Some(ref val) = self.not_rptd { val.validate()? }
		Ok(())
	}
}


// TradePartyIdentificationQuery11Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradePartyIdentificationQuery11Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<Vec<OrganisationIdentification15Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
	pub not_rptd: Option<NotReported1Code>,
}

impl TradePartyIdentificationQuery11Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.id { for item in vec { item.validate()? } }
		if let Some(ref val) = self.not_rptd { val.validate()? }
		Ok(())
	}
}


// TradePartyQueryCriteria7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradePartyQueryCriteria7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Oprtr") )]
	pub oprtr: Operation3Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none") )]
	pub rptg_ctr_pty: Option<TradePartyIdentificationQuery10Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none") )]
	pub othr_ctr_pty: Option<TradePartyIdentificationQuery10Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none") )]
	pub bnfcry: Option<TradePartyIdentificationQuery10Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none") )]
	pub ntty_rspnsbl_for_rpt: Option<TradePartyIdentificationQuery11Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubmitgAgt", skip_serializing_if = "Option::is_none") )]
	pub submitg_agt: Option<TradePartyIdentificationQuery11Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Brkr", skip_serializing_if = "Option::is_none") )]
	pub brkr: Option<TradePartyIdentificationQuery11Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CCP", skip_serializing_if = "Option::is_none") )]
	pub ccp: Option<TradePartyIdentificationQuery11Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrMmb", skip_serializing_if = "Option::is_none") )]
	pub clr_mmb: Option<TradePartyIdentificationQuery10Choice>,
}

impl TradePartyQueryCriteria7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.oprtr.validate()?;
		if let Some(ref val) = self.rptg_ctr_pty { val.validate()? }
		if let Some(ref val) = self.othr_ctr_pty { val.validate()? }
		if let Some(ref val) = self.bnfcry { val.validate()? }
		if let Some(ref val) = self.ntty_rspnsbl_for_rpt { val.validate()? }
		if let Some(ref val) = self.submitg_agt { val.validate()? }
		if let Some(ref val) = self.brkr { val.validate()? }
		if let Some(ref val) = self.ccp { val.validate()? }
		if let Some(ref val) = self.clr_mmb { val.validate()? }
		Ok(())
	}
}


// TradeQueryCriteria14 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradeQueryCriteria14 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradLifeCyclHstry", skip_serializing_if = "Option::is_none") )]
	pub trad_life_cycl_hstry: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MrgnLifeCyclHstry", skip_serializing_if = "Option::is_none") )]
	pub mrgn_life_cycl_hstry: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OutsdngTradInd") )]
	pub outsdng_trad_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradPtyCrit", skip_serializing_if = "Option::is_none") )]
	pub trad_pty_crit: Option<TradePartyQueryCriteria7>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmCrit", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_crit: Option<TradeSecurityIdentificationQueryCriteria3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TmCrit", skip_serializing_if = "Option::is_none") )]
	pub tm_crit: Option<TradeDateTimeQueryCriteria6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCrit", skip_serializing_if = "Option::is_none") )]
	pub othr_crit: Option<TradeAdditionalQueryCriteria9>,
}

impl TradeQueryCriteria14 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.trad_pty_crit { val.validate()? }
		if let Some(ref val) = self.fin_instrm_crit { val.validate()? }
		if let Some(ref val) = self.tm_crit { val.validate()? }
		if let Some(ref val) = self.othr_crit { val.validate()? }
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
		self.frqcy_tp.validate()?;
		if let Some(ref vec) = self.dlvry_day { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TradeRecurrentQuery7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradeRecurrentQuery7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "QryTp") )]
	pub qry_tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy") )]
	pub frqcy: Vec<TradeQueryExecutionFrequency3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldUntil") )]
	pub vld_until: String,
}

impl TradeRecurrentQuery7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.qry_tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "qry_tp is shorter than the minimum length of 1".to_string()));
		}
		if self.qry_tp.chars().count() > 1000 {
			return Err(ValidationError::new(1002, "qry_tp exceeds the maximum length of 1000".to_string()));
		}
		for item in &self.frqcy { item.validate()? }
		Ok(())
	}
}


// TradeReportQuery18Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradeReportQuery18Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdHocQry", skip_serializing_if = "Option::is_none") )]
	pub ad_hoc_qry: Option<TradeQueryCriteria14>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RcrntQry", skip_serializing_if = "Option::is_none") )]
	pub rcrnt_qry: Option<TradeRecurrentQuery7>,
}

impl TradeReportQuery18Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ad_hoc_qry { val.validate()? }
		if let Some(ref val) = self.rcrnt_qry { val.validate()? }
		Ok(())
	}
}


// TradeSecurityIdentificationQueryCriteria3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradeSecurityIdentificationQueryCriteria3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Oprtr") )]
	pub oprtr: Operation3Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<Vec<SecurityIdentificationQueryCriteria1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none") )]
	pub ctrct_tp: Option<Vec<FinancialInstrumentContractType2Code>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<Vec<ISINQueryCriteria1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none") )]
	pub unq_pdct_idr: Option<Vec<UPIQueryCriteria1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygInstrmId", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_instrm_id: Option<Vec<SecurityIdentificationQuery4Choice>>,
}

impl TradeSecurityIdentificationQueryCriteria3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.oprtr.validate()?;
		if let Some(ref vec) = self.id { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ctrct_tp { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.isin { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.unq_pdct_idr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.undrlyg_instrm_id { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TransactionOperationType8Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TransactionOperationType8Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "COMP") )]
	CodeCOMP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CORR") )]
	CodeCORR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EROR") )]
	CodeEROR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MODI") )]
	CodeMODI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEWT") )]
	CodeNEWT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "POSC") )]
	CodePOSC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REVI") )]
	CodeREVI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TERM") )]
	CodeTERM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VALU") )]
	CodeVALU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MARU") )]
	CodeMARU,
}

impl TransactionOperationType8Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UPIQueryCriteria1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UPIQueryCriteria1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Idr", skip_serializing_if = "Option::is_none") )]
	pub idr: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
	pub not_rptd: Option<NotReported1Code>,
}

impl UPIQueryCriteria1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.idr {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "idr is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 52 {
					return Err(ValidationError::new(1002, "idr exceeds the maximum length of 52".to_string()));
				}
			}
		}
		if let Some(ref val) = self.not_rptd { val.validate()? }
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
