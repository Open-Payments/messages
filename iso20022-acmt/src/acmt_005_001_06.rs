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
	pub acct_id: Max35Text,
	#[serde(rename = "RltdAcctDtls", skip_serializing_if = "Option::is_none")]
	pub rltd_acct_dtls: Option<GenericIdentification1>,
}


// AccountManagementMessageReference5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountManagementMessageReference5 {
	#[serde(rename = "LkdRef", skip_serializing_if = "Option::is_none")]
	pub lkd_ref: Option<LinkedMessage5Choice>,
	#[serde(rename = "StsReqTp")]
	pub sts_req_tp: AccountManagementType3Code,
	#[serde(rename = "AcctApplId", skip_serializing_if = "Option::is_none")]
	pub acct_appl_id: Option<Max35Text>,
	#[serde(rename = "ExstgAcctId", skip_serializing_if = "Option::is_none")]
	pub exstg_acct_id: Option<Account23>,
	#[serde(rename = "InvstmtAcct", skip_serializing_if = "Option::is_none")]
	pub invstmt_acct: Option<InvestmentAccount77>,
}


// AccountManagementType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AccountManagementType3Code {
	#[default]
	#[serde(rename = "ACCM")]
	CodeACCM,
	#[serde(rename = "ACCO")]
	CodeACCO,
	#[serde(rename = "GACC")]
	CodeGACC,
	#[serde(rename = "ACST")]
	CodeACST,
}


// AdditionalReference13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditionalReference13 {
	#[serde(rename = "Ref")]
	pub ref_attr: Max35Text,
	#[serde(rename = "RefIssr", skip_serializing_if = "Option::is_none")]
	pub ref_issr: Option<PartyIdentification125Choice>,
	#[serde(rename = "MsgNm", skip_serializing_if = "Option::is_none")]
	pub msg_nm: Option<Max35Text>,
}


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AddressType2Code {
	#[default]
	#[serde(rename = "ADDR")]
	CodeADDR,
	#[serde(rename = "PBOX")]
	CodePBOX,
	#[serde(rename = "HOME")]
	CodeHOME,
	#[serde(rename = "BIZZ")]
	CodeBIZZ,
	#[serde(rename = "MLTO")]
	CodeMLTO,
	#[serde(rename = "DLVY")]
	CodeDLVY,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}


// GenderCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum GenderCode {
	#[default]
	#[serde(rename = "MALE")]
	CodeMALE,
	#[serde(rename = "FEMA")]
	CodeFEMA,
}


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}


// GenericIdentification47 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification47 {
	#[serde(rename = "Id")]
	pub id: Exact4AlphaNumericText,
	#[serde(rename = "Issr")]
	pub issr: Max4AlphaNumericText,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max4AlphaNumericText>,
}


// GenericIdentification81 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification81 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "IdTp")]
	pub id_tp: OtherIdentification3Choice,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}


// IndividualPerson30 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndividualPerson30 {
	#[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
	pub gvn_nm: Option<Max35Text>,
	#[serde(rename = "MddlNm", skip_serializing_if = "Option::is_none")]
	pub mddl_nm: Option<Max35Text>,
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Gndr", skip_serializing_if = "Option::is_none")]
	pub gndr: Option<GenderCode>,
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
	pub acct_id: Max35Text,
	#[serde(rename = "AcctNm", skip_serializing_if = "Option::is_none")]
	pub acct_nm: Option<Max35Text>,
	#[serde(rename = "AcctDsgnt", skip_serializing_if = "Option::is_none")]
	pub acct_dsgnt: Option<Max35Text>,
	#[serde(rename = "OwnrId", skip_serializing_if = "Option::is_none")]
	pub ownr_id: Option<OwnerIdentification3Choice>,
	#[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
	pub acct_svcr: Option<PartyIdentification125Choice>,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
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
#[serde(transparent)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "$value")]
	pub max4_alpha_numeric_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}


// MessageIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}


// OtherIdentification3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PartyIdentificationType7Code>,
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
	pub any_bic: Option<AnyBICDec2014Identifier>,
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
	pub lei: Option<LEIIdentifier>,
}


// PartyIdentificationType7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PartyIdentificationType7Code {
	#[default]
	#[serde(rename = "ATIN")]
	CodeATIN,
	#[serde(rename = "IDCD")]
	CodeIDCD,
	#[serde(rename = "NRIN")]
	CodeNRIN,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "PASS")]
	CodePASS,
	#[serde(rename = "POCD")]
	CodePOCD,
	#[serde(rename = "SOCS")]
	CodeSOCS,
	#[serde(rename = "SRSA")]
	CodeSRSA,
	#[serde(rename = "GUNL")]
	CodeGUNL,
	#[serde(rename = "GTIN")]
	CodeGTIN,
	#[serde(rename = "ITIN")]
	CodeITIN,
	#[serde(rename = "CPFA")]
	CodeCPFA,
	#[serde(rename = "AREG")]
	CodeAREG,
	#[serde(rename = "DRLC")]
	CodeDRLC,
	#[serde(rename = "EMID")]
	CodeEMID,
	#[serde(rename = "NINV")]
	CodeNINV,
	#[serde(rename = "INCL")]
	CodeINCL,
	#[serde(rename = "GIIN")]
	CodeGIIN,
}


// PostalAddress1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType2Code>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<Max70Text>>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max70Text>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<Max16Text>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<Max16Text>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<Max35Text>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<Max35Text>,
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
}


// RequestForAccountManagementStatusReportV06 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestForAccountManagementStatusReportV06 {
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "ReqDtls")]
	pub req_dtls: AccountManagementMessageReference5,
}
