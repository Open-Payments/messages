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


// ErrorHandling3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalSystemErrorHandling1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ErrorHandling3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// ErrorHandling5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling5 {
	#[serde(rename = "Err")]
	pub err: ErrorHandling3Choice,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
}

impl ErrorHandling5 {
	pub fn validate(&self) -> bool {
		if !self.err.validate() { return false }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
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


// ExternalSystemErrorHandling1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalSystemErrorHandling1Code {
	#[serde(rename = "$value")]
	pub external_system_error_handling1_code: String,
}

impl ExternalSystemErrorHandling1Code {
	pub fn validate(&self) -> bool {
		if self.external_system_error_handling1_code.chars().count() < 1 {
			return false
		}
		if self.external_system_error_handling1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalSystemPartyType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalSystemPartyType1Code {
	#[serde(rename = "$value")]
	pub external_system_party_type1_code: String,
}

impl ExternalSystemPartyType1Code {
	pub fn validate(&self) -> bool {
		if self.external_system_party_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_system_party_type1_code.chars().count() > 4 {
			return false
		}
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

impl GenericIdentification30 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.issr.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		return true
	}
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

impl GenericIdentification36 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.issr.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
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


// MarketSpecificAttribute1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketSpecificAttribute1 {
	#[serde(rename = "Nm")]
	pub nm: Max35Text,
	#[serde(rename = "Val")]
	pub val: Max350Text,
}

impl MarketSpecificAttribute1 {
	pub fn validate(&self) -> bool {
		if !self.nm.validate() { return false }
		if !self.val.validate() { return false }
		return true
	}
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}

impl Max140Text {
	pub fn validate(&self) -> bool {
		if self.max140_text.chars().count() < 1 {
			return false
		}
		if self.max140_text.chars().count() > 140 {
			return false
		}
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


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max5NumericText {
	#[serde(rename = "$value")]
	pub max5_numeric_text: String,
}

impl Max5NumericText {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[0-9]{1,5}").unwrap();
		if !pattern.is_match(&self.max5_numeric_text) {
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


// MessageHeader3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader3 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "ReqTp", skip_serializing_if = "Option::is_none")]
	pub req_tp: Option<RequestType2Choice>,
	#[serde(rename = "OrgnlBizQry", skip_serializing_if = "Option::is_none")]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
	#[serde(rename = "QryNm", skip_serializing_if = "Option::is_none")]
	pub qry_nm: Option<Max35Text>,
}

impl MessageHeader3 {
	pub fn validate(&self) -> bool {
		if !self.msg_id.validate() { return false }
		if let Some(ref req_tp_value) = self.req_tp { if !req_tp_value.validate() { return false; } }
		if let Some(ref orgnl_biz_qry_value) = self.orgnl_biz_qry { if !orgnl_biz_qry_value.validate() { return false; } }
		if let Some(ref qry_nm_value) = self.qry_nm { if !qry_nm_value.validate() { return false; } }
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


// OriginalBusinessQuery1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalBusinessQuery1 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none")]
	pub msg_nm_id: Option<Max35Text>,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
}

impl OriginalBusinessQuery1 {
	pub fn validate(&self) -> bool {
		if !self.msg_id.validate() { return false }
		if let Some(ref msg_nm_id_value) = self.msg_nm_id { if !msg_nm_id_value.validate() { return false; } }
		return true
	}
}


// Pagination1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: Max5NumericText,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}

impl Pagination1 {
	pub fn validate(&self) -> bool {
		if !self.pg_nb.validate() { return false }
		return true
	}
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

impl PartyIdentification120Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref any_bic_value) = self.any_bic { if !any_bic_value.validate() { return false; } }
		if let Some(ref prtry_id_value) = self.prtry_id { if !prtry_id_value.validate() { return false; } }
		if let Some(ref nm_and_adr_value) = self.nm_and_adr { if !nm_and_adr_value.validate() { return false; } }
		return true
	}
}


// PartyIdentification136 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification136 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification120Choice,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
}

impl PartyIdentification136 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
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


// RequestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RequestType1Code {
	#[default]
	#[serde(rename = "RT01")]
	CodeRT01,
	#[serde(rename = "RT02")]
	CodeRT02,
	#[serde(rename = "RT03")]
	CodeRT03,
	#[serde(rename = "RT04")]
	CodeRT04,
	#[serde(rename = "RT05")]
	CodeRT05,
}

impl RequestType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// RequestType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestType2Choice {
	#[serde(rename = "PmtCtrl", skip_serializing_if = "Option::is_none")]
	pub pmt_ctrl: Option<RequestType1Code>,
	#[serde(rename = "Enqry", skip_serializing_if = "Option::is_none")]
	pub enqry: Option<RequestType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification1>,
}

impl RequestType2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref pmt_ctrl_value) = self.pmt_ctrl { if !pmt_ctrl_value.validate() { return false; } }
		if let Some(ref enqry_value) = self.enqry { if !enqry_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// RequestType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RequestType2Code {
	#[default]
	#[serde(rename = "RT11")]
	CodeRT11,
	#[serde(rename = "RT12")]
	CodeRT12,
	#[serde(rename = "RT13")]
	CodeRT13,
	#[serde(rename = "RT14")]
	CodeRT14,
	#[serde(rename = "RT15")]
	CodeRT15,
}

impl RequestType2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SecuritiesAccount19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesAccount19 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<GenericIdentification30>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max70Text>,
}

