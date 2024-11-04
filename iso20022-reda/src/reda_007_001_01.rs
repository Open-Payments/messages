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


// ActiveCurrencyAnd13DecimalAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// ActiveOrHistoricCurrencyAnd13DecimalAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAnd13DecimalAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AddressType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// AmountOrPercentageRange1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmountOrPercentageRange1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Opr", skip_serializing_if = "Option::is_none") )]
	pub opr: Option<Operation1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Term", skip_serializing_if = "Option::is_none") )]
	pub term: Option<Vec<Term1>>,
}

impl AmountOrPercentageRange1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.opr { val.validate()? }
		if let Some(ref vec) = self.term { for item in vec { item.validate()? } }
		Ok(())
	}
}


// Appearance1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum Appearance1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DELI") )]
	CodeDELI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NDEL") )]
	CodeNDEL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIMI") )]
	CodeLIMI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BENT") )]
	CodeBENT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DFBE") )]
	CodeDFBE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DLBE") )]
	CodeDLBE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TMPG") )]
	CodeTMPG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GLOB") )]
	CodeGLOB,
}

impl Appearance1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Appearance3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Appearance3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Appearance1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl Appearance3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// AssignmentMethod1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum AssignmentMethod1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RAND") )]
	CodeRAND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PROR") )]
	CodePROR,
}

impl AssignmentMethod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AssignmentMethod2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AssignmentMethod2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AssignmentMethod1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl AssignmentMethod2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// BenchmarkCurve6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BenchmarkCurve6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sprd", skip_serializing_if = "Option::is_none") )]
	pub sprd: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BchmkId", skip_serializing_if = "Option::is_none") )]
	pub bchmk_id: Option<SecurityIdentification39>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BchmkPric", skip_serializing_if = "Option::is_none") )]
	pub bchmk_pric: Option<Price8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BchmkCrvCcy", skip_serializing_if = "Option::is_none") )]
	pub bchmk_crv_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BchmkCrvNm", skip_serializing_if = "Option::is_none") )]
	pub bchmk_crv_nm: Option<BenchmarkCurveName7Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BchmkCrvPt", skip_serializing_if = "Option::is_none") )]
	pub bchmk_crv_pt: Option<String>,
}

impl BenchmarkCurve6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.bchmk_id { val.validate()? }
		if let Some(ref val) = self.bchmk_pric { val.validate()? }
		if let Some(ref val) = self.bchmk_crv_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "bchmk_crv_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.bchmk_crv_nm { val.validate()? }
		if let Some(ref val) = self.bchmk_crv_pt {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bchmk_crv_pt is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "bchmk_crv_pt exceeds the maximum length of 256".to_string()));
			}
		}
		Ok(())
	}
}


// BenchmarkCurveName1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum BenchmarkCurveName1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MAAA") )]
	CodeMAAA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FUSW") )]
	CodeFUSW,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIBI") )]
	CodeLIBI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIBO") )]
	CodeLIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SWAP") )]
	CodeSWAP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TREA") )]
	CodeTREA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EURI") )]
	CodeEURI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PFAN") )]
	CodePFAN,
}

impl BenchmarkCurveName1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// BenchmarkCurveName7Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BenchmarkCurveName7Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<BenchmarkCurveName1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl BenchmarkCurveName7Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// CalculationType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum CalculationType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AFTX") )]
	CodeAFTX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ANNU") )]
	CodeANNU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISSU") )]
	CodeISSU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AVMA") )]
	CodeAVMA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOOK") )]
	CodeBOOK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YTNC") )]
	CodeYTNC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHCL") )]
	CodeCHCL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLOS") )]
	CodeCLOS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CMPD") )]
	CodeCMPD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CUYI") )]
	CodeCUYI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRGR") )]
	CodeTRGR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GVEQ") )]
	CodeGVEQ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FLAS") )]
	CodeFLAS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NVFL") )]
	CodeNVFL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LSCL") )]
	CodeLSCL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LSMT") )]
	CodeLSMT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LSQR") )]
	CodeLSQR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LSYR") )]
	CodeLSYR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LGAL") )]
	CodeLGAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MARK") )]
	CodeMARK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YTMA") )]
	CodeYTMA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NXRF") )]
	CodeNXRF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PNAV") )]
	CodePNAV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NXPT") )]
	CodeNXPT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRCL") )]
	CodePRCL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRYL") )]
	CodePRYL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SEMI") )]
	CodeSEMI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SHLF") )]
	CodeSHLF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPLL") )]
	CodeSPLL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TXQV") )]
	CodeTXQV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TTDT") )]
	CodeTTDT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRYL") )]
	CodeTRYL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WRST") )]
	CodeWRST,
}

impl CalculationType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CalculationType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CalculationType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<CalculationType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl CalculationType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// CallType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum CallType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "LOTT") )]
	CodeLOTT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRTA") )]
	CodePRTA,
}

impl CallType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CallType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CallType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<CallType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl CallType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// ClassificationType2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ClassificationType2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none") )]
	pub clssfctn_fin_instrm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmPdctTpCd", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_pdct_tp_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none") )]
	pub altrn_clssfctn: Option<Vec<GenericIdentification36>>,
}

