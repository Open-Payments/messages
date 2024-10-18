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


// BusinessInformationCriteria1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BusinessInformationCriteria1 {
	#[serde(rename = "NewQryNm", skip_serializing_if = "Option::is_none")]
	pub new_qry_nm: Option<Max35Text>,
	#[serde(rename = "SchCrit", skip_serializing_if = "Option::is_none")]
	pub sch_crit: Option<Vec<GeneralBusinessInformationSearchCriteria1>>,
	#[serde(rename = "RtrCrit", skip_serializing_if = "Option::is_none")]
	pub rtr_crit: Option<GeneralBusinessInformationReturnCriteria1>,
}

impl BusinessInformationCriteria1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref new_qry_nm_value) = self.new_qry_nm { if let Err(e) = new_qry_nm_value.validate() { return Err(e); } }
		if let Some(ref sch_crit_vec) = self.sch_crit { for item in sch_crit_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref rtr_crit_value) = self.rtr_crit { if let Err(e) = rtr_crit_value.validate() { return Err(e); } }
		Ok(())
	}
}


// BusinessInformationQueryDefinition3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BusinessInformationQueryDefinition3 {
	#[serde(rename = "QryTp", skip_serializing_if = "Option::is_none")]
	pub qry_tp: Option<QueryType2Code>,
	#[serde(rename = "GnlBizInfCrit", skip_serializing_if = "Option::is_none")]
	pub gnl_biz_inf_crit: Option<GeneralBusinessInformationCriteriaDefinition1Choice>,
}

impl BusinessInformationQueryDefinition3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref qry_tp_value) = self.qry_tp { if let Err(e) = qry_tp_value.validate() { return Err(e); } }
		if let Some(ref gnl_biz_inf_crit_value) = self.gnl_biz_inf_crit { if let Err(e) = gnl_biz_inf_crit_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CharacterSearch1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CharacterSearch1Choice {
	#[serde(rename = "EQ", skip_serializing_if = "Option::is_none")]
	pub eq: Option<Max35Text>,
	#[serde(rename = "NEQ", skip_serializing_if = "Option::is_none")]
	pub neq: Option<Max35Text>,
	#[serde(rename = "CT", skip_serializing_if = "Option::is_none")]
	pub ct: Option<Max35Text>,
	#[serde(rename = "NCT", skip_serializing_if = "Option::is_none")]
	pub nct: Option<Max35Text>,
}

impl CharacterSearch1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref eq_value) = self.eq { if let Err(e) = eq_value.validate() { return Err(e); } }
		if let Some(ref neq_value) = self.neq { if let Err(e) = neq_value.validate() { return Err(e); } }
		if let Some(ref ct_value) = self.ct { if let Err(e) = ct_value.validate() { return Err(e); } }
		if let Some(ref nct_value) = self.nct { if let Err(e) = nct_value.validate() { return Err(e); } }
		Ok(())
	}
}


// GeneralBusinessInformationCriteriaDefinition1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GeneralBusinessInformationCriteriaDefinition1Choice {
	#[serde(rename = "QryNm", skip_serializing_if = "Option::is_none")]
	pub qry_nm: Option<Max35Text>,
	#[serde(rename = "NewCrit", skip_serializing_if = "Option::is_none")]
	pub new_crit: Option<BusinessInformationCriteria1>,
}

impl GeneralBusinessInformationCriteriaDefinition1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref qry_nm_value) = self.qry_nm { if let Err(e) = qry_nm_value.validate() { return Err(e); } }
		if let Some(ref new_crit_value) = self.new_crit { if let Err(e) = new_crit_value.validate() { return Err(e); } }
		Ok(())
	}
}


// GeneralBusinessInformationReturnCriteria1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GeneralBusinessInformationReturnCriteria1 {
	#[serde(rename = "QlfrInd", skip_serializing_if = "Option::is_none")]
	pub qlfr_ind: Option<bool>,
	#[serde(rename = "SbjtInd", skip_serializing_if = "Option::is_none")]
	pub sbjt_ind: Option<bool>,
	#[serde(rename = "SbjtDtlsInd", skip_serializing_if = "Option::is_none")]
	pub sbjt_dtls_ind: Option<bool>,
}

impl GeneralBusinessInformationReturnCriteria1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// GeneralBusinessInformationSearchCriteria1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GeneralBusinessInformationSearchCriteria1 {
	#[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
	pub ref_attr: Option<Vec<Max35Text>>,
	#[serde(rename = "Sbjt", skip_serializing_if = "Option::is_none")]
	pub sbjt: Option<Vec<CharacterSearch1Choice>>,
	#[serde(rename = "Qlfr", skip_serializing_if = "Option::is_none")]
	pub qlfr: Option<Vec<InformationQualifierType1>>,
}

impl GeneralBusinessInformationSearchCriteria1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ref_attr_vec) = self.ref_attr { for item in ref_attr_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref sbjt_vec) = self.sbjt { for item in sbjt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref qlfr_vec) = self.qlfr { for item in qlfr_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// GetGeneralBusinessInformationV04 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GetGeneralBusinessInformationV04 {
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: MessageHeader1,
	#[serde(rename = "GnlBizInfQryDef", skip_serializing_if = "Option::is_none")]
	pub gnl_biz_inf_qry_def: Option<BusinessInformationQueryDefinition3>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl GetGeneralBusinessInformationV04 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_hdr.validate() { return Err(e); }
		if let Some(ref gnl_biz_inf_qry_def_value) = self.gnl_biz_inf_qry_def { if let Err(e) = gnl_biz_inf_qry_def_value.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}

impl ISODateTime {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InformationQualifierType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InformationQualifierType1 {
	#[serde(rename = "IsFrmtd", skip_serializing_if = "Option::is_none")]
	pub is_frmtd: Option<bool>,
	#[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
	pub prty: Option<Priority1Code>,
}

impl InformationQualifierType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref prty_value) = self.prty { if let Err(e) = prty_value.validate() { return Err(e); } }
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


// MessageHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader1 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
}

impl MessageHeader1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_id.validate() { return Err(e); }
		Ok(())
	}
}


// Priority1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Priority1Code {
	#[default]
	#[serde(rename = "HIGH")]
	CodeHIGH,
	#[serde(rename = "NORM")]
	CodeNORM,
	#[serde(rename = "LOWW")]
	CodeLOWW,
}

impl Priority1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// QueryType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum QueryType2Code {
	#[default]
	#[serde(rename = "ALLL")]
	CodeALLL,
	#[serde(rename = "CHNG")]
	CodeCHNG,
	#[serde(rename = "MODF")]
	CodeMODF,
	#[serde(rename = "DELD")]
	CodeDELD,
}

impl QueryType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RequestedIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RequestedIndicator {
	#[serde(rename = "$value")]
	pub requested_indicator: bool,
}

impl RequestedIndicator {
	pub fn validate(&self) -> Result<(), ValidationError> {
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


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}

impl YesNoIndicator {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