impl SecuritiesAccount19 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		return true
	}
}


// SecuritiesAccountOrBusinessError3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesAccountOrBusinessError3Choice {
	#[serde(rename = "SctiesAcct", skip_serializing_if = "Option::is_none")]
	pub scties_acct: Option<SystemSecuritiesAccount6>,
	#[serde(rename = "BizErr", skip_serializing_if = "Option::is_none")]
	pub biz_err: Option<Vec<ErrorHandling5>>,
}

impl SecuritiesAccountOrBusinessError3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref scties_acct_value) = self.scties_acct { if !scties_acct_value.validate() { return false; } }
		if let Some(ref biz_err_vec) = self.biz_err { for item in biz_err_vec { if !item.validate() { return false; } } }
		return true
	}
}


// SecuritiesAccountOrOperationalError3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesAccountOrOperationalError3Choice {
	#[serde(rename = "SctiesAcctRpt", skip_serializing_if = "Option::is_none")]
	pub scties_acct_rpt: Option<Vec<SecuritiesAccountReport3>>,
	#[serde(rename = "OprlErr", skip_serializing_if = "Option::is_none")]
	pub oprl_err: Option<Vec<ErrorHandling5>>,
}

impl SecuritiesAccountOrOperationalError3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref scties_acct_rpt_vec) = self.scties_acct_rpt { for item in scties_acct_rpt_vec { if !item.validate() { return false; } } }
		if let Some(ref oprl_err_vec) = self.oprl_err { for item in oprl_err_vec { if !item.validate() { return false; } } }
		return true
	}
}


// SecuritiesAccountReport3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesAccountReport3 {
	#[serde(rename = "SctiesAcctId")]
	pub scties_acct_id: SecuritiesAccount19,
	#[serde(rename = "SctiesAcctOrErr")]
	pub scties_acct_or_err: SecuritiesAccountOrBusinessError3Choice,
}

impl SecuritiesAccountReport3 {
	pub fn validate(&self) -> bool {
		if !self.scties_acct_id.validate() { return false }
		if !self.scties_acct_or_err.validate() { return false }
		return true
	}
}


// SecuritiesAccountReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesAccountReportV01 {
	#[serde(rename = "MsgHdr", skip_serializing_if = "Option::is_none")]
	pub msg_hdr: Option<MessageHeader3>,
	#[serde(rename = "Pgntn")]
	pub pgntn: Pagination1,
	#[serde(rename = "RptOrErr")]
	pub rpt_or_err: SecuritiesAccountOrOperationalError3Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SecuritiesAccountReportV01 {
	pub fn validate(&self) -> bool {
		if let Some(ref msg_hdr_value) = self.msg_hdr { if !msg_hdr_value.validate() { return false; } }
		if !self.pgntn.validate() { return false }
		if !self.rpt_or_err.validate() { return false }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}

impl SupplementaryData1 {
	pub fn validate(&self) -> bool {
		if let Some(ref plc_and_nm_value) = self.plc_and_nm { if !plc_and_nm_value.validate() { return false; } }
		if !self.envlp.validate() { return false }
		return true
	}
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SystemPartyIdentification8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemPartyIdentification8 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification136,
	#[serde(rename = "RspnsblPtyId", skip_serializing_if = "Option::is_none")]
	pub rspnsbl_pty_id: Option<PartyIdentification136>,
}

impl SystemPartyIdentification8 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref rspnsbl_pty_id_value) = self.rspnsbl_pty_id { if !rspnsbl_pty_id_value.validate() { return false; } }
		return true
	}
}


