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
	
	
	// AnyBICDec2014Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
		pub nfi_sctr: Option<Vec<NACEDomainIdentifier>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
		pub not_rptd: Option<NotReported1Code>,
	}
	
	impl CorporateSectorCriteria5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref fi_sctr_vec) = self.fi_sctr { for item in fi_sctr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref nfi_sctr_vec) = self.nfi_sctr { for item in nfi_sctr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref not_rptd_value) = self.not_rptd { if let Err(e) = not_rptd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CountryCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref rg_value) = self.rg { if let Err(e) = rg_value.validate() { return Err(e); } }
			if let Some(ref not_rptd_value) = self.not_rptd { if let Err(e) = not_rptd_value.validate() { return Err(e); } }
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
	
	
	// DayOfMonthNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct DayOfMonthNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub day_of_month_number: f64,
	}
	
	impl DayOfMonthNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.day_of_month_number < 1.000000 {
				return Err(ValidationError::new(1003, "day_of_month_number is less than the minimum value of 1.000000".to_string()));
			}
			if self.day_of_month_number > 31.000000 {
				return Err(ValidationError::new(1004, "day_of_month_number exceeds the maximum value of 31.000000".to_string()));
			}
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
	
	
	// ISODate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// LEIIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// MICIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max1000Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max1000Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max1000_text: String,
	}
	
	impl Max1000Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max1000_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max1000_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max1000_text.chars().count() > 1000 {
				return Err(ValidationError::new(1002, "max1000_text exceeds the maximum length of 1000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max16Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max350Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max50Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max50Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max50_text: String,
	}
	
	impl Max50Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max50_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max50_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max50_text.chars().count() > 50 {
				return Err(ValidationError::new(1002, "max50_text exceeds the maximum length of 50".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max70Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// NACEDomainIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct NACEDomainIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub nace_domain_identifier: String,
	}
	
	impl NACEDomainIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-U]{1,1}").unwrap();
			if !pattern.is_match(&self.nace_domain_identifier) {
				return Err(ValidationError::new(1005, "nace_domain_identifier does not match the required pattern".to_string()));
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
		pub any_bic: Option<AnyBICDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none") )]
		pub lgl_ntty_idr: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
		pub nm_and_adr: Option<NameAndAddress5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<GenericIdentification1>,
	}
	
	impl PartyIdentification121Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
			if let Some(ref lgl_ntty_idr_value) = self.lgl_ntty_idr { if let Err(e) = lgl_ntty_idr_value.validate() { return Err(e); } }
			if let Some(ref nm_and_adr_value) = self.nm_and_adr { if let Err(e) = nm_and_adr_value.validate() { return Err(e); } }
			if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
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
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
		pub mic: Option<Vec<MICIdentifier>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyMIC", skip_serializing_if = "Option::is_none") )]
		pub any_mic: Option<AnyMIC1Code>,
	}
	
	impl SecuritiesTradeVenueCriteria1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref mic_vec) = self.mic { for item in mic_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref any_mic_value) = self.any_mic { if let Err(e) = any_mic_value.validate() { return Err(e); } }
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
			if let Some(ref actn_tp_vec) = self.actn_tp { for item in actn_tp_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref exctn_vn_value) = self.exctn_vn { if let Err(e) = exctn_vn_value.validate() { return Err(e); } }
			if let Some(ref ntr_of_ctr_pty_vec) = self.ntr_of_ctr_pty { for item in ntr_of_ctr_pty_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref corp_sctr_vec) = self.corp_sctr { for item in corp_sctr_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
			if let Some(ref rptg_dt_tm_value) = self.rptg_dt_tm { if let Err(e) = rptg_dt_tm_value.validate() { return Err(e); } }
			if let Some(ref exctn_dt_tm_value) = self.exctn_dt_tm { if let Err(e) = exctn_dt_tm_value.validate() { return Err(e); } }
			if let Some(ref mtrty_dt_value) = self.mtrty_dt { if let Err(e) = mtrty_dt_value.validate() { return Err(e); } }
			if let Some(ref termntn_dt_value) = self.termntn_dt { if let Err(e) = termntn_dt_value.validate() { return Err(e); } }
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
		pub lei: Option<Vec<LEIIdentifier>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<Vec<AnyBICDec2014Identifier>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClntId", skip_serializing_if = "Option::is_none") )]
		pub clnt_id: Option<Vec<Max50Text>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
		pub not_rptd: Option<NotReported1Code>,
	}
	
	impl TradePartyIdentificationQuery8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref lei_vec) = self.lei { for item in lei_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref any_bic_vec) = self.any_bic { for item in any_bic_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref clnt_id_vec) = self.clnt_id { for item in clnt_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref not_rptd_value) = self.not_rptd { if let Err(e) = not_rptd_value.validate() { return Err(e); } }
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
		pub lei: Option<Vec<LEIIdentifier>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryCd", skip_serializing_if = "Option::is_none") )]
		pub ctry_cd: Option<Vec<CountryCode>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<Vec<AnyBICDec2014Identifier>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClntId", skip_serializing_if = "Option::is_none") )]
		pub clnt_id: Option<Vec<Max50Text>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
		pub not_rptd: Option<NotReported1Code>,
	}
	
	impl TradePartyIdentificationQuery9 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref lei_vec) = self.lei { for item in lei_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ctry_cd_vec) = self.ctry_cd { for item in ctry_cd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref any_bic_vec) = self.any_bic { for item in any_bic_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref clnt_id_vec) = self.clnt_id { for item in clnt_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref not_rptd_value) = self.not_rptd { if let Err(e) = not_rptd_value.validate() { return Err(e); } }
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
			if let Some(ref rptg_ctr_pty_value) = self.rptg_ctr_pty { if let Err(e) = rptg_ctr_pty_value.validate() { return Err(e); } }
			if let Some(ref rptg_ctr_pty_brnch_value) = self.rptg_ctr_pty_brnch { if let Err(e) = rptg_ctr_pty_brnch_value.validate() { return Err(e); } }
			if let Some(ref othr_ctr_pty_value) = self.othr_ctr_pty { if let Err(e) = othr_ctr_pty_value.validate() { return Err(e); } }
			if let Some(ref othr_ctr_pty_brnch_value) = self.othr_ctr_pty_brnch { if let Err(e) = othr_ctr_pty_brnch_value.validate() { return Err(e); } }
			if let Some(ref bnfcry_value) = self.bnfcry { if let Err(e) = bnfcry_value.validate() { return Err(e); } }
			if let Some(ref submitg_agt_value) = self.submitg_agt { if let Err(e) = submitg_agt_value.validate() { return Err(e); } }
			if let Some(ref brkr_value) = self.brkr { if let Err(e) = brkr_value.validate() { return Err(e); } }
			if let Some(ref ccp_value) = self.ccp { if let Err(e) = ccp_value.validate() { return Err(e); } }
			if let Some(ref agt_lndr_value) = self.agt_lndr { if let Err(e) = agt_lndr_value.validate() { return Err(e); } }
			if let Some(ref trpty_agt_value) = self.trpty_agt { if let Err(e) = trpty_agt_value.validate() { return Err(e); } }
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
			if let Some(ref trad_pty_crit_value) = self.trad_pty_crit { if let Err(e) = trad_pty_crit_value.validate() { return Err(e); } }
			if let Some(ref trad_tp_crit_value) = self.trad_tp_crit { if let Err(e) = trad_tp_crit_value.validate() { return Err(e); } }
			if let Some(ref tm_crit_value) = self.tm_crit { if let Err(e) = tm_crit_value.validate() { return Err(e); } }
			if let Some(ref othr_crit_value) = self.othr_crit { if let Err(e) = othr_crit_value.validate() { return Err(e); } }
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
			if let Some(ref dlvry_day_vec) = self.dlvry_day { for item in dlvry_day_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
		pub qry_tp: Max1000Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy") )]
		pub frqcy: TradeQueryExecutionFrequency3,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VldUntil") )]
		pub vld_until: String,
	}
	
	impl TradeRecurrentQuery5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.qry_tp.validate() { return Err(e); }
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
			if let Some(ref ad_hoc_qry_value) = self.ad_hoc_qry { if let Err(e) = ad_hoc_qry_value.validate() { return Err(e); } }
			if let Some(ref rcrnt_qry_value) = self.rcrnt_qry { if let Err(e) = rcrnt_qry_value.validate() { return Err(e); } }
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
			if let Some(ref scties_fincg_tx_tp_vec) = self.scties_fincg_tx_tp { for item in scties_fincg_tx_tp_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref coll_cmpnt_tp_vec) = self.coll_cmpnt_tp { for item in coll_cmpnt_tp_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	
	
	// TrueFalseIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct TrueFalseIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub true_false_indicator: bool,
	}
	
	impl TrueFalseIndicator {
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