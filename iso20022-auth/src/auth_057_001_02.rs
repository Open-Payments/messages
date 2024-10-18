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


// Absolute1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Absolute1 {
	#[serde(rename = "Unit")]
	pub unit: Max35Text,
	#[serde(rename = "Qty")]
	pub qty: f64,
}

impl Absolute1 {
	pub fn validate(&self) -> bool {
		if !self.unit.validate() { return false }
		return true
	}
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
}

impl BaseOneRate {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CCPPortfolioStressTestingDefinitionReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPPortfolioStressTestingDefinitionReportV02 {
	#[serde(rename = "ScnroDef")]
	pub scnro_def: Vec<ScenarioDefinition2>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CCPPortfolioStressTestingDefinitionReportV02 {
	pub fn validate(&self) -> bool {
		for item in &self.scnro_def { if !item.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// GenericIdentification165 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification165 {
	#[serde(rename = "Id")]
	pub id: Max256Text,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<SchemeIdentificationType1Code>,
}

impl GenericIdentification165 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		return true
	}
}


// GenericIdentification168 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification168 {
	#[serde(rename = "Id")]
	pub id: Max256Text,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
}

impl GenericIdentification168 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		return true
	}
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}

impl Max140Text {
	pub fn validate(&self) -> bool {
		if self.max140_text.chars().count() < 1 {
			return false
		}
		if self.max140_text.chars().count() > 140 {
			return false
		}
		return true
	}
}


// Max2000Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max2000Text {
	#[serde(rename = "$value")]
	pub max2000_text: String,
}

impl Max2000Text {
	pub fn validate(&self) -> bool {
		if self.max2000_text.chars().count() < 1 {
			return false
		}
		if self.max2000_text.chars().count() > 2000 {
			return false
		}
		return true
	}
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max256Text {
	#[serde(rename = "$value")]
	pub max256_text: String,
}

impl Max256Text {
	pub fn validate(&self) -> bool {
		if self.max256_text.chars().count() < 1 {
			return false
		}
		if self.max256_text.chars().count() > 256 {
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


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}

impl Number {
	pub fn validate(&self) -> bool {
		return true
	}
}


// RiskFactor1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RiskFactor1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "StrssSz")]
	pub strss_sz: StressSize1Choice,
}

impl RiskFactor1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.strss_sz.validate() { return false }
		return true
	}
}


// ScenarioDefinition2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ScenarioDefinition2 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification165,
	#[serde(rename = "ScnroTp")]
	pub scnro_tp: ScenarioType1Code,
	#[serde(rename = "StrtgyStrssTp")]
	pub strtgy_strss_tp: StrategyStressType1Code,
	#[serde(rename = "StrssItm")]
	pub strss_itm: Vec<StressItem1>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max2000Text>,
}

impl ScenarioDefinition2 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.scnro_tp.validate() { return false }
		if !self.strtgy_strss_tp.validate() { return false }
		for item in &self.strss_itm { if !item.validate() { return false; } }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
		return true
	}
}


// ScenarioType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ScenarioType1Code {
	#[default]
	#[serde(rename = "HIST")]
	CodeHIST,
	#[serde(rename = "HYPT")]
	CodeHYPT,
}

impl ScenarioType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SchemeIdentificationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SchemeIdentificationType1Code {
	#[default]
	#[serde(rename = "MARG")]
	CodeMARG,
	#[serde(rename = "COLL")]
	CodeCOLL,
	#[serde(rename = "POSI")]
	CodePOSI,
	#[serde(rename = "CLIM")]
	CodeCLIM,
}

impl SchemeIdentificationType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Strategy1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Strategy1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "StrssSz")]
	pub strss_sz: StressSize1Choice,
}

impl Strategy1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.strss_sz.validate() { return false }
		return true
	}
}


// StrategyStressType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum StrategyStressType1Code {
	#[default]
	#[serde(rename = "FLEX")]
	CodeFLEX,
	#[serde(rename = "PRLL")]
	CodePRLL,
	#[serde(rename = "SPRD")]
	CodeSPRD,
}

impl StrategyStressType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// StressItem1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StressItem1 {
	#[serde(rename = "StrssPdct")]
	pub strss_pdct: StressItem1Choice,
}

impl StressItem1 {
	pub fn validate(&self) -> bool {
		if !self.strss_pdct.validate() { return false }
		return true
	}
}


// StressItem1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StressItem1Choice {
	#[serde(rename = "Pdct", skip_serializing_if = "Option::is_none")]
	pub pdct: Option<StressedProduct1>,
	#[serde(rename = "Strtgy", skip_serializing_if = "Option::is_none")]
	pub strtgy: Option<Strategy1>,
	#[serde(rename = "RskFctr", skip_serializing_if = "Option::is_none")]
	pub rsk_fctr: Option<RiskFactor1>,
}

impl StressItem1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref pdct_value) = self.pdct { if !pdct_value.validate() { return false; } }
		if let Some(ref strtgy_value) = self.strtgy { if !strtgy_value.validate() { return false; } }
		if let Some(ref rsk_fctr_value) = self.rsk_fctr { if !rsk_fctr_value.validate() { return false; } }
		return true
	}
}


// StressSize1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StressSize1Choice {
	#[serde(rename = "Rltv", skip_serializing_if = "Option::is_none")]
	pub rltv: Option<f64>,
	#[serde(rename = "Abs", skip_serializing_if = "Option::is_none")]
	pub abs: Option<Absolute1>,
}

impl StressSize1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref abs_value) = self.abs { if !abs_value.validate() { return false; } }
		return true
	}
}


// StressedProduct1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StressedProduct1 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification168,
	#[serde(rename = "MaxStrssSz")]
	pub max_strss_sz: StressSize1Choice,
	#[serde(rename = "MinStrssSz")]
	pub min_strss_sz: StressSize1Choice,
}

impl StressedProduct1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.max_strss_sz.validate() { return false }
		if !self.min_strss_sz.validate() { return false }
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
