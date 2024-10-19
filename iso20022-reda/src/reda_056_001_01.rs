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

use serde::{Deserialize, Serialize};
use regex::Regex;
use crate::validationerror::*;
// AccountIdentification26 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentification26 {
	#[serde(rename = "Prtry")]
	pub prtry: SimpleIdentificationInformation4,
}

impl AccountIdentification26 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.prtry.validate() { return Err(e); }
		Ok(())
	}
}


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
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


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AddressType2Code {
	#[default]
	#[serde(rename = "ADDR")]
	CodeADDR,
	#[serde(rename = "PBOX")]
	CodePBOX,
	#[serde(rename = "HOME")]
	CodeHOME,
	#[serde(rename = "BIZZ")]
	CodeBIZZ,
	#[serde(rename = "MLTO")]
	CodeMLTO,
	#[serde(rename = "DLVY")]
	CodeDLVY,
}

impl AddressType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AnyBICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyBICIdentifier {
	#[serde(rename = "$value")]
	pub any_bic_identifier: String,
}

impl AnyBICIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.any_bic_identifier) {
			return Err(ValidationError::new(1005, "any_bic_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// BICFIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BICFIIdentifier {
	#[serde(rename = "$value")]
	pub bicfi_identifier: String,
}

impl BICFIIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.bicfi_identifier) {
			return Err(ValidationError::new(1005, "bicfi_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "$value")]
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


// CashParties24 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashParties24 {
	#[serde(rename = "Cdtr")]
	pub cdtr: PartyIdentificationAndAccount96,
	#[serde(rename = "CdtrAgt")]
	pub cdtr_agt: PartyIdentificationAndAccount97,
	#[serde(rename = "Intrmy", skip_serializing_if = "Option::is_none")]
	pub intrmy: Option<PartyIdentificationAndAccount97>,
	#[serde(rename = "Intrmy2", skip_serializing_if = "Option::is_none")]
	pub intrmy2: Option<PartyIdentificationAndAccount97>,
}

impl CashParties24 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.cdtr.validate() { return Err(e); }
		if let Err(e) = self.cdtr_agt.validate() { return Err(e); }
		if let Some(ref intrmy_value) = self.intrmy { if let Err(e) = intrmy_value.validate() { return Err(e); } }
		if let Some(ref intrmy2_value) = self.intrmy2 { if let Err(e) = intrmy2_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ClassificationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClassificationType1Choice {
	#[serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none")]
	pub clssfctn_fin_instrm: Option<CFIOct2015Identifier>,
	#[serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none")]
	pub altrn_clssfctn: Option<GenericIdentification1>,
}

impl ClassificationType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref clssfctn_fin_instrm_value) = self.clssfctn_fin_instrm { if let Err(e) = clssfctn_fin_instrm_value.validate() { return Err(e); } }
		if let Some(ref altrn_clssfctn_value) = self.altrn_clssfctn { if let Err(e) = altrn_clssfctn_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountryCode {
	#[serde(rename = "$value")]
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


// EffectiveDate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EffectiveDate1 {
	#[serde(rename = "FctvDt")]
	pub fctv_dt: String,
	#[serde(rename = "FctvDtParam", skip_serializing_if = "Option::is_none")]
	pub fctv_dt_param: Option<ExternalEffectiveDateParameter1Code>,
}

impl EffectiveDate1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref fctv_dt_param_value) = self.fctv_dt_param { if let Err(e) = fctv_dt_param_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
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


// ExternalEffectiveDateParameter1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalEffectiveDateParameter1Code {
	#[serde(rename = "$value")]
	pub external_effective_date_parameter1_code: String,
}

impl ExternalEffectiveDateParameter1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_effective_date_parameter1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_effective_date_parameter1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_effective_date_parameter1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_effective_date_parameter1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalMarketArea1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalMarketArea1Code {
	#[serde(rename = "$value")]
	pub external_market_area1_code: String,
}

impl ExternalMarketArea1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_market_area1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_market_area1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_market_area1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_market_area1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalSecuritiesPurpose1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalSecuritiesPurpose1Code {
	#[serde(rename = "$value")]
	pub external_securities_purpose1_code: String,
}

impl ExternalSecuritiesPurpose1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_securities_purpose1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_securities_purpose1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_securities_purpose1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_securities_purpose1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
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


// GenericIdentification30 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: Exact4AlphaNumericText,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
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


// GenericIdentification49 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification49 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "IdTp")]
	pub id_tp: Max35Text,
}

impl GenericIdentification49 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Err(e) = self.id_tp.validate() { return Err(e); }
		Ok(())
	}
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}

