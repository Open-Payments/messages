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
use crate::validationerror::*;


// ExternalValidationRuleIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalValidationRuleIdentification1Code {
	#[serde(rename = "$value")]
	pub external_validation_rule_identification1_code: String,
}

impl ExternalValidationRuleIdentification1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_validation_rule_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_validation_rule_identification1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_validation_rule_identification1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_validation_rule_identification1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// FinancialInstrumentReportingStatusAdviceV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentReportingStatusAdviceV01 {
	#[serde(rename = "StsAdvc")]
	pub sts_advc: Vec<MessageReportHeader4>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl FinancialInstrumentReportingStatusAdviceV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.sts_advc { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
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

impl GenericValidationRuleIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
		if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
		if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max140_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max140_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max140_text.chars().count() > 140 {
			return Err(ValidationError::new(1002, "max140_text exceeds the maximum length of 140".to_string()));
		}
		Ok(())
	}
}


// Max15NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max15NumericText {
	#[serde(rename = "$value")]
	pub max15_numeric_text: String,
}

impl Max15NumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.max15_numeric_text) {
			return Err(ValidationError::new(1005, "max15_numeric_text does not match the required pattern".to_string()));
		}
		Ok(())
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
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

impl MessageReportHeader4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref msg_rpt_idr_value) = self.msg_rpt_idr { if let Err(e) = msg_rpt_idr_value.validate() { return Err(e); } }
		if let Some(ref msg_sts_value) = self.msg_sts { if let Err(e) = msg_sts_value.validate() { return Err(e); } }
		if let Some(ref rcrd_sts_vec) = self.rcrd_sts { for item in rcrd_sts_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// NumberOfRecordsPerStatus1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NumberOfRecordsPerStatus1 {
	#[serde(rename = "DtldNbOfRcrds")]
	pub dtld_nb_of_rcrds: Max15NumericText,
	#[serde(rename = "DtldSts")]
	pub dtld_sts: ReportingRecordStatus1Code,
}

impl NumberOfRecordsPerStatus1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.dtld_nb_of_rcrds.validate() { return Err(e); }
		if let Err(e) = self.dtld_sts.validate() { return Err(e); }
		Ok(())
	}
}


// OriginalReportStatistics3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalReportStatistics3 {
	#[serde(rename = "TtlNbOfRcrds")]
	pub ttl_nb_of_rcrds: Max15NumericText,
	#[serde(rename = "NbOfRcrdsPerSts")]
	pub nb_of_rcrds_per_sts: Vec<NumberOfRecordsPerStatus1>,
}

impl OriginalReportStatistics3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ttl_nb_of_rcrds.validate() { return Err(e); }
		for item in &self.nb_of_rcrds_per_sts { if let Err(e) = item.validate() { return Err(e); } }
		Ok(())
	}
}


// ReportingMessageStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportingMessageStatus1Code {
	#[default]
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
}

impl ReportingMessageStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ReportingRecordStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportingRecordStatus1Code {
	#[default]
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
}

impl ReportingRecordStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
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

impl StatusAdviceReport3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.sts.validate() { return Err(e); }
		if let Some(ref vldtn_rule_vec) = self.vldtn_rule { for item in vldtn_rule_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref sttstcs_value) = self.sttstcs { if let Err(e) = sttstcs_value.validate() { return Err(e); } }
		Ok(())
	}
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

impl StatusReportRecord3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.orgnl_rcrd_id.validate() { return Err(e); }
		if let Err(e) = self.sts.validate() { return Err(e); }
		if let Some(ref vldtn_rule_vec) = self.vldtn_rule { for item in vldtn_rule_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref plc_and_nm_value) = self.plc_and_nm { if let Err(e) = plc_and_nm_value.validate() { return Err(e); } }
		if let Err(e) = self.envlp.validate() { return Err(e); }
		Ok(())
	}
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ValidationRuleSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValidationRuleSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalValidationRuleIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ValidationRuleSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}
