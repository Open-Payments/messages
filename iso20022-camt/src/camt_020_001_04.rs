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
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BusinessInformationCriteria1 {
	#[serde(rename = "NewQryNm")]
	pub new_qry_nm: Option<String>,
	#[serde(rename = "SchCrit")]
	pub sch_crit: Option<Vec<GeneralBusinessInformationSearchCriteria1>>,
	#[serde(rename = "RtrCrit")]
	pub rtr_crit: Option<GeneralBusinessInformationReturnCriteria1>,
}


// BusinessInformationQueryDefinition3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BusinessInformationQueryDefinition3 {
	#[serde(rename = "QryTp")]
	pub qry_tp: Option<String>,
	#[serde(rename = "GnlBizInfCrit")]
	pub gnl_biz_inf_crit: Option<GeneralBusinessInformationCriteriaDefinition1Choice>,
}


// CharacterSearch1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CharacterSearch1Choice {
	#[serde(rename = "EQ")]
	pub eq: Option<String>,
	#[serde(rename = "NEQ")]
	pub neq: Option<String>,
	#[serde(rename = "CT")]
	pub ct: Option<String>,
	#[serde(rename = "NCT")]
	pub nct: Option<String>,
}


// GeneralBusinessInformationCriteriaDefinition1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GeneralBusinessInformationCriteriaDefinition1Choice {
	#[serde(rename = "QryNm")]
	pub qry_nm: Option<String>,
	#[serde(rename = "NewCrit")]
	pub new_crit: Option<BusinessInformationCriteria1>,
}


// GeneralBusinessInformationReturnCriteria1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GeneralBusinessInformationReturnCriteria1 {
	#[serde(rename = "QlfrInd")]
	pub qlfr_ind: Option<bool>,
	#[serde(rename = "SbjtInd")]
	pub sbjt_ind: Option<bool>,
	#[serde(rename = "SbjtDtlsInd")]
	pub sbjt_dtls_ind: Option<bool>,
}


// GeneralBusinessInformationSearchCriteria1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GeneralBusinessInformationSearchCriteria1 {
	#[serde(rename = "Ref")]
	pub ref_attr: Option<Vec<String>>,
	#[serde(rename = "Sbjt")]
	pub sbjt: Option<Vec<CharacterSearch1Choice>>,
	#[serde(rename = "Qlfr")]
	pub qlfr: Option<Vec<InformationQualifierType1>>,
}


// GetGeneralBusinessInformationV04 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GetGeneralBusinessInformationV04 {
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: MessageHeader1,
	#[serde(rename = "GnlBizInfQryDef")]
	pub gnl_biz_inf_qry_def: Option<BusinessInformationQueryDefinition3>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// InformationQualifierType1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct InformationQualifierType1 {
	#[serde(rename = "IsFrmtd")]
	pub is_frmtd: Option<bool>,
	#[serde(rename = "Prty")]
	pub prty: Option<String>,
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


// MessageHeader1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader1 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
}


// Priority1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Priority1Code {
	#[serde(rename = "Priority1Code")]
	pub priority1_code: String,
}


// QueryType2Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct QueryType2Code {
	#[serde(rename = "QueryType2Code")]
	pub query_type2_code: String,
}


// RequestedIndicator ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestedIndicator {
	#[serde(rename = "RequestedIndicator")]
	pub requested_indicator: bool,
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


// YesNoIndicator ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}
