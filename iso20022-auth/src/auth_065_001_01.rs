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


// BackTestingMethodology1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BackTestingMethodology1 {
	#[serde(rename = "RskMdlTp")]
	pub rsk_mdl_tp: ModelType1Choice,
	#[serde(rename = "MdlCnfdncLvl")]
	pub mdl_cnfdnc_lvl: f64,
	#[serde(rename = "VartnMrgnCleanInd")]
	pub vartn_mrgn_clean_ind: bool,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// BaseOneRate ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "BaseOneRate")]
	pub base_one_rate: f64,
}


// CCPBackTestingDefinitionReportV01 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPBackTestingDefinitionReportV01 {
	#[serde(rename = "Mthdlgy")]
	pub mthdlgy: Vec<BackTestingMethodology1>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// GenericIdentification36 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// Max2000Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max2000Text {
	#[serde(rename = "Max2000Text")]
	pub max2000_text: String,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// ModelType1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ModelType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// ModelType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ModelType1Code {
	#[serde(rename = "ModelType1Code")]
	pub model_type1_code: String,
}


// SupplementaryData1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// TrueFalseIndicator ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}