impl ClassificationType2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.clssfctn_fin_instrm {
			let pattern = Regex::new("[A-Z]{6,6}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "clssfctn_fin_instrm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.fin_instrm_pdct_tp_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "fin_instrm_pdct_tp_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "fin_instrm_pdct_tp_cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref vec) = self.altrn_clssfctn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CommonFinancialInstrumentAttributes12 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CommonFinancialInstrumentAttributes12 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctySts", skip_serializing_if = "Option::is_none") )]
	pub scty_sts: Option<SecurityStatus3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISOSctyLngNm", skip_serializing_if = "Option::is_none") )]
	pub iso_scty_lng_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISOSctyShrtNm", skip_serializing_if = "Option::is_none") )]
	pub iso_scty_shrt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmVldFr", skip_serializing_if = "Option::is_none") )]
	pub nm_vld_fr: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DnmtnCcy", skip_serializing_if = "Option::is_none") )]
	pub dnmtn_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CertNb", skip_serializing_if = "Option::is_none") )]
	pub cert_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctVrsnNb", skip_serializing_if = "Option::is_none") )]
	pub ctrct_vrsn_nb: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CpnAttchdNb", skip_serializing_if = "Option::is_none") )]
	pub cpn_attchd_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxLotNb", skip_serializing_if = "Option::is_none") )]
	pub tax_lot_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PoolNb", skip_serializing_if = "Option::is_none") )]
	pub pool_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CvrdInd", skip_serializing_if = "Option::is_none") )]
	pub cvrd_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglRstrctns", skip_serializing_if = "Option::is_none") )]
	pub lgl_rstrctns: Option<LegalRestrictions4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PosLmt", skip_serializing_if = "Option::is_none") )]
	pub pos_lmt: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NearTermPosLmt", skip_serializing_if = "Option::is_none") )]
	pub near_term_pos_lmt: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ListgDt", skip_serializing_if = "Option::is_none") )]
	pub listg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RcrdDt", skip_serializing_if = "Option::is_none") )]
	pub rcrd_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt", skip_serializing_if = "Option::is_none") )]
	pub xpry_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none") )]
	pub clssfctn_tp: Option<ClassificationType2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issnc", skip_serializing_if = "Option::is_none") )]
	pub issnc: Option<Issuance5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgMkt", skip_serializing_if = "Option::is_none") )]
	pub tradg_mkt: Option<Vec<TradingParameters2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SprdAndBchmkCrv", skip_serializing_if = "Option::is_none") )]
	pub sprd_and_bchmk_crv: Option<Vec<BenchmarkCurve6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PutTp", skip_serializing_if = "Option::is_none") )]
	pub put_tp: Option<PutType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CallTp", skip_serializing_if = "Option::is_none") )]
	pub call_tp: Option<CallType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FngbInd", skip_serializing_if = "Option::is_none") )]
	pub fngb_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cnfdtl", skip_serializing_if = "Option::is_none") )]
	pub cnfdtl: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvtPlcmnt", skip_serializing_if = "Option::is_none") )]
	pub prvt_plcmnt: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvtblInd", skip_serializing_if = "Option::is_none") )]
	pub convtbl_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvsPrd", skip_serializing_if = "Option::is_none") )]
	pub convs_prd: Option<DateTimePeriod1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvsRatioNmrtr", skip_serializing_if = "Option::is_none") )]
	pub convs_ratio_nmrtr: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvsRatioDnmtr", skip_serializing_if = "Option::is_none") )]
	pub convs_ratio_dnmtr: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmryPlcOfDpst", skip_serializing_if = "Option::is_none") )]
	pub pmry_plc_of_dpst: Option<PartyIdentification136>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgMtd", skip_serializing_if = "Option::is_none") )]
	pub tradg_mtd: Option<UnitOrFaceAmount1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TEFRARule", skip_serializing_if = "Option::is_none") )]
	pub tefra_rule: Option<TEFRARules3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SrNb", skip_serializing_if = "Option::is_none") )]
	pub sr_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Clss", skip_serializing_if = "Option::is_none") )]
	pub clss: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WhldgTaxRgm", skip_serializing_if = "Option::is_none") )]
	pub whldg_tax_rgm: Option<Vec<SecurityWithHoldingTax1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtSts", skip_serializing_if = "Option::is_none") )]
	pub pmt_sts: Option<SecuritiesPaymentStatus5Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlPhysForm", skip_serializing_if = "Option::is_none") )]
	pub initl_phys_form: Option<InitialPhysicalForm4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AftrXchgPhysForm", skip_serializing_if = "Option::is_none") )]
	pub aftr_xchg_phys_form: Option<InitialPhysicalForm3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CmonSfkpr", skip_serializing_if = "Option::is_none") )]
	pub cmon_sfkpr: Option<PartyIdentification177Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RedTp", skip_serializing_if = "Option::is_none") )]
	pub red_tp: Option<MaturityRedemptionType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RedPmtCcy", skip_serializing_if = "Option::is_none") )]
	pub red_pmt_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none") )]
	pub rstrctn: Option<Vec<SecurityRestriction3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_id: Option<SecurityIdentification39>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmInf", skip_serializing_if = "Option::is_none") )]
	pub sttlm_inf: Option<Vec<SettlementInformation17>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmForm", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_form: Option<FinancialInstrumentForm2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtctNm", skip_serializing_if = "Option::is_none") )]
	pub ctct_nm: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LeadMgr", skip_serializing_if = "Option::is_none") )]
	pub lead_mgr: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrncplPngAgt", skip_serializing_if = "Option::is_none") )]
	pub prncpl_png_agt: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PngAgt", skip_serializing_if = "Option::is_none") )]
	pub png_agt: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dpstry", skip_serializing_if = "Option::is_none") )]
	pub dpstry: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygRsk", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_rsk: Option<Organisation38>,
}

impl CommonFinancialInstrumentAttributes12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.scty_sts { val.validate()? }
		if let Some(ref val) = self.iso_scty_lng_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "iso_scty_lng_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "iso_scty_lng_nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.iso_scty_shrt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "iso_scty_shrt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "iso_scty_shrt_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.nm_vld_fr { val.validate()? }
		if let Some(ref val) = self.dnmtn_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "dnmtn_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.cert_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cert_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cert_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.cpn_attchd_nb {
			let pattern = Regex::new("[0-9]{1,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "cpn_attchd_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tax_lot_nb {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "tax_lot_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pool_nb {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pool_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.lgl_rstrctns { val.validate()? }
		if let Some(ref val) = self.pos_lmt { val.validate()? }
		if let Some(ref val) = self.near_term_pos_lmt { val.validate()? }
		if let Some(ref val) = self.purp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "purp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "purp exceeds the maximum length of 256".to_string()));
			}
		}
		if let Some(ref val) = self.clssfctn_tp { val.validate()? }
		if let Some(ref val) = self.issnc { val.validate()? }
		if let Some(ref vec) = self.tradg_mkt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.sprd_and_bchmk_crv { for item in vec { item.validate()? } }
		if let Some(ref val) = self.put_tp { val.validate()? }
		if let Some(ref val) = self.call_tp { val.validate()? }
		if let Some(ref val) = self.convs_prd { val.validate()? }
		if let Some(ref val) = self.convs_ratio_nmrtr { val.validate()? }
		if let Some(ref val) = self.convs_ratio_dnmtr { val.validate()? }
		if let Some(ref val) = self.pmry_plc_of_dpst { val.validate()? }
		if let Some(ref val) = self.tradg_mtd { val.validate()? }
		if let Some(ref val) = self.tefra_rule { val.validate()? }
		if let Some(ref val) = self.sr_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sr_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "sr_nb exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.clss {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clss is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "clss exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref vec) = self.whldg_tax_rgm { for item in vec { item.validate()? } }
		if let Some(ref val) = self.pmt_sts { val.validate()? }
		if let Some(ref val) = self.initl_phys_form { val.validate()? }
		if let Some(ref val) = self.aftr_xchg_phys_form { val.validate()? }
		if let Some(ref val) = self.cmon_sfkpr { val.validate()? }
		if let Some(ref val) = self.red_tp { val.validate()? }
		if let Some(ref val) = self.red_pmt_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "red_pmt_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.rstrctn { for item in vec { item.validate()? } }
		if let Some(ref val) = self.fin_instrm_id { val.validate()? }
		if let Some(ref vec) = self.sttlm_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.fin_instrm_form { val.validate()? }
		if let Some(ref val) = self.ctct_nm { val.validate()? }
		if let Some(ref val) = self.lead_mgr { val.validate()? }
		if let Some(ref val) = self.prncpl_png_agt { val.validate()? }
		if let Some(ref val) = self.png_agt { val.validate()? }
		if let Some(ref val) = self.dpstry { val.validate()? }
		if let Some(ref val) = self.undrlyg_rsk { val.validate()? }
		Ok(())
	}
}


// CommunicationAddress3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
		if let Some(ref val) = self.dt_tm_rg { val.validate()? }
		Ok(())
	}
}


