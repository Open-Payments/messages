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


// BusinessInformationQueryDefinition3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BusinessInformationQueryDefinition3 {
	#[serde(rename = "QryTp", skip_serializing_if = "Option::is_none")]
	pub qry_tp: Option<QueryType2Code>,
	#[serde(rename = "GnlBizInfCrit", skip_serializing_if = "Option::is_none")]
	pub gnl_biz_inf_crit: Option<GeneralBusinessInformationCriteriaDefinition1Choice>,
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


// GeneralBusinessInformationCriteriaDefinition1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GeneralBusinessInformationCriteriaDefinition1Choice {
	#[serde(rename = "QryNm", skip_serializing_if = "Option::is_none")]
	pub qry_nm: Option<Max35Text>,
	#[serde(rename = "NewCrit", skip_serializing_if = "Option::is_none")]
	pub new_crit: Option<BusinessInformationCriteria1>,
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


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}


// InformationQualifierType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InformationQualifierType1 {
	#[serde(rename = "IsFrmtd", skip_serializing_if = "Option::is_none")]
	pub is_frmtd: Option<bool>,
	#[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
	pub prty: Option<Priority1Code>,
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


// MessageHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader1 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
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


// RequestedIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestedIndicator {
	#[serde(rename = "$value")]
	pub requested_indicator: bool,
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


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}
