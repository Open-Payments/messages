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


// AddressType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AddressType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// BICFIDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "$value")]
	pub bicfi_dec2014_identifier: String,
}


// CodeOrProprietary1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CodeOrProprietary1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Max4Text>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification13>,
}


// Contact14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Contact14 {
	#[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
	pub nm_prfx: Option<NamePrefix2Code>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
	pub phne_nb: Option<PhoneNumber>,
	#[serde(rename = "MobNb", skip_serializing_if = "Option::is_none")]
	pub mob_nb: Option<PhoneNumber>,
	#[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
	pub fax_nb: Option<PhoneNumber>,
	#[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
	pub url_adr: Option<Max2048Text>,
	#[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
	pub email_adr: Option<Max256Text>,
	#[serde(rename = "EmailPurp", skip_serializing_if = "Option::is_none")]
	pub email_purp: Option<Max35Text>,
	#[serde(rename = "JobTitl", skip_serializing_if = "Option::is_none")]
	pub job_titl: Option<Max35Text>,
	#[serde(rename = "Rspnsblty", skip_serializing_if = "Option::is_none")]
	pub rspnsblty: Option<Max35Text>,
	#[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
	pub dept: Option<Max70Text>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<OtherContact1>>,
	#[serde(rename = "PrefrdMtd", skip_serializing_if = "Option::is_none")]
	pub prefrd_mtd: Option<PreferredContactMethod2Code>,
	#[serde(rename = "VldFr", skip_serializing_if = "Option::is_none")]
	pub vld_fr: Option<String>,
	#[serde(rename = "VldTo", skip_serializing_if = "Option::is_none")]
	pub vld_to: Option<String>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// DatePeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// DatePeriod3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod3Choice {
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
	pub to_dt: Option<String>,
	#[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt: Option<DatePeriod2>,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
}


// ErrorHandling3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalSystemErrorHandling1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// ErrorHandling5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling5 {
	#[serde(rename = "Err")]
	pub err: ErrorHandling3Choice,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}


// ExternalSystemErrorHandling1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalSystemErrorHandling1Code {
	#[serde(rename = "$value")]
	pub external_system_error_handling1_code: String,
}


// ExternalSystemPartyType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalSystemPartyType1Code {
	#[serde(rename = "$value")]
	pub external_system_party_type1_code: String,
}


// GenericIdentification13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification13 {
	#[serde(rename = "Id")]
	pub id: Max4AlphaNumericText,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
}


// GenericIdentification30 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: Exact4AlphaNumericText,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
}


// GenericIdentification36 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// LockStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum LockStatus1Code {
	#[default]
	#[serde(rename = "LOCK")]
	CodeLOCK,
	#[serde(rename = "ULCK")]
	CodeULCK,

}


// MarketSpecificAttribute1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketSpecificAttribute1 {
	#[serde(rename = "Nm")]
	pub nm: Max35Text,
	#[serde(rename = "Val")]
	pub val: Max350Text,
}


// Max128Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max128Text {
	#[serde(rename = "$value")]
	pub max128_text: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
}


// Max2048Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max2048Text {
	#[serde(rename = "$value")]
	pub max2048_text: String,
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max256Text {
	#[serde(rename = "$value")]
	pub max256_text: String,
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


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "$value")]
	pub max4_alpha_numeric_text: String,
}


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "$value")]
	pub max4_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}


// MessageHeader12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader12 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "OrgnlBizInstr", skip_serializing_if = "Option::is_none")]
	pub orgnl_biz_instr: Option<OriginalBusinessInstruction1>,
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}


// NamePrefix2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NamePrefix2Code {
	#[default]
	#[serde(rename = "DOCT")]
	CodeDOCT,
	#[serde(rename = "MADM")]
	CodeMADM,
	#[serde(rename = "MISS")]
	CodeMISS,
	#[serde(rename = "MIST")]
	CodeMIST,
	#[serde(rename = "MIKS")]
	CodeMIKS,

}


// OriginalBusinessInstruction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalBusinessInstruction1 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none")]
	pub msg_nm_id: Option<Max35Text>,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
}


// OtherContact1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherContact1 {
	#[serde(rename = "ChanlTp")]
	pub chanl_tp: Max4Text,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max128Text>,
}


// PartyAuditTrail2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyAuditTrail2 {
	#[serde(rename = "Rcrd")]
	pub rcrd: Vec<UpdateLogPartyRecord2Choice>,
	#[serde(rename = "OprTmStmp")]
	pub opr_tm_stmp: String,
	#[serde(rename = "InstgUsr")]
	pub instg_usr: Max256Text,
	#[serde(rename = "ApprvgUsr", skip_serializing_if = "Option::is_none")]
	pub apprvg_usr: Option<Max256Text>,
}