// DateTimePeriod2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// Debt5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Debt5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCcy", skip_serializing_if = "Option::is_none") )]
	pub pmt_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none") )]
	pub face_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtFrqcy", skip_serializing_if = "Option::is_none") )]
	pub pmt_frqcy: Option<Frequency35Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFxgDt", skip_serializing_if = "Option::is_none") )]
	pub intrst_fxg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtdDt", skip_serializing_if = "Option::is_none") )]
	pub dtd_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrstPmtDt", skip_serializing_if = "Option::is_none") )]
	pub frst_pmt_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none") )]
	pub mtrty_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NxtCpnDt", skip_serializing_if = "Option::is_none") )]
	pub nxt_cpn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PutblDt", skip_serializing_if = "Option::is_none") )]
	pub putbl_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NxtCllblDt", skip_serializing_if = "Option::is_none") )]
	pub nxt_cllbl_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NxtFctrDt", skip_serializing_if = "Option::is_none") )]
	pub nxt_fctr_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XprtnDt", skip_serializing_if = "Option::is_none") )]
	pub xprtn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtDrctnInd", skip_serializing_if = "Option::is_none") )]
	pub pmt_drctn_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none") )]
	pub intrst_rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NxtIntrstRate", skip_serializing_if = "Option::is_none") )]
	pub nxt_intrst_rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OddCpnInd", skip_serializing_if = "Option::is_none") )]
	pub odd_cpn_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CllblInd", skip_serializing_if = "Option::is_none") )]
	pub cllbl_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CPPrgm", skip_serializing_if = "Option::is_none") )]
	pub cp_prgm: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CPRegnTp", skip_serializing_if = "Option::is_none") )]
	pub cp_regn_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstAcrlDt", skip_serializing_if = "Option::is_none") )]
	pub intrst_acrl_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PutblInd", skip_serializing_if = "Option::is_none") )]
	pub putbl_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PreFnddInd", skip_serializing_if = "Option::is_none") )]
	pub pre_fndd_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EscrwdInd", skip_serializing_if = "Option::is_none") )]
	pub escrwd_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PerptlInd", skip_serializing_if = "Option::is_none") )]
	pub perptl_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubrdntdInd", skip_serializing_if = "Option::is_none") )]
	pub subrdntd_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XtndblInd", skip_serializing_if = "Option::is_none") )]
	pub xtndbl_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XtndblPrd", skip_serializing_if = "Option::is_none") )]
	pub xtndbl_prd: Option<DateTimePeriod1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VarblRateInd", skip_serializing_if = "Option::is_none") )]
	pub varbl_rate_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OverAlltmtAmt", skip_serializing_if = "Option::is_none") )]
	pub over_alltmt_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OverAlltmtRate", skip_serializing_if = "Option::is_none") )]
	pub over_alltmt_rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtsblInd", skip_serializing_if = "Option::is_none") )]
	pub amtsbl_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstClctnMtd", skip_serializing_if = "Option::is_none") )]
	pub intrst_clctn_mtd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CptlsdIntrst", skip_serializing_if = "Option::is_none") )]
	pub cptlsd_intrst: Option<DistributionPolicy2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ActlDnmtnAmt", skip_serializing_if = "Option::is_none") )]
	pub actl_dnmtn_amt: Option<Vec<ActiveCurrencyAndAmount>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CurFctr", skip_serializing_if = "Option::is_none") )]
	pub cur_fctr: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NxtFctr", skip_serializing_if = "Option::is_none") )]
	pub nxt_fctr: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsFctr", skip_serializing_if = "Option::is_none") )]
	pub prvs_fctr: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pcs", skip_serializing_if = "Option::is_none") )]
	pub pcs: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlsMax", skip_serializing_if = "Option::is_none") )]
	pub pls_max: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlsPerMln", skip_serializing_if = "Option::is_none") )]
	pub pls_per_mln: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlsPerLot", skip_serializing_if = "Option::is_none") )]
	pub pls_per_lot: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlsPerTrad", skip_serializing_if = "Option::is_none") )]
	pub pls_per_trad: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CstPrePmtPnltyInd", skip_serializing_if = "Option::is_none") )]
	pub cst_pre_pmt_pnlty_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LotId", skip_serializing_if = "Option::is_none") )]
	pub lot_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CstPrePmtYld", skip_serializing_if = "Option::is_none") )]
	pub cst_pre_pmt_yld: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WghtdAvrgCpn", skip_serializing_if = "Option::is_none") )]
	pub wghtd_avrg_cpn: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WghtdAvrgLife", skip_serializing_if = "Option::is_none") )]
	pub wghtd_avrg_life: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WghtdAvrgLn", skip_serializing_if = "Option::is_none") )]
	pub wghtd_avrg_ln: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WghtdAvrgMtrty", skip_serializing_if = "Option::is_none") )]
	pub wghtd_avrg_mtrty: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InsrdInd", skip_serializing_if = "Option::is_none") )]
	pub insrd_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BkQlfdInd", skip_serializing_if = "Option::is_none") )]
	pub bk_qlfd_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YldClctn", skip_serializing_if = "Option::is_none") )]
	pub yld_clctn: Option<Vec<YieldCalculation6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstTp", skip_serializing_if = "Option::is_none") )]
	pub intrst_tp: Option<InterestType3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrmStrTp", skip_serializing_if = "Option::is_none") )]
	pub instrm_str_tp: Option<InstrumentSubStructureType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GblTp", skip_serializing_if = "Option::is_none") )]
	pub gbl_tp: Option<GlobalNote2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PotntlEuroSysElgblty", skip_serializing_if = "Option::is_none") )]
	pub potntl_euro_sys_elgblty: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Geogcs", skip_serializing_if = "Option::is_none") )]
	pub geogcs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YldRg", skip_serializing_if = "Option::is_none") )]
	pub yld_rg: Option<AmountOrPercentageRange1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CpnRg", skip_serializing_if = "Option::is_none") )]
	pub cpn_rg: Option<AmountOrPercentageRange1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AltrntvMinTaxInd", skip_serializing_if = "Option::is_none") )]
	pub altrntv_min_tax_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AutoRinvstmt", skip_serializing_if = "Option::is_none") )]
	pub auto_rinvstmt: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Hrcut", skip_serializing_if = "Option::is_none") )]
	pub hrcut: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxConds", skip_serializing_if = "Option::is_none") )]
	pub tx_conds: Option<TradeTransactionCondition7Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LookBck", skip_serializing_if = "Option::is_none") )]
	pub look_bck: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MaxSbstitn", skip_serializing_if = "Option::is_none") )]
	pub max_sbstitn: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinIncrmt", skip_serializing_if = "Option::is_none") )]
	pub min_incrmt: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinQty", skip_serializing_if = "Option::is_none") )]
	pub min_qty: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pdctn", skip_serializing_if = "Option::is_none") )]
	pub pdctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RstrctdInd", skip_serializing_if = "Option::is_none") )]
	pub rstrctd_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricFrqcy", skip_serializing_if = "Option::is_none") )]
	pub pric_frqcy: Option<Frequency35Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sctr", skip_serializing_if = "Option::is_none") )]
	pub sctr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SbstitnFrqcy", skip_serializing_if = "Option::is_none") )]
	pub sbstitn_frqcy: Option<Frequency35Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SbstitnLft", skip_serializing_if = "Option::is_none") )]
	pub sbstitn_lft: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WhlPoolInd", skip_serializing_if = "Option::is_none") )]
	pub whl_pool_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricSrc", skip_serializing_if = "Option::is_none") )]
	pub pric_src: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricRg", skip_serializing_if = "Option::is_none") )]
	pub pric_rg: Option<AmountOrPercentageRange1>,
}

