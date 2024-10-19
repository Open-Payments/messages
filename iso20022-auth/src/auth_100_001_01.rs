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

#![allow(unused_imports)]

pub mod iso20022 {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	
	
	// ActiveCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Contact9 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Contact9 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: Max140Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PhneNb") )]
		pub phne_nb: PhoneNumber,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr") )]
		pub email_adr: Max256Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Fctn", skip_serializing_if = "Option::is_none") )]
		pub fctn: Option<Max140Text>,
	}
	
	impl Contact9 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.nm.validate() { return Err(e); }
			if let Err(e) = self.phne_nb.validate() { return Err(e); }
			if let Err(e) = self.email_adr.validate() { return Err(e); }
			if let Some(ref fctn_value) = self.fctn { if let Err(e) = fctn_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CountryCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// DatePeriod2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// ExternalFinancialInstrumentIdentificationType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ISINOct2015Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISINOct2015Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub isin_oct2015_identifier: String,
	}
	
	impl ISINOct2015Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(&self.isin_oct2015_identifier) {
				return Err(ValidationError::new(1005, "isin_oct2015_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ISODate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// IdentificationSource3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// LEIIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max140Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max16Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max2048Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max2048Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max2048_text: String,
	}
	
	impl Max2048Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max2048_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max2048_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max2048_text.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "max2048_text exceeds the maximum length of 2048".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max20PositiveDecimalNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max20PositiveDecimalNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max20_positive_decimal_number: f64,
	}
	
	impl Max20PositiveDecimalNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max20_positive_decimal_number < 0.000000 {
				return Err(ValidationError::new(1003, "max20_positive_decimal_number is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max20PositiveNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max20PositiveNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max20_positive_number: f64,
	}
	
	impl Max20PositiveNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max20_positive_number < 0.000000 {
				return Err(ValidationError::new(1003, "max20_positive_number is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max256Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max2Fraction1NonNegativeNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max2Fraction1NonNegativeNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max2_fraction1_non_negative_number: f64,
	}
	
	impl Max2Fraction1NonNegativeNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max2_fraction1_non_negative_number < 0.000000 {
				return Err(ValidationError::new(1003, "max2_fraction1_non_negative_number is less than the minimum value of 0.000000".to_string()));
			}
			if self.max2_fraction1_non_negative_number > 9.900000 {
				return Err(ValidationError::new(1004, "max2_fraction1_non_negative_number exceeds the maximum value of 9.900000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max2NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max2NumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max2_numeric_text: String,
	}
	
	impl Max2NumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{1,2}").unwrap();
			if !pattern.is_match(&self.max2_numeric_text) {
				return Err(ValidationError::new(1005, "max2_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max350Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// OtherIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// PercentageRate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ReportPeriodActivity1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecuritiesSettlementSystemIdentification2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SysId") )]
		pub sys_id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SysNm", skip_serializing_if = "Option::is_none") )]
		pub sys_nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfJursdctn", skip_serializing_if = "Option::is_none") )]
		pub ctry_of_jursdctn: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CSDLglNm", skip_serializing_if = "Option::is_none") )]
		pub csd_lgl_nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RspnsblPty", skip_serializing_if = "Option::is_none") )]
		pub rspnsbl_pty: Option<Vec<Contact9>>,
	}
	
	impl SecuritiesSettlementSystemIdentification2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.sys_id.validate() { return Err(e); }
			if let Some(ref sys_nm_value) = self.sys_nm { if let Err(e) = sys_nm_value.validate() { return Err(e); } }
			if let Some(ref ctry_of_jursdctn_value) = self.ctry_of_jursdctn { if let Err(e) = ctry_of_jursdctn_value.validate() { return Err(e); } }
			if let Some(ref csd_lgl_nm_value) = self.csd_lgl_nm { if let Err(e) = csd_lgl_nm_value.validate() { return Err(e); } }
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			if let Some(ref rspnsbl_pty_vec) = self.rspnsbl_pty { for item in rspnsbl_pty_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// SecurityIdentification19 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecurityIdentification19 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
		pub isin: Option<ISINOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
		pub othr_id: Option<Vec<OtherIdentification1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max140Text>,
	}
	
	impl SecurityIdentification19 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
			if let Some(ref othr_id_vec) = self.othr_id { for item in othr_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SettlementDailyFailureReason1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementDailyFailureReason1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Data", skip_serializing_if = "Option::is_none") )]
		pub data: Option<SettlementDailyFailureReason3>,
	}
	
	impl SettlementDailyFailureReason1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref data_set_actn_value) = self.data_set_actn { if let Err(e) = data_set_actn_value.validate() { return Err(e); } }
			if let Some(ref data_value) = self.data { if let Err(e) = data_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SettlementDailyFailureReason3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementDailyFailureReason3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FaildScties") )]
		pub faild_scties: SettlementTotalData1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FaildCsh") )]
		pub faild_csh: SettlementTotalData1Choice,
	}
	
	impl SettlementDailyFailureReason3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.faild_scties.validate() { return Err(e); }
			if let Err(e) = self.faild_csh.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SettlementDataRate2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementFailsCurrency2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
		pub ccy: ActiveCurrencyCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Data") )]
		pub data: SettlementTotalData1,
	}
	
	impl SettlementFailsCurrency2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ccy.validate() { return Err(e); }
			if let Err(e) = self.data.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SettlementFailsDailyCSD1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementFailsDailyCSD1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Data", skip_serializing_if = "Option::is_none") )]
		pub data: Option<SettlementFailsDailyCSD3>,
	}
	
	impl SettlementFailsDailyCSD1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref data_set_actn_value) = self.data_set_actn { if let Err(e) = data_set_actn_value.validate() { return Err(e); } }
			if let Some(ref data_value) = self.data { if let Err(e) = data_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SettlementFailsDailyCSD3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementFailsDailyCSD3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntraCSD") )]
		pub intra_csd: SettlementFailsDailyInstructionType1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CrossCSD") )]
		pub cross_csd: SettlementFailsDailyInstructionType1Choice,
	}
	
	impl SettlementFailsDailyCSD3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.intra_csd.validate() { return Err(e); }
			if let Err(e) = self.cross_csd.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SettlementFailsDailyData3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementFailsDailyData3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgDt") )]
		pub rptg_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DalyRcrd") )]
		pub daly_rcrd: SettlementFailsDailyInstrument3,
	}
	
	impl SettlementFailsDailyData3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.daly_rcrd.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SettlementFailsDailyInstructionType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementFailsDailyInstructionType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Data", skip_serializing_if = "Option::is_none") )]
		pub data: Option<SettlementFailsDailyInstructionType3>,
	}
	
	impl SettlementFailsDailyInstructionType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref data_set_actn_value) = self.data_set_actn { if let Err(e) = data_set_actn_value.validate() { return Err(e); } }
			if let Some(ref data_value) = self.data { if let Err(e) = data_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SettlementFailsDailyInstructionType3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Err(e) = self.dlvry_vrss_pmt.validate() { return Err(e); }
			if let Err(e) = self.dlvry_wth_pmt.validate() { return Err(e); }
			if let Err(e) = self.pmt_free_of_dlvry.validate() { return Err(e); }
			if let Err(e) = self.free_of_pmt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SettlementFailsDailyInstrument3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Err(e) = self.eqty.validate() { return Err(e); }
			if let Err(e) = self.svrgn_debt.validate() { return Err(e); }
			if let Err(e) = self.bd.validate() { return Err(e); }
			if let Err(e) = self.othr_trfbl_scties.validate() { return Err(e); }
			if let Err(e) = self.xchg_tradd_fnds.validate() { return Err(e); }
			if let Err(e) = self.cllctv_invstmt_udrtkgs.validate() { return Err(e); }
			if let Err(e) = self.mny_mkt_instrm.validate() { return Err(e); }
			if let Err(e) = self.emssn_allwnc.validate() { return Err(e); }
			if let Err(e) = self.othr.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SettlementFailsDailyTransactionType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementFailsDailyTransactionType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Data", skip_serializing_if = "Option::is_none") )]
		pub data: Option<SettlementFailsDailyTransactionType3>,
	}
	
	impl SettlementFailsDailyTransactionType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref data_set_actn_value) = self.data_set_actn { if let Err(e) = data_set_actn_value.validate() { return Err(e); } }
			if let Some(ref data_value) = self.data { if let Err(e) = data_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SettlementFailsDailyTransactionType3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Err(e) = self.scties_buy_or_sell.validate() { return Err(e); }
			if let Err(e) = self.coll_mgmt_opr.validate() { return Err(e); }
			if let Err(e) = self.scties_lndg_or_brrwg.validate() { return Err(e); }
			if let Err(e) = self.rp_agrmt.validate() { return Err(e); }
			if let Err(e) = self.othr.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SettlementFailsData3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Err(e) = self.ttl.validate() { return Err(e); }
			if let Some(ref ptcpt_in_fail_value) = self.ptcpt_in_fail { if let Err(e) = ptcpt_in_fail_value.validate() { return Err(e); } }
			if let Some(ref fls_per_ccy_vec) = self.fls_per_ccy { for item in fls_per_ccy_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref fls_per_fin_instrm_tp_value) = self.fls_per_fin_instrm_tp { if let Err(e) = fls_per_fin_instrm_tp_value.validate() { return Err(e); } }
			if let Some(ref scties_in_fail_value) = self.scties_in_fail { if let Err(e) = scties_in_fail_value.validate() { return Err(e); } }
			if let Some(ref fls_per_tx_tp_value) = self.fls_per_tx_tp { if let Err(e) = fls_per_tx_tp_value.validate() { return Err(e); } }
			if let Some(ref ttl_sttlm_pnlties_value) = self.ttl_sttlm_pnlties { if let Err(e) = ttl_sttlm_pnlties_value.validate() { return Err(e); } }
			if let Err(e) = self.failr_rsn.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SettlementFailsInstrument2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Err(e) = self.eqty.validate() { return Err(e); }
			if let Err(e) = self.svrgn_debt.validate() { return Err(e); }
			if let Err(e) = self.bd.validate() { return Err(e); }
			if let Err(e) = self.othr_trfbl_scties.validate() { return Err(e); }
			if let Err(e) = self.xchg_tradd_fnds.validate() { return Err(e); }
			if let Err(e) = self.cllctv_invstmt_udrtkgs.validate() { return Err(e); }
			if let Err(e) = self.mny_mkt_instrm.validate() { return Err(e); }
			if let Err(e) = self.emssn_allwnc.validate() { return Err(e); }
			if let Err(e) = self.othr.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SettlementFailsMonthlyReportV01 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Err(e) = self.rpt_hdr.validate() { return Err(e); }
			if let Err(e) = self.mnthly_aggt.validate() { return Err(e); }
			for item in &self.daly_data { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// SettlementFailsParticipant1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementFailsParticipant1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI") )]
		pub lei: LEIIdentifier,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rank") )]
		pub rank: Max2NumericText,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Aggt") )]
		pub aggt: SettlementTotalData1,
	}
	
	impl SettlementFailsParticipant1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.lei.validate() { return Err(e); }
			if let Err(e) = self.rank.validate() { return Err(e); }
			if let Err(e) = self.aggt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SettlementFailsParticipantRange1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementFailsParticipantRange1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "HghstInVol") )]
		pub hghst_in_vol: Vec<SettlementFailsParticipant1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HghstInVal") )]
		pub hghst_in_val: Vec<SettlementFailsParticipant1>,
	}
	
	impl SettlementFailsParticipantRange1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.hghst_in_vol { if let Err(e) = item.validate() { return Err(e); } }
			for item in &self.hghst_in_val { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SettlementFailsReportHeader2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementFailsReportHeader2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
		pub cre_dt_tm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgPrd") )]
		pub rptg_prd: DatePeriod2,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
		pub ccy: ActiveCurrencyCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptSts") )]
		pub rpt_sts: TransactionOperationType4Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesSttlmSys") )]
		pub scties_sttlm_sys: SecuritiesSettlementSystemIdentification2,
	}
	
	impl SettlementFailsReportHeader2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rptg_prd.validate() { return Err(e); }
			if let Err(e) = self.ccy.validate() { return Err(e); }
			if let Err(e) = self.rpt_sts.validate() { return Err(e); }
			if let Err(e) = self.scties_sttlm_sys.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SettlementFailsSecurities1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementFailsSecurities1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId") )]
		pub fin_instrm_id: SecurityIdentification19,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rank") )]
		pub rank: Max2NumericText,
	}
	
	impl SettlementFailsSecurities1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.fin_instrm_id.validate() { return Err(e); }
			if let Err(e) = self.rank.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SettlementFailsSecuritiesRange1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementFailsSecuritiesRange1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "HghstInVol") )]
		pub hghst_in_vol: Vec<SettlementFailsSecurities1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HghstInVal") )]
		pub hghst_in_val: Vec<SettlementFailsSecurities1>,
	}
	
	impl SettlementFailsSecuritiesRange1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.hghst_in_vol { if let Err(e) = item.validate() { return Err(e); } }
			for item in &self.hghst_in_val { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SettlementFailsTransactionType2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Err(e) = self.scties_buy_or_sell.validate() { return Err(e); }
			if let Err(e) = self.coll_mgmt_opr.validate() { return Err(e); }
			if let Err(e) = self.scties_lndg_or_brrwg.validate() { return Err(e); }
			if let Err(e) = self.rp_agrmt.validate() { return Err(e); }
			if let Err(e) = self.othr.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SettlementFailureReason2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementFailureReason2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MainRsns") )]
		pub main_rsns: Max2048Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EffcncyImprvmt") )]
		pub effcncy_imprvmt: Max2048Text,
	}
	
	impl SettlementFailureReason2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.main_rsns.validate() { return Err(e); }
			if let Err(e) = self.effcncy_imprvmt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SettlementFailureReason3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// SettlementTotalData1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementTotalData1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Data", skip_serializing_if = "Option::is_none") )]
		pub data: Option<SettlementTotalData1>,
	}
	
	impl SettlementTotalData1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref data_set_actn_value) = self.data_set_actn { if let Err(e) = data_set_actn_value.validate() { return Err(e); } }
			if let Some(ref data_value) = self.data { if let Err(e) = data_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SupplementaryData1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SupplementaryDataEnvelope1 {
	}
	
	impl SupplementaryDataEnvelope1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TransactionOperationType4Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
}