// SystemPartyType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemPartyType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalSystemPartyType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl SystemPartyType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// SystemRestriction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemRestriction1 {
	#[serde(rename = "VldFr")]
	pub vld_fr: String,
	#[serde(rename = "VldTo", skip_serializing_if = "Option::is_none")]
	pub vld_to: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
}

impl SystemRestriction1 {
	pub fn validate(&self) -> bool {
		if !self.tp.validate() { return false }
		return true
	}
}


// SystemSecuritiesAccount6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemSecuritiesAccount6 {
	#[serde(rename = "OpngDt", skip_serializing_if = "Option::is_none")]
	pub opng_dt: Option<String>,
	#[serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none")]
	pub clsg_dt: Option<String>,
	#[serde(rename = "HldInd", skip_serializing_if = "Option::is_none")]
	pub hld_ind: Option<bool>,
	#[serde(rename = "NegPos", skip_serializing_if = "Option::is_none")]
	pub neg_pos: Option<bool>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<SystemSecuritiesAccountType1Choice>,
	#[serde(rename = "AcctOwnr")]
	pub acct_ownr: SystemPartyIdentification8,
	#[serde(rename = "PtyTp", skip_serializing_if = "Option::is_none")]
	pub pty_tp: Option<SystemPartyType1Choice>,
	#[serde(rename = "MktSpcfcAttr", skip_serializing_if = "Option::is_none")]
	pub mkt_spcfc_attr: Option<Vec<MarketSpecificAttribute1>>,
	#[serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none")]
	pub rstrctn: Option<Vec<SystemRestriction1>>,
	#[serde(rename = "EndInvstrFlg", skip_serializing_if = "Option::is_none")]
	pub end_invstr_flg: Option<Exact4AlphaNumericText>,
	#[serde(rename = "PricgSchme", skip_serializing_if = "Option::is_none")]
	pub pricg_schme: Option<Exact4AlphaNumericText>,
}

impl SystemSecuritiesAccount6 {
	pub fn validate(&self) -> bool {
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if !self.acct_ownr.validate() { return false }
		if let Some(ref pty_tp_value) = self.pty_tp { if !pty_tp_value.validate() { return false; } }
		if let Some(ref mkt_spcfc_attr_vec) = self.mkt_spcfc_attr { for item in mkt_spcfc_attr_vec { if !item.validate() { return false; } } }
		if let Some(ref rstrctn_vec) = self.rstrctn { for item in rstrctn_vec { if !item.validate() { return false; } } }
		if let Some(ref end_invstr_flg_value) = self.end_invstr_flg { if !end_invstr_flg_value.validate() { return false; } }
		if let Some(ref pricg_schme_value) = self.pricg_schme { if !pricg_schme_value.validate() { return false; } }
		return true
	}
}


// SystemSecuritiesAccountType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemSecuritiesAccountType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<SystemSecuritiesAccountType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl SystemSecuritiesAccountType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// SystemSecuritiesAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SystemSecuritiesAccountType1Code {
	#[default]
	#[serde(rename = "CSDP")]
	CodeCSDP,
	#[serde(rename = "CSDM")]
	CodeCSDM,
	#[serde(rename = "ICSA")]
	CodeICSA,
	#[serde(rename = "TOFF")]
	CodeTOFF,
	#[serde(rename = "CSDO")]
	CodeCSDO,
	#[serde(rename = "ISSA")]
	CodeISSA,
}

impl SystemSecuritiesAccountType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}

impl TrueFalseIndicator {
	pub fn validate(&self) -> bool {
		return true
	}
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}

impl YesNoIndicator {
	pub fn validate(&self) -> bool {
		return true
	}
}
