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


// AdditionalInformation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AdditionalInformation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InfTp") )]
	pub inf_tp: InformationType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InfVal") )]
	pub inf_val: String,
}

impl AdditionalInformation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.inf_tp.validate()?;
		if self.inf_val.chars().count() < 1 {
			return Err(ValidationError::new(1001, "inf_val is shorter than the minimum length of 1".to_string()));
		}
		if self.inf_val.chars().count() > 350 {
			return Err(ValidationError::new(1002, "inf_val exceeds the maximum length of 350".to_string()));
		}
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


// BinaryFile1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BinaryFile1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIMETp", skip_serializing_if = "Option::is_none") )]
	pub mime_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NcodgTp", skip_serializing_if = "Option::is_none") )]
	pub ncodg_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CharSet", skip_serializing_if = "Option::is_none") )]
	pub char_set: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InclBinryObjct", skip_serializing_if = "Option::is_none") )]
	pub incl_binry_objct: Option<String>,
}

impl BinaryFile1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mime_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mime_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mime_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ncodg_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ncodg_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ncodg_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.char_set {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "char_set is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "char_set exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.incl_binry_objct {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "incl_binry_objct is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 102400 {
				return Err(ValidationError::new(1002, "incl_binry_objct exceeds the maximum length of 102400".to_string()));
			}
		}
		Ok(())
	}
}


// ContactDetails2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ContactDetails2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none") )]
	pub nm_prfx: Option<NamePrefix1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PhneNb", skip_serializing_if = "Option::is_none") )]
	pub phne_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MobNb", skip_serializing_if = "Option::is_none") )]
	pub mob_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaxNb", skip_serializing_if = "Option::is_none") )]
	pub fax_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none") )]
	pub email_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<String>,
}

impl ContactDetails2 {
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
		if let Some(ref val) = self.email_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "email_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "email_adr exceeds the maximum length of 2048".to_string()));
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


// CreditorReferenceInformation2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CreditorReferenceInformation2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<CreditorReferenceType2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ref", skip_serializing_if = "Option::is_none") )]
	pub ref_attr: Option<String>,
}

impl CreditorReferenceInformation2 {
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


// CreditorReferenceType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CreditorReferenceType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<DocumentType3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl CreditorReferenceType1Choice {
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


// CreditorReferenceType2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CreditorReferenceType2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
	pub cd_or_prtry: CreditorReferenceType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl CreditorReferenceType2 {
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


// CurrencyAndAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CurrencyAndAmount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl CurrencyAndAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CurrencyReference3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CurrencyReference3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrgtCcy") )]
	pub trgt_ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SrcCcy") )]
	pub src_ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRateInf", skip_serializing_if = "Option::is_none") )]
	pub xchg_rate_inf: Option<Vec<ExchangeRateInformation1>>,
}

impl CurrencyReference3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.trgt_ccy) {
			return Err(ValidationError::new(1005, "trgt_ccy does not match the required pattern".to_string()));
		}
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.src_ccy) {
			return Err(ValidationError::new(1005, "src_ccy does not match the required pattern".to_string()));
		}
		if let Some(ref vec) = self.xchg_rate_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// DateAndPlaceOfBirth ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DateAndPlaceOfBirth {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BirthDt") )]
	pub birth_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none") )]
	pub prvc_of_birth: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CityOfBirth") )]
	pub city_of_birth: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfBirth") )]
	pub ctry_of_birth: String,
}

impl DateAndPlaceOfBirth {
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


// DocumentGeneralInformation2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DocumentGeneralInformation2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DocTp") )]
	pub doc_tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DocNb") )]
	pub doc_nb: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SndrRcvrSeqId", skip_serializing_if = "Option::is_none") )]
	pub sndr_rcvr_seq_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsseDt", skip_serializing_if = "Option::is_none") )]
	pub isse_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "URL", skip_serializing_if = "Option::is_none") )]
	pub url: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AttchdBinryFile", skip_serializing_if = "Option::is_none") )]
	pub attchd_binry_file: Option<Vec<BinaryFile1>>,
}

impl DocumentGeneralInformation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.doc_tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "doc_tp is shorter than the minimum length of 1".to_string()));
		}
		if self.doc_tp.chars().count() > 4 {
			return Err(ValidationError::new(1002, "doc_tp exceeds the maximum length of 4".to_string()));
		}
		if self.doc_nb.chars().count() < 1 {
			return Err(ValidationError::new(1001, "doc_nb is shorter than the minimum length of 1".to_string()));
		}
		if self.doc_nb.chars().count() > 35 {
			return Err(ValidationError::new(1002, "doc_nb exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.sndr_rcvr_seq_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sndr_rcvr_seq_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "sndr_rcvr_seq_id exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.url {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "url is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "url exceeds the maximum length of 256".to_string()));
			}
		}
		if let Some(ref vec) = self.attchd_binry_file { for item in vec { item.validate()? } }
		Ok(())
	}
}


