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
	
	
	// ContactDetails4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ContactDetails4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: Max140Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PhneNb") )]
		pub phne_nb: PhoneNumber,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr") )]
		pub email_adr: Max2048Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Fctn") )]
		pub fctn: Max140Text,
	}
	
	impl ContactDetails4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.nm.validate() { return Err(e); }
			if let Err(e) = self.phne_nb.validate() { return Err(e); }
			if let Err(e) = self.email_adr.validate() { return Err(e); }
			if let Err(e) = self.fctn.validate() { return Err(e); }
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
	
	
	// Exact2UpperCaseAlphaText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Exact2UpperCaseAlphaText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub exact2_upper_case_alpha_text: String,
	}
	
	impl Exact2UpperCaseAlphaText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2}").unwrap();
			if !pattern.is_match(&self.exact2_upper_case_alpha_text) {
				return Err(ValidationError::new(1005, "exact2_upper_case_alpha_text does not match the required pattern".to_string()));
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
	
	
	// InternalisationData1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct InternalisationData1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Aggt") )]
		pub aggt: InternalisationData2,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FaildRate") )]
		pub faild_rate: InternalisationDataRate1,
	}
	
	impl InternalisationData1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.aggt.validate() { return Err(e); }
			if let Err(e) = self.faild_rate.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// InternalisationData2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct InternalisationData2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sttld") )]
		pub sttld: InternalisationDataVolume1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Faild") )]
		pub faild: InternalisationDataVolume1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ttl") )]
		pub ttl: InternalisationDataVolume1,
	}
	
	impl InternalisationData2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.sttld.validate() { return Err(e); }
			if let Err(e) = self.faild.validate() { return Err(e); }
			if let Err(e) = self.ttl.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// InternalisationDataRate1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct InternalisationDataRate1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "VolPctg") )]
		pub vol_pctg: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
		pub val: f64,
	}
	
	impl InternalisationDataRate1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// InternalisationDataVolume1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct InternalisationDataVolume1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Vol") )]
		pub vol: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
		pub val: f64,
	}
	
	impl InternalisationDataVolume1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// IssuerCSDIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct IssuerCSDIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrstTwoCharsInstrmId") )]
		pub frst_two_chars_instrm_id: Exact2UpperCaseAlphaText,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
		pub ctry: Option<CountryCode>,
	}
	
	impl IssuerCSDIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			if let Err(e) = self.frst_two_chars_instrm_id.validate() { return Err(e); }
			if let Some(ref ctry_value) = self.ctry { if let Err(e) = ctry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// IssuerCSDReport1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct IssuerCSDReport1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: IssuerCSDIdentification1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OvrllTtl") )]
		pub ovrll_ttl: InternalisationData1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrm") )]
		pub fin_instrm: SettlementInternaliserFinancialInstrument1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxTp") )]
		pub tx_tp: SettlementInternaliserTransactionType1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClntTp") )]
		pub clnt_tp: SettlementInternaliserClientType1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlCshTrf") )]
		pub ttl_csh_trf: InternalisationData1,
	}
	
	impl IssuerCSDReport1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.ovrll_ttl.validate() { return Err(e); }
			if let Err(e) = self.fin_instrm.validate() { return Err(e); }
			if let Err(e) = self.tx_tp.validate() { return Err(e); }
			if let Err(e) = self.clnt_tp.validate() { return Err(e); }
			if let Err(e) = self.ttl_csh_trf.validate() { return Err(e); }
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
	
	
	// SettlementInternaliser1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementInternaliser1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: SettlementInternaliserIdentification1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OvrllTtl") )]
		pub ovrll_ttl: InternalisationData1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrm") )]
		pub fin_instrm: SettlementInternaliserFinancialInstrument1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxTp") )]
		pub tx_tp: SettlementInternaliserTransactionType1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClntTp") )]
		pub clnt_tp: SettlementInternaliserClientType1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlCshTrf") )]
		pub ttl_csh_trf: InternalisationData1,
	}
	
	impl SettlementInternaliser1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.ovrll_ttl.validate() { return Err(e); }
			if let Err(e) = self.fin_instrm.validate() { return Err(e); }
			if let Err(e) = self.tx_tp.validate() { return Err(e); }
			if let Err(e) = self.clnt_tp.validate() { return Err(e); }
			if let Err(e) = self.ttl_csh_trf.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SettlementInternaliserClientType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementInternaliserClientType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prfssnl") )]
		pub prfssnl: InternalisationData1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rtl") )]
		pub rtl: InternalisationData1,
	}
	
	impl SettlementInternaliserClientType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.prfssnl.validate() { return Err(e); }
			if let Err(e) = self.rtl.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SettlementInternaliserFinancialInstrument1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementInternaliserFinancialInstrument1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Eqty") )]
		pub eqty: InternalisationData1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvrgnDebt") )]
		pub svrgn_debt: InternalisationData1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Bd") )]
		pub bd: InternalisationData1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrTrfblScties") )]
		pub othr_trfbl_scties: InternalisationData1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XchgTradgFnds") )]
		pub xchg_tradg_fnds: InternalisationData1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CllctvInvstmtUdrtkgs") )]
		pub cllctv_invstmt_udrtkgs: InternalisationData1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MnyMktInstrm") )]
		pub mny_mkt_instrm: InternalisationData1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EmssnAllwnc") )]
		pub emssn_allwnc: InternalisationData1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrFinInstrms") )]
		pub othr_fin_instrms: InternalisationData1,
	}
	
	impl SettlementInternaliserFinancialInstrument1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.eqty.validate() { return Err(e); }
			if let Err(e) = self.svrgn_debt.validate() { return Err(e); }
			if let Err(e) = self.bd.validate() { return Err(e); }
			if let Err(e) = self.othr_trfbl_scties.validate() { return Err(e); }
			if let Err(e) = self.xchg_tradg_fnds.validate() { return Err(e); }
			if let Err(e) = self.cllctv_invstmt_udrtkgs.validate() { return Err(e); }
			if let Err(e) = self.mny_mkt_instrm.validate() { return Err(e); }
			if let Err(e) = self.emssn_allwnc.validate() { return Err(e); }
			if let Err(e) = self.othr_fin_instrms.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SettlementInternaliserIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementInternaliserIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI") )]
		pub lei: LEIIdentifier,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RspnsblPrsn") )]
		pub rspnsbl_prsn: ContactDetails4,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
		pub ctry: CountryCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BrnchId", skip_serializing_if = "Option::is_none") )]
		pub brnch_id: Option<Exact2UpperCaseAlphaText>,
	}
	
	impl SettlementInternaliserIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.lei.validate() { return Err(e); }
			if let Err(e) = self.rspnsbl_prsn.validate() { return Err(e); }
			if let Err(e) = self.ctry.validate() { return Err(e); }
			if let Some(ref brnch_id_value) = self.brnch_id { if let Err(e) = brnch_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SettlementInternaliserReportHeader1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementInternaliserReportHeader1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
		pub cre_dt_tm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgDt") )]
		pub rptg_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
		pub ccy: ActiveCurrencyCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptSts") )]
		pub rpt_sts: TransactionOperationType4Code,
	}
	
	impl SettlementInternaliserReportHeader1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ccy.validate() { return Err(e); }
			if let Err(e) = self.rpt_sts.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SettlementInternaliserReportV01 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementInternaliserReportV01 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptHdr") )]
		pub rpt_hdr: SettlementInternaliserReportHeader1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmIntlr") )]
		pub sttlm_intlr: SettlementInternaliser1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IssrCSD") )]
		pub issr_csd: Vec<IssuerCSDReport1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl SettlementInternaliserReportV01 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rpt_hdr.validate() { return Err(e); }
			if let Err(e) = self.sttlm_intlr.validate() { return Err(e); }
			for item in &self.issr_csd { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// SettlementInternaliserTransactionType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementInternaliserTransactionType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesBuyOrSell") )]
		pub scties_buy_or_sell: InternalisationData1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CollMgmtOpr") )]
		pub coll_mgmt_opr: InternalisationData1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesLndgOrBrrwg") )]
		pub scties_lndg_or_brrwg: InternalisationData1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RpAgrmt") )]
		pub rp_agrmt: InternalisationData1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrTxs") )]
		pub othr_txs: InternalisationData1,
	}
	
	impl SettlementInternaliserTransactionType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.scties_buy_or_sell.validate() { return Err(e); }
			if let Err(e) = self.coll_mgmt_opr.validate() { return Err(e); }
			if let Err(e) = self.scties_lndg_or_brrwg.validate() { return Err(e); }
			if let Err(e) = self.rp_agrmt.validate() { return Err(e); }
			if let Err(e) = self.othr_txs.validate() { return Err(e); }
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