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


// AcceptedStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AcceptedStatusReason1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AcceptedStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// AcceptedStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AcceptedStatusReason1Code {
	#[default]
	#[serde(rename = "PLAC")]
	CodePLAC,
	#[serde(rename = "SECT")]
	CodeSECT,

}


// Account23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Account23 {
	#[serde(rename = "AcctId")]
	pub acct_id: Max35Text,
	#[serde(rename = "RltdAcctDtls", skip_serializing_if = "Option::is_none")]
	pub rltd_acct_dtls: Option<GenericIdentification1>,
}


// AccountManagementStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AccountManagementStatus1Code {
	#[default]
	#[serde(rename = "RECE")]
	CodeRECE,
	#[serde(rename = "ACCP")]
	CodeACCP,
	#[serde(rename = "EXEC")]
	CodeEXEC,
	#[serde(rename = "STNP")]
	CodeSTNP,

}


// AccountManagementStatusAndReason5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountManagementStatusAndReason5 {
	#[serde(rename = "Sts")]
	pub sts: Status25Choice,
	#[serde(rename = "StsRsn", skip_serializing_if = "Option::is_none")]
	pub sts_rsn: Option<Vec<AcceptedStatusReason1Choice>>,
	#[serde(rename = "AcctApplId", skip_serializing_if = "Option::is_none")]
	pub acct_appl_id: Option<Max35Text>,
	#[serde(rename = "ExstgAcctId", skip_serializing_if = "Option::is_none")]
	pub exstg_acct_id: Option<Vec<Account23>>,
	#[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
	pub acct_id: Option<Max35Text>,
	#[serde(rename = "AcctSts", skip_serializing_if = "Option::is_none")]
	pub acct_sts: Option<AccountStatus2>,
	#[serde(rename = "BlckdSts", skip_serializing_if = "Option::is_none")]
	pub blckd_sts: Option<BlockedStatusReason2Choice>,
	#[serde(rename = "FATCARptgDt", skip_serializing_if = "Option::is_none")]
	pub fatca_rptg_dt: Option<String>,
	#[serde(rename = "CRSRptgDt", skip_serializing_if = "Option::is_none")]
	pub crs_rptg_dt: Option<String>,
}


// AccountManagementStatusReportV07 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountManagementStatusReportV07 {
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "RltdRef")]
	pub rltd_ref: Vec<AdditionalReference13>,
	#[serde(rename = "StsRpt")]
	pub sts_rpt: AccountManagementStatusAndReason5,
	#[serde(rename = "MktPrctcVrsn", skip_serializing_if = "Option::is_none")]
	pub mkt_prctc_vrsn: Option<MarketPracticeVersion1>,
	#[serde(rename = "Xtnsn", skip_serializing_if = "Option::is_none")]
	pub xtnsn: Option<Vec<Extension1>>,
}


// AccountStatus2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountStatus2 {
	#[serde(rename = "Nbld", skip_serializing_if = "Option::is_none")]
	pub nbld: Option<EnabledStatusReason1Choice>,
	#[serde(rename = "Dsbld", skip_serializing_if = "Option::is_none")]
	pub dsbld: Option<DisabledStatusReason1Choice>,
	#[serde(rename = "Pdg", skip_serializing_if = "Option::is_none")]
	pub pdg: Option<PendingStatusReason1Choice>,
	#[serde(rename = "PdgOpng", skip_serializing_if = "Option::is_none")]
	pub pdg_opng: Option<PendingOpeningStatusReason1Choice>,
	#[serde(rename = "Profrm", skip_serializing_if = "Option::is_none")]
	pub profrm: Option<ProformaStatusReason1Choice>,
	#[serde(rename = "Clsd", skip_serializing_if = "Option::is_none")]
	pub clsd: Option<ClosedStatusReason1Choice>,
	#[serde(rename = "ClsrPdg", skip_serializing_if = "Option::is_none")]
	pub clsr_pdg: Option<ClosurePendingStatusReason1Choice>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<OtherAccountStatus1>>,
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
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// BlockedReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockedReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<BlockedReason2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// BlockedReason2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BlockedReason2Code {
	#[default]
	#[serde(rename = "BKRP")]
	CodeBKRP,
	#[serde(rename = "CMMT")]
	CodeCMMT,
	#[serde(rename = "CNFS")]
	CodeCNFS,
	#[serde(rename = "MORT")]
	CodeMORT,
	#[serde(rename = "PCOM")]
	CodePCOM,
	#[serde(rename = "PLDG")]
	CodePLDG,
	#[serde(rename = "TRPE")]
	CodeTRPE,
	#[serde(rename = "SANC")]
	CodeSANC,
	#[serde(rename = "TRAN")]
	CodeTRAN,

}