impl Debt5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pmt_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pmt_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.face_amt { val.validate()? }
		if let Some(ref val) = self.pmt_frqcy { val.validate()? }
		if let Some(ref val) = self.cp_regn_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cp_regn_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "cp_regn_tp exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.xtndbl_prd { val.validate()? }
		if let Some(ref val) = self.over_alltmt_amt { val.validate()? }
		if let Some(ref val) = self.intrst_clctn_mtd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "intrst_clctn_mtd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "intrst_clctn_mtd exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.cptlsd_intrst { val.validate()? }
		if let Some(ref vec) = self.actl_dnmtn_amt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.lot_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "lot_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "lot_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.yld_clctn { for item in vec { item.validate()? } }
		if let Some(ref val) = self.intrst_tp { val.validate()? }
		if let Some(ref val) = self.instrm_str_tp { val.validate()? }
		if let Some(ref val) = self.gbl_tp { val.validate()? }
		if let Some(ref val) = self.geogcs {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "geogcs is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "geogcs exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.yld_rg { val.validate()? }
		if let Some(ref val) = self.cpn_rg { val.validate()? }
		if let Some(ref val) = self.purp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "purp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "purp exceeds the maximum length of 256".to_string()));
			}
		}
		if let Some(ref val) = self.tx_conds { val.validate()? }
		if let Some(ref val) = self.min_incrmt { val.validate()? }
		if let Some(ref val) = self.min_qty { val.validate()? }
		if let Some(ref val) = self.pdctn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pdctn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "pdctn exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.pric_frqcy { val.validate()? }
		if let Some(ref val) = self.sctr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sctr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sctr exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.sbstitn_frqcy { val.validate()? }
		if let Some(ref val) = self.pric_src {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pric_src is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "pric_src exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.pric_rg { val.validate()? }
		Ok(())
	}
}


// Derivative4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Derivative4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Futr", skip_serializing_if = "Option::is_none") )]
	pub futr: Option<Future4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Optn", skip_serializing_if = "Option::is_none") )]
	pub optn: Option<Option15>,
}

impl Derivative4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.futr { val.validate()? }
		if let Some(ref val) = self.optn { val.validate()? }
		Ok(())
	}
}


// DistributionPolicy1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// DistributionPolicy2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DistributionPolicy2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<DistributionPolicy1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl DistributionPolicy2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// Equity3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Equity3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrefToIncm") )]
	pub pref_to_incm: PreferenceToIncome5Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none") )]
	pub mtrty_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NonPdAmt", skip_serializing_if = "Option::is_none") )]
	pub non_pd_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ParVal", skip_serializing_if = "Option::is_none") )]
	pub par_val: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VtngRghtsPerShr", skip_serializing_if = "Option::is_none") )]
	pub vtng_rghts_per_shr: Option<f64>,
}

impl Equity3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pref_to_incm.validate()?;
		if let Some(ref val) = self.non_pd_amt { val.validate()? }
		if let Some(ref val) = self.par_val { val.validate()? }
		Ok(())
	}
}


// FinancialInstrument97 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrument97 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Eqty", skip_serializing_if = "Option::is_none") )]
	pub eqty: Option<Equity3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Warrt", skip_serializing_if = "Option::is_none") )]
	pub warrt: Option<Warrant4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Debt", skip_serializing_if = "Option::is_none") )]
	pub debt: Option<Debt5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Deriv", skip_serializing_if = "Option::is_none") )]
	pub deriv: Option<Derivative4>,
}

impl FinancialInstrument97 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.eqty { val.validate()? }
		if let Some(ref val) = self.warrt { val.validate()? }
		if let Some(ref val) = self.debt { val.validate()? }
		if let Some(ref val) = self.deriv { val.validate()? }
		Ok(())
	}
}


// FinancialInstrumentForm2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrumentForm2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BookgApprnc", skip_serializing_if = "Option::is_none") )]
	pub bookg_apprnc: Option<Appearance3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglForm", skip_serializing_if = "Option::is_none") )]
	pub lgl_form: Option<FormOfSecurity8Choice>,
}

impl FinancialInstrumentForm2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.bookg_apprnc { val.validate()? }
		if let Some(ref val) = self.lgl_form { val.validate()? }
		Ok(())
	}
}


// FinancialInstrumentQuantity1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrumentQuantity1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
	pub unit: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none") )]
	pub face_amt: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none") )]
	pub amtsd_val: Option<f64>,
}

impl FinancialInstrumentQuantity1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.face_amt {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "face_amt is less than the minimum value of 0.000000".to_string()));
			}
		}
		if let Some(ref val) = self.amtsd_val {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "amtsd_val is less than the minimum value of 0.000000".to_string()));
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


// FormOfSecurity8Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FormOfSecurity8Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<FormOfSecurity1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl FormOfSecurity8Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// Frequency35Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Frequency35Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Frequency5Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl Frequency35Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// Frequency5Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum Frequency5Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
	CodeYEAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
	CodeMNTH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QURT") )]
	CodeQURT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIAN") )]
	CodeMIAN,
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
	#[cfg_attr( feature = "derive_serde", serde(rename = "TEND") )]
	CodeTEND,
}

impl Frequency5Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Future4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Future4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctSz", skip_serializing_if = "Option::is_none") )]
	pub ctrct_sz: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExrcPric", skip_serializing_if = "Option::is_none") )]
	pub exrc_pric: Option<Price8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FutrDt", skip_serializing_if = "Option::is_none") )]
	pub futr_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinSz", skip_serializing_if = "Option::is_none") )]
	pub min_sz: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none") )]
	pub unit_of_measr: Option<UnitOfMeasure7Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TmUnit", skip_serializing_if = "Option::is_none") )]
	pub tm_unit: Option<TimeUnit3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlUndrlygAttrbts", skip_serializing_if = "Option::is_none") )]
	pub addtl_undrlyg_attrbts: Option<Vec<UnderlyingAttributes4>>,
}

impl Future4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.exrc_pric { val.validate()? }
		if let Some(ref val) = self.min_sz { val.validate()? }
		if let Some(ref val) = self.unit_of_measr { val.validate()? }
		if let Some(ref val) = self.tm_unit { val.validate()? }
		if let Some(ref vec) = self.addtl_undrlyg_attrbts { for item in vec { item.validate()? } }
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


// GenericIdentification13 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// GlobalNote1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum GlobalNote1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NGNO") )]
	CodeNGNO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CGNO") )]
	CodeCGNO,
}

impl GlobalNote1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// GlobalNote2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GlobalNote2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<GlobalNote1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl GlobalNote2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
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


// InitialPhysicalForm1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum InitialPhysicalForm1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "GTGT") )]
	CodeGTGT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GPGP") )]
	CodeGPGP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DERN") )]
	CodeDERN,
}

impl InitialPhysicalForm1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InitialPhysicalForm2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum InitialPhysicalForm2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "GPGP") )]
	CodeGPGP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DERN") )]
	CodeDERN,
}

impl InitialPhysicalForm2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InitialPhysicalForm3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InitialPhysicalForm3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InitialPhysicalForm2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl InitialPhysicalForm3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// InitialPhysicalForm4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InitialPhysicalForm4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InitialPhysicalForm1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl InitialPhysicalForm4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// InstrumentSubStructureType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum InstrumentSubStructureType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ABSE") )]
	CodeABSE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AIRT") )]
	CodeAIRT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AUTT") )]
	CodeAUTT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CBOB") )]
	CodeCBOB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CDOB") )]
	CodeCDOB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLNO") )]
	CodeCLNO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLOB") )]
	CodeCLOB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CMBS") )]
	CodeCMBS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSMR") )]
	CodeCSMR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRCT") )]
	CodeCRCT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HELO") )]
	CodeHELO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LPNO") )]
	CodeLPNO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PFAB") )]
	CodePFAB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PYRT") )]
	CodePYRT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REPK") )]
	CodeREPK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RMBS") )]
	CodeRMBS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCBO") )]
	CodeSCBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STRB") )]
	CodeSTRB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STUT") )]
	CodeSTUT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WBSE") )]
	CodeWBSE,
}