// PartyAuditTrailOrError3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyAuditTrailOrError3Choice {
	#[serde(rename = "PtyAudtTrlRpt", skip_serializing_if = "Option::is_none")]
	pub pty_audt_trl_rpt: Option<Vec<PartyAuditTrailReport4>>,
	#[serde(rename = "OprlErr", skip_serializing_if = "Option::is_none")]
	pub oprl_err: Option<Vec<ErrorHandling5>>,
}


// PartyAuditTrailOrError4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyAuditTrailOrError4Choice {
	#[serde(rename = "AudtTrl", skip_serializing_if = "Option::is_none")]
	pub audt_trl: Option<Vec<PartyAuditTrail2>>,
	#[serde(rename = "BizErr", skip_serializing_if = "Option::is_none")]
	pub biz_err: Option<Vec<ErrorHandling5>>,
}


// PartyAuditTrailReport4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyAuditTrailReport4 {
	#[serde(rename = "PtyAudtTrlOrErr")]
	pub pty_audt_trl_or_err: PartyAuditTrailOrError4Choice,
	#[serde(rename = "DtPrd", skip_serializing_if = "Option::is_none")]
	pub dt_prd: Option<DatePeriod3Choice>,
	#[serde(rename = "PtyId")]
	pub pty_id: SystemPartyIdentification8,
}


// PartyAuditTrailReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyAuditTrailReportV02 {
	#[serde(rename = "MsgHdr", skip_serializing_if = "Option::is_none")]
	pub msg_hdr: Option<MessageHeader12>,
	#[serde(rename = "RptOrErr")]
	pub rpt_or_err: PartyAuditTrailOrError3Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// PartyIdentification120Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification120Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification36>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification136 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification136 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification120Choice,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
}


// PartyLockStatus1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyLockStatus1 {
	#[serde(rename = "VldFr", skip_serializing_if = "Option::is_none")]
	pub vld_fr: Option<String>,
	#[serde(rename = "Sts")]
	pub sts: LockStatus1Code,
	#[serde(rename = "LckRsn", skip_serializing_if = "Option::is_none")]
	pub lck_rsn: Option<Vec<Max35Text>>,
}


// PartyName4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyName4 {
	#[serde(rename = "VldFr", skip_serializing_if = "Option::is_none")]
	pub vld_fr: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
	pub shrt_nm: Option<Max35Text>,
}


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[serde(rename = "$value")]
	pub phone_number: String,
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


// PostalAddress28 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress28 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType3Choice>,
	#[serde(rename = "CareOf", skip_serializing_if = "Option::is_none")]
	pub care_of: Option<Max140Text>,
	#[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
	pub dept: Option<Max70Text>,
	#[serde(rename = "SubDept", skip_serializing_if = "Option::is_none")]
	pub sub_dept: Option<Max70Text>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max140Text>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<Max16Text>,
	#[serde(rename = "BldgNm", skip_serializing_if = "Option::is_none")]
	pub bldg_nm: Option<Max140Text>,
	#[serde(rename = "Flr", skip_serializing_if = "Option::is_none")]
	pub flr: Option<Max70Text>,
	#[serde(rename = "UnitNb", skip_serializing_if = "Option::is_none")]
	pub unit_nb: Option<Max16Text>,
	#[serde(rename = "PstBx", skip_serializing_if = "Option::is_none")]
	pub pst_bx: Option<Max16Text>,
	#[serde(rename = "Room", skip_serializing_if = "Option::is_none")]
	pub room: Option<Max70Text>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<Max16Text>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<Max140Text>,
	#[serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none")]
	pub twn_lctn_nm: Option<Max140Text>,
	#[serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none")]
	pub dstrct_nm: Option<Max140Text>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<Max35Text>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<Max70Text>>,
	#[serde(rename = "VldFr", skip_serializing_if = "Option::is_none")]
	pub vld_fr: Option<String>,
}


// PreferredContactMethod2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PreferredContactMethod2Code {
	#[default]
	#[serde(rename = "MAIL")]
	CodeMAIL,
	#[serde(rename = "FAXX")]
	CodeFAXX,
	#[serde(rename = "LETT")]
	CodeLETT,
	#[serde(rename = "CELL")]
	CodeCELL,
	#[serde(rename = "ONLI")]
	CodeONLI,
	#[serde(rename = "PHON")]
	CodePHON,

}


// ResidenceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ResidenceType1Code {
	#[default]
	#[serde(rename = "DMST")]
	CodeDMST,
	#[serde(rename = "FRGN")]
	CodeFRGN,
	#[serde(rename = "MXED")]
	CodeMXED,

}