impl ISODate {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// MarketIdentification87 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketIdentification87 {
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
	#[serde(rename = "ClssfctnTp")]
	pub clssfctn_tp: ClassificationType1Choice,
	#[serde(rename = "SttlmPurp", skip_serializing_if = "Option::is_none")]
	pub sttlm_purp: Option<Purpose3Choice>,
}

impl MarketIdentification87 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ctry.validate() { return Err(e); }
		if let Err(e) = self.clssfctn_tp.validate() { return Err(e); }
		if let Some(ref sttlm_purp_value) = self.sttlm_purp { if let Err(e) = sttlm_purp_value.validate() { return Err(e); } }
		Ok(())
	}
}


// MarketIdentificationOrCashPurpose1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketIdentificationOrCashPurpose1Choice {
	#[serde(rename = "SttlmInstrMktId", skip_serializing_if = "Option::is_none")]
	pub sttlm_instr_mkt_id: Option<MarketIdentification87>,
	#[serde(rename = "CshSSIPurp", skip_serializing_if = "Option::is_none")]
	pub csh_ssi_purp: Option<Vec<ExternalMarketArea1Code>>,
}

impl MarketIdentificationOrCashPurpose1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref sttlm_instr_mkt_id_value) = self.sttlm_instr_mkt_id { if let Err(e) = sttlm_instr_mkt_id_value.validate() { return Err(e); } }
		if let Some(ref csh_ssi_purp_vec) = self.csh_ssi_purp { for item in csh_ssi_purp_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max16Text {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
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


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max70Text {
	#[serde(rename = "$value")]
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


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}

impl NameAndAddress5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.nm.validate() { return Err(e); }
		if let Some(ref adr_value) = self.adr { if let Err(e) = adr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// NameAndAddress8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress8 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
	#[serde(rename = "AltrntvIdr", skip_serializing_if = "Option::is_none")]
	pub altrntv_idr: Option<Vec<Max35Text>>,
}

impl NameAndAddress8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.nm.validate() { return Err(e); }
		if let Some(ref adr_value) = self.adr { if let Err(e) = adr_value.validate() { return Err(e); } }
		if let Some(ref altrntv_idr_vec) = self.altrntv_idr { for item in altrntv_idr_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// PartyIdentification44 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification44 {
	#[serde(rename = "AnyBIC")]
	pub any_bic: AnyBICIdentifier,
	#[serde(rename = "AltrntvIdr", skip_serializing_if = "Option::is_none")]
	pub altrntv_idr: Option<Vec<Max35Text>>,
}

impl PartyIdentification44 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.any_bic.validate() { return Err(e); }
		if let Some(ref altrntv_idr_vec) = self.altrntv_idr { for item in altrntv_idr_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// PartyIdentification62 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification62 {
	#[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
	pub bicfi: Option<BICFIIdentifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification1>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
}

impl PartyIdentification62 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref bicfi_value) = self.bicfi { if let Err(e) = bicfi_value.validate() { return Err(e); } }
		if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
		if let Some(ref nm_and_adr_value) = self.nm_and_adr { if let Err(e) = nm_and_adr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyIdentification63 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification63 {
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification75Choice,
	#[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
	pub prcg_id: Option<Max35Text>,
}

impl PartyIdentification63 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pty_id.validate() { return Err(e); }
		if let Some(ref prcg_id_value) = self.prcg_id { if let Err(e) = prcg_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyIdentification64 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification64 {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICIdentifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification1>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
}

impl PartyIdentification64 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
		if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
		if let Some(ref nm_and_adr_value) = self.nm_and_adr { if let Err(e) = nm_and_adr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyIdentification71Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification71Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICIdentifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification36>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
}

impl PartyIdentification71Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
		if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
		if let Some(ref nm_and_adr_value) = self.nm_and_adr { if let Err(e) = nm_and_adr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyIdentification75Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification75Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICIdentifier>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}

impl PartyIdentification75Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
		if let Some(ref nm_and_adr_value) = self.nm_and_adr { if let Err(e) = nm_and_adr_value.validate() { return Err(e); } }
		if let Some(ref ctry_value) = self.ctry { if let Err(e) = ctry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyIdentification99Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification99Choice {
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress8>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<PartyIdentification44>,
}

impl PartyIdentification99Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref nm_and_adr_value) = self.nm_and_adr { if let Err(e) = nm_and_adr_value.validate() { return Err(e); } }
		if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyIdentificationAndAccount95 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentificationAndAccount95 {
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification71Choice,
	#[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
	pub acct_id: Option<SecuritiesAccount22>,
	#[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
	pub prcg_id: Option<Max35Text>,
}

impl PartyIdentificationAndAccount95 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pty_id.validate() { return Err(e); }
		if let Some(ref acct_id_value) = self.acct_id { if let Err(e) = acct_id_value.validate() { return Err(e); } }
		if let Some(ref prcg_id_value) = self.prcg_id { if let Err(e) = prcg_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyIdentificationAndAccount96 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentificationAndAccount96 {
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification64,
	#[serde(rename = "AcctId")]
	pub acct_id: AccountIdentification26,
}

impl PartyIdentificationAndAccount96 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pty_id.validate() { return Err(e); }
		if let Err(e) = self.acct_id.validate() { return Err(e); }
		Ok(())
	}
}


// PartyIdentificationAndAccount97 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentificationAndAccount97 {
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification62,
	#[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
	pub acct_id: Option<AccountIdentification26>,
}

impl PartyIdentificationAndAccount97 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pty_id.validate() { return Err(e); }
		if let Some(ref acct_id_value) = self.acct_id { if let Err(e) = acct_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PostalAddress1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType2Code>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<Max70Text>>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max70Text>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<Max16Text>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<Max16Text>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<Max35Text>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<Max35Text>,
	#[serde(rename = "Ctry")]
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


// Purpose3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Purpose3Choice {
	#[serde(rename = "SctiesPurpCd", skip_serializing_if = "Option::is_none")]
	pub scties_purp_cd: Option<ExternalSecuritiesPurpose1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification1>,
}

impl Purpose3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref scties_purp_cd_value) = self.scties_purp_cd { if let Err(e) = scties_purp_cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecuritiesAccount22 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesAccount22 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<GenericIdentification30>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max70Text>,
}

