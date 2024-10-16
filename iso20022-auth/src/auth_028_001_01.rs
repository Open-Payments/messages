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


// DateTimePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}


// ExternalValidationRuleIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalValidationRuleIdentification1Code {
	#[serde(rename = "$value")]
	pub external_validation_rule_identification1_code: String,
}


// GenericValidationRuleIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericValidationRuleIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max350Text>,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<ValidationRuleSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "$value")]
	pub max105_text: String,
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


// MoneyMarketStatisticalReportStatusAdviceV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MoneyMarketStatisticalReportStatusAdviceV01 {
	#[serde(rename = "StsRptHdr")]
	pub sts_rpt_hdr: MoneyMarketStatusReportHeader1,
	#[serde(rename = "TxSts", skip_serializing_if = "Option::is_none")]
	pub tx_sts: Option<Vec<MoneyMarketTransactionStatus2>>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// MoneyMarketStatusReportHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MoneyMarketStatusReportHeader1 {
	#[serde(rename = "RptgAgt")]
	pub rptg_agt: LEIIdentifier,
	#[serde(rename = "RptgPrd")]
	pub rptg_prd: DateTimePeriod1,
	#[serde(rename = "RptSts")]
	pub rpt_sts: StatisticalReportingStatus1Code,
	#[serde(rename = "VldtnRule", skip_serializing_if = "Option::is_none")]
	pub vldtn_rule: Option<Vec<GenericValidationRuleIdentification1>>,
}


// MoneyMarketTransactionStatus2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MoneyMarketTransactionStatus2 {
	#[serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none")]
	pub unq_tx_idr: Option<Max105Text>,
	#[serde(rename = "PrtryTxId")]
	pub prtry_tx_id: Max105Text,
	#[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
	pub brnch_id: Option<LEIIdentifier>,
	#[serde(rename = "Sts")]
	pub sts: StatisticalReportingStatus2Code,
	#[serde(rename = "VldtnRule", skip_serializing_if = "Option::is_none")]
	pub vldtn_rule: Option<Vec<GenericValidationRuleIdentification1>>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// StatisticalReportingStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum StatisticalReportingStatus1Code {
	#[default]
	#[serde(rename = "ACPT")]
	CodeACPT,
	#[serde(rename = "ACTC")]
	CodeACTC,
	#[serde(rename = "PART")]
	CodePART,
	#[serde(rename = "PDNG")]
	CodePDNG,
	#[serde(rename = "RCVD")]
	CodeRCVD,
	#[serde(rename = "RJCT")]
	CodeRJCT,
	#[serde(rename = "RMDR")]
	CodeRMDR,
	#[serde(rename = "INCF")]
	CodeINCF,
	#[serde(rename = "CRPT")]
	CodeCRPT,

}


// StatisticalReportingStatus2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum StatisticalReportingStatus2Code {
	#[default]
	#[serde(rename = "ACPT")]
	CodeACPT,
	#[serde(rename = "RJCT")]
	CodeRJCT,
	#[serde(rename = "WARN")]
	CodeWARN,

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


// ValidationRuleSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValidationRuleSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalValidationRuleIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}
