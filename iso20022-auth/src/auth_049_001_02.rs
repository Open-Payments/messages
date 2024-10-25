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


// CountryCodeAndName3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CountryCodeAndName3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
	pub cd: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
}

impl CountryCodeAndName3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.cd) {
			return Err(ValidationError::new(1005, "cd does not match the required pattern".to_string()));
		}
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 70 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 70".to_string()));
		}
		Ok(())
	}
}


// FinancialInstrumentReportingMarketIdentificationCodeReportV02 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrumentReportingMarketIdentificationCodeReportV02 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktId") )]
	pub mkt_id: Vec<MarketIdentification95>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl FinancialInstrumentReportingMarketIdentificationCodeReportV02 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.mkt_id { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// MICEntityType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// MarketIdentification1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MarketIdentification95 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Oprg") )]
	pub oprg: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sgmt") )]
	pub sgmt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: MarketIdentification1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctgy", skip_serializing_if = "Option::is_none") )]
	pub ctgy: Option<MICEntityType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstnNm") )]
	pub instn_nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Acrnm", skip_serializing_if = "Option::is_none") )]
	pub acrnm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "City", skip_serializing_if = "Option::is_none") )]
	pub city: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
	pub ctry: CountryCodeAndName3,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AuthrtyNm", skip_serializing_if = "Option::is_none") )]
	pub authrty_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WebSite", skip_serializing_if = "Option::is_none") )]
	pub web_site: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Note", skip_serializing_if = "Option::is_none") )]
	pub note: Option<String>,
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
		let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
		if !pattern.is_match(&self.oprg) {
			return Err(ValidationError::new(1005, "oprg does not match the required pattern".to_string()));
		}
		let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
		if !pattern.is_match(&self.sgmt) {
			return Err(ValidationError::new(1005, "sgmt does not match the required pattern".to_string()));
		}
		if let Err(e) = self.tp.validate() { return Err(e); }
		if let Some(ref val) = self.ctgy { if let Err(e) = val.validate() { return Err(e); } }
		if self.instn_nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "instn_nm is shorter than the minimum length of 1".to_string()));
		}
		if self.instn_nm.chars().count() > 450 {
			return Err(ValidationError::new(1002, "instn_nm exceeds the maximum length of 450".to_string()));
		}
		if let Some(ref val) = self.acrnm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acrnm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acrnm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.city {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "city is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "city exceeds the maximum length of 35".to_string()));
			}
		}
		if let Err(e) = self.ctry.validate() { return Err(e); }
		if let Some(ref val) = self.authrty_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "authrty_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 450 {
				return Err(ValidationError::new(1002, "authrty_nm exceeds the maximum length of 450".to_string()));
			}
		}
		if let Some(ref val) = self.web_site {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "web_site is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 210 {
				return Err(ValidationError::new(1002, "web_site exceeds the maximum length of 210".to_string()));
			}
		}
		if let Some(ref val) = self.note {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "note is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 450 {
				return Err(ValidationError::new(1002, "note exceeds the maximum length of 450".to_string()));
			}
		}
		if let Some(ref val) = self.mod_attr { if let Err(e) = val.validate() { return Err(e); } }
		if let Err(e) = self.vldty_prd.validate() { return Err(e); }
		Ok(())
	}
}


// Modification1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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


// Period4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
		if let Some(ref val) = self.fr_dt_to_dt { if let Err(e) = val.validate() { return Err(e); } }
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