impl SecuritiesAccount22 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecuritiesOrCash1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesOrCash1Choice {
	#[serde(rename = "SctiesDtls", skip_serializing_if = "Option::is_none")]
	pub scties_dtls: Option<SettlementParties35>,
	#[serde(rename = "CshPtiesDtls", skip_serializing_if = "Option::is_none")]
	pub csh_pties_dtls: Option<CashParties24>,
}

impl SecuritiesOrCash1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref scties_dtls_value) = self.scties_dtls { if let Err(e) = scties_dtls_value.validate() { return Err(e); } }
		if let Some(ref csh_pties_dtls_value) = self.csh_pties_dtls { if let Err(e) = csh_pties_dtls_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SettlementParties32 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementParties32 {
	#[serde(rename = "Dpstry")]
	pub dpstry: PartyIdentification63,
	#[serde(rename = "Pty1", skip_serializing_if = "Option::is_none")]
	pub pty1: Option<PartyIdentificationAndAccount95>,
	#[serde(rename = "Pty2", skip_serializing_if = "Option::is_none")]
	pub pty2: Option<PartyIdentificationAndAccount95>,
	#[serde(rename = "Pty3", skip_serializing_if = "Option::is_none")]
	pub pty3: Option<PartyIdentificationAndAccount95>,
	#[serde(rename = "Pty4", skip_serializing_if = "Option::is_none")]
	pub pty4: Option<PartyIdentificationAndAccount95>,
	#[serde(rename = "Pty5", skip_serializing_if = "Option::is_none")]
	pub pty5: Option<PartyIdentificationAndAccount95>,
}

impl SettlementParties32 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.dpstry.validate() { return Err(e); }
		if let Some(ref pty1_value) = self.pty1 { if let Err(e) = pty1_value.validate() { return Err(e); } }
		if let Some(ref pty2_value) = self.pty2 { if let Err(e) = pty2_value.validate() { return Err(e); } }
		if let Some(ref pty3_value) = self.pty3 { if let Err(e) = pty3_value.validate() { return Err(e); } }
		if let Some(ref pty4_value) = self.pty4 { if let Err(e) = pty4_value.validate() { return Err(e); } }
		if let Some(ref pty5_value) = self.pty5 { if let Err(e) = pty5_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SettlementParties35 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementParties35 {
	#[serde(rename = "StgSttlmPties")]
	pub stg_sttlm_pties: SettlementParties32,
	#[serde(rename = "LclMktId", skip_serializing_if = "Option::is_none")]
	pub lcl_mkt_id: Option<Vec<GenericIdentification49>>,
	#[serde(rename = "RegnDtls", skip_serializing_if = "Option::is_none")]
	pub regn_dtls: Option<PartyIdentification99Choice>,
}

impl SettlementParties35 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.stg_sttlm_pties.validate() { return Err(e); }
		if let Some(ref lcl_mkt_id_vec) = self.lcl_mkt_id { for item in lcl_mkt_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref regn_dtls_value) = self.regn_dtls { if let Err(e) = regn_dtls_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SimpleIdentificationInformation4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SimpleIdentificationInformation4 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
}

impl SimpleIdentificationInformation4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		Ok(())
	}
}


