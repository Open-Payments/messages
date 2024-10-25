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


// BusinessInformationCriteria1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BusinessInformationCriteria1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NewQryNm", skip_serializing_if = "Option::is_none") )]
	pub new_qry_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchCrit", skip_serializing_if = "Option::is_none") )]
	pub sch_crit: Option<Vec<GeneralBusinessInformationSearchCriteria1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrCrit", skip_serializing_if = "Option::is_none") )]
	pub rtr_crit: Option<GeneralBusinessInformationReturnCriteria1>,
}

impl BusinessInformationCriteria1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.new_qry_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "new_qry_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "new_qry_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.sch_crit { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.rtr_crit { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// BusinessInformationQueryDefinition3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BusinessInformationQueryDefinition3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "QryTp", skip_serializing_if = "Option::is_none") )]
	pub qry_tp: Option<QueryType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GnlBizInfCrit", skip_serializing_if = "Option::is_none") )]
	pub gnl_biz_inf_crit: Option<GeneralBusinessInformationCriteriaDefinition1Choice>,
}

impl BusinessInformationQueryDefinition3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.qry_tp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.gnl_biz_inf_crit { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// CharacterSearch1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CharacterSearch1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "EQ", skip_serializing_if = "Option::is_none") )]
	pub eq: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEQ", skip_serializing_if = "Option::is_none") )]
	pub neq: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CT", skip_serializing_if = "Option::is_none") )]
	pub ct: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NCT", skip_serializing_if = "Option::is_none") )]
	pub nct: Option<String>,
}

impl CharacterSearch1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.eq {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "eq is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "eq exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.neq {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "neq is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "neq exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ct {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ct is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ct exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.nct {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nct is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nct exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GeneralBusinessInformationCriteriaDefinition1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GeneralBusinessInformationCriteriaDefinition1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "QryNm", skip_serializing_if = "Option::is_none") )]
	pub qry_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NewCrit", skip_serializing_if = "Option::is_none") )]
	pub new_crit: Option<BusinessInformationCriteria1>,
}

impl GeneralBusinessInformationCriteriaDefinition1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.qry_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "qry_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "qry_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.new_crit { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// GeneralBusinessInformationReturnCriteria1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GeneralBusinessInformationReturnCriteria1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "QlfrInd", skip_serializing_if = "Option::is_none") )]
	pub qlfr_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SbjtInd", skip_serializing_if = "Option::is_none") )]
	pub sbjt_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SbjtDtlsInd", skip_serializing_if = "Option::is_none") )]
	pub sbjt_dtls_ind: Option<bool>,
}

impl GeneralBusinessInformationReturnCriteria1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// GeneralBusinessInformationSearchCriteria1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GeneralBusinessInformationSearchCriteria1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ref", skip_serializing_if = "Option::is_none") )]
	pub ref_attr: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sbjt", skip_serializing_if = "Option::is_none") )]
	pub sbjt: Option<Vec<CharacterSearch1Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Qlfr", skip_serializing_if = "Option::is_none") )]
	pub qlfr: Option<Vec<InformationQualifierType1>>,
}

impl GeneralBusinessInformationSearchCriteria1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.ref_attr {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "ref_attr is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "ref_attr exceeds the maximum length of 35".to_string()));
				}
			}
		}
		if let Some(ref vec) = self.sbjt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref vec) = self.qlfr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// GetGeneralBusinessInformationV04 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GetGeneralBusinessInformationV04 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgHdr") )]
	pub msg_hdr: MessageHeader1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GnlBizInfQryDef", skip_serializing_if = "Option::is_none") )]
	pub gnl_biz_inf_qry_def: Option<BusinessInformationQueryDefinition3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl GetGeneralBusinessInformationV04 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_hdr.validate() { return Err(e); }
		if let Some(ref val) = self.gnl_biz_inf_qry_def { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// InformationQualifierType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InformationQualifierType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsFrmtd", skip_serializing_if = "Option::is_none") )]
	pub is_frmtd: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prty", skip_serializing_if = "Option::is_none") )]
	pub prty: Option<Priority1Code>,
}

impl InformationQualifierType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.prty { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// MessageHeader1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MessageHeader1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
}

impl MessageHeader1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// Priority1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum Priority1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "HIGH") )]
	CodeHIGH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NORM") )]
	CodeNORM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LOWW") )]
	CodeLOWW,
}

impl Priority1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// QueryType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum QueryType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ALLL") )]
	CodeALLL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHNG") )]
	CodeCHNG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MODF") )]
	CodeMODF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DELD") )]
	CodeDELD,
}

impl QueryType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
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
		if let Err(e) = self.envlp.validate() { return Err(e); }
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
