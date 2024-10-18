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
use regex::Regex;


// Account23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Account23 {
	#[serde(rename = "AcctId")]
	pub acct_id: Max35Text,
	#[serde(rename = "RltdAcctDtls", skip_serializing_if = "Option::is_none")]
	pub rltd_acct_dtls: Option<GenericIdentification1>,
}

impl Account23 {
	pub fn validate(&self) -> bool {
		if !self.acct_id.validate() { return false }
		if let Some(ref rltd_acct_dtls_value) = self.rltd_acct_dtls { if !rltd_acct_dtls_value.validate() { return false; } }
		return true
	}
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

impl AccountManagementMessageReference5 {
	pub fn validate(&self) -> bool {
		if let Some(ref lkd_ref_value) = self.lkd_ref { if !lkd_ref_value.validate() { return false; } }
		if !self.sts_req_tp.validate() { return false }
		if let Some(ref acct_appl_id_value) = self.acct_appl_id { if !acct_appl_id_value.validate() { return false; } }
		if let Some(ref exstg_acct_id_value) = self.exstg_acct_id { if !exstg_acct_id_value.validate() { return false; } }
		if let Some(ref invstmt_acct_value) = self.invstmt_acct { if !invstmt_acct_value.validate() { return false; } }
		return true
	}
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

impl AccountManagementType3Code {
	pub fn validate(&self) -> bool {
		return true
	}
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

impl AdditionalReference13 {
	pub fn validate(&self) -> bool {
		if !self.ref_attr.validate() { return false }
		if let Some(ref ref_issr_value) = self.ref_issr { if !ref_issr_value.validate() { return false; } }
		if let Some(ref msg_nm_value) = self.msg_nm { if !msg_nm_value.validate() { return false; } }
		return true
	}
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

impl AddressType2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}

impl AnyBICDec2014Identifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.any_bic_dec2014_identifier) {
			return false
		}
		return true
	}
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}

impl CountryCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.country_code) {
			return false
		}
		return true
	}
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}

impl Exact4AlphaNumericText {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
		if !pattern.is_match(&self.exact4_alpha_numeric_text) {
			return false
		}
		return true
	}
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

impl GenderCode {
	pub fn validate(&self) -> bool {
		return true
	}
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

impl GenericIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
	}
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

impl GenericIdentification47 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.issr.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		return true
	}
}


// GenericIdentification81 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification81 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "IdTp")]
	pub id_tp: OtherIdentification3Choice,
}

impl GenericIdentification81 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.id_tp.validate() { return false }
		return true
	}
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}

impl ISODate {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}

impl ISODateTime {
	pub fn validate(&self) -> bool {
		return true
	}
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

impl IndividualPerson30 {
	pub fn validate(&self) -> bool {
		if let Some(ref gvn_nm_value) = self.gvn_nm { if !gvn_nm_value.validate() { return false; } }
		if let Some(ref mddl_nm_value) = self.mddl_nm { if !mddl_nm_value.validate() { return false; } }
		if !self.nm.validate() { return false }
		if let Some(ref gndr_value) = self.gndr { if !gndr_value.validate() { return false; } }
		return true
	}
}


// IndividualPersonIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndividualPersonIdentification2Choice {
	#[serde(rename = "IdNb", skip_serializing_if = "Option::is_none")]
	pub id_nb: Option<GenericIdentification81>,
	#[serde(rename = "PrsnNm", skip_serializing_if = "Option::is_none")]
	pub prsn_nm: Option<IndividualPerson30>,
}

impl IndividualPersonIdentification2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref id_nb_value) = self.id_nb { if !id_nb_value.validate() { return false; } }
		if let Some(ref prsn_nm_value) = self.prsn_nm { if !prsn_nm_value.validate() { return false; } }
		return true
	}
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

impl InvestmentAccount77 {
	pub fn validate(&self) -> bool {
		if !self.acct_id.validate() { return false }
		if let Some(ref acct_nm_value) = self.acct_nm { if !acct_nm_value.validate() { return false; } }
		if let Some(ref acct_dsgnt_value) = self.acct_dsgnt { if !acct_dsgnt_value.validate() { return false; } }
		if let Some(ref ownr_id_value) = self.ownr_id { if !ownr_id_value.validate() { return false; } }
		if let Some(ref acct_svcr_value) = self.acct_svcr { if !acct_svcr_value.validate() { return false; } }
		return true
	}
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}

impl LEIIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.lei_identifier) {
			return false
		}
		return true
	}
}


// LinkedMessage5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LinkedMessage5Choice {
	#[serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none")]
	pub prvs_ref: Option<AdditionalReference13>,
	#[serde(rename = "OthrRef", skip_serializing_if = "Option::is_none")]
	pub othr_ref: Option<AdditionalReference13>,
}