// DocumentType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum DocumentType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RADM") )]
	CodeRADM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RPIN") )]
	CodeRPIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FXDR") )]
	CodeFXDR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DISP") )]
	CodeDISP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUOR") )]
	CodePUOR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCOR") )]
	CodeSCOR,
}

impl DocumentType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// EarlyPayment1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct EarlyPayment1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "EarlyPmtDt") )]
	pub early_pmt_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DscntPct") )]
	pub dscnt_pct: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DscntAmt") )]
	pub dscnt_amt: CurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EarlyPmtTaxSpcfctn", skip_serializing_if = "Option::is_none") )]
	pub early_pmt_tax_spcfctn: Option<Vec<EarlyPaymentsVAT1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EarlyPmtTaxTtl", skip_serializing_if = "Option::is_none") )]
	pub early_pmt_tax_ttl: Option<CurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DuePyblAmtWthEarlyPmt", skip_serializing_if = "Option::is_none") )]
	pub due_pybl_amt_wth_early_pmt: Option<CurrencyAndAmount>,
}

impl EarlyPayment1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.dscnt_amt.validate()?;
		if let Some(ref vec) = self.early_pmt_tax_spcfctn { for item in vec { item.validate()? } }
		if let Some(ref val) = self.early_pmt_tax_ttl { val.validate()? }
		if let Some(ref val) = self.due_pybl_amt_wth_early_pmt { val.validate()? }
		Ok(())
	}
}


// EarlyPaymentsVAT1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct EarlyPaymentsVAT1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRate") )]
	pub tax_rate: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DscntTaxTp") )]
	pub dscnt_tax_tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DscntTaxAmt") )]
	pub dscnt_tax_amt: CurrencyAndAmount,
}

impl EarlyPaymentsVAT1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.dscnt_tax_tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "dscnt_tax_tp is shorter than the minimum length of 1".to_string()));
		}
		if self.dscnt_tax_tp.chars().count() > 4 {
			return Err(ValidationError::new(1002, "dscnt_tax_tp exceeds the maximum length of 4".to_string()));
		}
		self.dscnt_tax_amt.validate()?;
		Ok(())
	}
}


// ExchangeRateInformation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ExchangeRateInformation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRate", skip_serializing_if = "Option::is_none") )]
	pub xchg_rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RateTp", skip_serializing_if = "Option::is_none") )]
	pub rate_tp: Option<ExchangeRateType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctId", skip_serializing_if = "Option::is_none") )]
	pub ctrct_id: Option<String>,
}

impl ExchangeRateInformation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rate_tp { val.validate()? }
		if let Some(ref val) = self.ctrct_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctrct_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctrct_id exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ExchangeRateType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ExchangeRateType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPOT") )]
	CodeSPOT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SALE") )]
	CodeSALE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AGRD") )]
	CodeAGRD,
}

impl ExchangeRateType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// GenericOrganisationIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericOrganisationIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericOrganisationIdentification1 {
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


// GenericPersonIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericPersonIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericPersonIdentification1 {
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


// GroupHeader69 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GroupHeader69 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssdDt") )]
	pub issd_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptCtgy") )]
	pub rpt_ctgy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRptPurp") )]
	pub tax_rpt_purp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SellrTaxRprtv", skip_serializing_if = "Option::is_none") )]
	pub sellr_tax_rprtv: Option<PartyIdentification116>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BuyrTaxRprtv", skip_serializing_if = "Option::is_none") )]
	pub buyr_tax_rprtv: Option<PartyIdentification116>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LangCd", skip_serializing_if = "Option::is_none") )]
	pub lang_cd: Option<String>,
}