impl InstrumentSubStructureType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InstrumentSubStructureType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InstrumentSubStructureType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InstrumentSubStructureType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl InstrumentSubStructureType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// InterestType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum InterestType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ZCPN") )]
	CodeZCPN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FIXD") )]
	CodeFIXD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FLRN") )]
	CodeFLRN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DUAL") )]
	CodeDUAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INDE") )]
	CodeINDE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DSCO") )]
	CodeDSCO,
}

impl InterestType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestorRestrictionType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum InvestorRestrictionType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "LERE") )]
	CodeLERE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CITI") )]
	CodeCITI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INDV") )]
	CodeINDV,
}

impl InvestorRestrictionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestorRestrictionType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InvestorRestrictionType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InvestorRestrictionType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl InvestorRestrictionType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// InvestorType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum InvestorType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RETL") )]
	CodeRETL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PROF") )]
	CodePROF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STAF") )]
	CodeSTAF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PPER") )]
	CodePPER,
}

impl InvestorType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestorType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InvestorType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InvestorType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl InvestorType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// Issuance5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Issuance5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssePlc", skip_serializing_if = "Option::is_none") )]
	pub isse_plc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfIsse", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_isse: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsseDt", skip_serializing_if = "Option::is_none") )]
	pub isse_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnncmntDt", skip_serializing_if = "Option::is_none") )]
	pub anncmnt_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISINVldFr", skip_serializing_if = "Option::is_none") )]
	pub isin_vld_fr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssrOrg", skip_serializing_if = "Option::is_none") )]
	pub issr_org: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsseNmnlAmt", skip_serializing_if = "Option::is_none") )]
	pub isse_nmnl_amt: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FullIssdAmt", skip_serializing_if = "Option::is_none") )]
	pub full_issd_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsseSz", skip_serializing_if = "Option::is_none") )]
	pub isse_sz: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssePric", skip_serializing_if = "Option::is_none") )]
	pub isse_pric: Option<PriceValue1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssncDstrbtn", skip_serializing_if = "Option::is_none") )]
	pub issnc_dstrbtn: Option<SecuritiesTransactionType31Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GovngLaw", skip_serializing_if = "Option::is_none") )]
	pub govng_law: Option<Vec<Jurisdiction1>>,
}

impl Issuance5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.isse_plc {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isse_plc does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctry_of_isse {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_isse does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.issr_org { val.validate()? }
		if let Some(ref val) = self.isse_nmnl_amt { val.validate()? }
		if let Some(ref val) = self.full_issd_amt { val.validate()? }
		if let Some(ref val) = self.isse_pric { val.validate()? }
		if let Some(ref val) = self.issnc_dstrbtn { val.validate()? }
		if let Some(ref vec) = self.govng_law { for item in vec { item.validate()? } }
		Ok(())
	}
}


// Jurisdiction1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Jurisdiction1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
}

impl Jurisdiction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 70".to_string()));
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


// LegalRestrictions1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum LegalRestrictions1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "USLE") )]
	CodeUSLE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NORE") )]
	CodeNORE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REST") )]
	CodeREST,
}

impl LegalRestrictions1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// LegalRestrictions2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum LegalRestrictions2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "JURO") )]
	CodeJURO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PPLA") )]
	CodePPLA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACRI") )]
	CodeACRI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MARG") )]
	CodeMARG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRIV") )]
	CodePRIV,
}

impl LegalRestrictions2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// LegalRestrictions4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct LegalRestrictions4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<LegalRestrictions1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl LegalRestrictions4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// LegalRestrictions5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct LegalRestrictions5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<LegalRestrictions2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl LegalRestrictions5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// MaturityRedemptionType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum MaturityRedemptionType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FRED") )]
	CodeFRED,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRNR") )]
	CodePRNR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRWR") )]
	CodePRWR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RNDM") )]
	CodeRNDM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRRA") )]
	CodePRRA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CALL") )]
	CodeCALL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUUT") )]
	CodePUUT,
}

impl MaturityRedemptionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// MaturityRedemptionType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MaturityRedemptionType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<MaturityRedemptionType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl MaturityRedemptionType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// MessageHeader1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MessageHeader1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
}

impl MessageHeader1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// NameAndAddress4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// Operation1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum Operation1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "TILL") )]
	CodeTILL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ORRR") )]
	CodeORRR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ANDD") )]
	CodeANDD,
}

impl Operation1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Operator1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum Operator1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SMAL") )]
	CodeSMAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SMEQ") )]
	CodeSMEQ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GREA") )]
	CodeGREA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GREQ") )]
	CodeGREQ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EQAL") )]
	CodeEQAL,
}

impl Operator1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Option15 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Option15 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OptnSttlmStyle", skip_serializing_if = "Option::is_none") )]
	pub optn_sttlm_style: Option<SettleStyle2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvsDt", skip_serializing_if = "Option::is_none") )]
	pub convs_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StrkPric", skip_serializing_if = "Option::is_none") )]
	pub strk_pric: Option<Price8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinExrcblQty", skip_serializing_if = "Option::is_none") )]
	pub min_exrcbl_qty: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvsPrd", skip_serializing_if = "Option::is_none") )]
	pub convs_prd: Option<DateTimePeriod1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OptnStyle", skip_serializing_if = "Option::is_none") )]
	pub optn_style: Option<OptionStyle1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OptnTp", skip_serializing_if = "Option::is_none") )]
	pub optn_tp: Option<OptionType8Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StrkVal", skip_serializing_if = "Option::is_none") )]
	pub strk_val: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StrkMltplr", skip_serializing_if = "Option::is_none") )]
	pub strk_mltplr: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrmAssgnmtMtd", skip_serializing_if = "Option::is_none") )]
	pub instrm_assgnmt_mtd: Option<AssignmentMethod2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VrsnNb", skip_serializing_if = "Option::is_none") )]
	pub vrsn_nb: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XpryLctn", skip_serializing_if = "Option::is_none") )]
	pub xpry_lctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Stdstn", skip_serializing_if = "Option::is_none") )]
	pub stdstn: Option<Standardisation3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgPtyRole", skip_serializing_if = "Option::is_none") )]
	pub tradg_pty_role: Option<OptionParty3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctSz", skip_serializing_if = "Option::is_none") )]
	pub ctrct_sz: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlUndrlygAttrbts", skip_serializing_if = "Option::is_none") )]
	pub addtl_undrlyg_attrbts: Option<Vec<UnderlyingAttributes4>>,
}

impl Option15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.optn_sttlm_style { val.validate()? }
		if let Some(ref val) = self.strk_pric { val.validate()? }
		if let Some(ref val) = self.min_exrcbl_qty { val.validate()? }
		if let Some(ref val) = self.convs_prd { val.validate()? }
		if let Some(ref val) = self.optn_style { val.validate()? }
		if let Some(ref val) = self.optn_tp { val.validate()? }
		if let Some(ref val) = self.instrm_assgnmt_mtd { val.validate()? }
		if let Some(ref val) = self.xpry_lctn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "xpry_lctn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "xpry_lctn exceeds the maximum length of 4".to_string()));
			}
			let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "xpry_lctn does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.stdstn { val.validate()? }
		if let Some(ref val) = self.tradg_pty_role { val.validate()? }
		if let Some(ref vec) = self.addtl_undrlyg_attrbts { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OptionParty1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OptionParty1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SLLR") )]
	CodeSLLR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BYER") )]
	CodeBYER,
}

