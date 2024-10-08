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


// BICFIDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "BICFIDec2014Identifier")]
	pub bicfi_dec2014_identifier: String,
}


// ClearingSystemIdentification2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemIdentification2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ClearingSystemMemberIdentification2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemMemberIdentification2 {
	#[serde(rename = "ClrSysId")]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[serde(rename = "MmbId")]
	pub mmb_id: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "ExternalClearingSystemIdentification1Code")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalEnquiryRequestType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalEnquiryRequestType1Code {
	#[serde(rename = "ExternalEnquiryRequestType1Code")]
	pub external_enquiry_request_type1_code: String,
}


// ExternalFinancialInstitutionIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "ExternalFinancialInstitutionIdentification1Code")]
	pub external_financial_institution_identification1_code: String,
}


// ExternalPaymentControlRequestType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPaymentControlRequestType1Code {
	#[serde(rename = "ExternalPaymentControlRequestType1Code")]
	pub external_payment_control_request_type1_code: String,
}


// ExternalSystemMemberType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalSystemMemberType1Code {
	#[serde(rename = "ExternalSystemMemberType1Code")]
	pub external_system_member_type1_code: String,
}


// FinancialIdentificationSchemeName1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// GenericFinancialIdentification1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericFinancialIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GetMemberV04 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GetMemberV04 {
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: MessageHeader9,
	#[serde(rename = "MmbQryDef")]
	pub mmb_qry_def: Option<MemberQueryDefinition4>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
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


// MemberCriteria4 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MemberCriteria4 {
	#[serde(rename = "NewQryNm")]
	pub new_qry_nm: Option<String>,
	#[serde(rename = "SchCrit")]
	pub sch_crit: Option<Vec<MemberSearchCriteria4>>,
	#[serde(rename = "RtrCrit")]
	pub rtr_crit: Option<MemberReturnCriteria1>,
}


// MemberCriteriaDefinition2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MemberCriteriaDefinition2Choice {
	#[serde(rename = "QryNm")]
	pub qry_nm: Option<String>,
	#[serde(rename = "NewCrit")]
	pub new_crit: Option<MemberCriteria4>,
}


// MemberIdentification3Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MemberIdentification3Choice {
	#[serde(rename = "BICFI")]
	pub bicfi: Option<String>,
	#[serde(rename = "ClrSysMmbId")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[serde(rename = "Othr")]
	pub othr: Option<GenericFinancialIdentification1>,
}


// MemberQueryDefinition4 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MemberQueryDefinition4 {
	#[serde(rename = "QryTp")]
	pub qry_tp: Option<String>,
	#[serde(rename = "MmbCrit")]
	pub mmb_crit: Option<MemberCriteriaDefinition2Choice>,
}


// MemberReturnCriteria1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MemberReturnCriteria1 {
	#[serde(rename = "NmInd")]
	pub nm_ind: Option<bool>,
	#[serde(rename = "MmbRtrAdrInd")]
	pub mmb_rtr_adr_ind: Option<bool>,
	#[serde(rename = "AcctInd")]
	pub acct_ind: Option<bool>,
	#[serde(rename = "TpInd")]
	pub tp_ind: Option<bool>,
	#[serde(rename = "StsInd")]
	pub sts_ind: Option<bool>,
	#[serde(rename = "CtctRefInd")]
	pub ctct_ref_ind: Option<bool>,
	#[serde(rename = "ComAdrInd")]
	pub com_adr_ind: Option<bool>,
}


// MemberSearchCriteria4 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MemberSearchCriteria4 {
	#[serde(rename = "Id")]
	pub id: Option<Vec<MemberIdentification3Choice>>,
	#[serde(rename = "Tp")]
	pub tp: Option<Vec<SystemMemberType1Choice>>,
	#[serde(rename = "Sts")]
	pub sts: Option<Vec<SystemMemberStatus1Choice>>,
}


// MemberStatus1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MemberStatus1Code {
	#[serde(rename = "MemberStatus1Code")]
	pub member_status1_code: String,
}


// MessageHeader9 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader9 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "ReqTp")]
	pub req_tp: Option<RequestType4Choice>,
}


// QueryType2Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct QueryType2Code {
	#[serde(rename = "QueryType2Code")]
	pub query_type2_code: String,
}


// RequestType4Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestType4Choice {
	#[serde(rename = "PmtCtrl")]
	pub pmt_ctrl: Option<String>,
	#[serde(rename = "Enqry")]
	pub enqry: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification1>,
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


// SystemMemberStatus1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemMemberStatus1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// SystemMemberType1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemMemberType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}