impl GroupHeader69 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if self.rpt_ctgy.chars().count() < 1 {
			return Err(ValidationError::new(1001, "rpt_ctgy is shorter than the minimum length of 1".to_string()));
		}
		if self.rpt_ctgy.chars().count() > 4 {
			return Err(ValidationError::new(1002, "rpt_ctgy exceeds the maximum length of 4".to_string()));
		}
		if self.tax_rpt_purp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tax_rpt_purp is shorter than the minimum length of 1".to_string()));
		}
		if self.tax_rpt_purp.chars().count() > 4 {
			return Err(ValidationError::new(1002, "tax_rpt_purp exceeds the maximum length of 4".to_string()));
		}
		if let Some(ref val) = self.orgnl_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.sellr_tax_rprtv { val.validate()? }
		if let Some(ref val) = self.buyr_tax_rprtv { val.validate()? }
		Ok(())
	}
}


// InformationType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InformationType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InformationType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl InformationType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// InformationType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum InformationType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "INST") )]
	CodeINST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RELY") )]
	CodeRELY,
}

impl InformationType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvoiceTaxReportV01 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InvoiceTaxReportV01 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvcTaxRptHdr") )]
	pub invc_tax_rpt_hdr: TaxReportHeader1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRpt") )]
	pub tax_rpt: Vec<TaxReport1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl InvoiceTaxReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.invc_tax_rpt_hdr.validate()?;
		for item in &self.tax_rpt { item.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// LegalOrganisation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct LegalOrganisation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
}

impl LegalOrganisation1 {
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
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
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


// NamePrefix1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// OrganisationIdentification28 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrganisationIdentification28 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<PostalAddress6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<OrganisationIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_res: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls") )]
	pub ctct_dtls: ContactDetails2,
}

impl OrganisationIdentification28 {
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
		self.ctct_dtls.validate()?;
		Ok(())
	}
}


// OrganisationIdentification8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrganisationIdentification8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<GenericOrganisationIdentification1>>,
}

impl OrganisationIdentification8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
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


// Party11Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Party11Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgId", skip_serializing_if = "Option::is_none") )]
	pub org_id: Option<OrganisationIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvtId", skip_serializing_if = "Option::is_none") )]
	pub prvt_id: Option<PersonIdentification5>,
}

impl Party11Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.org_id { val.validate()? }
		if let Some(ref val) = self.prvt_id { val.validate()? }
		Ok(())
	}
}


// PartyIdentification116 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification116 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId") )]
	pub pty_id: OrganisationIdentification28,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglOrg", skip_serializing_if = "Option::is_none") )]
	pub lgl_org: Option<LegalOrganisation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxPty", skip_serializing_if = "Option::is_none") )]
	pub tax_pty: Option<TaxParty1>,
}

impl PartyIdentification116 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty_id.validate()?;
		if let Some(ref val) = self.lgl_org { val.validate()? }
		if let Some(ref val) = self.tax_pty { val.validate()? }
		Ok(())
	}
}


// PartyIdentification43 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification43 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<PostalAddress6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<Party11Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_res: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none") )]
	pub ctct_dtls: Option<ContactDetails2>,
}

impl PartyIdentification43 {
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


// PartyIdentification72 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification72 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId") )]
	pub pty_id: PartyIdentification43,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglOrg", skip_serializing_if = "Option::is_none") )]
	pub lgl_org: Option<LegalOrganisation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxPty", skip_serializing_if = "Option::is_none") )]
	pub tax_pty: Option<TaxParty1>,
}

impl PartyIdentification72 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty_id.validate()?;
		if let Some(ref val) = self.lgl_org { val.validate()? }
		if let Some(ref val) = self.tax_pty { val.validate()? }
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


// PersonIdentification5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PersonIdentification5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none") )]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<GenericPersonIdentification1>>,
}

impl PersonIdentification5 {
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


// PostalAddress6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PostalAddress6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
	pub adr_tp: Option<AddressType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dept", skip_serializing_if = "Option::is_none") )]
	pub dept: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubDept", skip_serializing_if = "Option::is_none") )]
	pub sub_dept: Option<String>,
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
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
	pub adr_line: Option<Vec<String>>,
}

impl PostalAddress6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.adr_tp { val.validate()? }
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


// SettlementSubTotalCalculatedTax2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementSubTotalCalculatedTax2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TpCd", skip_serializing_if = "Option::is_none") )]
	pub tp_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClctdRate", skip_serializing_if = "Option::is_none") )]
	pub clctd_rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BsisAmt", skip_serializing_if = "Option::is_none") )]
	pub bsis_amt: Option<Vec<CurrencyAndAmount>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClctdAmt", skip_serializing_if = "Option::is_none") )]
	pub clctd_amt: Option<Vec<CurrencyAndAmount>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XmptnRsnCd", skip_serializing_if = "Option::is_none") )]
	pub xmptn_rsn_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XmptnRsnTxt", skip_serializing_if = "Option::is_none") )]
	pub xmptn_rsn_txt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxCcyXchg", skip_serializing_if = "Option::is_none") )]
	pub tax_ccy_xchg: Option<CurrencyReference3>,
}

