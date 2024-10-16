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


// Absolute1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Absolute1 {
	#[serde(rename = "Unit")]
	pub unit: Max35Text,
	#[serde(rename = "Qty")]
	pub qty: f64,
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
}


// CCPPortfolioStressTestingDefinitionReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPPortfolioStressTestingDefinitionReportV02 {
	#[serde(rename = "ScnroDef")]
	pub scnro_def: Vec<ScenarioDefinition2>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
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


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}


// Max2000Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max2000Text {
	#[serde(rename = "$value")]
	pub max2000_text: String,
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max256Text {
	#[serde(rename = "$value")]
	pub max256_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}


// RiskFactor1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RiskFactor1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "StrssSz")]
	pub strss_sz: StressSize1Choice,
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


// ScenarioType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ScenarioType1Code {
	#[default]
	#[serde(rename = "HIST")]
	CodeHIST,
	#[serde(rename = "HYPT")]
	CodeHYPT,
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


// Strategy1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Strategy1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "StrssSz")]
	pub strss_sz: StressSize1Choice,
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


// StressItem1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StressItem1 {
	#[serde(rename = "StrssPdct")]
	pub strss_pdct: StressItem1Choice,
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


// StressSize1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StressSize1Choice {
	#[serde(rename = "Rltv", skip_serializing_if = "Option::is_none")]
	pub rltv: Option<f64>,
	#[serde(rename = "Abs", skip_serializing_if = "Option::is_none")]
	pub abs: Option<Absolute1>,
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


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}
