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


// AddressType2Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DateAndDateTimeSearch4Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTimeSearch4Choice {
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<DateTimeSearch2Choice>,
	#[serde(rename = "Dt")]
	pub dt: Option<DatePeriodSearch1Choice>,
}


// DatePeriod2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// DatePeriodSearch1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriodSearch1Choice {
	#[serde(rename = "FrDt")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt")]
	pub to_dt: Option<String>,
	#[serde(rename = "FrToDt")]
	pub fr_to_dt: Option<DatePeriod2>,
	#[serde(rename = "EQDt")]
	pub eq_dt: Option<String>,
	#[serde(rename = "NEQDt")]
	pub neq_dt: Option<String>,
}


// DateTimePeriod1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}


// DateTimeSearch2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimeSearch2Choice {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: Option<String>,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: Option<String>,
	#[serde(rename = "FrToDtTm")]
	pub fr_to_dt_tm: Option<DateTimePeriod1>,
	#[serde(rename = "EQDtTm")]
	pub eq_dt_tm: Option<String>,
	#[serde(rename = "NEQDtTm")]
	pub neq_dt_tm: Option<String>,
}


// ExternalSystemPartyType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalSystemPartyType1Code {
	#[serde(rename = "ExternalSystemPartyType1Code")]
	pub external_system_party_type1_code: String,
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


// GenericIdentification36 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// ISODate ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LockStatus1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LockStatus1Code {
	#[serde(rename = "LockStatus1Code")]
	pub lock_status1_code: String,
}


// Max16Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
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


// Max70Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// MessageHeader2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader2 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "ReqTp")]
	pub req_tp: Option<RequestType2Choice>,
}


// NameAndAddress5 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Adr")]
	pub adr: Option<PostalAddress1>,
}


// PartyDataReturnCriteria2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyDataReturnCriteria2 {
	#[serde(rename = "OpngDt")]
	pub opng_dt: Option<bool>,
	#[serde(rename = "ClsgDt")]
	pub clsg_dt: Option<bool>,
	#[serde(rename = "Tp")]
	pub tp: Option<bool>,
	#[serde(rename = "PtyId")]
	pub pty_id: Option<bool>,
	#[serde(rename = "RspnsblPtyId")]
	pub rspnsbl_pty_id: Option<bool>,
	#[serde(rename = "RstrctnId")]
	pub rstrctn_id: Option<bool>,
	#[serde(rename = "RstrctdOnDt")]
	pub rstrctd_on_dt: Option<bool>,
	#[serde(rename = "Nm")]
	pub nm: Option<bool>,
	#[serde(rename = "ShrtNm")]
	pub shrt_nm: Option<bool>,
	#[serde(rename = "Adr")]
	pub adr: Option<bool>,
	#[serde(rename = "TechAdr")]
	pub tech_adr: Option<bool>,
	#[serde(rename = "CtctDtls")]
	pub ctct_dtls: Option<bool>,
	#[serde(rename = "ResTp")]
	pub res_tp: Option<bool>,
	#[serde(rename = "LckSts")]
	pub lck_sts: Option<bool>,
	#[serde(rename = "MktSpcfcAttr")]
	pub mkt_spcfc_attr: Option<bool>,
}


// PartyDataSearchCriteria2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyDataSearchCriteria2 {
	#[serde(rename = "OpngDt")]
	pub opng_dt: Option<DatePeriodSearch1Choice>,
	#[serde(rename = "ClsgDt")]
	pub clsg_dt: Option<DatePeriodSearch1Choice>,
	#[serde(rename = "Tp")]
	pub tp: Option<SystemPartyType1Choice>,
	#[serde(rename = "RspnsblPtyId")]
	pub rspnsbl_pty_id: Option<PartyIdentification136>,
	#[serde(rename = "PtyId")]
	pub pty_id: Option<PartyIdentification136>,
	#[serde(rename = "RstrctnId")]
	pub rstrctn_id: Option<String>,
	#[serde(rename = "RstrctnIsseDt")]
	pub rstrctn_isse_dt: Option<DateAndDateTimeSearch4Choice>,
	#[serde(rename = "ResTp")]
	pub res_tp: Option<String>,
	#[serde(rename = "LckSts")]
	pub lck_sts: Option<PartyLockStatus1>,
}


// PartyIdentification120Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification120Choice {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<GenericIdentification36>,
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification136 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification136 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification120Choice,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
}


// PartyLockStatus1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyLockStatus1 {
	#[serde(rename = "VldFr")]
	pub vld_fr: Option<String>,
	#[serde(rename = "Sts")]
	pub sts: String,
	#[serde(rename = "LckRsn")]
	pub lck_rsn: Option<Vec<String>>,
}


// PartyQueryV01 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyQueryV01 {
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: Option<MessageHeader2>,
	#[serde(rename = "SchCrit")]
	pub sch_crit: PartyDataSearchCriteria2,
	#[serde(rename = "RtrCrit")]
	pub rtr_crit: Option<PartyDataReturnCriteria2>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// PostalAddress1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: Option<String>,
	#[serde(rename = "AdrLine")]
	pub adr_line: Option<Vec<String>>,
	#[serde(rename = "StrtNm")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "PstCd")]
	pub pst_cd: Option<String>,
	#[serde(rename = "TwnNm")]
	pub twn_nm: Option<String>,
	#[serde(rename = "CtrySubDvsn")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "Ctry")]
	pub ctry: String,
}


// RequestType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestType1Code {
	#[serde(rename = "RequestType1Code")]
	pub request_type1_code: String,
}


// RequestType2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestType2Choice {
	#[serde(rename = "PmtCtrl")]
	pub pmt_ctrl: Option<String>,
	#[serde(rename = "Enqry")]
	pub enqry: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification1>,
}


// RequestType2Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestType2Code {
	#[serde(rename = "RequestType2Code")]
	pub request_type2_code: String,
}


// RequestedIndicator ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestedIndicator {
	#[serde(rename = "RequestedIndicator")]
	pub requested_indicator: bool,
}


// ResidenceType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ResidenceType1Code {
	#[serde(rename = "ResidenceType1Code")]
	pub residence_type1_code: String,
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


// SystemPartyType1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemPartyType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}