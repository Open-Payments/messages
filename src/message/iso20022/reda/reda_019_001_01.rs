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
use serde_valid::Validate;


// AddressType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[validate(enumerate = ["ADDR", "PBOX", "HOME", "BIZZ", "MLTO", "DLVY"])]
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[validate(pattern = "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}")]
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CountryCode {
	#[validate(pattern = "[A-Z]{2,2}")]
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DatePeriod2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DatePeriod2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// DatePeriodSearch1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DatePeriodSearch1Choice {
	#[serde(rename = "FrDt")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt")]
	pub to_dt: Option<String>,
	#[validate]
	#[serde(rename = "FrToDt")]
	pub fr_to_dt: Option<DatePeriod2>,
	#[serde(rename = "EQDt")]
	pub eq_dt: Option<String>,
	#[serde(rename = "NEQDt")]
	pub neq_dt: Option<String>,
}


// Exact4AlphaNumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[validate(pattern = "[a-zA-Z0-9]{4}")]
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// ExternalSystemPartyType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalSystemPartyType1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalSystemPartyType1Code")]
	pub external_system_party_type1_code: String,
}


// GenericIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification30 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// GenericIdentification36 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// ISODate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[validate(pattern = "[A-Z0-9]{18,18}[0-9]{2,2}")]
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// Max16Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max16Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 16)]
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max350Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 350)]
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max35Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max70Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max70Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 70)]
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// MessageHeader2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MessageHeader2 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
	#[validate]
	#[serde(rename = "ReqTp")]
	pub req_tp: Option<RequestType2Choice>,
}


// NameAndAddress5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[validate]
	#[serde(rename = "Adr")]
	pub adr: Option<PostalAddress1>,
}


// PartyIdentification120Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification120Choice {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[validate]
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<GenericIdentification36>,
	#[validate]
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification136 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification136 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: PartyIdentification120Choice,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
}


// PostalAddress1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
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
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RequestType1Code {
	#[validate(enumerate = ["RT01", "RT02", "RT03", "RT04", "RT05"])]
	#[serde(rename = "RequestType1Code")]
	pub request_type1_code: String,
}


// RequestType2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RequestType2Choice {
	#[serde(rename = "PmtCtrl")]
	pub pmt_ctrl: Option<String>,
	#[serde(rename = "Enqry")]
	pub enqry: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification1>,
}


// RequestType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RequestType2Code {
	#[validate(enumerate = ["RT11", "RT12", "RT13", "RT14", "RT15"])]
	#[serde(rename = "RequestType2Code")]
	pub request_type2_code: String,
}


// RequestedIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RequestedIndicator {
	#[serde(rename = "RequestedIndicator")]
	pub requested_indicator: bool,
}


// SecuritiesAccountQueryV01 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesAccountQueryV01 {
	#[validate]
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: Option<MessageHeader2>,
	#[validate]
	#[serde(rename = "SchCrit")]
	pub sch_crit: SecuritiesAccountSearchCriteria2,
	#[validate]
	#[serde(rename = "RtrCrit")]
	pub rtr_crit: Option<SecuritiesAccountReturnCriteria1>,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SecuritiesAccountReturnCriteria1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesAccountReturnCriteria1 {
	#[serde(rename = "AcctId")]
	pub acct_id: Option<bool>,
	#[serde(rename = "PtyId")]
	pub pty_id: Option<bool>,
	#[serde(rename = "PtyTp")]
	pub pty_tp: Option<bool>,
	#[serde(rename = "AcctSvcr")]
	pub acct_svcr: Option<bool>,
	#[serde(rename = "AcctTp")]
	pub acct_tp: Option<bool>,
	#[serde(rename = "OpngDt")]
	pub opng_dt: Option<bool>,
	#[serde(rename = "ClsgDt")]
	pub clsg_dt: Option<bool>,
	#[serde(rename = "EndInvstrFlg")]
	pub end_invstr_flg: Option<bool>,
	#[serde(rename = "PricgSchme")]
	pub pricg_schme: Option<bool>,
}


// SecuritiesAccountSearchCriteria2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesAccountSearchCriteria2 {
	#[serde(rename = "AcctId")]
	pub acct_id: Option<String>,
	#[validate]
	#[serde(rename = "AcctSvcr")]
	pub acct_svcr: Option<PartyIdentification136>,
	#[validate]
	#[serde(rename = "AcctOwnr")]
	pub acct_ownr: Option<SystemPartyIdentification8>,
	#[validate]
	#[serde(rename = "PtyTp")]
	pub pty_tp: Option<SystemPartyType1Choice>,
	#[validate]
	#[serde(rename = "OpngDt")]
	pub opng_dt: Option<DatePeriodSearch1Choice>,
	#[validate]
	#[serde(rename = "ClsgDt")]
	pub clsg_dt: Option<DatePeriodSearch1Choice>,
	#[validate]
	#[serde(rename = "AcctTp")]
	pub acct_tp: Option<SystemSecuritiesAccountType1Choice>,
	#[serde(rename = "EndInvstrFlg")]
	pub end_invstr_flg: Option<String>,
	#[serde(rename = "PricgSchme")]
	pub pricg_schme: Option<String>,
}


// SupplementaryData1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[validate]
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// SystemPartyIdentification8 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SystemPartyIdentification8 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: PartyIdentification136,
	#[validate]
	#[serde(rename = "RspnsblPtyId")]
	pub rspnsbl_pty_id: Option<PartyIdentification136>,
}


// SystemPartyType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SystemPartyType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// SystemSecuritiesAccountType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SystemSecuritiesAccountType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// SystemSecuritiesAccountType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SystemSecuritiesAccountType1Code {
	#[validate(enumerate = ["CSDP", "CSDM", "ICSA", "TOFF", "CSDO", "ISSA"])]
	#[serde(rename = "SystemSecuritiesAccountType1Code")]
	pub system_securities_account_type1_code: String,
}