// Restriction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Restriction1 {
	#[serde(rename = "RstrctnTp")]
	pub rstrctn_tp: CodeOrProprietary1Choice,
	#[serde(rename = "VldFr")]
	pub vld_fr: String,
	#[serde(rename = "VldUntil", skip_serializing_if = "Option::is_none")]
	pub vld_until: Option<String>,
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


// SystemPartyIdentification8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemPartyIdentification8 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification136,
	#[serde(rename = "RspnsblPtyId", skip_serializing_if = "Option::is_none")]
	pub rspnsbl_pty_id: Option<PartyIdentification136>,
}


// SystemPartyType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemPartyType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalSystemPartyType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// TechnicalIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TechnicalIdentification2Choice {
	#[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
	pub bicfi: Option<BICFIDec2014Identifier>,
	#[serde(rename = "TechAdr", skip_serializing_if = "Option::is_none")]
	pub tech_adr: Option<Max256Text>,
}


// UpdateLogAddress2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogAddress2 {
	#[serde(rename = "Od")]
	pub od: PostalAddress28,
	#[serde(rename = "New")]
	pub new: PostalAddress28,
}


// UpdateLogContact2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogContact2 {
	#[serde(rename = "Od")]
	pub od: Contact14,
	#[serde(rename = "New")]
	pub new: Contact14,
}


// UpdateLogDate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogDate1 {
	#[serde(rename = "Od")]
	pub od: String,
	#[serde(rename = "New")]
	pub new: String,
}


// UpdateLogMarketSpecificAttribute1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogMarketSpecificAttribute1 {
	#[serde(rename = "Od")]
	pub od: MarketSpecificAttribute1,
	#[serde(rename = "New")]
	pub new: MarketSpecificAttribute1,
}


// UpdateLogPartyLockStatus1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogPartyLockStatus1 {
	#[serde(rename = "Od")]
	pub od: PartyLockStatus1,
	#[serde(rename = "New")]
	pub new: PartyLockStatus1,
}


// UpdateLogPartyName1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogPartyName1 {
	#[serde(rename = "Od")]
	pub od: PartyName4,
	#[serde(rename = "New")]
	pub new: PartyName4,
}


// UpdateLogPartyRecord2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogPartyRecord2Choice {
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<UpdateLogAddress2>,
	#[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
	pub ctct_dtls: Option<UpdateLogContact2>,
	#[serde(rename = "OpngDt", skip_serializing_if = "Option::is_none")]
	pub opng_dt: Option<UpdateLogDate1>,
	#[serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none")]
	pub clsg_dt: Option<UpdateLogDate1>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<UpdateLogSystemPartyType1>,
	#[serde(rename = "TechAdr", skip_serializing_if = "Option::is_none")]
	pub tech_adr: Option<UpdateLogTechnicalAddress1>,
	#[serde(rename = "MktSpcfcAttr", skip_serializing_if = "Option::is_none")]
	pub mkt_spcfc_attr: Option<UpdateLogMarketSpecificAttribute1>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<UpdateLogPartyName1>,
	#[serde(rename = "ResTp", skip_serializing_if = "Option::is_none")]
	pub res_tp: Option<UpdateLogResidenceType1>,
	#[serde(rename = "LckSts", skip_serializing_if = "Option::is_none")]
	pub lck_sts: Option<UpdateLogPartyLockStatus1>,
	#[serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none")]
	pub rstrctn: Option<UpdateLogRestriction1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<UpdateLogProprietary1>>,
}


// UpdateLogProprietary1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogProprietary1 {
	#[serde(rename = "FldNm")]
	pub fld_nm: Max35Text,
	#[serde(rename = "OdFldVal")]
	pub od_fld_val: Max350Text,
	#[serde(rename = "NewFldVal")]
	pub new_fld_val: Max350Text,
}


// UpdateLogResidenceType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogResidenceType1 {
	#[serde(rename = "Od")]
	pub od: ResidenceType1Code,
	#[serde(rename = "New")]
	pub new: ResidenceType1Code,
}


// UpdateLogRestriction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogRestriction1 {
	#[serde(rename = "Od")]
	pub od: Restriction1,
	#[serde(rename = "New")]
	pub new: Restriction1,
}


// UpdateLogSystemPartyType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogSystemPartyType1 {
	#[serde(rename = "Od")]
	pub od: SystemPartyType1Choice,
	#[serde(rename = "New")]
	pub new: SystemPartyType1Choice,
}


// UpdateLogTechnicalAddress1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogTechnicalAddress1 {
	#[serde(rename = "Od")]
	pub od: TechnicalIdentification2Choice,
	#[serde(rename = "New")]
	pub new: TechnicalIdentification2Choice,
}
