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
	
	
	// CountryCodeAndName3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CountryCodeAndName3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: CountryCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: Max70Text,
	}
	
	impl CountryCodeAndName3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Err(e) = self.nm.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FinancialInstrumentReportingMarketIdentificationCodeReportV02 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FinancialInstrumentReportingMarketIdentificationCodeReportV02 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MktId") )]
		pub mkt_id: Vec<MarketIdentification95>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl FinancialInstrumentReportingMarketIdentificationCodeReportV02 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.mkt_id { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	
	
	// MICEntityType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum MICEntityType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "APPA") )]
		CodeAPPA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CTPS") )]
		CodeCTPS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MLTF") )]
		CodeMLTF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTFS") )]
		CodeOTFS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RMKT") )]
		CodeRMKT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SINT") )]
		CodeSINT,
	}
	
	impl MICEntityType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// MarketIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum MarketIdentification1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "SGMT") )]
		CodeSGMT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OPRT") )]
		CodeOPRT,
	}
	
	impl MarketIdentification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// MarketIdentification95 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct MarketIdentification95 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Oprg") )]
		pub oprg: MICIdentifier,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sgmt") )]
		pub sgmt: MICIdentifier,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: MarketIdentification1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctgy", skip_serializing_if = "Option::is_none") )]
		pub ctgy: Option<MICEntityType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstnNm") )]
		pub instn_nm: Max450Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Acrnm", skip_serializing_if = "Option::is_none") )]
		pub acrnm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "City", skip_serializing_if = "Option::is_none") )]
		pub city: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
		pub ctry: CountryCodeAndName3,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AuthrtyNm", skip_serializing_if = "Option::is_none") )]
		pub authrty_nm: Option<Max450Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WebSite", skip_serializing_if = "Option::is_none") )]
		pub web_site: Option<Max210Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Note", skip_serializing_if = "Option::is_none") )]
		pub note: Option<Max450Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mod", skip_serializing_if = "Option::is_none") )]
		pub mod_attr: Option<Modification1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDt", skip_serializing_if = "Option::is_none") )]
		pub cre_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VldtyPrd") )]
		pub vldty_prd: Period4Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StsDt", skip_serializing_if = "Option::is_none") )]
		pub sts_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LastUpdtdDt", skip_serializing_if = "Option::is_none") )]
		pub last_updtd_dt: Option<String>,
	}
	
	impl MarketIdentification95 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.oprg.validate() { return Err(e); }
			if let Err(e) = self.sgmt.validate() { return Err(e); }
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Some(ref ctgy_value) = self.ctgy { if let Err(e) = ctgy_value.validate() { return Err(e); } }
			if let Err(e) = self.instn_nm.validate() { return Err(e); }
			if let Some(ref acrnm_value) = self.acrnm { if let Err(e) = acrnm_value.validate() { return Err(e); } }
			if let Some(ref city_value) = self.city { if let Err(e) = city_value.validate() { return Err(e); } }
			if let Err(e) = self.ctry.validate() { return Err(e); }
			if let Some(ref authrty_nm_value) = self.authrty_nm { if let Err(e) = authrty_nm_value.validate() { return Err(e); } }
			if let Some(ref web_site_value) = self.web_site { if let Err(e) = web_site_value.validate() { return Err(e); } }
			if let Some(ref note_value) = self.note { if let Err(e) = note_value.validate() { return Err(e); } }
			if let Some(ref mod_attr_value) = self.mod_attr { if let Err(e) = mod_attr_value.validate() { return Err(e); } }
			if let Err(e) = self.vldty_prd.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// Max210Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max210Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max210_text: String,
	}
	
	impl Max210Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max210_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max210_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max210_text.chars().count() > 210 {
				return Err(ValidationError::new(1002, "max210_text exceeds the maximum length of 210".to_string()));
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
	
	
	// Max450Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max450Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max450_text: String,
	}
	
	impl Max450Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max450_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max450_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max450_text.chars().count() > 450 {
				return Err(ValidationError::new(1002, "max450_text exceeds the maximum length of 450".to_string()));
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
	
	
	// Modification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum Modification1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOCH") )]
		CodeNOCH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MODI") )]
		CodeMODI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DELE") )]
		CodeDELE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADDD") )]
		CodeADDD,
	}
	
	impl Modification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Period2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// Period4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Period4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
		pub dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt", skip_serializing_if = "Option::is_none") )]
		pub fr_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt", skip_serializing_if = "Option::is_none") )]
		pub to_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none") )]
		pub fr_dt_to_dt: Option<Period2>,
	}
	
	impl Period4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref fr_dt_to_dt_value) = self.fr_dt_to_dt { if let Err(e) = fr_dt_to_dt_value.validate() { return Err(e); } }
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
	
}