// BlockedStatusReason2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockedStatusReason2 {
	#[serde(rename = "TxTp")]
	pub tx_tp: TransactionType5Choice,
	#[serde(rename = "Blckd")]
	pub blckd: bool,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<BlockedReason2Choice>>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Max350Text,
}


// BlockedStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockedStatusReason2Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<BlockedStatusReason2>>,
}


// ClosedStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosedStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: ClosedStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}


// ClosedStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosedStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<ClosedStatusReason1>>,
}


// ClosedStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ClosedStatusReason1Code {
	#[default]
	#[serde(rename = "ASIN")]
	CodeASIN,
	#[serde(rename = "CLIN")]
	CodeCLIN,

}


// ClosedStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosedStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ClosedStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// ClosurePendingStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosurePendingStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: ClosurePendingStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}


// ClosurePendingStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosurePendingStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<ClosurePendingStatusReason1>>,
}


// ClosurePendingStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ClosurePendingStatusReason1Code {
	#[default]
	#[serde(rename = "CLOS")]
	CodeCLOS,
	#[serde(rename = "PEND")]
	CodePEND,

}


// ClosurePendingStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosurePendingStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ClosurePendingStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// DisabledReason2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DisabledReason2Code {
	#[default]
	#[serde(rename = "CLOS")]
	CodeCLOS,
	#[serde(rename = "BKRP")]
	CodeBKRP,
	#[serde(rename = "CMMT")]
	CodeCMMT,
	#[serde(rename = "CNFS")]
	CodeCNFS,
	#[serde(rename = "MORT")]
	CodeMORT,
	#[serde(rename = "PCOM")]
	CodePCOM,
	#[serde(rename = "PLDG")]
	CodePLDG,
	#[serde(rename = "TRPE")]
	CodeTRPE,
	#[serde(rename = "SANC")]
	CodeSANC,
	#[serde(rename = "TRAN")]
	CodeTRAN,
	#[serde(rename = "REJT")]
	CodeREJT,

}


// DisabledStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DisabledStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: DisabledStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}


// DisabledStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DisabledStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<DisabledStatusReason1>>,
}


// DisabledStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DisabledStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<DisabledReason2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// EnabledStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnabledStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: EnabledStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}


// EnabledStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnabledStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<EnabledStatusReason1>>,
}


// EnabledStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EnabledStatusReason1Code {
	#[default]
	#[serde(rename = "MODI")]
	CodeMODI,

}


// EnabledStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnabledStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<EnabledStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}


// Extension1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Extension1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Max350Text,
	#[serde(rename = "Txt")]
	pub txt: Max350Text,
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


// ISOYearMonth ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISOYearMonth {
	#[serde(rename = "$value")]
	pub iso_year_month: String,
}


// InvestmentFundTransactionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestmentFundTransactionType1Code {
	#[default]
	#[serde(rename = "ALLL")]
	CodeALLL,
	#[serde(rename = "SELL")]
	CodeSELL,
	#[serde(rename = "BUYI")]
	CodeBUYI,
	#[serde(rename = "SWIO")]
	CodeSWIO,
	#[serde(rename = "TRIN")]
	CodeTRIN,
	#[serde(rename = "TOUT")]
	CodeTOUT,
	#[serde(rename = "SUBS")]
	CodeSUBS,
	#[serde(rename = "REDM")]
	CodeREDM,
	#[serde(rename = "CDEP")]
	CodeCDEP,
	#[serde(rename = "CWIT")]
	CodeCWIT,
	#[serde(rename = "DIVP")]
	CodeDIVP,
	#[serde(rename = "CAEV")]
	CodeCAEV,
	#[serde(rename = "CROI")]
	CodeCROI,
	#[serde(rename = "CROO")]
	CodeCROO,
	#[serde(rename = "DIVI")]
	CodeDIVI,
	#[serde(rename = "INSP")]
	CodeINSP,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "REAA")]
	CodeREAA,
	#[serde(rename = "RWPL")]
	CodeRWPL,
	#[serde(rename = "RDIV")]
	CodeRDIV,
	#[serde(rename = "SSPL")]
	CodeSSPL,
	#[serde(rename = "SUAA")]
	CodeSUAA,

}


// MarketPracticeVersion1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketPracticeVersion1 {
	#[serde(rename = "Nm")]
	pub nm: Max35Text,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
	pub nb: Option<Max35Text>,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
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


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
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


// NoReasonCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NoReasonCode {
	#[default]
	#[serde(rename = "NORE")]
	CodeNORE,

}


// OtherAccountStatus1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherAccountStatus1 {
	#[serde(rename = "Sts")]
	pub sts: GenericIdentification36,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<GenericIdentification36>,
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


// PendingOpeningStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingOpeningStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: PendingOpeningStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}


// PendingOpeningStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingOpeningStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<PendingOpeningStatusReason1>>,
}


// PendingOpeningStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PendingOpeningStatusReason1Code {
	#[default]
	#[serde(rename = "ATHR")]
	CodeATHR,
	#[serde(rename = "ATHP")]
	CodeATHP,
	#[serde(rename = "FRDM")]
	CodeFRDM,
	#[serde(rename = "KYCM")]
	CodeKYCM,
	#[serde(rename = "NOTO")]
	CodeNOTO,
	#[serde(rename = "REST")]
	CodeREST,
	#[serde(rename = "RIGH")]
	CodeRIGH,

}


// PendingOpeningStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingOpeningStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PendingOpeningStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// PendingStatusReason14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingStatusReason14 {
	#[serde(rename = "Cd")]
	pub cd: PendingStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}


// PendingStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<PendingStatusReason14>>,
}


// PendingStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PendingStatusReason1Code {
	#[default]
	#[serde(rename = "KYCM")]
	CodeKYCM,
	#[serde(rename = "FRDM")]
	CodeFRDM,
	#[serde(rename = "RIGH")]
	CodeRIGH,
	#[serde(rename = "ATHR")]
	CodeATHR,
	#[serde(rename = "ATHP")]
	CodeATHP,
	#[serde(rename = "MODI")]
	CodeMODI,

}


// PendingStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PendingStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
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


// ProformaStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProformaStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: ProformaStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}


// ProformaStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProformaStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<ProformaStatusReason1>>,
}


// ProformaStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ProformaStatusReason1Code {
	#[default]
	#[serde(rename = "MODI")]
	CodeMODI,
	#[serde(rename = "RIGH")]
	CodeRIGH,

}


// ProformaStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProformaStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ProformaStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// RejectedReason16Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RejectedReason16Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<RejectedStatusReason6Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// RejectedStatusReason6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RejectedStatusReason6Code {
	#[default]
	#[serde(rename = "SAFE")]
	CodeSAFE,
	#[serde(rename = "NSLA")]
	CodeNSLA,

}


// RejectionReason31 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RejectionReason31 {
	#[serde(rename = "Rsn")]
	pub rsn: RejectedReason16Choice,
	#[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rsn_inf: Option<Max350Text>,
}


// Status25Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Status25Choice {
	#[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
	pub sts: Option<AccountManagementStatus1Code>,
	#[serde(rename = "Rjctd", skip_serializing_if = "Option::is_none")]
	pub rjctd: Option<Vec<RejectionReason31>>,
}


// TransactionType5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionType5Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestmentFundTransactionType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}
