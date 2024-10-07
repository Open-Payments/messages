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


// DateTimePeriod1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}


// ExternalValidationRuleIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalValidationRuleIdentification1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalValidationRuleIdentification1Code")]
	pub external_validation_rule_identification1_code: String,
}


// GenericValidationRuleIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericValidationRuleIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[validate]
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<ValidationRuleSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[validate(pattern = "[A-Z0-9]{18,18}[0-9]{2,2}")]
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// Max105Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max105Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 105)]
	#[serde(rename = "Max105Text")]
	pub max105_text: String,
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


// MoneyMarketStatisticalReportStatusAdviceV01 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MoneyMarketStatisticalReportStatusAdviceV01 {
	#[validate]
	#[serde(rename = "StsRptHdr")]
	pub sts_rpt_hdr: MoneyMarketStatusReportHeader1,
	#[validate]
	#[serde(rename = "TxSts")]
	pub tx_sts: Option<Vec<MoneyMarketTransactionStatus2>>,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// MoneyMarketStatusReportHeader1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MoneyMarketStatusReportHeader1 {
	#[serde(rename = "RptgAgt")]
	pub rptg_agt: String,
	#[validate]
	#[serde(rename = "RptgPrd")]
	pub rptg_prd: DateTimePeriod1,
	#[serde(rename = "RptSts")]
	pub rpt_sts: String,
	#[validate]
	#[serde(rename = "VldtnRule")]
	pub vldtn_rule: Option<Vec<GenericValidationRuleIdentification1>>,
}


// MoneyMarketTransactionStatus2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MoneyMarketTransactionStatus2 {
	#[serde(rename = "UnqTxIdr")]
	pub unq_tx_idr: Option<String>,
	#[serde(rename = "PrtryTxId")]
	pub prtry_tx_id: String,
	#[serde(rename = "BrnchId")]
	pub brnch_id: Option<String>,
	#[serde(rename = "Sts")]
	pub sts: String,
	#[validate]
	#[serde(rename = "VldtnRule")]
	pub vldtn_rule: Option<Vec<GenericValidationRuleIdentification1>>,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// StatisticalReportingStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct StatisticalReportingStatus1Code {
	#[validate(enumerate = ["ACPT", "ACTC", "PART", "PDNG", "RCVD", "RJCT", "RMDR", "INCF", "CRPT"])]
	#[serde(rename = "StatisticalReportingStatus1Code")]
	pub statistical_reporting_status1_code: String,
}


// StatisticalReportingStatus2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct StatisticalReportingStatus2Code {
	#[validate(enumerate = ["ACPT", "RJCT", "WARN"])]
	#[serde(rename = "StatisticalReportingStatus2Code")]
	pub statistical_reporting_status2_code: String,
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


// ValidationRuleSchemeName1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ValidationRuleSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}
