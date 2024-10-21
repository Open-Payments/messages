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

#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub mod iso20022 {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	
	
	// ActiveCurrencyAnd13DecimalAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_currency_and13_decimal_amount_simple_type: f64,
	}
	
	impl ActiveCurrencyAnd13DecimalAmountSimpleType {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.active_currency_and13_decimal_amount_simple_type < 0.000000 {
				return Err(ValidationError::new(1003, "active_currency_and13_decimal_amount_simple_type is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ActiveCurrencyAnd13DecimalAmount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// ActiveCurrencyAndAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveCurrencyAndAmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_currency_and_amount_simple_type: f64,
	}
	
	impl ActiveCurrencyAndAmountSimpleType {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.active_currency_and_amount_simple_type < 0.000000 {
				return Err(ValidationError::new(1003, "active_currency_and_amount_simple_type is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ActiveCurrencyAndAmount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// ActiveCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveCurrencyCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_currency_code: String,
	}
	
	impl ActiveCurrencyCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.active_currency_code) {
				return Err(ValidationError::new(1005, "active_currency_code does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_or_historic_currency_and13_decimal_amount_simple_type: f64,
	}
	
	impl ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.active_or_historic_currency_and13_decimal_amount_simple_type < 0.000000 {
				return Err(ValidationError::new(1003, "active_or_historic_currency_and13_decimal_amount_simple_type is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ActiveOrHistoricCurrencyAnd13DecimalAmount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// ActiveOrHistoricCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveOrHistoricCurrencyCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_or_historic_currency_code: String,
	}
	
	impl ActiveOrHistoricCurrencyCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.active_or_historic_currency_code) {
				return Err(ValidationError::new(1005, "active_or_historic_currency_code does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// AddressType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct AmountOrPercentageRange1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Opr", skip_serializing_if = "Option::is_none") )]
		pub opr: Option<Operation1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Term", skip_serializing_if = "Option::is_none") )]
		pub term: Option<Vec<Term1>>,
	}
	
	impl AmountOrPercentageRange1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref opr_value) = self.opr { if let Err(e) = opr_value.validate() { return Err(e); } }
			if let Some(ref term_vec) = self.term { for item in term_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// AnyBICDec2014Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct AnyBICDec2014Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub any_bic_dec2014_identifier: String,
	}
	
	impl AnyBICDec2014Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(&self.any_bic_dec2014_identifier) {
				return Err(ValidationError::new(1005, "any_bic_dec2014_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Appearance1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct Appearance3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<Appearance1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl Appearance3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssignmentMethod1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct AssignmentMethod2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<AssignmentMethod1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl AssignmentMethod2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BaseOneRate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct BaseOneRate {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub base_one_rate: f64,
	}
	
	impl BaseOneRate {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BenchmarkCurve6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BenchmarkCurve6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sprd", skip_serializing_if = "Option::is_none") )]
		pub sprd: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BchmkId", skip_serializing_if = "Option::is_none") )]
		pub bchmk_id: Option<SecurityIdentification39>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BchmkPric", skip_serializing_if = "Option::is_none") )]
		pub bchmk_pric: Option<Price8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BchmkCrvCcy", skip_serializing_if = "Option::is_none") )]
		pub bchmk_crv_ccy: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BchmkCrvNm", skip_serializing_if = "Option::is_none") )]
		pub bchmk_crv_nm: Option<BenchmarkCurveName7Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BchmkCrvPt", skip_serializing_if = "Option::is_none") )]
		pub bchmk_crv_pt: Option<Max256Text>,
	}
	
	impl BenchmarkCurve6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref bchmk_id_value) = self.bchmk_id { if let Err(e) = bchmk_id_value.validate() { return Err(e); } }
			if let Some(ref bchmk_pric_value) = self.bchmk_pric { if let Err(e) = bchmk_pric_value.validate() { return Err(e); } }
			if let Some(ref bchmk_crv_ccy_value) = self.bchmk_crv_ccy { if let Err(e) = bchmk_crv_ccy_value.validate() { return Err(e); } }
			if let Some(ref bchmk_crv_nm_value) = self.bchmk_crv_nm { if let Err(e) = bchmk_crv_nm_value.validate() { return Err(e); } }
			if let Some(ref bchmk_crv_pt_value) = self.bchmk_crv_pt { if let Err(e) = bchmk_crv_pt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BenchmarkCurveName1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct BenchmarkCurveName7Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<BenchmarkCurveName1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl BenchmarkCurveName7Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CFIOct2015Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct CFIOct2015Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub cfi_oct2015_identifier: String,
	}
	
	impl CFIOct2015Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{6,6}").unwrap();
			if !pattern.is_match(&self.cfi_oct2015_identifier) {
				return Err(ValidationError::new(1005, "cfi_oct2015_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// CalculationType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct CalculationType3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<CalculationType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl CalculationType3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CallType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct CallType3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<CallType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl CallType3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ClassificationType2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ClassificationType2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none") )]
		pub clssfctn_fin_instrm: Option<CFIOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmPdctTpCd", skip_serializing_if = "Option::is_none") )]
		pub fin_instrm_pdct_tp_cd: Option<ExternalFinancialInstrumentProductType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none") )]
		pub altrn_clssfctn: Option<Vec<GenericIdentification36>>,
	}
	
	impl ClassificationType2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref clssfctn_fin_instrm_value) = self.clssfctn_fin_instrm { if let Err(e) = clssfctn_fin_instrm_value.validate() { return Err(e); } }
			if let Some(ref fin_instrm_pdct_tp_cd_value) = self.fin_instrm_pdct_tp_cd { if let Err(e) = fin_instrm_pdct_tp_cd_value.validate() { return Err(e); } }
			if let Some(ref altrn_clssfctn_vec) = self.altrn_clssfctn { for item in altrn_clssfctn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// CommonFinancialInstrumentAttributes10 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CommonFinancialInstrumentAttributes10 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SctySts", skip_serializing_if = "Option::is_none") )]
		pub scty_sts: Option<SecurityStatus3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISOSctyLngNm", skip_serializing_if = "Option::is_none") )]
		pub iso_scty_lng_nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISOSctyShrtNm", skip_serializing_if = "Option::is_none") )]
		pub iso_scty_shrt_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmVldFr", skip_serializing_if = "Option::is_none") )]
		pub nm_vld_fr: Option<DateAndDateTime2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DnmtnCcy") )]
		pub dnmtn_ccy: ActiveOrHistoricCurrencyCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CertNb", skip_serializing_if = "Option::is_none") )]
		pub cert_nb: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctVrsnNb", skip_serializing_if = "Option::is_none") )]
		pub ctrct_vrsn_nb: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CpnAttchdNb", skip_serializing_if = "Option::is_none") )]
		pub cpn_attchd_nb: Option<Max3NumericText>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxLotNb", skip_serializing_if = "Option::is_none") )]
		pub tax_lot_nb: Option<Max15NumericText>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PoolNb", skip_serializing_if = "Option::is_none") )]
		pub pool_nb: Option<Max15NumericText>,
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
		pub purp: Option<Max256Text>,
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
		pub sr_nb: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Clss", skip_serializing_if = "Option::is_none") )]
		pub clss: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WhldgTaxRgm", skip_serializing_if = "Option::is_none") )]
		pub whldg_tax_rgm: Option<Vec<SecurityWithHoldingTax1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtSts", skip_serializing_if = "Option::is_none") )]
		pub pmt_sts: Option<SecuritiesPaymentStatus5Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InitlPhysForm", skip_serializing_if = "Option::is_none") )]
		pub initl_phys_form: Option<InitialPhysicalForm4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AftrXchgPhysForm", skip_serializing_if = "Option::is_none") )]
		pub aftr_xchg_phys_form: Option<InitialPhysicalForm3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CmonSfkpr", skip_serializing_if = "Option::is_none") )]
		pub cmon_sfkpr: Option<AnyBICDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RedTp", skip_serializing_if = "Option::is_none") )]
		pub red_tp: Option<MaturityRedemptionType3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RedPmtCcy", skip_serializing_if = "Option::is_none") )]
		pub red_pmt_ccy: Option<ActiveCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none") )]
		pub rstrctn: Option<Vec<SecurityRestriction3>>,
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
	
	impl CommonFinancialInstrumentAttributes10 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref scty_sts_value) = self.scty_sts { if let Err(e) = scty_sts_value.validate() { return Err(e); } }
			if let Some(ref iso_scty_lng_nm_value) = self.iso_scty_lng_nm { if let Err(e) = iso_scty_lng_nm_value.validate() { return Err(e); } }
			if let Some(ref iso_scty_shrt_nm_value) = self.iso_scty_shrt_nm { if let Err(e) = iso_scty_shrt_nm_value.validate() { return Err(e); } }
			if let Some(ref nm_vld_fr_value) = self.nm_vld_fr { if let Err(e) = nm_vld_fr_value.validate() { return Err(e); } }
			if let Err(e) = self.dnmtn_ccy.validate() { return Err(e); }
			if let Some(ref cert_nb_value) = self.cert_nb { if let Err(e) = cert_nb_value.validate() { return Err(e); } }
			if let Some(ref cpn_attchd_nb_value) = self.cpn_attchd_nb { if let Err(e) = cpn_attchd_nb_value.validate() { return Err(e); } }
			if let Some(ref tax_lot_nb_value) = self.tax_lot_nb { if let Err(e) = tax_lot_nb_value.validate() { return Err(e); } }
			if let Some(ref pool_nb_value) = self.pool_nb { if let Err(e) = pool_nb_value.validate() { return Err(e); } }
			if let Some(ref lgl_rstrctns_value) = self.lgl_rstrctns { if let Err(e) = lgl_rstrctns_value.validate() { return Err(e); } }
			if let Some(ref pos_lmt_value) = self.pos_lmt { if let Err(e) = pos_lmt_value.validate() { return Err(e); } }
			if let Some(ref near_term_pos_lmt_value) = self.near_term_pos_lmt { if let Err(e) = near_term_pos_lmt_value.validate() { return Err(e); } }
			if let Some(ref purp_value) = self.purp { if let Err(e) = purp_value.validate() { return Err(e); } }
			if let Some(ref clssfctn_tp_value) = self.clssfctn_tp { if let Err(e) = clssfctn_tp_value.validate() { return Err(e); } }
			if let Some(ref issnc_value) = self.issnc { if let Err(e) = issnc_value.validate() { return Err(e); } }
			if let Some(ref tradg_mkt_vec) = self.tradg_mkt { for item in tradg_mkt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref sprd_and_bchmk_crv_vec) = self.sprd_and_bchmk_crv { for item in sprd_and_bchmk_crv_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref put_tp_value) = self.put_tp { if let Err(e) = put_tp_value.validate() { return Err(e); } }
			if let Some(ref call_tp_value) = self.call_tp { if let Err(e) = call_tp_value.validate() { return Err(e); } }
			if let Some(ref convs_prd_value) = self.convs_prd { if let Err(e) = convs_prd_value.validate() { return Err(e); } }
			if let Some(ref convs_ratio_nmrtr_value) = self.convs_ratio_nmrtr { if let Err(e) = convs_ratio_nmrtr_value.validate() { return Err(e); } }
			if let Some(ref convs_ratio_dnmtr_value) = self.convs_ratio_dnmtr { if let Err(e) = convs_ratio_dnmtr_value.validate() { return Err(e); } }
			if let Some(ref pmry_plc_of_dpst_value) = self.pmry_plc_of_dpst { if let Err(e) = pmry_plc_of_dpst_value.validate() { return Err(e); } }
			if let Some(ref tradg_mtd_value) = self.tradg_mtd { if let Err(e) = tradg_mtd_value.validate() { return Err(e); } }
			if let Some(ref tefra_rule_value) = self.tefra_rule { if let Err(e) = tefra_rule_value.validate() { return Err(e); } }
			if let Some(ref sr_nb_value) = self.sr_nb { if let Err(e) = sr_nb_value.validate() { return Err(e); } }
			if let Some(ref clss_value) = self.clss { if let Err(e) = clss_value.validate() { return Err(e); } }
			if let Some(ref whldg_tax_rgm_vec) = self.whldg_tax_rgm { for item in whldg_tax_rgm_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref pmt_sts_value) = self.pmt_sts { if let Err(e) = pmt_sts_value.validate() { return Err(e); } }
			if let Some(ref initl_phys_form_value) = self.initl_phys_form { if let Err(e) = initl_phys_form_value.validate() { return Err(e); } }
			if let Some(ref aftr_xchg_phys_form_value) = self.aftr_xchg_phys_form { if let Err(e) = aftr_xchg_phys_form_value.validate() { return Err(e); } }
			if let Some(ref cmon_sfkpr_value) = self.cmon_sfkpr { if let Err(e) = cmon_sfkpr_value.validate() { return Err(e); } }
			if let Some(ref red_tp_value) = self.red_tp { if let Err(e) = red_tp_value.validate() { return Err(e); } }
			if let Some(ref red_pmt_ccy_value) = self.red_pmt_ccy { if let Err(e) = red_pmt_ccy_value.validate() { return Err(e); } }
			if let Some(ref rstrctn_vec) = self.rstrctn { for item in rstrctn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref sttlm_inf_vec) = self.sttlm_inf { for item in sttlm_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref fin_instrm_form_value) = self.fin_instrm_form { if let Err(e) = fin_instrm_form_value.validate() { return Err(e); } }
			if let Some(ref ctct_nm_value) = self.ctct_nm { if let Err(e) = ctct_nm_value.validate() { return Err(e); } }
			if let Some(ref lead_mgr_value) = self.lead_mgr { if let Err(e) = lead_mgr_value.validate() { return Err(e); } }
			if let Some(ref prncpl_png_agt_value) = self.prncpl_png_agt { if let Err(e) = prncpl_png_agt_value.validate() { return Err(e); } }
			if let Some(ref png_agt_value) = self.png_agt { if let Err(e) = png_agt_value.validate() { return Err(e); } }
			if let Some(ref dpstry_value) = self.dpstry { if let Err(e) = dpstry_value.validate() { return Err(e); } }
			if let Some(ref undrlyg_rsk_value) = self.undrlyg_rsk { if let Err(e) = undrlyg_rsk_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CommunicationAddress3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CommunicationAddress3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Email", skip_serializing_if = "Option::is_none") )]
		pub email: Option<Max256Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Phne", skip_serializing_if = "Option::is_none") )]
		pub phne: Option<PhoneNumber>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mob", skip_serializing_if = "Option::is_none") )]
		pub mob: Option<PhoneNumber>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FaxNb", skip_serializing_if = "Option::is_none") )]
		pub fax_nb: Option<PhoneNumber>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TlxAdr", skip_serializing_if = "Option::is_none") )]
		pub tlx_adr: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "URLAdr", skip_serializing_if = "Option::is_none") )]
		pub url_adr: Option<Max256Text>,
	}
	
	impl CommunicationAddress3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref email_value) = self.email { if let Err(e) = email_value.validate() { return Err(e); } }
			if let Some(ref phne_value) = self.phne { if let Err(e) = phne_value.validate() { return Err(e); } }
			if let Some(ref mob_value) = self.mob { if let Err(e) = mob_value.validate() { return Err(e); } }
			if let Some(ref fax_nb_value) = self.fax_nb { if let Err(e) = fax_nb_value.validate() { return Err(e); } }
			if let Some(ref tlx_adr_value) = self.tlx_adr { if let Err(e) = tlx_adr_value.validate() { return Err(e); } }
			if let Some(ref url_adr_value) = self.url_adr { if let Err(e) = url_adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CountryCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct CountryCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub country_code: String,
	}
	
	impl CountryCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&self.country_code) {
				return Err(ValidationError::new(1005, "country_code does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// DateAndDateTime2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref dt_tm_rg_value) = self.dt_tm_rg { if let Err(e) = dt_tm_rg_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DateTimePeriod2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct Debt5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCcy", skip_serializing_if = "Option::is_none") )]
		pub pmt_ccy: Option<ActiveCurrencyCode>,
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
		pub cp_regn_tp: Option<Max350Text>,
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
		pub intrst_clctn_mtd: Option<Max70Text>,
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
		pub lot_id: Option<Max35Text>,
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
		pub geogcs: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "YldRg", skip_serializing_if = "Option::is_none") )]
		pub yld_rg: Option<AmountOrPercentageRange1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CpnRg", skip_serializing_if = "Option::is_none") )]
		pub cpn_rg: Option<AmountOrPercentageRange1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
		pub purp: Option<Max256Text>,
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
		pub pdctn: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RstrctdInd", skip_serializing_if = "Option::is_none") )]
		pub rstrctd_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PricFrqcy", skip_serializing_if = "Option::is_none") )]
		pub pric_frqcy: Option<Frequency35Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sctr", skip_serializing_if = "Option::is_none") )]
		pub sctr: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SbstitnFrqcy", skip_serializing_if = "Option::is_none") )]
		pub sbstitn_frqcy: Option<Frequency35Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SbstitnLft", skip_serializing_if = "Option::is_none") )]
		pub sbstitn_lft: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WhlPoolInd", skip_serializing_if = "Option::is_none") )]
		pub whl_pool_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PricSrc", skip_serializing_if = "Option::is_none") )]
		pub pric_src: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PricRg", skip_serializing_if = "Option::is_none") )]
		pub pric_rg: Option<AmountOrPercentageRange1>,
	}
	
	impl Debt5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref pmt_ccy_value) = self.pmt_ccy { if let Err(e) = pmt_ccy_value.validate() { return Err(e); } }
			if let Some(ref face_amt_value) = self.face_amt { if let Err(e) = face_amt_value.validate() { return Err(e); } }
			if let Some(ref pmt_frqcy_value) = self.pmt_frqcy { if let Err(e) = pmt_frqcy_value.validate() { return Err(e); } }
			if let Some(ref cp_regn_tp_value) = self.cp_regn_tp { if let Err(e) = cp_regn_tp_value.validate() { return Err(e); } }
			if let Some(ref xtndbl_prd_value) = self.xtndbl_prd { if let Err(e) = xtndbl_prd_value.validate() { return Err(e); } }
			if let Some(ref over_alltmt_amt_value) = self.over_alltmt_amt { if let Err(e) = over_alltmt_amt_value.validate() { return Err(e); } }
			if let Some(ref intrst_clctn_mtd_value) = self.intrst_clctn_mtd { if let Err(e) = intrst_clctn_mtd_value.validate() { return Err(e); } }
			if let Some(ref cptlsd_intrst_value) = self.cptlsd_intrst { if let Err(e) = cptlsd_intrst_value.validate() { return Err(e); } }
			if let Some(ref actl_dnmtn_amt_vec) = self.actl_dnmtn_amt { for item in actl_dnmtn_amt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref lot_id_value) = self.lot_id { if let Err(e) = lot_id_value.validate() { return Err(e); } }
			if let Some(ref yld_clctn_vec) = self.yld_clctn { for item in yld_clctn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref intrst_tp_value) = self.intrst_tp { if let Err(e) = intrst_tp_value.validate() { return Err(e); } }
			if let Some(ref instrm_str_tp_value) = self.instrm_str_tp { if let Err(e) = instrm_str_tp_value.validate() { return Err(e); } }
			if let Some(ref gbl_tp_value) = self.gbl_tp { if let Err(e) = gbl_tp_value.validate() { return Err(e); } }
			if let Some(ref geogcs_value) = self.geogcs { if let Err(e) = geogcs_value.validate() { return Err(e); } }
			if let Some(ref yld_rg_value) = self.yld_rg { if let Err(e) = yld_rg_value.validate() { return Err(e); } }
			if let Some(ref cpn_rg_value) = self.cpn_rg { if let Err(e) = cpn_rg_value.validate() { return Err(e); } }
			if let Some(ref purp_value) = self.purp { if let Err(e) = purp_value.validate() { return Err(e); } }
			if let Some(ref tx_conds_value) = self.tx_conds { if let Err(e) = tx_conds_value.validate() { return Err(e); } }
			if let Some(ref min_incrmt_value) = self.min_incrmt { if let Err(e) = min_incrmt_value.validate() { return Err(e); } }
			if let Some(ref min_qty_value) = self.min_qty { if let Err(e) = min_qty_value.validate() { return Err(e); } }
			if let Some(ref pdctn_value) = self.pdctn { if let Err(e) = pdctn_value.validate() { return Err(e); } }
			if let Some(ref pric_frqcy_value) = self.pric_frqcy { if let Err(e) = pric_frqcy_value.validate() { return Err(e); } }
			if let Some(ref sctr_value) = self.sctr { if let Err(e) = sctr_value.validate() { return Err(e); } }
			if let Some(ref sbstitn_frqcy_value) = self.sbstitn_frqcy { if let Err(e) = sbstitn_frqcy_value.validate() { return Err(e); } }
			if let Some(ref pric_src_value) = self.pric_src { if let Err(e) = pric_src_value.validate() { return Err(e); } }
			if let Some(ref pric_rg_value) = self.pric_rg { if let Err(e) = pric_rg_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DecimalNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct DecimalNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub decimal_number: f64,
	}
	
	impl DecimalNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Derivative4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Derivative4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Futr", skip_serializing_if = "Option::is_none") )]
		pub futr: Option<Future4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Optn", skip_serializing_if = "Option::is_none") )]
		pub optn: Option<Option15>,
	}
	
	impl Derivative4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref futr_value) = self.futr { if let Err(e) = futr_value.validate() { return Err(e); } }
			if let Some(ref optn_value) = self.optn { if let Err(e) = optn_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DistributionPolicy1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct DistributionPolicy2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<DistributionPolicy1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl DistributionPolicy2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Equity3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Err(e) = self.pref_to_incm.validate() { return Err(e); }
			if let Some(ref non_pd_amt_value) = self.non_pd_amt { if let Err(e) = non_pd_amt_value.validate() { return Err(e); } }
			if let Some(ref par_val_value) = self.par_val { if let Err(e) = par_val_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Exact4AlphaNumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Exact4AlphaNumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub exact4_alpha_numeric_text: String,
	}
	
	impl Exact4AlphaNumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
			if !pattern.is_match(&self.exact4_alpha_numeric_text) {
				return Err(ValidationError::new(1005, "exact4_alpha_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalFinancialInstrumentIdentificationType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalFinancialInstrumentIdentificationType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_financial_instrument_identification_type1_code: String,
	}
	
	impl ExternalFinancialInstrumentIdentificationType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_financial_instrument_identification_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_financial_instrument_identification_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_financial_instrument_identification_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_financial_instrument_identification_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalFinancialInstrumentProductType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalFinancialInstrumentProductType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_financial_instrument_product_type1_code: String,
	}
	
	impl ExternalFinancialInstrumentProductType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_financial_instrument_product_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_financial_instrument_product_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_financial_instrument_product_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_financial_instrument_product_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// FinancialInstrument97 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref eqty_value) = self.eqty { if let Err(e) = eqty_value.validate() { return Err(e); } }
			if let Some(ref warrt_value) = self.warrt { if let Err(e) = warrt_value.validate() { return Err(e); } }
			if let Some(ref debt_value) = self.debt { if let Err(e) = debt_value.validate() { return Err(e); } }
			if let Some(ref deriv_value) = self.deriv { if let Err(e) = deriv_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FinancialInstrumentForm2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FinancialInstrumentForm2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BookgApprnc", skip_serializing_if = "Option::is_none") )]
		pub bookg_apprnc: Option<Appearance3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LglForm", skip_serializing_if = "Option::is_none") )]
		pub lgl_form: Option<FormOfSecurity8Choice>,
	}
	
	impl FinancialInstrumentForm2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref bookg_apprnc_value) = self.bookg_apprnc { if let Err(e) = bookg_apprnc_value.validate() { return Err(e); } }
			if let Some(ref lgl_form_value) = self.lgl_form { if let Err(e) = lgl_form_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FinancialInstrumentQuantity1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			Ok(())
		}
	}
	
	
	// FormOfSecurity1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct FormOfSecurity8Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<FormOfSecurity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl FormOfSecurity8Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Frequency35Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Frequency35Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<Frequency5Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl Frequency35Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Frequency5Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref exrc_pric_value) = self.exrc_pric { if let Err(e) = exrc_pric_value.validate() { return Err(e); } }
			if let Some(ref min_sz_value) = self.min_sz { if let Err(e) = min_sz_value.validate() { return Err(e); } }
			if let Some(ref unit_of_measr_value) = self.unit_of_measr { if let Err(e) = unit_of_measr_value.validate() { return Err(e); } }
			if let Some(ref tm_unit_value) = self.tm_unit { if let Err(e) = tm_unit_value.validate() { return Err(e); } }
			if let Some(ref addtl_undrlyg_attrbts_vec) = self.addtl_undrlyg_attrbts { for item in addtl_undrlyg_attrbts_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// GenericIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericIdentification13 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification13 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max4AlphaNumericText,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: Max35Text,
	}
	
	impl GenericIdentification13 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Err(e) = self.issr.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// GenericIdentification30 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification30 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Exact4AlphaNumericText,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
	}
	
	impl GenericIdentification30 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.issr.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericIdentification36 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification36 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
	}
	
	impl GenericIdentification36 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.issr.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GlobalNote1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct GlobalNote2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<GlobalNote1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl GlobalNote2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ISIN2021Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISIN2021Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub isin2021_identifier: String,
	}
	
	impl ISIN2021Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(&self.isin2021_identifier) {
				return Err(ValidationError::new(1005, "isin2021_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ISODate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISODate {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_date: String,
	}
	
	impl ISODate {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ISODateTime ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISODateTime {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_date_time: String,
	}
	
	impl ISODateTime {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ISOYearMonth ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISOYearMonth {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_year_month: String,
	}
	
	impl ISOYearMonth {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// IdentificationSource3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct IdentificationSource3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl IdentificationSource3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ImpliedCurrencyAndAmount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ImpliedCurrencyAndAmount {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub implied_currency_and_amount: f64,
	}
	
	impl ImpliedCurrencyAndAmount {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.implied_currency_and_amount < 0.000000 {
				return Err(ValidationError::new(1003, "implied_currency_and_amount is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// InitialPhysicalForm1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct InitialPhysicalForm3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<InitialPhysicalForm2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl InitialPhysicalForm3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InitialPhysicalForm4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct InitialPhysicalForm4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<InitialPhysicalForm1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl InitialPhysicalForm4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InstrumentSubStructureType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct InstrumentSubStructureType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<InstrumentSubStructureType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl InstrumentSubStructureType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InterestType3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct InvestorRestrictionType3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<InvestorRestrictionType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl InvestorRestrictionType3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InvestorType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct InvestorType3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<InvestorType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl InvestorType3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Issuance5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Issuance5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "IssePlc", skip_serializing_if = "Option::is_none") )]
		pub isse_plc: Option<MICIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfIsse", skip_serializing_if = "Option::is_none") )]
		pub ctry_of_isse: Option<CountryCode>,
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
			if let Some(ref isse_plc_value) = self.isse_plc { if let Err(e) = isse_plc_value.validate() { return Err(e); } }
			if let Some(ref ctry_of_isse_value) = self.ctry_of_isse { if let Err(e) = ctry_of_isse_value.validate() { return Err(e); } }
			if let Some(ref issr_org_value) = self.issr_org { if let Err(e) = issr_org_value.validate() { return Err(e); } }
			if let Some(ref isse_nmnl_amt_value) = self.isse_nmnl_amt { if let Err(e) = isse_nmnl_amt_value.validate() { return Err(e); } }
			if let Some(ref full_issd_amt_value) = self.full_issd_amt { if let Err(e) = full_issd_amt_value.validate() { return Err(e); } }
			if let Some(ref isse_pric_value) = self.isse_pric { if let Err(e) = isse_pric_value.validate() { return Err(e); } }
			if let Some(ref issnc_dstrbtn_value) = self.issnc_dstrbtn { if let Err(e) = issnc_dstrbtn_value.validate() { return Err(e); } }
			if let Some(ref govng_law_vec) = self.govng_law { for item in govng_law_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// Jurisdiction1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Jurisdiction1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
		pub ctry: Option<CountryCode>,
	}
	
	impl Jurisdiction1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref ctry_value) = self.ctry { if let Err(e) = ctry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// LEIIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct LEIIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub lei_identifier: String,
	}
	
	impl LEIIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&self.lei_identifier) {
				return Err(ValidationError::new(1005, "lei_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// LegalRestrictions1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct LegalRestrictions4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<LegalRestrictions1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl LegalRestrictions4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// LegalRestrictions5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct LegalRestrictions5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<LegalRestrictions2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl LegalRestrictions5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// MICIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct MICIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub mic_identifier: String,
	}
	
	impl MICIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(&self.mic_identifier) {
				return Err(ValidationError::new(1005, "mic_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// MaturityRedemptionType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct MaturityRedemptionType3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<MaturityRedemptionType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl MaturityRedemptionType3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Max140Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max140Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max140_text: String,
	}
	
	impl Max140Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max140_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max140_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max140_text.chars().count() > 140 {
				return Err(ValidationError::new(1002, "max140_text exceeds the maximum length of 140".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max15NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max15NumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max15_numeric_text: String,
	}
	
	impl Max15NumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(&self.max15_numeric_text) {
				return Err(ValidationError::new(1005, "max15_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max16Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max16Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max16_text: String,
	}
	
	impl Max16Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max16_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max16_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max16_text.chars().count() > 16 {
				return Err(ValidationError::new(1002, "max16_text exceeds the maximum length of 16".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max256Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max256Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max256_text: String,
	}
	
	impl Max256Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max256_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max256_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max256_text.chars().count() > 256 {
				return Err(ValidationError::new(1002, "max256_text exceeds the maximum length of 256".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max350Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max350Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max350_text: String,
	}
	
	impl Max350Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max350_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max350_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max350_text.chars().count() > 350 {
				return Err(ValidationError::new(1002, "max350_text exceeds the maximum length of 350".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max35Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max35Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max35_text: String,
	}
	
	impl Max35Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max35_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max35_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max35_text.chars().count() > 35 {
				return Err(ValidationError::new(1002, "max35_text exceeds the maximum length of 35".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max3NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max3NumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max3_numeric_text: String,
	}
	
	impl Max3NumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{1,3}").unwrap();
			if !pattern.is_match(&self.max3_numeric_text) {
				return Err(ValidationError::new(1005, "max3_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max4AlphaNumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max4AlphaNumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max4_alpha_numeric_text: String,
	}
	
	impl Max4AlphaNumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max4_alpha_numeric_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max4_alpha_numeric_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max4_alpha_numeric_text.chars().count() > 4 {
				return Err(ValidationError::new(1002, "max4_alpha_numeric_text exceeds the maximum length of 4".to_string()));
			}
			let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
			if !pattern.is_match(&self.max4_alpha_numeric_text) {
				return Err(ValidationError::new(1005, "max4_alpha_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max70Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max70Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max70_text: String,
	}
	
	impl Max70Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max70_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max70_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max70_text.chars().count() > 70 {
				return Err(ValidationError::new(1002, "max70_text exceeds the maximum length of 70".to_string()));
			}
			Ok(())
		}
	}
	
	
	// MessageHeader1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct MessageHeader1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
		pub msg_id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
		pub cre_dt_tm: Option<String>,
	}
	
	impl MessageHeader1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.msg_id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// NameAndAddress4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct NameAndAddress4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Adr") )]
		pub adr: PostalAddress1,
	}
	
	impl NameAndAddress4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Err(e) = self.adr.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// NameAndAddress5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct NameAndAddress5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: Max350Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Adr", skip_serializing_if = "Option::is_none") )]
		pub adr: Option<PostalAddress1>,
	}
	
	impl NameAndAddress5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.nm.validate() { return Err(e); }
			if let Some(ref adr_value) = self.adr { if let Err(e) = adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Number ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Number {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub number: f64,
	}
	
	impl Number {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Operation1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
		pub xpry_lctn: Option<Max4AlphaNumericText>,
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
			if let Some(ref optn_sttlm_style_value) = self.optn_sttlm_style { if let Err(e) = optn_sttlm_style_value.validate() { return Err(e); } }
			if let Some(ref strk_pric_value) = self.strk_pric { if let Err(e) = strk_pric_value.validate() { return Err(e); } }
			if let Some(ref min_exrcbl_qty_value) = self.min_exrcbl_qty { if let Err(e) = min_exrcbl_qty_value.validate() { return Err(e); } }
			if let Some(ref convs_prd_value) = self.convs_prd { if let Err(e) = convs_prd_value.validate() { return Err(e); } }
			if let Some(ref optn_style_value) = self.optn_style { if let Err(e) = optn_style_value.validate() { return Err(e); } }
			if let Some(ref optn_tp_value) = self.optn_tp { if let Err(e) = optn_tp_value.validate() { return Err(e); } }
			if let Some(ref instrm_assgnmt_mtd_value) = self.instrm_assgnmt_mtd { if let Err(e) = instrm_assgnmt_mtd_value.validate() { return Err(e); } }
			if let Some(ref xpry_lctn_value) = self.xpry_lctn { if let Err(e) = xpry_lctn_value.validate() { return Err(e); } }
			if let Some(ref stdstn_value) = self.stdstn { if let Err(e) = stdstn_value.validate() { return Err(e); } }
			if let Some(ref tradg_pty_role_value) = self.tradg_pty_role { if let Err(e) = tradg_pty_role_value.validate() { return Err(e); } }
			if let Some(ref addtl_undrlyg_attrbts_vec) = self.addtl_undrlyg_attrbts { for item in addtl_undrlyg_attrbts_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// OptionParty1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct OptionParty3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<Vec<OptionParty1Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl OptionParty3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_vec) = self.cd { for item in cd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OptionStyle1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct OptionStyle1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<OptionStyle1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification13>,
	}
	
	impl OptionStyle1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OptionStyle1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct OptionType8Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<Vec<OptionType1Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl OptionType8Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_vec) = self.cd { for item in cd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Organisation38 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Organisation38 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: Max140Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<PartyIdentification177Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
		pub purp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxtnCtry", skip_serializing_if = "Option::is_none") )]
		pub taxtn_ctry: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RegnCtry", skip_serializing_if = "Option::is_none") )]
		pub regn_ctry: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RegnDt", skip_serializing_if = "Option::is_none") )]
		pub regn_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxIdNb", skip_serializing_if = "Option::is_none") )]
		pub tax_id_nb: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtlRegnNb", skip_serializing_if = "Option::is_none") )]
		pub ntl_regn_nb: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr") )]
		pub pstl_adr: Vec<PostalAddress3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmryComAdr", skip_serializing_if = "Option::is_none") )]
		pub pmry_com_adr: Option<CommunicationAddress3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ScndryComAdr", skip_serializing_if = "Option::is_none") )]
		pub scndry_com_adr: Option<CommunicationAddress3>,
	}
	
	impl Organisation38 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.nm.validate() { return Err(e); }
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref purp_value) = self.purp { if let Err(e) = purp_value.validate() { return Err(e); } }
			if let Some(ref taxtn_ctry_value) = self.taxtn_ctry { if let Err(e) = taxtn_ctry_value.validate() { return Err(e); } }
			if let Some(ref regn_ctry_value) = self.regn_ctry { if let Err(e) = regn_ctry_value.validate() { return Err(e); } }
			if let Some(ref tax_id_nb_value) = self.tax_id_nb { if let Err(e) = tax_id_nb_value.validate() { return Err(e); } }
			if let Some(ref ntl_regn_nb_value) = self.ntl_regn_nb { if let Err(e) = ntl_regn_nb_value.validate() { return Err(e); } }
			for item in &self.pstl_adr { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref pmry_com_adr_value) = self.pmry_com_adr { if let Err(e) = pmry_com_adr_value.validate() { return Err(e); } }
			if let Some(ref scndry_com_adr_value) = self.scndry_com_adr { if let Err(e) = scndry_com_adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OtherIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct OtherIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sfx", skip_serializing_if = "Option::is_none") )]
		pub sfx: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: IdentificationSource3Choice,
	}
	
	impl OtherIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref sfx_value) = self.sfx { if let Err(e) = sfx_value.validate() { return Err(e); } }
			if let Err(e) = self.tp.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// PartyIdentification120Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PartyIdentification120Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<AnyBICDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<GenericIdentification36>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
		pub nm_and_adr: Option<NameAndAddress5>,
	}
	
	impl PartyIdentification120Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
			if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
			if let Some(ref nm_and_adr_value) = self.nm_and_adr { if let Err(e) = nm_and_adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyIdentification136 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PartyIdentification136 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: PartyIdentification120Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
	}
	
	impl PartyIdentification136 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyIdentification177Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PartyIdentification177Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<AnyBICDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<GenericIdentification1>,
	}
	
	impl PartyIdentification177Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
			if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaymentDirectionIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct PaymentDirectionIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub payment_direction_indicator: bool,
	}
	
	impl PaymentDirectionIndicator {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PercentageRate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct PercentageRate {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub percentage_rate: f64,
	}
	
	impl PercentageRate {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PhoneNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct PhoneNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub phone_number: String,
	}
	
	impl PhoneNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(&self.phone_number) {
				return Err(ValidationError::new(1005, "phone_number does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// PostalAddress1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PostalAddress1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
		pub adr_tp: Option<AddressType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
		pub adr_line: Option<Vec<Max70Text>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
		pub strt_nm: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
		pub bldg_nb: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
		pub pst_cd: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
		pub twn_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
		pub ctry_sub_dvsn: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
		pub ctry: CountryCode,
	}
	
	impl PostalAddress1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref adr_tp_value) = self.adr_tp { if let Err(e) = adr_tp_value.validate() { return Err(e); } }
			if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref strt_nm_value) = self.strt_nm { if let Err(e) = strt_nm_value.validate() { return Err(e); } }
			if let Some(ref bldg_nb_value) = self.bldg_nb { if let Err(e) = bldg_nb_value.validate() { return Err(e); } }
			if let Some(ref pst_cd_value) = self.pst_cd { if let Err(e) = pst_cd_value.validate() { return Err(e); } }
			if let Some(ref twn_nm_value) = self.twn_nm { if let Err(e) = twn_nm_value.validate() { return Err(e); } }
			if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if let Err(e) = ctry_sub_dvsn_value.validate() { return Err(e); } }
			if let Err(e) = self.ctry.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// PostalAddress3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Err(e) = self.adr_tp.validate() { return Err(e); }
			if let Err(e) = self.nm_and_adr.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// PreferenceToIncome1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct PreferenceToIncome5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<PreferenceToIncome1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl PreferenceToIncome5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Price8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref val_tp_value) = self.val_tp { if let Err(e) = val_tp_value.validate() { return Err(e); } }
			if let Err(e) = self.val.validate() { return Err(e); }
			if let Some(ref pric_tp_value) = self.pric_tp { if let Err(e) = pric_tp_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PriceRateOrAmount3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PriceRateOrAmount3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
		pub rate: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
	}
	
	impl PriceRateOrAmount3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PriceValue1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PriceValue1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveCurrencyAnd13DecimalAmount,
	}
	
	impl PriceValue1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// PriceValueType3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct PutType3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<PutType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl PutType3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RateAndAmountFormat1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
			if let Some(ref not_spcfd_rate_value) = self.not_spcfd_rate { if let Err(e) = not_spcfd_rate_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RateOrAbsoluteValue1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct RateType12FormatChoice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<RateType12Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification13>,
	}
	
	impl RateType12FormatChoice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RestrictionType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct SecuritiesPaymentStatus5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<SecuritiesPaymentStatus1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl SecuritiesPaymentStatus5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SecuritiesTransactionType11Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct SecuritiesTransactionType31Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<SecuritiesTransactionType11Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl SecuritiesTransactionType31Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SecurityAttributes10 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecurityAttributes10 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId") )]
		pub fin_instrm_id: SecurityIdentification39,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmTp", skip_serializing_if = "Option::is_none") )]
		pub fin_instrm_tp: Option<Vec<FinancialInstrument97>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmAttrbts", skip_serializing_if = "Option::is_none") )]
		pub fin_instrm_attrbts: Option<Vec<CommonFinancialInstrumentAttributes10>>,
	}
	
	impl SecurityAttributes10 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.fin_instrm_id.validate() { return Err(e); }
			if let Some(ref fin_instrm_tp_vec) = self.fin_instrm_tp { for item in fin_instrm_tp_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref fin_instrm_attrbts_vec) = self.fin_instrm_attrbts { for item in fin_instrm_attrbts_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// SecurityCreationRequestV01 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecurityCreationRequestV01 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgHdr", skip_serializing_if = "Option::is_none") )]
		pub msg_hdr: Option<MessageHeader1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Scty") )]
		pub scty: SecurityAttributes10,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl SecurityCreationRequestV01 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref msg_hdr_value) = self.msg_hdr { if let Err(e) = msg_hdr_value.validate() { return Err(e); } }
			if let Err(e) = self.scty.validate() { return Err(e); }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// SecurityIdentification39 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecurityIdentification39 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
		pub isin: Option<ISIN2021Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
		pub othr_id: Option<Vec<OtherIdentification1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max140Text>,
	}
	
	impl SecurityIdentification39 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
			if let Some(ref othr_id_vec) = self.othr_id { for item in othr_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SecurityRestriction3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref fctv_prd_value) = self.fctv_prd { if let Err(e) = fctv_prd_value.validate() { return Err(e); } }
			if let Some(ref rstrctn_tp_value) = self.rstrctn_tp { if let Err(e) = rstrctn_tp_value.validate() { return Err(e); } }
			if let Some(ref lgl_rstrctn_tp_value) = self.lgl_rstrctn_tp { if let Err(e) = lgl_rstrctn_tp_value.validate() { return Err(e); } }
			if let Some(ref invstr_rstrctn_tp_vec) = self.invstr_rstrctn_tp { for item in invstr_rstrctn_tp_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref invstr_tp_vec) = self.invstr_tp { for item in invstr_tp_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// SecurityRestrictionType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecurityRestrictionType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RstrctnTp", skip_serializing_if = "Option::is_none") )]
		pub rstrctn_tp: Option<RestrictionType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryRstrctn", skip_serializing_if = "Option::is_none") )]
		pub prtry_rstrctn: Option<GenericIdentification30>,
	}
	
	impl SecurityRestrictionType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rstrctn_tp_value) = self.rstrctn_tp { if let Err(e) = rstrctn_tp_value.validate() { return Err(e); } }
			if let Some(ref prtry_rstrctn_value) = self.prtry_rstrctn { if let Err(e) = prtry_rstrctn_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SecurityStatus2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct SecurityStatus3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<SecurityStatus2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl SecurityStatus3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SecurityWithHoldingTax1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecurityWithHoldingTax1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "WhldgTaxVal") )]
		pub whldg_tax_val: RateAndAmountFormat1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
		pub ctry: CountryCode,
	}
	
	impl SecurityWithHoldingTax1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.whldg_tax_val.validate() { return Err(e); }
			if let Err(e) = self.ctry.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SettleStyle1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct SettleStyle2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<Vec<SettleStyle1Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl SettleStyle2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_vec) = self.cd { for item in cd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SettlementInformation17 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref scties_qty_tp_value) = self.scties_qty_tp { if let Err(e) = scties_qty_tp_value.validate() { return Err(e); } }
			if let Some(ref min_dnmtn_value) = self.min_dnmtn { if let Err(e) = min_dnmtn_value.validate() { return Err(e); } }
			if let Some(ref min_mltpl_qty_value) = self.min_mltpl_qty { if let Err(e) = min_mltpl_qty_value.validate() { return Err(e); } }
			if let Some(ref devtg_sttlm_unit_vec) = self.devtg_sttlm_unit { for item in devtg_sttlm_unit_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// SettlementType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct SettlementType3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<SettlementType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl SettlementType3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SettlementUnitType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct SettlementUnitType3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<SettlementUnitType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl SettlementUnitType3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Standardisation1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct Standardisation3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<Vec<Standardisation1Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl Standardisation3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_vec) = self.cd { for item in cd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SupplementaryData1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SupplementaryData1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none") )]
		pub plc_and_nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Envlp") )]
		pub envlp: SupplementaryDataEnvelope1,
	}
	
	impl SupplementaryData1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref plc_and_nm_value) = self.plc_and_nm { if let Err(e) = plc_and_nm_value.validate() { return Err(e); } }
			if let Err(e) = self.envlp.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SupplementaryDataEnvelope1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct TEFRARules3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<TEFRARules1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl TEFRARules3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Term1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Term1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Oprtr") )]
		pub oprtr: Operator1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
		pub val: RateOrAbsoluteValue1Choice,
	}
	
	impl Term1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.oprtr.validate() { return Err(e); }
			if let Err(e) = self.val.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// TimeUnit1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct TimeUnit3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<TimeUnit1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl TimeUnit3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradeTransactionCondition2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct TradeTransactionCondition7Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<TradeTransactionCondition2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl TradeTransactionCondition7Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradingParameters2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TradingParameters2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MktId", skip_serializing_if = "Option::is_none") )]
		pub mkt_id: Option<MICIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RndLot", skip_serializing_if = "Option::is_none") )]
		pub rnd_lot: Option<FinancialInstrumentQuantity1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradLotSz", skip_serializing_if = "Option::is_none") )]
		pub trad_lot_sz: Option<FinancialInstrumentQuantity1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ScndryPlcOfListg", skip_serializing_if = "Option::is_none") )]
		pub scndry_plc_of_listg: Option<Vec<MICIdentifier>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MinTraddNmnlQty", skip_serializing_if = "Option::is_none") )]
		pub min_tradd_nmnl_qty: Option<UnitOrFaceAmount1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MaxTraddNmnlQty", skip_serializing_if = "Option::is_none") )]
		pub max_tradd_nmnl_qty: Option<UnitOrFaceAmount1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MinTradgPricgIncrmt", skip_serializing_if = "Option::is_none") )]
		pub min_tradg_pricg_incrmt: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmryPlcOfListgId", skip_serializing_if = "Option::is_none") )]
		pub pmry_plc_of_listg_id: Option<MICIdentifier>,
	}
	
	impl TradingParameters2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref mkt_id_value) = self.mkt_id { if let Err(e) = mkt_id_value.validate() { return Err(e); } }
			if let Some(ref rnd_lot_value) = self.rnd_lot { if let Err(e) = rnd_lot_value.validate() { return Err(e); } }
			if let Some(ref trad_lot_sz_value) = self.trad_lot_sz { if let Err(e) = trad_lot_sz_value.validate() { return Err(e); } }
			if let Some(ref scndry_plc_of_listg_vec) = self.scndry_plc_of_listg { for item in scndry_plc_of_listg_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref min_tradd_nmnl_qty_value) = self.min_tradd_nmnl_qty { if let Err(e) = min_tradd_nmnl_qty_value.validate() { return Err(e); } }
			if let Some(ref max_tradd_nmnl_qty_value) = self.max_tradd_nmnl_qty { if let Err(e) = max_tradd_nmnl_qty_value.validate() { return Err(e); } }
			if let Some(ref pmry_plc_of_listg_id_value) = self.pmry_plc_of_listg_id { if let Err(e) = pmry_plc_of_listg_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TypeOfPrice1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
		pub csh_tp: Option<Max35Text>,
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
			if let Some(ref qty_value) = self.qty { if let Err(e) = qty_value.validate() { return Err(e); } }
			if let Some(ref sttlm_tp_value) = self.sttlm_tp { if let Err(e) = sttlm_tp_value.validate() { return Err(e); } }
			if let Some(ref csh_amt_value) = self.csh_amt { if let Err(e) = csh_amt_value.validate() { return Err(e); } }
			if let Some(ref csh_tp_value) = self.csh_tp { if let Err(e) = csh_tp_value.validate() { return Err(e); } }
			if let Some(ref pric_value) = self.pric { if let Err(e) = pric_value.validate() { return Err(e); } }
			if let Some(ref drty_pric_value) = self.drty_pric { if let Err(e) = drty_pric_value.validate() { return Err(e); } }
			if let Some(ref end_pric_value) = self.end_pric { if let Err(e) = end_pric_value.validate() { return Err(e); } }
			if let Some(ref start_val_value) = self.start_val { if let Err(e) = start_val_value.validate() { return Err(e); } }
			if let Some(ref cur_val_value) = self.cur_val { if let Err(e) = cur_val_value.validate() { return Err(e); } }
			if let Some(ref end_val_value) = self.end_val { if let Err(e) = end_val_value.validate() { return Err(e); } }
			if let Some(ref adjstd_qty_value) = self.adjstd_qty { if let Err(e) = adjstd_qty_value.validate() { return Err(e); } }
			if let Some(ref cap_val_value) = self.cap_val { if let Err(e) = cap_val_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// UnitOfMeasure7Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct UnitOfMeasure7Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<UnitOfMeasure9Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl UnitOfMeasure7Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// UnitOfMeasure9Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct UnitOrFaceAmount1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
		pub unit: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none") )]
		pub face_amt: Option<ActiveCurrencyAndAmount>,
	}
	
	impl UnitOrFaceAmount1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref face_amt_value) = self.face_amt { if let Err(e) = face_amt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Warrant4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref sbcpt_pric_value) = self.sbcpt_pric { if let Err(e) = sbcpt_pric_value.validate() { return Err(e); } }
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref warrt_agt_vec) = self.warrt_agt { for item in warrt_agt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// WarrantStyle1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct WarrantStyle3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<WarrantStyle1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl WarrantStyle3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// YesNoIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct YesNoIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub yes_no_indicator: bool,
	}
	
	impl YesNoIndicator {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// YieldCalculation6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref clctn_tp_value) = self.clctn_tp { if let Err(e) = clctn_tp_value.validate() { return Err(e); } }
			if let Some(ref red_pric_value) = self.red_pric { if let Err(e) = red_pric_value.validate() { return Err(e); } }
			if let Err(e) = self.val_prd.validate() { return Err(e); }
			Ok(())
		}
	}
	
}