impl OptionParty1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OptionParty3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OptionParty3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Vec<OptionParty1Code>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl OptionParty3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.cd { for item in vec { item.validate()? } }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// OptionStyle1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OptionStyle1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<OptionStyle1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification13>,
}

impl OptionStyle1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// OptionStyle1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OptionStyle1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AMER") )]
	CodeAMER,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EURO") )]
	CodeEURO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BERM") )]
	CodeBERM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ASIA") )]
	CodeASIA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CANA") )]
	CodeCANA,
}

impl OptionStyle1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OptionType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OptionType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CALL") )]
	CodeCALL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUTO") )]
	CodePUTO,
}

impl OptionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OptionType8Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OptionType8Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Vec<OptionType1Code>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl OptionType8Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.cd { for item in vec { item.validate()? } }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// Organisation38 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Organisation38 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<PartyIdentification177Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxtnCtry", skip_serializing_if = "Option::is_none") )]
	pub taxtn_ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnCtry", skip_serializing_if = "Option::is_none") )]
	pub regn_ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnDt", skip_serializing_if = "Option::is_none") )]
	pub regn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxIdNb", skip_serializing_if = "Option::is_none") )]
	pub tax_id_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtlRegnNb", skip_serializing_if = "Option::is_none") )]
	pub ntl_regn_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr") )]
	pub pstl_adr: Vec<PostalAddress3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmryComAdr", skip_serializing_if = "Option::is_none") )]
	pub pmry_com_adr: Option<CommunicationAddress3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ScndryComAdr", skip_serializing_if = "Option::is_none") )]
	pub scndry_com_adr: Option<CommunicationAddress3>,
}

impl Organisation38 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 140 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
		}
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.purp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "purp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "purp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.taxtn_ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "taxtn_ctry does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.regn_ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "regn_ctry does not match the required pattern".to_string()));
			}
		}
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
		for item in &self.pstl_adr { item.validate()? }
		if let Some(ref val) = self.pmry_com_adr { val.validate()? }
		if let Some(ref val) = self.scndry_com_adr { val.validate()? }
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
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry_id { val.validate()? }
		if let Some(ref val) = self.nm_and_adr { val.validate()? }
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
		self.id.validate()?;
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


// PostalAddress3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PostalAddress3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp") )]
	pub adr_tp: AddressType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MlngInd") )]
	pub mlng_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnAdrInd") )]
	pub regn_adr_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr") )]
	pub nm_and_adr: NameAndAddress4,
}

impl PostalAddress3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.adr_tp.validate()?;
		self.nm_and_adr.validate()?;
		Ok(())
	}
}


// PreferenceToIncome1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PreferenceToIncome1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ORDN") )]
	CodeORDN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PFRD") )]
	CodePFRD,
}

impl PreferenceToIncome1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PreferenceToIncome5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PreferenceToIncome5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<PreferenceToIncome1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl PreferenceToIncome5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// Price8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Price8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValTp", skip_serializing_if = "Option::is_none") )]
	pub val_tp: Option<PriceValueType3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: PriceRateOrAmount3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricTp", skip_serializing_if = "Option::is_none") )]
	pub pric_tp: Option<TypeOfPrice1Code>,
}

impl Price8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.val_tp { val.validate()? }
		self.val.validate()?;
		if let Some(ref val) = self.pric_tp { val.validate()? }
		Ok(())
	}
}


// PriceRateOrAmount3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PriceRateOrAmount3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
}

impl PriceRateOrAmount3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt { val.validate()? }
		Ok(())
	}
}


// PriceValue1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PriceValue1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
}

impl PriceValue1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		Ok(())
	}
}


// PriceValueType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PriceValueType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DISC") )]
	CodeDISC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PREM") )]
	CodePREM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PARV") )]
	CodePARV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YIEL") )]
	CodeYIEL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPRE") )]
	CodeSPRE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PEUN") )]
	CodePEUN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ABSO") )]
	CodeABSO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TEDP") )]
	CodeTEDP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TEDY") )]
	CodeTEDY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FICT") )]
	CodeFICT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VACT") )]
	CodeVACT,
}

impl PriceValueType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PutType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PutType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MAND") )]
	CodeMAND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OPTI") )]
	CodeOPTI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TWOS") )]
	CodeTWOS,
}

impl PutType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PutType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PutType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<PutType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl PutType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// RateAndAmountFormat1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RateAndAmountFormat1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none") )]
	pub not_spcfd_rate: Option<RateType12FormatChoice>,
}

impl RateAndAmountFormat1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref val) = self.not_spcfd_rate { val.validate()? }
		Ok(())
	}
}


// RateOrAbsoluteValue1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RateOrAbsoluteValue1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RateVal", skip_serializing_if = "Option::is_none") )]
	pub rate_val: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AbsVal", skip_serializing_if = "Option::is_none") )]
	pub abs_val: Option<f64>,
}

impl RateOrAbsoluteValue1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RateType12Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum RateType12Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "OPEN") )]
	CodeOPEN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UKWN") )]
	CodeUKWN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NILP") )]
	CodeNILP,
}

impl RateType12Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RateType12FormatChoice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RateType12FormatChoice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<RateType12Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification13>,
}

impl RateType12FormatChoice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// RestrictionType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum RestrictionType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SELR") )]
	CodeSELR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BUYR") )]
	CodeBUYR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PLAR") )]
	CodePLAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HOLR") )]
	CodeHOLR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VOTR") )]
	CodeVOTR,
}

impl RestrictionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecuritiesPaymentStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum SecuritiesPaymentStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FULL") )]
	CodeFULL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NILL") )]
	CodeNILL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PART") )]
	CodePART,
}

impl SecuritiesPaymentStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecuritiesPaymentStatus5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesPaymentStatus5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<SecuritiesPaymentStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl SecuritiesPaymentStatus5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// SecuritiesTransactionType11Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum SecuritiesTransactionType11Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NSYN") )]
	CodeNSYN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SYND") )]
	CodeSYND,
}

impl SecuritiesTransactionType11Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecuritiesTransactionType31Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionType31Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<SecuritiesTransactionType11Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl SecuritiesTransactionType31Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// SecuritiesUpdateReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesUpdateReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl SecuritiesUpdateReason1Choice {
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


// SecurityAttributes12 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityAttributes12 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmTp", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_tp: Option<Vec<FinancialInstrument97>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmAttrbts", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_attrbts: Option<Vec<CommonFinancialInstrumentAttributes12>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SecurityAttributes12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.fin_instrm_tp { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.fin_instrm_attrbts { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecurityIdentification39 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityIdentification39 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
}

impl SecurityIdentification39 {
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


// SecurityMaintenanceRequestV01 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityMaintenanceRequestV01 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgHdr", skip_serializing_if = "Option::is_none") )]
	pub msg_hdr: Option<MessageHeader1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UpdTp") )]
	pub upd_tp: UpdateType36Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UpdRsn", skip_serializing_if = "Option::is_none") )]
	pub upd_rsn: Option<SecuritiesUpdateReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId") )]
	pub fin_instrm_id: SecurityIdentification39,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SecurityMaintenanceRequestV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.msg_hdr { val.validate()? }
		self.upd_tp.validate()?;
		if let Some(ref val) = self.upd_rsn { val.validate()? }
		self.fin_instrm_id.validate()?;
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecurityRestriction3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityRestriction3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FctvPrd", skip_serializing_if = "Option::is_none") )]
	pub fctv_prd: Option<DateTimePeriod2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RstrctnTp", skip_serializing_if = "Option::is_none") )]
	pub rstrctn_tp: Option<SecurityRestrictionType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglRstrctnTp", skip_serializing_if = "Option::is_none") )]
	pub lgl_rstrctn_tp: Option<LegalRestrictions5Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrRstrctnTp", skip_serializing_if = "Option::is_none") )]
	pub invstr_rstrctn_tp: Option<Vec<InvestorRestrictionType3Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrTp", skip_serializing_if = "Option::is_none") )]
	pub invstr_tp: Option<Vec<InvestorType3Choice>>,
}

