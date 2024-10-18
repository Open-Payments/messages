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


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}

impl CountryCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.country_code) {
			return false
		}
		return true
	}
}


// CountryCodeAndName3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCodeAndName3 {
	#[serde(rename = "Cd")]
	pub cd: CountryCode,
	#[serde(rename = "Nm")]
	pub nm: Max70Text,
}

impl CountryCodeAndName3 {
	pub fn validate(&self) -> bool {
		if !self.cd.validate() { return false }
		if !self.nm.validate() { return false }
		return true
	}
}


// FinancialInstrumentReportingMarketIdentificationCodeReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentReportingMarketIdentificationCodeReportV02 {
	#[serde(rename = "MktId")]
	pub mkt_id: Vec<MarketIdentification95>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl FinancialInstrumentReportingMarketIdentificationCodeReportV02 {
	pub fn validate(&self) -> bool {
		for item in &self.mkt_id { if !item.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
	}
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}

impl ISODateTime {
	pub fn validate(&self) -> bool {
		return true
	}
}


// MICEntityType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum MICEntityType1Code {
	#[default]
	#[serde(rename = "APPA")]
	CodeAPPA,
	#[serde(rename = "CTPS")]
	CodeCTPS,
	#[serde(rename = "MLTF")]
	CodeMLTF,
	#[serde(rename = "OTFS")]
	CodeOTFS,
	#[serde(rename = "RMKT")]
	CodeRMKT,
	#[serde(rename = "SINT")]
	CodeSINT,
}

impl MICEntityType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MICIdentifier {
	#[serde(rename = "$value")]
	pub mic_identifier: String,
}

impl MICIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
		if !pattern.is_match(&self.mic_identifier) {
			return false
		}
		return true
	}
}


// MarketIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum MarketIdentification1Code {
	#[default]
	#[serde(rename = "SGMT")]
	CodeSGMT,
	#[serde(rename = "OPRT")]
	CodeOPRT,
}

impl MarketIdentification1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// MarketIdentification95 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketIdentification95 {
	#[serde(rename = "Oprg")]
	pub oprg: MICIdentifier,
	#[serde(rename = "Sgmt")]
	pub sgmt: MICIdentifier,
	#[serde(rename = "Tp")]
	pub tp: MarketIdentification1Code,
	#[serde(rename = "Ctgy", skip_serializing_if = "Option::is_none")]
	pub ctgy: Option<MICEntityType1Code>,
	#[serde(rename = "InstnNm")]
	pub instn_nm: Max450Text,
	#[serde(rename = "Acrnm", skip_serializing_if = "Option::is_none")]
	pub acrnm: Option<Max35Text>,
	#[serde(rename = "City", skip_serializing_if = "Option::is_none")]
	pub city: Option<Max35Text>,
	#[serde(rename = "Ctry")]
	pub ctry: CountryCodeAndName3,
	#[serde(rename = "AuthrtyNm", skip_serializing_if = "Option::is_none")]
	pub authrty_nm: Option<Max450Text>,
	#[serde(rename = "WebSite", skip_serializing_if = "Option::is_none")]
	pub web_site: Option<Max210Text>,
	#[serde(rename = "Note", skip_serializing_if = "Option::is_none")]
	pub note: Option<Max450Text>,
	#[serde(rename = "Mod", skip_serializing_if = "Option::is_none")]
	pub mod_attr: Option<Modification1Code>,
	#[serde(rename = "CreDt", skip_serializing_if = "Option::is_none")]
	pub cre_dt: Option<String>,
	#[serde(rename = "VldtyPrd")]
	pub vldty_prd: Period4Choice,
	#[serde(rename = "StsDt", skip_serializing_if = "Option::is_none")]
	pub sts_dt: Option<String>,
	#[serde(rename = "LastUpdtdDt", skip_serializing_if = "Option::is_none")]
	pub last_updtd_dt: Option<String>,
}

impl MarketIdentification95 {
	pub fn validate(&self) -> bool {
		if !self.oprg.validate() { return false }
		if !self.sgmt.validate() { return false }
		if !self.tp.validate() { return false }
		if let Some(ref ctgy_value) = self.ctgy { if !ctgy_value.validate() { return false; } }
		if !self.instn_nm.validate() { return false }
		if let Some(ref acrnm_value) = self.acrnm { if !acrnm_value.validate() { return false; } }
		if let Some(ref city_value) = self.city { if !city_value.validate() { return false; } }
		if !self.ctry.validate() { return false }
		if let Some(ref authrty_nm_value) = self.authrty_nm { if !authrty_nm_value.validate() { return false; } }
		if let Some(ref web_site_value) = self.web_site { if !web_site_value.validate() { return false; } }
		if let Some(ref note_value) = self.note { if !note_value.validate() { return false; } }
		if let Some(ref mod_attr_value) = self.mod_attr { if !mod_attr_value.validate() { return false; } }
		if !self.vldty_prd.validate() { return false }
		return true
	}
}


// Max210Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max210Text {
	#[serde(rename = "$value")]
	pub max210_text: String,
}

impl Max210Text {
	pub fn validate(&self) -> bool {
		if self.max210_text.chars().count() < 1 {
			return false
		}
		if self.max210_text.chars().count() > 210 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max350_text.chars().count() < 1 {
			return false
		}
		if self.max350_text.chars().count() > 350 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max35_text.chars().count() < 1 {
			return false
		}
		if self.max35_text.chars().count() > 35 {
			return false
		}
		return true
	}
}


// Max450Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max450Text {
	#[serde(rename = "$value")]
	pub max450_text: String,
}

impl Max450Text {
	pub fn validate(&self) -> bool {
		if self.max450_text.chars().count() < 1 {
			return false
		}
		if self.max450_text.chars().count() > 450 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max70_text.chars().count() < 1 {
			return false
		}
		if self.max70_text.chars().count() > 70 {
			return false
		}
		return true
	}
}


// Modification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Modification1Code {
	#[default]
	#[serde(rename = "NOCH")]
	CodeNOCH,
	#[serde(rename = "MODI")]
	CodeMODI,
	#[serde(rename = "DELE")]
	CodeDELE,
	#[serde(rename = "ADDD")]
	CodeADDD,
}

impl Modification1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Period2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}

impl Period2 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Period4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period4Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
	pub to_dt: Option<String>,
	#[serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt_to_dt: Option<Period2>,
}

impl Period4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref fr_dt_to_dt_value) = self.fr_dt_to_dt { if !fr_dt_to_dt_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref plc_and_nm_value) = self.plc_and_nm { if !plc_and_nm_value.validate() { return false; } }
		if !self.envlp.validate() { return false }
		return true
	}
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> bool {
		return true
	}
}
