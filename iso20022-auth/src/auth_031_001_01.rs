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


// FinancialInstrumentReportingStatusAdviceV01 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrumentReportingStatusAdviceV01 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsAdvc") )]
	pub sts_advc: Vec<MessageReportHeader4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl FinancialInstrumentReportingStatusAdviceV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.sts_advc { item.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// GenericValidationRuleIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericValidationRuleIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<ValidationRuleSchemeName1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericValidationRuleIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.schme_nm { val.validate()? }
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// MessageReportHeader4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MessageReportHeader4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgRptIdr", skip_serializing_if = "Option::is_none") )]
	pub msg_rpt_idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgSts", skip_serializing_if = "Option::is_none") )]
	pub msg_sts: Option<StatusAdviceReport3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RcrdSts", skip_serializing_if = "Option::is_none") )]
	pub rcrd_sts: Option<Vec<StatusReportRecord3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl MessageReportHeader4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.msg_rpt_idr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_rpt_idr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "msg_rpt_idr exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.msg_sts { val.validate()? }
		if let Some(ref vec) = self.rcrd_sts { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// NumberOfRecordsPerStatus1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NumberOfRecordsPerStatus1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtldNbOfRcrds") )]
	pub dtld_nb_of_rcrds: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtldSts") )]
	pub dtld_sts: ReportingRecordStatus1Code,
}

impl NumberOfRecordsPerStatus1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.dtld_nb_of_rcrds) {
			return Err(ValidationError::new(1005, "dtld_nb_of_rcrds does not match the required pattern".to_string()));
		}
		self.dtld_sts.validate()?;
		Ok(())
	}
}


// OriginalReportStatistics3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OriginalReportStatistics3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfRcrds") )]
	pub ttl_nb_of_rcrds: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfRcrdsPerSts") )]
	pub nb_of_rcrds_per_sts: Vec<NumberOfRecordsPerStatus1>,
}

impl OriginalReportStatistics3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.ttl_nb_of_rcrds) {
			return Err(ValidationError::new(1005, "ttl_nb_of_rcrds does not match the required pattern".to_string()));
		}
		for item in &self.nb_of_rcrds_per_sts { item.validate()? }
		Ok(())
	}
}


// ReportingMessageStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ReportingMessageStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACPT") )]
	CodeACPT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACTC") )]
	CodeACTC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PART") )]
	CodePART,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RCVD") )]
	CodeRCVD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RJCT") )]
	CodeRJCT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RMDR") )]
	CodeRMDR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WARN") )]
	CodeWARN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INCF") )]
	CodeINCF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRPT") )]
	CodeCRPT,
}

impl ReportingMessageStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ReportingRecordStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ReportingRecordStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACPT") )]
	CodeACPT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACPD") )]
	CodeACPD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PDNG") )]
	CodePDNG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RCVD") )]
	CodeRCVD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RJCT") )]
	CodeRJCT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RJPD") )]
	CodeRJPD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WARN") )]
	CodeWARN,
}

impl ReportingRecordStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// StatusAdviceReport3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct StatusAdviceReport3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
	pub sts: ReportingMessageStatus1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldtnRule", skip_serializing_if = "Option::is_none") )]
	pub vldtn_rule: Option<Vec<GenericValidationRuleIdentification1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgDt", skip_serializing_if = "Option::is_none") )]
	pub msg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sttstcs", skip_serializing_if = "Option::is_none") )]
	pub sttstcs: Option<OriginalReportStatistics3>,
}

impl StatusAdviceReport3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.sts.validate()?;
		if let Some(ref vec) = self.vldtn_rule { for item in vec { item.validate()? } }
		if let Some(ref val) = self.sttstcs { val.validate()? }
		Ok(())
	}
}


// StatusReportRecord3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct StatusReportRecord3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlRcrdId") )]
	pub orgnl_rcrd_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
	pub sts: ReportingRecordStatus1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldtnRule", skip_serializing_if = "Option::is_none") )]
	pub vldtn_rule: Option<Vec<GenericValidationRuleIdentification1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl StatusReportRecord3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.orgnl_rcrd_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_rcrd_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_rcrd_id.chars().count() > 140 {
			return Err(ValidationError::new(1002, "orgnl_rcrd_id exceeds the maximum length of 140".to_string()));
		}
		self.sts.validate()?;
		if let Some(ref vec) = self.vldtn_rule { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
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
		self.envlp.validate()?;
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


// ValidationRuleSchemeName1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ValidationRuleSchemeName1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ValidationRuleSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}