impl SecurityRestriction3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fctv_prd { val.validate()? }
		if let Some(ref val) = self.rstrctn_tp { val.validate()? }
		if let Some(ref val) = self.lgl_rstrctn_tp { val.validate()? }
		if let Some(ref vec) = self.invstr_rstrctn_tp { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.invstr_tp { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecurityRestrictionType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityRestrictionType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RstrctnTp", skip_serializing_if = "Option::is_none") )]
	pub rstrctn_tp: Option<RestrictionType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryRstrctn", skip_serializing_if = "Option::is_none") )]
	pub prtry_rstrctn: Option<GenericIdentification30>,
}

impl SecurityRestrictionType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rstrctn_tp { val.validate()? }
		if let Some(ref val) = self.prtry_rstrctn { val.validate()? }
		Ok(())
	}
}


// SecurityStatus2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum SecurityStatus2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACTV") )]
	CodeACTV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INAC") )]
	CodeINAC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SUSP") )]
	CodeSUSP,
}

impl SecurityStatus2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecurityStatus3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityStatus3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<SecurityStatus2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl SecurityStatus3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// SecurityWithHoldingTax1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityWithHoldingTax1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "WhldgTaxVal") )]
	pub whldg_tax_val: RateAndAmountFormat1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
	pub ctry: String,
}

impl SecurityWithHoldingTax1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.whldg_tax_val.validate()?;
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry) {
			return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// SettleStyle1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum SettleStyle1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SETC") )]
	CodeSETC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SETO") )]
	CodeSETO,
}

impl SettleStyle1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SettleStyle2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettleStyle2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Vec<SettleStyle1Code>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl SettleStyle2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.cd { for item in vec { item.validate()? } }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// SettlementInformation17 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementInformation17 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesQtyTp", skip_serializing_if = "Option::is_none") )]
	pub scties_qty_tp: Option<SettlementUnitType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctSttlmMnth", skip_serializing_if = "Option::is_none") )]
	pub ctrct_sttlm_mnth: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinDnmtn", skip_serializing_if = "Option::is_none") )]
	pub min_dnmtn: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinMltplQty", skip_serializing_if = "Option::is_none") )]
	pub min_mltpl_qty: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DevtgSttlmUnit", skip_serializing_if = "Option::is_none") )]
	pub devtg_sttlm_unit: Option<Vec<FinancialInstrumentQuantity1Choice>>,
}

impl SettlementInformation17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.scties_qty_tp { val.validate()? }
		if let Some(ref val) = self.min_dnmtn { val.validate()? }
		if let Some(ref val) = self.min_mltpl_qty { val.validate()? }
		if let Some(ref vec) = self.devtg_sttlm_unit { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SettlementType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum SettlementType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRIN") )]
	CodePRIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NETO") )]
	CodeNETO,
}

impl SettlementType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SettlementType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<SettlementType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl SettlementType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// SettlementUnitType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum SettlementUnitType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FAMT") )]
	CodeFAMT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UNIT") )]
	CodeUNIT,
}

impl SettlementUnitType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SettlementUnitType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementUnitType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<SettlementUnitType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl SettlementUnitType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// Standardisation1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum Standardisation1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FLEX") )]
	CodeFLEX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NSTA") )]
	CodeNSTA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STAN") )]
	CodeSTAN,
}

impl Standardisation1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Standardisation3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Standardisation3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Vec<Standardisation1Code>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl Standardisation3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.cd { for item in vec { item.validate()? } }
		if let Some(ref val) = self.prtry { val.validate()? }
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


// TEFRARules1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TEFRARules1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RULC") )]
	CodeRULC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RULD") )]
	CodeRULD,
}

impl TEFRARules1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TEFRARules3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TEFRARules3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<TEFRARules1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl TEFRARules3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// Term1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Term1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Oprtr") )]
	pub oprtr: Operator1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: RateOrAbsoluteValue1Choice,
}

impl Term1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.oprtr.validate()?;
		self.val.validate()?;
		Ok(())
	}
}


// TimeUnit1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TimeUnit1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DAYC") )]
	CodeDAYC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HOUR") )]
	CodeHOUR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MINU") )]
	CodeMINU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
	CodeMNTH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SECO") )]
	CodeSECO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
	CodeWEEK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
	CodeYEAR,
}

impl TimeUnit1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TimeUnit3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TimeUnit3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<TimeUnit1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl TimeUnit3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// TradeTransactionCondition2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TradeTransactionCondition2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPCC") )]
	CodeSPCC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SECN") )]
	CodeSECN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SEBN") )]
	CodeSEBN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCBN") )]
	CodeSCBN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCRT") )]
	CodeSCRT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SERT") )]
	CodeSERT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCCR") )]
	CodeSCCR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SECR") )]
	CodeSECR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CAST") )]
	CodeCAST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPPR") )]
	CodeSPPR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPCU") )]
	CodeSPCU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPEX") )]
	CodeSPEX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GTDL") )]
	CodeGTDL,
}

impl TradeTransactionCondition2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TradeTransactionCondition7Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradeTransactionCondition7Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<TradeTransactionCondition2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl TradeTransactionCondition7Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// TradingParameters2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradingParameters2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktId", skip_serializing_if = "Option::is_none") )]
	pub mkt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RndLot", skip_serializing_if = "Option::is_none") )]
	pub rnd_lot: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradLotSz", skip_serializing_if = "Option::is_none") )]
	pub trad_lot_sz: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ScndryPlcOfListg", skip_serializing_if = "Option::is_none") )]
	pub scndry_plc_of_listg: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinTraddNmnlQty", skip_serializing_if = "Option::is_none") )]
	pub min_tradd_nmnl_qty: Option<UnitOrFaceAmount1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MaxTraddNmnlQty", skip_serializing_if = "Option::is_none") )]
	pub max_tradd_nmnl_qty: Option<UnitOrFaceAmount1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinTradgPricgIncrmt", skip_serializing_if = "Option::is_none") )]
	pub min_tradg_pricg_incrmt: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmryPlcOfListgId", skip_serializing_if = "Option::is_none") )]
	pub pmry_plc_of_listg_id: Option<String>,
}

impl TradingParameters2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mkt_id {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "mkt_id does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.rnd_lot { val.validate()? }
		if let Some(ref val) = self.trad_lot_sz { val.validate()? }
		if let Some(ref vec) = self.scndry_plc_of_listg {
			for item in vec {
				let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "scndry_plc_of_listg does not match the required pattern".to_string()));
				}
			}
		}
		if let Some(ref val) = self.min_tradd_nmnl_qty { val.validate()? }
		if let Some(ref val) = self.max_tradd_nmnl_qty { val.validate()? }
		if let Some(ref val) = self.pmry_plc_of_listg_id {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pmry_plc_of_listg_id does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// TypeOfPrice1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TypeOfPrice1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AVER") )]
	CodeAVER,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AVOV") )]
	CodeAVOV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COMB") )]
	CodeCOMB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GREX") )]
	CodeGREX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIMI") )]
	CodeLIMI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NET2") )]
	CodeNET2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NDIS") )]
	CodeNDIS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NET1") )]
	CodeNET1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NUND") )]
	CodeNUND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOGR") )]
	CodeNOGR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PARV") )]
	CodePARV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RDAV") )]
	CodeRDAV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STOP") )]
	CodeSTOP,
}

