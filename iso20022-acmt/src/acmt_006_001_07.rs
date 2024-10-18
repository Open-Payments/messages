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


// AcceptedStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AcceptedStatusReason1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AcceptedStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}

impl AcceptedStatusReason1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
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

impl AcceptedStatusReason1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


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

impl AccountManagementStatus1Code {
	pub fn validate(&self) -> bool {
		return true
	}
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

impl AccountManagementStatusAndReason5 {
	pub fn validate(&self) -> bool {
		if !self.sts.validate() { return false }
		if let Some(ref sts_rsn_vec) = self.sts_rsn { for item in sts_rsn_vec { if !item.validate() { return false; } } }
		if let Some(ref acct_appl_id_value) = self.acct_appl_id { if !acct_appl_id_value.validate() { return false; } }
		if let Some(ref exstg_acct_id_vec) = self.exstg_acct_id { for item in exstg_acct_id_vec { if !item.validate() { return false; } } }
		if let Some(ref acct_id_value) = self.acct_id { if !acct_id_value.validate() { return false; } }
		if let Some(ref acct_sts_value) = self.acct_sts { if !acct_sts_value.validate() { return false; } }
		if let Some(ref blckd_sts_value) = self.blckd_sts { if !blckd_sts_value.validate() { return false; } }
		return true
	}
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

impl AccountManagementStatusReportV07 {
	pub fn validate(&self) -> bool {
		if !self.msg_id.validate() { return false }
		for item in &self.rltd_ref { if !item.validate() { return false; } }
		if !self.sts_rpt.validate() { return false }
		if let Some(ref mkt_prctc_vrsn_value) = self.mkt_prctc_vrsn { if !mkt_prctc_vrsn_value.validate() { return false; } }
		if let Some(ref xtnsn_vec) = self.xtnsn { for item in xtnsn_vec { if !item.validate() { return false; } } }
		return true
	}
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

impl AccountStatus2 {
	pub fn validate(&self) -> bool {
		if let Some(ref nbld_value) = self.nbld { if !nbld_value.validate() { return false; } }
		if let Some(ref dsbld_value) = self.dsbld { if !dsbld_value.validate() { return false; } }
		if let Some(ref pdg_value) = self.pdg { if !pdg_value.validate() { return false; } }
		if let Some(ref pdg_opng_value) = self.pdg_opng { if !pdg_opng_value.validate() { return false; } }
		if let Some(ref profrm_value) = self.profrm { if !profrm_value.validate() { return false; } }
		if let Some(ref clsd_value) = self.clsd { if !clsd_value.validate() { return false; } }
		if let Some(ref clsr_pdg_value) = self.clsr_pdg { if !clsr_pdg_value.validate() { return false; } }
		if let Some(ref othr_vec) = self.othr { for item in othr_vec { if !item.validate() { return false; } } }
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


// BlockedReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockedReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<BlockedReason2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl BlockedReason2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
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

impl BlockedReason2Code {
	pub fn validate(&self) -> bool {
		return true
	}
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

impl BlockedStatusReason2 {
	pub fn validate(&self) -> bool {
		if !self.tx_tp.validate() { return false }
		if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if !item.validate() { return false; } } }
		if !self.addtl_inf.validate() { return false }
		return true
	}
}


// BlockedStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockedStatusReason2Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<BlockedStatusReason2>>,
}

impl BlockedStatusReason2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if !no_spcfd_rsn_value.validate() { return false; } }
		if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if !item.validate() { return false; } } }
		return true
	}
}


// ClosedStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosedStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: ClosedStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}

impl ClosedStatusReason1 {
	pub fn validate(&self) -> bool {
		if !self.cd.validate() { return false }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if !addtl_inf_value.validate() { return false; } }
		return true
	}
}


// ClosedStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosedStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<ClosedStatusReason1>>,
}

impl ClosedStatusReason1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if !no_spcfd_rsn_value.validate() { return false; } }
		if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if !item.validate() { return false; } } }
		return true
	}
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

impl ClosedStatusReason1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ClosedStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosedStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ClosedStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}

impl ClosedStatusReason2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// ClosurePendingStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosurePendingStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: ClosurePendingStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}

impl ClosurePendingStatusReason1 {
	pub fn validate(&self) -> bool {
		if !self.cd.validate() { return false }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if !addtl_inf_value.validate() { return false; } }
		return true
	}
}


// ClosurePendingStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosurePendingStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<ClosurePendingStatusReason1>>,
}

impl ClosurePendingStatusReason1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if !no_spcfd_rsn_value.validate() { return false; } }
		if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if !item.validate() { return false; } } }
		return true
	}
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

impl ClosurePendingStatusReason1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ClosurePendingStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosurePendingStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ClosurePendingStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}

impl ClosurePendingStatusReason2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
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

impl DisabledReason2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DisabledStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DisabledStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: DisabledStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}

impl DisabledStatusReason1 {
	pub fn validate(&self) -> bool {
		if !self.cd.validate() { return false }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if !addtl_inf_value.validate() { return false; } }
		return true
	}
}


// DisabledStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DisabledStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<DisabledStatusReason1>>,
}

impl DisabledStatusReason1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if !no_spcfd_rsn_value.validate() { return false; } }
		if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if !item.validate() { return false; } } }
		return true
	}
}


// DisabledStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DisabledStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<DisabledReason2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}

