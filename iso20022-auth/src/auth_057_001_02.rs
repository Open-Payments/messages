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
use serde_valid::Validate;


// Absolute1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Absolute1 {
	#[serde(rename = "Unit")]
	pub unit: String,
	#[serde(rename = "Qty")]
	pub qty: f64,
}


// BaseOneRate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "BaseOneRate")]
	pub base_one_rate: f64,
}


// CCPPortfolioStressTestingDefinitionReportV02 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CCPPortfolioStressTestingDefinitionReportV02 {
	#[validate]
	#[serde(rename = "ScnroDef")]
	pub scnro_def: Vec<ScenarioDefinition2>,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// GenericIdentification165 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification165 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// GenericIdentification168 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification168 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max140Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 140)]
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max2000Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max2000Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 2000)]
	#[serde(rename = "Max2000Text")]
	pub max2000_text: String,
}


// Max256Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max256Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 256)]
	#[serde(rename = "Max256Text")]
	pub max256_text: String,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max350Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 350)]
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max35Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Number ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// RiskFactor1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RiskFactor1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[validate]
	#[serde(rename = "StrssSz")]
	pub strss_sz: StressSize1Choice,
}


// ScenarioDefinition2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ScenarioDefinition2 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: GenericIdentification165,
	#[serde(rename = "ScnroTp")]
	pub scnro_tp: String,
	#[serde(rename = "StrtgyStrssTp")]
	pub strtgy_strss_tp: String,
	#[validate]
	#[serde(rename = "StrssItm")]
	pub strss_itm: Vec<StressItem1>,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// ScenarioType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ScenarioType1Code {
	#[validate(enumerate = ["HIST", "HYPT"])]
	#[serde(rename = "ScenarioType1Code")]
	pub scenario_type1_code: String,
}


// SchemeIdentificationType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SchemeIdentificationType1Code {
	#[validate(enumerate = ["MARG", "COLL", "POSI", "CLIM"])]
	#[serde(rename = "SchemeIdentificationType1Code")]
	pub scheme_identification_type1_code: String,
}


// Strategy1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Strategy1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[validate]
	#[serde(rename = "StrssSz")]
	pub strss_sz: StressSize1Choice,
}


// StrategyStressType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct StrategyStressType1Code {
	#[validate(enumerate = ["FLEX", "PRLL", "SPRD"])]
	#[serde(rename = "StrategyStressType1Code")]
	pub strategy_stress_type1_code: String,
}


// StressItem1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct StressItem1 {
	#[validate]
	#[serde(rename = "StrssPdct")]
	pub strss_pdct: StressItem1Choice,
}


// StressItem1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct StressItem1Choice {
	#[validate]
	#[serde(rename = "Pdct")]
	pub pdct: Option<StressedProduct1>,
	#[validate]
	#[serde(rename = "Strtgy")]
	pub strtgy: Option<Strategy1>,
	#[validate]
	#[serde(rename = "RskFctr")]
	pub rsk_fctr: Option<RiskFactor1>,
}


// StressSize1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct StressSize1Choice {
	#[serde(rename = "Rltv")]
	pub rltv: Option<f64>,
	#[validate]
	#[serde(rename = "Abs")]
	pub abs: Option<Absolute1>,
}


// StressedProduct1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct StressedProduct1 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: GenericIdentification168,
	#[validate]
	#[serde(rename = "MaxStrssSz")]
	pub max_strss_sz: StressSize1Choice,
	#[validate]
	#[serde(rename = "MinStrssSz")]
	pub min_strss_sz: StressSize1Choice,
}


// SupplementaryData1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[validate]
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}