impl TypeOfPrice1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UnderlyingAttributes4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnderlyingAttributes4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AllcnPctg", skip_serializing_if = "Option::is_none") )]
	pub allcn_pctg: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Qty", skip_serializing_if = "Option::is_none") )]
	pub qty: Option<UnitOrFaceAmount1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmTp", skip_serializing_if = "Option::is_none") )]
	pub sttlm_tp: Option<SettlementType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshAmt", skip_serializing_if = "Option::is_none") )]
	pub csh_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshTp", skip_serializing_if = "Option::is_none") )]
	pub csh_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pric", skip_serializing_if = "Option::is_none") )]
	pub pric: Option<Price8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DrtyPric", skip_serializing_if = "Option::is_none") )]
	pub drty_pric: Option<Price8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndPric", skip_serializing_if = "Option::is_none") )]
	pub end_pric: Option<Price8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartVal", skip_serializing_if = "Option::is_none") )]
	pub start_val: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CurVal", skip_serializing_if = "Option::is_none") )]
	pub cur_val: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndVal", skip_serializing_if = "Option::is_none") )]
	pub end_val: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdjstdQty", skip_serializing_if = "Option::is_none") )]
	pub adjstd_qty: Option<UnitOrFaceAmount1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRate", skip_serializing_if = "Option::is_none") )]
	pub xchg_rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CapVal", skip_serializing_if = "Option::is_none") )]
	pub cap_val: Option<ActiveCurrencyAndAmount>,
}

impl UnderlyingAttributes4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.qty { val.validate()? }
		if let Some(ref val) = self.sttlm_tp { val.validate()? }
		if let Some(ref val) = self.csh_amt { val.validate()? }
		if let Some(ref val) = self.csh_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "csh_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "csh_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.pric { val.validate()? }
		if let Some(ref val) = self.drty_pric { val.validate()? }
		if let Some(ref val) = self.end_pric { val.validate()? }
		if let Some(ref val) = self.start_val { val.validate()? }
		if let Some(ref val) = self.cur_val { val.validate()? }
		if let Some(ref val) = self.end_val { val.validate()? }
		if let Some(ref val) = self.adjstd_qty { val.validate()? }
		if let Some(ref val) = self.cap_val { val.validate()? }
		Ok(())
	}
}


// UnitOfMeasure7Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnitOfMeasure7Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<UnitOfMeasure9Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl UnitOfMeasure7Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// UnitOfMeasure9Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum UnitOfMeasure9Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BAGG") )]
	CodeBAGG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BALE") )]
	CodeBALE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOTL") )]
	CodeBOTL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOXX") )]
	CodeBOXX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRTN") )]
	CodeCRTN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CELI") )]
	CodeCELI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CMET") )]
	CodeCMET,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CNTR") )]
	CodeCNTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRAT") )]
	CodeCRAT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CBIN") )]
	CodeCBIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CBME") )]
	CodeCBME,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CBML") )]
	CodeCBML,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PIEC") )]
	CodePIEC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FOOT") )]
	CodeFOOT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBFO") )]
	CodeGBFO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBGA") )]
	CodeGBGA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBPI") )]
	CodeGBPI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBQA") )]
	CodeGBQA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBTN") )]
	CodeGBTN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GRAM") )]
	CodeGRAM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INCH") )]
	CodeINCH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KILO") )]
	CodeKILO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KMET") )]
	CodeKMET,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LITR") )]
	CodeLITR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "METR") )]
	CodeMETR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TONE") )]
	CodeTONE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MILE") )]
	CodeMILE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MMET") )]
	CodeMMET,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MILI") )]
	CodeMILI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUND") )]
	CodePUND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USOU") )]
	CodeUSOU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCMT") )]
	CodeSCMT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQFO") )]
	CodeSQFO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQIN") )]
	CodeSQIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQKI") )]
	CodeSQKI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SMET") )]
	CodeSMET,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQMI") )]
	CodeSQMI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SMIL") )]
	CodeSMIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQYA") )]
	CodeSQYA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USBA") )]
	CodeUSBA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USFO") )]
	CodeUSFO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USGA") )]
	CodeUSGA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USPI") )]
	CodeUSPI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USQA") )]
	CodeUSQA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USTN") )]
	CodeUSTN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YARD") )]
	CodeYARD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBOU") )]
	CodeGBOU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACRE") )]
	CodeACRE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ARES") )]
	CodeARES,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HECT") )]
	CodeHECT,
}

impl UnitOfMeasure9Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UnitOrFaceAmount1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnitOrFaceAmount1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
	pub unit: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none") )]
	pub face_amt: Option<ActiveCurrencyAndAmount>,
}

impl UnitOrFaceAmount1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.face_amt { val.validate()? }
		Ok(())
	}
}


// UpdateType35Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UpdateType35Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Add", skip_serializing_if = "Option::is_none") )]
	pub add: Option<SecurityAttributes12>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Del", skip_serializing_if = "Option::is_none") )]
	pub del: Option<SecurityAttributes12>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Modfy", skip_serializing_if = "Option::is_none") )]
	pub modfy: Option<SecurityAttributes12>,
}

impl UpdateType35Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.add { val.validate()? }
		if let Some(ref val) = self.del { val.validate()? }
		if let Some(ref val) = self.modfy { val.validate()? }
		Ok(())
	}
}


// UpdateType36Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UpdateType36Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UpdTp", skip_serializing_if = "Option::is_none") )]
	pub upd_tp: Option<Vec<UpdateType35Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rplc", skip_serializing_if = "Option::is_none") )]
	pub rplc: Option<SecurityAttributes12>,
}

impl UpdateType36Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.upd_tp { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rplc { val.validate()? }
		Ok(())
	}
}


// Warrant4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Warrant4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mltplr", skip_serializing_if = "Option::is_none") )]
	pub mltplr: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SbcptPric", skip_serializing_if = "Option::is_none") )]
	pub sbcpt_pric: Option<Price8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<WarrantStyle3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WarrtAgt", skip_serializing_if = "Option::is_none") )]
	pub warrt_agt: Option<Vec<Organisation38>>,
}

impl Warrant4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sbcpt_pric { val.validate()? }
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref vec) = self.warrt_agt { for item in vec { item.validate()? } }
		Ok(())
	}
}


// WarrantStyle1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum WarrantStyle1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AMER") )]
	CodeAMER,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EURO") )]
	CodeEURO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BERM") )]
	CodeBERM,
}

impl WarrantStyle1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// WarrantStyle3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct WarrantStyle3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<WarrantStyle1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl WarrantStyle3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// YieldCalculation6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct YieldCalculation6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClctnTp", skip_serializing_if = "Option::is_none") )]
	pub clctn_tp: Option<CalculationType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RedPric", skip_serializing_if = "Option::is_none") )]
	pub red_pric: Option<Price8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValDt") )]
	pub val_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValPrd") )]
	pub val_prd: DateTimePeriod1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClctnDt") )]
	pub clctn_dt: String,
}

impl YieldCalculation6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.clctn_tp { val.validate()? }
		if let Some(ref val) = self.red_pric { val.validate()? }
		self.val_prd.validate()?;
		Ok(())
	}
}
