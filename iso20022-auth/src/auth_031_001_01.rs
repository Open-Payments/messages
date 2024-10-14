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


// ExternalValidationRuleIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalValidationRuleIdentification1Code {
	#[serde(rename = "$value")]
	pub external_validation_rule_identification1_code: String,
}


// FinancialInstrumentReportingStatusAdviceV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentReportingStatusAdviceV01 {
	#[serde(rename = "StsAdvc")]
	pub sts_advc: Vec<MessageReportHeader4>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
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


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}


// Max15NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max15NumericText {
	#[serde(rename = "$value")]
	pub max15_numeric_text: String,
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


// MessageReportHeader4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageReportHeader4 {
	#[serde(rename = "MsgRptIdr", skip_serializing_if = "Option::is_none")]
	pub msg_rpt_idr: Option<Max140Text>,
	#[serde(rename = "MsgSts", skip_serializing_if = "Option::is_none")]
	pub msg_sts: Option<StatusAdviceReport3>,
	#[serde(rename = "RcrdSts", skip_serializing_if = "Option::is_none")]
	pub rcrd_sts: Option<Vec<StatusReportRecord3>>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// NumberOfRecordsPerStatus1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NumberOfRecordsPerStatus1 {
	#[serde(rename = "DtldNbOfRcrds")]
	pub dtld_nb_of_rcrds: Max15NumericText,
	#[serde(rename = "DtldSts")]
	pub dtld_sts: ReportingRecordStatus1Code,
}


// OriginalReportStatistics3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalReportStatistics3 {
	#[serde(rename = "TtlNbOfRcrds")]
	pub ttl_nb_of_rcrds: Max15NumericText,
	#[serde(rename = "NbOfRcrdsPerSts")]
	pub nb_of_rcrds_per_sts: Vec<NumberOfRecordsPerStatus1>,
}


// ReportingMessageStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportingMessageStatus1Code {
	#[serde(rename = "ACPT")]
	CodeACPT,
	#[serde(rename = "ACTC")]
	CodeACTC,
	#[serde(rename = "PART")]
	CodePART,
	#[serde(rename = "RCVD")]
	CodeRCVD,
	#[serde(rename = "RJCT")]
	CodeRJCT,
	#[serde(rename = "RMDR")]
	CodeRMDR,
	#[serde(rename = "WARN")]
	CodeWARN,
	#[serde(rename = "INCF")]
	CodeINCF,
	#[serde(rename = "CRPT")]
	CodeCRPT,

	#[default]
	UNKOWN
}


// ReportingRecordStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportingRecordStatus1Code {
	#[serde(rename = "ACPT")]
	CodeACPT,
	#[serde(rename = "ACPD")]
	CodeACPD,
	#[serde(rename = "PDNG")]
	CodePDNG,
	#[serde(rename = "RCVD")]
	CodeRCVD,
	#[serde(rename = "RJCT")]
	CodeRJCT,
	#[serde(rename = "RJPD")]
	CodeRJPD,
	#[serde(rename = "WARN")]
	CodeWARN,

	#[default]
	UNKOWN
}


// StatusAdviceReport3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatusAdviceReport3 {
	#[serde(rename = "Sts")]
	pub sts: ReportingMessageStatus1Code,
	#[serde(rename = "VldtnRule", skip_serializing_if = "Option::is_none")]
	pub vldtn_rule: Option<Vec<GenericValidationRuleIdentification1>>,
	#[serde(rename = "MsgDt", skip_serializing_if = "Option::is_none")]
	pub msg_dt: Option<String>,
	#[serde(rename = "Sttstcs", skip_serializing_if = "Option::is_none")]
	pub sttstcs: Option<OriginalReportStatistics3>,
}


// StatusReportRecord3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatusReportRecord3 {
	#[serde(rename = "OrgnlRcrdId")]
	pub orgnl_rcrd_id: Max140Text,
	#[serde(rename = "Sts")]
	pub sts: ReportingRecordStatus1Code,
	#[serde(rename = "VldtnRule", skip_serializing_if = "Option::is_none")]
	pub vldtn_rule: Option<Vec<GenericValidationRuleIdentification1>>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
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