// StandingSettlementInstructionV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StandingSettlementInstructionV01 {
	#[serde(rename = "MsgRefId")]
	pub msg_ref_id: Max35Text,
	#[serde(rename = "FctvDtDtls", skip_serializing_if = "Option::is_none")]
	pub fctv_dt_dtls: Option<EffectiveDate1>,
	#[serde(rename = "AcctId")]
	pub acct_id: Vec<AccountIdentification26>,
	#[serde(rename = "MktId")]
	pub mkt_id: MarketIdentificationOrCashPurpose1Choice,
	#[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy: Option<ActiveCurrencyCode>,
	#[serde(rename = "SttlmDtls")]
	pub sttlm_dtls: SecuritiesOrCash1Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl StandingSettlementInstructionV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_ref_id.validate() { return Err(e); }
		if let Some(ref fctv_dt_dtls_value) = self.fctv_dt_dtls { if let Err(e) = fctv_dt_dtls_value.validate() { return Err(e); } }
		for item in &self.acct_id { if let Err(e) = item.validate() { return Err(e); } }
		if let Err(e) = self.mkt_id.validate() { return Err(e); }
		if let Some(ref sttlm_ccy_value) = self.sttlm_ccy { if let Err(e) = sttlm_ccy_value.validate() { return Err(e); } }
		if let Err(e) = self.sttlm_dtls.validate() { return Err(e); }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
