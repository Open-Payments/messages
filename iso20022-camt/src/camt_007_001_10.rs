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
	
	
	// AddressType3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AddressType3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<AddressType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl AddressType3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BICFIDec2014Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct BICFIDec2014Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub bicfi_dec2014_identifier: String,
	}
	
	impl BICFIDec2014Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(&self.bicfi_dec2014_identifier) {
				return Err(ValidationError::new(1005, "bicfi_dec2014_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// BranchAndFinancialInstitutionIdentification8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BranchAndFinancialInstitutionIdentification8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstnId") )]
		pub fin_instn_id: FinancialInstitutionIdentification23,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BrnchId", skip_serializing_if = "Option::is_none") )]
		pub brnch_id: Option<BranchData5>,
	}
	
	impl BranchAndFinancialInstitutionIdentification8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.fin_instn_id.validate() { return Err(e); }
			if let Some(ref brnch_id_value) = self.brnch_id { if let Err(e) = brnch_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BranchData5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BranchData5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress27>,
	}
	
	impl BranchData5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ClearingSystemIdentification2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ClearingSystemIdentification2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalClearingSystemIdentification1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl ClearingSystemIdentification2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ClearingSystemMemberIdentification2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ClearingSystemMemberIdentification2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none") )]
		pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MmbId") )]
		pub mmb_id: Max35Text,
	}
	
	impl ClearingSystemMemberIdentification2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref clr_sys_id_value) = self.clr_sys_id { if let Err(e) = clr_sys_id_value.validate() { return Err(e); } }
			if let Err(e) = self.mmb_id.validate() { return Err(e); }
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
	
	
	// EntryTypeIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct EntryTypeIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub entry_type_identifier: String,
	}
	
	impl EntryTypeIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[BEOVW]{1,1}[0-9]{2,2}|DUM").unwrap();
			if !pattern.is_match(&self.entry_type_identifier) {
				return Err(ValidationError::new(1005, "entry_type_identifier does not match the required pattern".to_string()));
			}
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
	
	
	// ExternalClearingSystemIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalClearingSystemIdentification1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_clearing_system_identification1_code: String,
	}
	
	impl ExternalClearingSystemIdentification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_clearing_system_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_clearing_system_identification1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_clearing_system_identification1_code.chars().count() > 5 {
				return Err(ValidationError::new(1002, "external_clearing_system_identification1_code exceeds the maximum length of 5".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalFinancialInstitutionIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalFinancialInstitutionIdentification1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_financial_institution_identification1_code: String,
	}
	
	impl ExternalFinancialInstitutionIdentification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_financial_institution_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_financial_institution_identification1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_financial_institution_identification1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_financial_institution_identification1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// FinancialIdentificationSchemeName1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FinancialIdentificationSchemeName1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalFinancialInstitutionIdentification1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl FinancialIdentificationSchemeName1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FinancialInstitutionIdentification23 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FinancialInstitutionIdentification23 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BICFI", skip_serializing_if = "Option::is_none") )]
		pub bicfi: Option<BICFIDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none") )]
		pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress27>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<GenericFinancialIdentification1>,
	}
	
	impl FinancialInstitutionIdentification23 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref bicfi_value) = self.bicfi { if let Err(e) = bicfi_value.validate() { return Err(e); } }
			if let Some(ref clr_sys_mmb_id_value) = self.clr_sys_mmb_id { if let Err(e) = clr_sys_mmb_id_value.validate() { return Err(e); } }
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericFinancialIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericFinancialIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericFinancialIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
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
	
	
	// Instruction1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum Instruction1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PBEN") )]
		CodePBEN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TTIL") )]
		CodeTTIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TFRO") )]
		CodeTFRO,
	}
	
	impl Instruction1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// LongPaymentIdentification4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct LongPaymentIdentification4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
		pub tx_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
		pub uetr: Option<UUIDv4Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmAmt") )]
		pub intr_bk_sttlm_amt: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt") )]
		pub intr_bk_sttlm_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none") )]
		pub pmt_mtd: Option<PaymentOrigin1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt") )]
		pub instg_agt: BranchAndFinancialInstitutionIdentification8,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAgt") )]
		pub instd_agt: BranchAndFinancialInstitutionIdentification8,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtryTp", skip_serializing_if = "Option::is_none") )]
		pub ntry_tp: Option<EntryTypeIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none") )]
		pub end_to_end_id: Option<Max35Text>,
	}
	
	impl LongPaymentIdentification4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tx_id_value) = self.tx_id { if let Err(e) = tx_id_value.validate() { return Err(e); } }
			if let Some(ref uetr_value) = self.uetr { if let Err(e) = uetr_value.validate() { return Err(e); } }
			if let Some(ref pmt_mtd_value) = self.pmt_mtd { if let Err(e) = pmt_mtd_value.validate() { return Err(e); } }
			if let Err(e) = self.instg_agt.validate() { return Err(e); }
			if let Err(e) = self.instd_agt.validate() { return Err(e); }
			if let Some(ref ntry_tp_value) = self.ntry_tp { if let Err(e) = ntry_tp_value.validate() { return Err(e); } }
			if let Some(ref end_to_end_id_value) = self.end_to_end_id { if let Err(e) = end_to_end_id_value.validate() { return Err(e); } }
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
	
	
	// ModifyTransactionV10 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ModifyTransactionV10 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgHdr") )]
		pub msg_hdr: MessageHeader1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mod") )]
		pub mod_attr: Vec<TransactionModification7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl ModifyTransactionV10 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.msg_hdr.validate() { return Err(e); }
			for item in &self.mod_attr { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// PaymentIdentification8Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PaymentIdentification8Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
		pub tx_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
		pub uetr: Option<UUIDv4Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QId", skip_serializing_if = "Option::is_none") )]
		pub q_id: Option<QueueTransactionIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LngBizId", skip_serializing_if = "Option::is_none") )]
		pub lng_biz_id: Option<LongPaymentIdentification4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtBizId", skip_serializing_if = "Option::is_none") )]
		pub shrt_biz_id: Option<ShortPaymentIdentification4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<Max70Text>,
	}
	
	impl PaymentIdentification8Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tx_id_value) = self.tx_id { if let Err(e) = tx_id_value.validate() { return Err(e); } }
			if let Some(ref uetr_value) = self.uetr { if let Err(e) = uetr_value.validate() { return Err(e); } }
			if let Some(ref q_id_value) = self.q_id { if let Err(e) = q_id_value.validate() { return Err(e); } }
			if let Some(ref lng_biz_id_value) = self.lng_biz_id { if let Err(e) = lng_biz_id_value.validate() { return Err(e); } }
			if let Some(ref shrt_biz_id_value) = self.shrt_biz_id { if let Err(e) = shrt_biz_id_value.validate() { return Err(e); } }
			if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaymentInstruction33 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PaymentInstruction33 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Instr", skip_serializing_if = "Option::is_none") )]
		pub instr: Option<Instruction1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<PaymentType4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prty", skip_serializing_if = "Option::is_none") )]
		pub prty: Option<Priority1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgVldtyTm", skip_serializing_if = "Option::is_none") )]
		pub prcg_vldty_tm: Option<DateTimePeriod1Choice>,
	}
	
	impl PaymentInstruction33 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref instr_value) = self.instr { if let Err(e) = instr_value.validate() { return Err(e); } }
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref prty_value) = self.prty { if let Err(e) = prty_value.validate() { return Err(e); } }
			if let Some(ref prcg_vldty_tm_value) = self.prcg_vldty_tm { if let Err(e) = prcg_vldty_tm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaymentInstrument1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum PaymentInstrument1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "BDT") )]
		CodeBDT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BCT") )]
		CodeBCT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CDT") )]
		CodeCDT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CCT") )]
		CodeCCT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CHK") )]
		CodeCHK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BKT") )]
		CodeBKT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DCP") )]
		CodeDCP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CCP") )]
		CodeCCP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RTI") )]
		CodeRTI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CAN") )]
		CodeCAN,
	}
	
	impl PaymentInstrument1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PaymentOrigin1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PaymentOrigin1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FINMT", skip_serializing_if = "Option::is_none") )]
		pub finmt: Option<Max3NumericText>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XMLMsgNm", skip_serializing_if = "Option::is_none") )]
		pub xml_msg_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Instrm", skip_serializing_if = "Option::is_none") )]
		pub instrm: Option<PaymentInstrument1Code>,
	}
	
	impl PaymentOrigin1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref finmt_value) = self.finmt { if let Err(e) = finmt_value.validate() { return Err(e); } }
			if let Some(ref xml_msg_nm_value) = self.xml_msg_nm { if let Err(e) = xml_msg_nm_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			if let Some(ref instrm_value) = self.instrm { if let Err(e) = instrm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaymentType3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum PaymentType3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CBS") )]
		CodeCBS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BCK") )]
		CodeBCK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BAL") )]
		CodeBAL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLS") )]
		CodeCLS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CTR") )]
		CodeCTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CBH") )]
		CodeCBH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CBP") )]
		CodeCBP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DPG") )]
		CodeDPG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DPN") )]
		CodeDPN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EXP") )]
		CodeEXP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TCH") )]
		CodeTCH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LMT") )]
		CodeLMT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LIQ") )]
		CodeLIQ,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DPP") )]
		CodeDPP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DPH") )]
		CodeDPH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DPS") )]
		CodeDPS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "STF") )]
		CodeSTF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRP") )]
		CodeTRP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TCS") )]
		CodeTCS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LOA") )]
		CodeLOA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LOR") )]
		CodeLOR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TCP") )]
		CodeTCP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OND") )]
		CodeOND,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MGL") )]
		CodeMGL,
	}
	
	impl PaymentType3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PaymentType4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PaymentType4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<PaymentType3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl PaymentType4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PostalAddress27 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PostalAddress27 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
		pub adr_tp: Option<AddressType3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CareOf", skip_serializing_if = "Option::is_none") )]
		pub care_of: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dept", skip_serializing_if = "Option::is_none") )]
		pub dept: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubDept", skip_serializing_if = "Option::is_none") )]
		pub sub_dept: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
		pub strt_nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
		pub bldg_nb: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNm", skip_serializing_if = "Option::is_none") )]
		pub bldg_nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Flr", skip_serializing_if = "Option::is_none") )]
		pub flr: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitNb", skip_serializing_if = "Option::is_none") )]
		pub unit_nb: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstBx", skip_serializing_if = "Option::is_none") )]
		pub pst_bx: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Room", skip_serializing_if = "Option::is_none") )]
		pub room: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
		pub pst_cd: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
		pub twn_nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none") )]
		pub twn_lctn_nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none") )]
		pub dstrct_nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
		pub ctry_sub_dvsn: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
		pub ctry: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
		pub adr_line: Option<Vec<Max70Text>>,
	}
	
	impl PostalAddress27 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref adr_tp_value) = self.adr_tp { if let Err(e) = adr_tp_value.validate() { return Err(e); } }
			if let Some(ref care_of_value) = self.care_of { if let Err(e) = care_of_value.validate() { return Err(e); } }
			if let Some(ref dept_value) = self.dept { if let Err(e) = dept_value.validate() { return Err(e); } }
			if let Some(ref sub_dept_value) = self.sub_dept { if let Err(e) = sub_dept_value.validate() { return Err(e); } }
			if let Some(ref strt_nm_value) = self.strt_nm { if let Err(e) = strt_nm_value.validate() { return Err(e); } }
			if let Some(ref bldg_nb_value) = self.bldg_nb { if let Err(e) = bldg_nb_value.validate() { return Err(e); } }
			if let Some(ref bldg_nm_value) = self.bldg_nm { if let Err(e) = bldg_nm_value.validate() { return Err(e); } }
			if let Some(ref flr_value) = self.flr { if let Err(e) = flr_value.validate() { return Err(e); } }
			if let Some(ref unit_nb_value) = self.unit_nb { if let Err(e) = unit_nb_value.validate() { return Err(e); } }
			if let Some(ref pst_bx_value) = self.pst_bx { if let Err(e) = pst_bx_value.validate() { return Err(e); } }
			if let Some(ref room_value) = self.room { if let Err(e) = room_value.validate() { return Err(e); } }
			if let Some(ref pst_cd_value) = self.pst_cd { if let Err(e) = pst_cd_value.validate() { return Err(e); } }
			if let Some(ref twn_nm_value) = self.twn_nm { if let Err(e) = twn_nm_value.validate() { return Err(e); } }
			if let Some(ref twn_lctn_nm_value) = self.twn_lctn_nm { if let Err(e) = twn_lctn_nm_value.validate() { return Err(e); } }
			if let Some(ref dstrct_nm_value) = self.dstrct_nm { if let Err(e) = dstrct_nm_value.validate() { return Err(e); } }
			if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if let Err(e) = ctry_sub_dvsn_value.validate() { return Err(e); } }
			if let Some(ref ctry_value) = self.ctry { if let Err(e) = ctry_value.validate() { return Err(e); } }
			if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// Priority1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Priority1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<Priority5Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl Priority1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Priority5Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum Priority5Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "HIGH") )]
		CodeHIGH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LOWW") )]
		CodeLOWW,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NORM") )]
		CodeNORM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "URGT") )]
		CodeURGT,
	}
	
	impl Priority5Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// QueueTransactionIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct QueueTransactionIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "QId") )]
		pub q_id: Max16Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PosInQ") )]
		pub pos_in_q: Max16Text,
	}
	
	impl QueueTransactionIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.q_id.validate() { return Err(e); }
			if let Err(e) = self.pos_in_q.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ShortPaymentIdentification4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ShortPaymentIdentification4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
		pub tx_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
		pub uetr: Option<UUIDv4Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt") )]
		pub intr_bk_sttlm_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt") )]
		pub instg_agt: BranchAndFinancialInstitutionIdentification8,
	}
	
	impl ShortPaymentIdentification4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tx_id_value) = self.tx_id { if let Err(e) = tx_id_value.validate() { return Err(e); } }
			if let Some(ref uetr_value) = self.uetr { if let Err(e) = uetr_value.validate() { return Err(e); } }
			if let Err(e) = self.instg_agt.validate() { return Err(e); }
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
	
	
	// TransactionModification7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TransactionModification7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtId") )]
		pub pmt_id: PaymentIdentification8Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NewPmtValSet") )]
		pub new_pmt_val_set: PaymentInstruction33,
	}
	
	impl TransactionModification7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pmt_id.validate() { return Err(e); }
			if let Err(e) = self.new_pmt_val_set.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// UUIDv4Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct UUIDv4Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub uui_dv4_identifier: String,
	}
	
	impl UUIDv4Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(&self.uui_dv4_identifier) {
				return Err(ValidationError::new(1005, "uui_dv4_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
}