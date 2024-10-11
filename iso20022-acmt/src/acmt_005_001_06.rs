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


// Account23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Account23 {
	#[serde(rename = "AcctId")]
	pub acct_id: String,
	#[serde(rename = "RltdAcctDtls", skip_serializing_if = "Option::is_none")]
	pub rltd_acct_dtls: Option<GenericIdentification1>,
}


// AccountManagementMessageReference5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountManagementMessageReference5 {
	#[serde(rename = "LkdRef", skip_serializing_if = "Option::is_none")]
	pub lkd_ref: Option<LinkedMessage5Choice>,
	#[serde(rename = "StsReqTp")]
	pub sts_req_tp: String,
	#[serde(rename = "AcctApplId", skip_serializing_if = "Option::is_none")]
	pub acct_appl_id: Option<String>,
	#[serde(rename = "ExstgAcctId", skip_serializing_if = "Option::is_none")]
	pub exstg_acct_id: Option<Account23>,
	#[serde(rename = "InvstmtAcct", skip_serializing_if = "Option::is_none")]
	pub invstmt_acct: Option<InvestmentAccount77>,
}


// AccountManagementType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountManagementType3Code {
	#[serde(rename = "AccountManagementType3Code")]
	pub account_management_type3_code: String,
}


// AdditionalReference13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditionalReference13 {
	#[serde(rename = "Ref")]
	pub ref_attr: String,
	#[serde(rename = "RefIssr", skip_serializing_if = "Option::is_none")]
	pub ref_issr: Option<PartyIdentification125Choice>,
	#[serde(rename = "MsgNm", skip_serializing_if = "Option::is_none")]
	pub msg_nm: Option<String>,
}


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// GenderCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenderCode {
	#[serde(rename = "GenderCode")]
	pub gender_code: String,
}


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// GenericIdentification47 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification47 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<String>,
}


// GenericIdentification81 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification81 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "IdTp")]
	pub id_tp: OtherIdentification3Choice,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// IndividualPerson30 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndividualPerson30 {
	#[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
	pub gvn_nm: Option<String>,
	#[serde(rename = "MddlNm", skip_serializing_if = "Option::is_none")]
	pub mddl_nm: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Gndr", skip_serializing_if = "Option::is_none")]
	pub gndr: Option<String>,
	#[serde(rename = "BirthDt", skip_serializing_if = "Option::is_none")]
	pub birth_dt: Option<String>,
}


// IndividualPersonIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndividualPersonIdentification2Choice {
	#[serde(rename = "IdNb", skip_serializing_if = "Option::is_none")]
	pub id_nb: Option<GenericIdentification81>,
	#[serde(rename = "PrsnNm", skip_serializing_if = "Option::is_none")]
	pub prsn_nm: Option<IndividualPerson30>,
}


// InvestmentAccount77 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentAccount77 {
	#[serde(rename = "AcctId")]
	pub acct_id: String,
	#[serde(rename = "AcctNm", skip_serializing_if = "Option::is_none")]
	pub acct_nm: Option<String>,
	#[serde(rename = "AcctDsgnt", skip_serializing_if = "Option::is_none")]
	pub acct_dsgnt: Option<String>,
	#[serde(rename = "OwnrId", skip_serializing_if = "Option::is_none")]
	pub ownr_id: Option<OwnerIdentification3Choice>,
	#[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
	pub acct_svcr: Option<PartyIdentification125Choice>,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LinkedMessage5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LinkedMessage5Choice {
	#[serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none")]
	pub prvs_ref: Option<AdditionalReference13>,
	#[serde(rename = "OthrRef", skip_serializing_if = "Option::is_none")]
	pub othr_ref: Option<AdditionalReference13>,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "Max4AlphaNumericText")]
	pub max4_alpha_numeric_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// MessageIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}


// OtherIdentification3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// OwnerIdentification3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OwnerIdentification3Choice {
	#[serde(rename = "IndvOwnrId", skip_serializing_if = "Option::is_none")]
	pub indv_ownr_id: Option<IndividualPersonIdentification2Choice>,
	#[serde(rename = "OrgOwnrId", skip_serializing_if = "Option::is_none")]
	pub org_ownr_id: Option<PartyIdentification139>,
}


// PartyIdentification125Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification125Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<String>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification1>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification139 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification139 {
	#[serde(rename = "Pty")]
	pub pty: PartyIdentification125Choice,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
}


// PartyIdentificationType7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentificationType7Code {
	#[serde(rename = "PartyIdentificationType7Code")]
	pub party_identification_type7_code: String,
}


// PostalAddress1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<String>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<String>>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<String>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<String>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "Ctry")]
	pub ctry: String,
}


// RequestForAccountManagementStatusReportV06 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestForAccountManagementStatusReportV06 {
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "ReqDtls")]
	pub req_dtls: AccountManagementMessageReference5,
}