impl LinkedMessage5Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref prvs_ref_value) = self.prvs_ref { if !prvs_ref_value.validate() { return false; } }
		if let Some(ref othr_ref_value) = self.othr_ref { if !othr_ref_value.validate() { return false; } }
		return true
	}
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
}

impl Max16Text {
	pub fn validate(&self) -> bool {
		if self.max16_text.chars().count() < 1 {
			return false
		}
		if self.max16_text.chars().count() > 16 {
			return false
		}
		return true
	}
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}

impl Max350Text {
	pub fn validate(&self) -> bool {
		if self.max350_text.chars().count() < 1 {
			return false
		}
		if self.max350_text.chars().count() > 350 {
			return false
		}
		return true
	}
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}

impl Max35Text {
	pub fn validate(&self) -> bool {
		if self.max35_text.chars().count() < 1 {
			return false
		}
		if self.max35_text.chars().count() > 35 {
			return false
		}
		return true
	}
}


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "$value")]
	pub max4_alpha_numeric_text: String,
}

impl Max4AlphaNumericText {
	pub fn validate(&self) -> bool {
		if self.max4_alpha_numeric_text.chars().count() < 1 {
			return false
		}
		if self.max4_alpha_numeric_text.chars().count() > 4 {
			return false
		}
		let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
		if !pattern.is_match(&self.max4_alpha_numeric_text) {
			return false
		}
		return true
	}
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}

impl Max70Text {
	pub fn validate(&self) -> bool {
		if self.max70_text.chars().count() < 1 {
			return false
		}
		if self.max70_text.chars().count() > 70 {
			return false
		}
		return true
	}
}


// MessageIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
}

impl MessageIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		return true
	}
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}

impl NameAndAddress5 {
	pub fn validate(&self) -> bool {
		if !self.nm.validate() { return false }
		if let Some(ref adr_value) = self.adr { if !adr_value.validate() { return false; } }
		return true
	}
}


// OtherIdentification3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PartyIdentificationType7Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl OtherIdentification3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// OwnerIdentification3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OwnerIdentification3Choice {
	#[serde(rename = "IndvOwnrId", skip_serializing_if = "Option::is_none")]
	pub indv_ownr_id: Option<IndividualPersonIdentification2Choice>,
	#[serde(rename = "OrgOwnrId", skip_serializing_if = "Option::is_none")]
	pub org_ownr_id: Option<PartyIdentification139>,
}

impl OwnerIdentification3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref indv_ownr_id_value) = self.indv_ownr_id { if !indv_ownr_id_value.validate() { return false; } }
		if let Some(ref org_ownr_id_value) = self.org_ownr_id { if !org_ownr_id_value.validate() { return false; } }
		return true
	}
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

impl PartyIdentification125Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref any_bic_value) = self.any_bic { if !any_bic_value.validate() { return false; } }
		if let Some(ref prtry_id_value) = self.prtry_id { if !prtry_id_value.validate() { return false; } }
		if let Some(ref nm_and_adr_value) = self.nm_and_adr { if !nm_and_adr_value.validate() { return false; } }
		return true
	}
}


// PartyIdentification139 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification139 {
	#[serde(rename = "Pty")]
	pub pty: PartyIdentification125Choice,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
}

impl PartyIdentification139 {
	pub fn validate(&self) -> bool {
		if !self.pty.validate() { return false }
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		return true
	}
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

impl PartyIdentificationType7Code {
	pub fn validate(&self) -> bool {
		return true
	}
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

impl PostalAddress1 {
	pub fn validate(&self) -> bool {
		if let Some(ref adr_tp_value) = self.adr_tp { if !adr_tp_value.validate() { return false; } }
		if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if !item.validate() { return false; } } }
		if let Some(ref strt_nm_value) = self.strt_nm { if !strt_nm_value.validate() { return false; } }
		if let Some(ref bldg_nb_value) = self.bldg_nb { if !bldg_nb_value.validate() { return false; } }
		if let Some(ref pst_cd_value) = self.pst_cd { if !pst_cd_value.validate() { return false; } }
		if let Some(ref twn_nm_value) = self.twn_nm { if !twn_nm_value.validate() { return false; } }
		if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if !ctry_sub_dvsn_value.validate() { return false; } }
		if !self.ctry.validate() { return false }
		return true
	}
}


// RequestForAccountManagementStatusReportV06 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestForAccountManagementStatusReportV06 {
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "ReqDtls")]
	pub req_dtls: AccountManagementMessageReference5,
}

impl RequestForAccountManagementStatusReportV06 {
	pub fn validate(&self) -> bool {
		if !self.msg_id.validate() { return false }
		if !self.req_dtls.validate() { return false }
		return true
	}
}