impl SettlementSubTotalCalculatedTax2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tp_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "tp_cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref vec) = self.bsis_amt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.clctd_amt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.xmptn_rsn_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "xmptn_rsn_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "xmptn_rsn_cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.xmptn_rsn_txt {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "xmptn_rsn_txt is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 500 {
				return Err(ValidationError::new(1002, "xmptn_rsn_txt exceeds the maximum length of 500".to_string()));
			}
		}
		if let Some(ref val) = self.tax_ccy_xchg { val.validate()? }
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


// TaxOrganisationIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TaxOrganisationIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<PostalAddress6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none") )]
	pub ctct_dtls: Option<ContactDetails2>,
}

impl TaxOrganisationIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 140 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		if let Some(ref val) = self.ctct_dtls { val.validate()? }
		Ok(())
	}
}


// TaxParty1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// TaxReport1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TaxReport1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRptHdr") )]
	pub tax_rpt_hdr: GroupHeader69,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sellr") )]
	pub sellr: PartyIdentification72,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Buyr", skip_serializing_if = "Option::is_none") )]
	pub buyr: Option<PartyIdentification72>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradSttlm") )]
	pub trad_sttlm: TradeSettlement2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPty", skip_serializing_if = "Option::is_none") )]
	pub othr_pty: Option<Vec<PartyIdentification72>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<AdditionalInformation1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRef", skip_serializing_if = "Option::is_none") )]
	pub addtl_ref: Option<Vec<DocumentGeneralInformation2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl TaxReport1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tax_rpt_hdr.validate()?;
		self.sellr.validate()?;
		if let Some(ref val) = self.buyr { val.validate()? }
		self.trad_sttlm.validate()?;
		if let Some(ref vec) = self.othr_pty { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.addtl_inf { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.addtl_ref { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TaxReportHeader1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TaxReportHeader1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTaxRpts", skip_serializing_if = "Option::is_none") )]
	pub nb_of_tax_rpts: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxAuthrty", skip_serializing_if = "Option::is_none") )]
	pub tax_authrty: Option<Vec<TaxOrganisationIdentification1>>,
}

impl TaxReportHeader1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.msg_id.validate()?;
		if let Some(ref vec) = self.tax_authrty { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TradeSettlement2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradeSettlement2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtRef", skip_serializing_if = "Option::is_none") )]
	pub pmt_ref: Option<CreditorReferenceInformation2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DueDt", skip_serializing_if = "Option::is_none") )]
	pub due_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DuePyblAmt") )]
	pub due_pybl_amt: CurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvcCcyXchg", skip_serializing_if = "Option::is_none") )]
	pub invc_ccy_xchg: Option<CurrencyReference3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryDt", skip_serializing_if = "Option::is_none") )]
	pub dlvry_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BllgPrd", skip_serializing_if = "Option::is_none") )]
	pub bllg_prd: Option<Period2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxTtlAmt") )]
	pub tax_ttl_amt: CurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XmptnRsnCd", skip_serializing_if = "Option::is_none") )]
	pub xmptn_rsn_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XmptnRsn", skip_serializing_if = "Option::is_none") )]
	pub xmptn_rsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubTtlClctdTax", skip_serializing_if = "Option::is_none") )]
	pub sub_ttl_clctd_tax: Option<Vec<SettlementSubTotalCalculatedTax2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EarlyPmts", skip_serializing_if = "Option::is_none") )]
	pub early_pmts: Option<Vec<EarlyPayment1>>,
}

impl TradeSettlement2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pmt_ref { val.validate()? }
		self.due_pybl_amt.validate()?;
		if let Some(ref val) = self.invc_ccy_xchg { val.validate()? }
		if let Some(ref val) = self.bllg_prd { val.validate()? }
		self.tax_ttl_amt.validate()?;
		if let Some(ref val) = self.xmptn_rsn_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "xmptn_rsn_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "xmptn_rsn_cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.xmptn_rsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "xmptn_rsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 500 {
				return Err(ValidationError::new(1002, "xmptn_rsn exceeds the maximum length of 500".to_string()));
			}
		}
		if let Some(ref vec) = self.sub_ttl_clctd_tax { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.early_pmts { for item in vec { item.validate()? } }
		Ok(())
	}
}