impl DisabledStatusReason2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// EnabledStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnabledStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: EnabledStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}

impl EnabledStatusReason1 {
	pub fn validate(&self) -> bool {
		if !self.cd.validate() { return false }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if !addtl_inf_value.validate() { return false; } }
		return true
	}
}


// EnabledStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnabledStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<EnabledStatusReason1>>,
}

impl EnabledStatusReason1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if !no_spcfd_rsn_value.validate() { return false; } }
		if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if !item.validate() { return false; } } }
		return true
	}
}


// EnabledStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EnabledStatusReason1Code {
	#[default]
	#[serde(rename = "MODI")]
	CodeMODI,
}

impl EnabledStatusReason1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// EnabledStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnabledStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<EnabledStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}

impl EnabledStatusReason2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
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


// Extension1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Extension1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Max350Text,
	#[serde(rename = "Txt")]
	pub txt: Max350Text,
}

impl Extension1 {
	pub fn validate(&self) -> bool {
		if !self.plc_and_nm.validate() { return false }
		if !self.txt.validate() { return false }
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


// ISOYearMonth ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISOYearMonth {
	#[serde(rename = "$value")]
	pub iso_year_month: String,
}

impl ISOYearMonth {
	pub fn validate(&self) -> bool {
		return true
	}
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

impl InvestmentFundTransactionType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
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

impl MarketPracticeVersion1 {
	pub fn validate(&self) -> bool {
		if !self.nm.validate() { return false }
		if let Some(ref nb_value) = self.nb { if !nb_value.validate() { return false; } }
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


// NoReasonCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NoReasonCode {
	#[default]
	#[serde(rename = "NORE")]
	CodeNORE,
}

impl NoReasonCode {
	pub fn validate(&self) -> bool {
		return true
	}
}


// OtherAccountStatus1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherAccountStatus1 {
	#[serde(rename = "Sts")]
	pub sts: GenericIdentification36,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<GenericIdentification36>,
}

impl OtherAccountStatus1 {
	pub fn validate(&self) -> bool {
		if !self.sts.validate() { return false }
		if let Some(ref rsn_value) = self.rsn { if !rsn_value.validate() { return false; } }
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


// PendingOpeningStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingOpeningStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: PendingOpeningStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}

impl PendingOpeningStatusReason1 {
	pub fn validate(&self) -> bool {
		if !self.cd.validate() { return false }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if !addtl_inf_value.validate() { return false; } }
		return true
	}
}


// PendingOpeningStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingOpeningStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<PendingOpeningStatusReason1>>,
}

impl PendingOpeningStatusReason1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if !no_spcfd_rsn_value.validate() { return false; } }
		if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if !item.validate() { return false; } } }
		return true
	}
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

impl PendingOpeningStatusReason1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PendingOpeningStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingOpeningStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PendingOpeningStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}

impl PendingOpeningStatusReason2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// PendingStatusReason14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingStatusReason14 {
	#[serde(rename = "Cd")]
	pub cd: PendingStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}

impl PendingStatusReason14 {
	pub fn validate(&self) -> bool {
		if !self.cd.validate() { return false }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if !addtl_inf_value.validate() { return false; } }
		return true
	}
}


// PendingStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<PendingStatusReason14>>,
}

impl PendingStatusReason1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if !no_spcfd_rsn_value.validate() { return false; } }
		if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if !item.validate() { return false; } } }
		return true
	}
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

impl PendingStatusReason1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PendingStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PendingStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}

impl PendingStatusReason2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
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


// ProformaStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProformaStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: ProformaStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}

impl ProformaStatusReason1 {
	pub fn validate(&self) -> bool {
		if !self.cd.validate() { return false }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if !addtl_inf_value.validate() { return false; } }
		return true
	}
}


// ProformaStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProformaStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<ProformaStatusReason1>>,
}

impl ProformaStatusReason1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if !no_spcfd_rsn_value.validate() { return false; } }
		if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if !item.validate() { return false; } } }
		return true
	}
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

impl ProformaStatusReason1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ProformaStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProformaStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ProformaStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}

impl ProformaStatusReason2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// RejectedReason16Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RejectedReason16Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<RejectedStatusReason6Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}

impl RejectedReason16Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
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

impl RejectedStatusReason6Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// RejectionReason31 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RejectionReason31 {
	#[serde(rename = "Rsn")]
	pub rsn: RejectedReason16Choice,
	#[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rsn_inf: Option<Max350Text>,
}

impl RejectionReason31 {
	pub fn validate(&self) -> bool {
		if !self.rsn.validate() { return false }
		if let Some(ref addtl_rsn_inf_value) = self.addtl_rsn_inf { if !addtl_rsn_inf_value.validate() { return false; } }
		return true
	}
}


// Status25Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Status25Choice {
	#[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
	pub sts: Option<AccountManagementStatus1Code>,
	#[serde(rename = "Rjctd", skip_serializing_if = "Option::is_none")]
	pub rjctd: Option<Vec<RejectionReason31>>,
}

impl Status25Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref sts_value) = self.sts { if !sts_value.validate() { return false; } }
		if let Some(ref rjctd_vec) = self.rjctd { for item in rjctd_vec { if !item.validate() { return false; } } }
		return true
	}
}


// TransactionType5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionType5Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestmentFundTransactionType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl TransactionType5Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
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
