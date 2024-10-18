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


// Account32 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Account32 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "AcctSvcr")]
	pub acct_svcr: PartyIdentification125Choice,
}

impl Account32 {
	pub fn validate(&self) -> bool {
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if !self.acct_svcr.validate() { return false }
		return true
	}
}


// AccountDesignation1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountDesignation1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Rank1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl AccountDesignation1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// AccountDetailsConfirmationV08 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountDetailsConfirmationV08 {
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "OrdrRef", skip_serializing_if = "Option::is_none")]
	pub ordr_ref: Option<InvestmentFundOrder4>,
	#[serde(rename = "RltdRef", skip_serializing_if = "Option::is_none")]
	pub rltd_ref: Option<AdditionalReference13>,
	#[serde(rename = "ConfDtls")]
	pub conf_dtls: AccountManagementConfirmation5,
	#[serde(rename = "InvstmtAcct", skip_serializing_if = "Option::is_none")]
	pub invstmt_acct: Option<InvestmentAccount74>,
	#[serde(rename = "AcctPties", skip_serializing_if = "Option::is_none")]
	pub acct_pties: Option<AccountParties17>,
	#[serde(rename = "Intrmies", skip_serializing_if = "Option::is_none")]
	pub intrmies: Option<Vec<Intermediary46>>,
	#[serde(rename = "Plcmnt", skip_serializing_if = "Option::is_none")]
	pub plcmnt: Option<ReferredAgent3>,
	#[serde(rename = "NewIsseAllcn", skip_serializing_if = "Option::is_none")]
	pub new_isse_allcn: Option<NewIssueAllocation2>,
	#[serde(rename = "SvgsInvstmtPlan", skip_serializing_if = "Option::is_none")]
	pub svgs_invstmt_plan: Option<Vec<InvestmentPlan17>>,
	#[serde(rename = "WdrwlInvstmtPlan", skip_serializing_if = "Option::is_none")]
	pub wdrwl_invstmt_plan: Option<Vec<InvestmentPlan17>>,
	#[serde(rename = "CshSttlm", skip_serializing_if = "Option::is_none")]
	pub csh_sttlm: Option<Vec<CashSettlement3>>,
	#[serde(rename = "SvcLvlAgrmt", skip_serializing_if = "Option::is_none")]
	pub svc_lvl_agrmt: Option<Vec<DocumentToSend4>>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditiononalInformation13>>,
	#[serde(rename = "MktPrctcVrsn", skip_serializing_if = "Option::is_none")]
	pub mkt_prctc_vrsn: Option<MarketPracticeVersion1>,
	#[serde(rename = "Xtnsn", skip_serializing_if = "Option::is_none")]
	pub xtnsn: Option<Vec<Extension1>>,
}

impl AccountDetailsConfirmationV08 {
	pub fn validate(&self) -> bool {
		if !self.msg_id.validate() { return false }
		if let Some(ref ordr_ref_value) = self.ordr_ref { if !ordr_ref_value.validate() { return false; } }
		if let Some(ref rltd_ref_value) = self.rltd_ref { if !rltd_ref_value.validate() { return false; } }
		if !self.conf_dtls.validate() { return false }
		if let Some(ref invstmt_acct_value) = self.invstmt_acct { if !invstmt_acct_value.validate() { return false; } }
		if let Some(ref acct_pties_value) = self.acct_pties { if !acct_pties_value.validate() { return false; } }
		if let Some(ref intrmies_vec) = self.intrmies { for item in intrmies_vec { if !item.validate() { return false; } } }
		if let Some(ref plcmnt_value) = self.plcmnt { if !plcmnt_value.validate() { return false; } }
		if let Some(ref new_isse_allcn_value) = self.new_isse_allcn { if !new_isse_allcn_value.validate() { return false; } }
		if let Some(ref svgs_invstmt_plan_vec) = self.svgs_invstmt_plan { for item in svgs_invstmt_plan_vec { if !item.validate() { return false; } } }
		if let Some(ref wdrwl_invstmt_plan_vec) = self.wdrwl_invstmt_plan { for item in wdrwl_invstmt_plan_vec { if !item.validate() { return false; } } }
		if let Some(ref csh_sttlm_vec) = self.csh_sttlm { for item in csh_sttlm_vec { if !item.validate() { return false; } } }
		if let Some(ref svc_lvl_agrmt_vec) = self.svc_lvl_agrmt { for item in svc_lvl_agrmt_vec { if !item.validate() { return false; } } }
		if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if !item.validate() { return false; } } }
		if let Some(ref mkt_prctc_vrsn_value) = self.mkt_prctc_vrsn { if !mkt_prctc_vrsn_value.validate() { return false; } }
		if let Some(ref xtnsn_vec) = self.xtnsn { for item in xtnsn_vec { if !item.validate() { return false; } } }
		return true
	}
}


// AccountIdentification4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentification4Choice {
	#[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
	pub iban: Option<IBAN2007Identifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericAccountIdentification1>,
}

impl AccountIdentification4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref iban_value) = self.iban { if !iban_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// AccountIdentificationAndName5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentificationAndName5 {
	#[serde(rename = "Id")]
	pub id: AccountIdentification4Choice,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max35Text>,
}

impl AccountIdentificationAndName5 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		return true
	}
}


// AccountManagementConfirmation5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountManagementConfirmation5 {
	#[serde(rename = "ConfTp")]
	pub conf_tp: ConfirmationType1Choice,
	#[serde(rename = "AcctApplId", skip_serializing_if = "Option::is_none")]
	pub acct_appl_id: Option<Max35Text>,
	#[serde(rename = "ClntRef", skip_serializing_if = "Option::is_none")]
	pub clnt_ref: Option<Max35Text>,
	#[serde(rename = "CtrPtyRef", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_ref: Option<AdditionalReference13>,
	#[serde(rename = "ExstgAcctId", skip_serializing_if = "Option::is_none")]
	pub exstg_acct_id: Option<Vec<Account23>>,
}

impl AccountManagementConfirmation5 {
	pub fn validate(&self) -> bool {
		if !self.conf_tp.validate() { return false }
		if let Some(ref acct_appl_id_value) = self.acct_appl_id { if !acct_appl_id_value.validate() { return false; } }
		if let Some(ref clnt_ref_value) = self.clnt_ref { if !clnt_ref_value.validate() { return false; } }
		if let Some(ref ctr_pty_ref_value) = self.ctr_pty_ref { if !ctr_pty_ref_value.validate() { return false; } }
		if let Some(ref exstg_acct_id_vec) = self.exstg_acct_id { for item in exstg_acct_id_vec { if !item.validate() { return false; } } }
		return true
	}
}


// AccountManagementType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AccountManagementType2Code {
	#[default]
	#[serde(rename = "ACCO")]
	CodeACCO,
	#[serde(rename = "ACCM")]
	CodeACCM,
	#[serde(rename = "GACC")]
	CodeGACC,
}

impl AccountManagementType2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AccountOwnershipType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AccountOwnershipType4Code {
	#[default]
	#[serde(rename = "UNCO")]
	CodeUNCO,
	#[serde(rename = "LIPA")]
	CodeLIPA,
	#[serde(rename = "ENTR")]
	CodeENTR,
	#[serde(rename = "CORP")]
	CodeCORP,
	#[serde(rename = "CUST")]
	CodeCUST,
	#[serde(rename = "EURE")]
	CodeEURE,
	#[serde(rename = "PART")]
	CodePART,
	#[serde(rename = "TRUS")]
	CodeTRUS,
	#[serde(rename = "GOVO")]
	CodeGOVO,
	#[serde(rename = "JOIT")]
	CodeJOIT,
	#[serde(rename = "COMO")]
	CodeCOMO,
	#[serde(rename = "JOIN")]
	CodeJOIN,
	#[serde(rename = "LLCO")]
	CodeLLCO,
	#[serde(rename = "NOMI")]
	CodeNOMI,
	#[serde(rename = "NFPO")]
	CodeNFPO,
	#[serde(rename = "ONIS")]
	CodeONIS,
	#[serde(rename = "RGIC")]
	CodeRGIC,
	#[serde(rename = "SIGL")]
	CodeSIGL,
}

impl AccountOwnershipType4Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AccountParties12Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountParties12Choice {
	#[serde(rename = "PmryOwnr", skip_serializing_if = "Option::is_none")]
	pub pmry_ownr: Option<InvestmentAccountOwnershipInformation16>,
	#[serde(rename = "Trstee", skip_serializing_if = "Option::is_none")]
	pub trstee: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "Nmnee", skip_serializing_if = "Option::is_none")]
	pub nmnee: Option<InvestmentAccountOwnershipInformation16>,
	#[serde(rename = "JntOwnr", skip_serializing_if = "Option::is_none")]
	pub jnt_ownr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
}

impl AccountParties12Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref pmry_ownr_value) = self.pmry_ownr { if !pmry_ownr_value.validate() { return false; } }
		if let Some(ref trstee_vec) = self.trstee { for item in trstee_vec { if !item.validate() { return false; } } }
		if let Some(ref nmnee_value) = self.nmnee { if !nmnee_value.validate() { return false; } }
		if let Some(ref jnt_ownr_vec) = self.jnt_ownr { for item in jnt_ownr_vec { if !item.validate() { return false; } } }
		return true
	}
}


// AccountParties17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountParties17 {
	#[serde(rename = "PrncplAcctPty")]
	pub prncpl_acct_pty: AccountParties12Choice,
	#[serde(rename = "ScndryOwnr", skip_serializing_if = "Option::is_none")]
	pub scndry_ownr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none")]
	pub bnfcry: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "PwrOfAttny", skip_serializing_if = "Option::is_none")]
	pub pwr_of_attny: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "LglGuardn", skip_serializing_if = "Option::is_none")]
	pub lgl_guardn: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "CtdnForMnr", skip_serializing_if = "Option::is_none")]
	pub ctdn_for_mnr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "SucssrOnDth", skip_serializing_if = "Option::is_none")]
	pub sucssr_on_dth: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "Admstr", skip_serializing_if = "Option::is_none")]
	pub admstr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "OthrPty", skip_serializing_if = "Option::is_none")]
	pub othr_pty: Option<Vec<ExtendedParty14>>,
	#[serde(rename = "Grntr", skip_serializing_if = "Option::is_none")]
	pub grntr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "Sttlr", skip_serializing_if = "Option::is_none")]
	pub sttlr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "SnrMggOffcl", skip_serializing_if = "Option::is_none")]
	pub snr_mgg_offcl: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "Prtctr", skip_serializing_if = "Option::is_none")]
	pub prtctr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "RegdShrhldrNm", skip_serializing_if = "Option::is_none")]
	pub regd_shrhldr_nm: Option<RegisteredShareholderName1Choice>,
}

impl AccountParties17 {
	pub fn validate(&self) -> bool {
		if !self.prncpl_acct_pty.validate() { return false }
		if let Some(ref scndry_ownr_vec) = self.scndry_ownr { for item in scndry_ownr_vec { if !item.validate() { return false; } } }
		if let Some(ref bnfcry_vec) = self.bnfcry { for item in bnfcry_vec { if !item.validate() { return false; } } }
		if let Some(ref pwr_of_attny_vec) = self.pwr_of_attny { for item in pwr_of_attny_vec { if !item.validate() { return false; } } }
		if let Some(ref lgl_guardn_vec) = self.lgl_guardn { for item in lgl_guardn_vec { if !item.validate() { return false; } } }
		if let Some(ref ctdn_for_mnr_vec) = self.ctdn_for_mnr { for item in ctdn_for_mnr_vec { if !item.validate() { return false; } } }
		if let Some(ref sucssr_on_dth_vec) = self.sucssr_on_dth { for item in sucssr_on_dth_vec { if !item.validate() { return false; } } }
		if let Some(ref admstr_vec) = self.admstr { for item in admstr_vec { if !item.validate() { return false; } } }
		if let Some(ref othr_pty_vec) = self.othr_pty { for item in othr_pty_vec { if !item.validate() { return false; } } }
		if let Some(ref grntr_vec) = self.grntr { for item in grntr_vec { if !item.validate() { return false; } } }
		if let Some(ref sttlr_vec) = self.sttlr { for item in sttlr_vec { if !item.validate() { return false; } } }
		if let Some(ref snr_mgg_offcl_vec) = self.snr_mgg_offcl { for item in snr_mgg_offcl_vec { if !item.validate() { return false; } } }
		if let Some(ref prtctr_vec) = self.prtctr { for item in prtctr_vec { if !item.validate() { return false; } } }
		if let Some(ref regd_shrhldr_nm_value) = self.regd_shrhldr_nm { if !regd_shrhldr_nm_value.validate() { return false; } }
		return true
	}
}


// AccountSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalAccountIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl AccountSchemeName1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
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


// AccountType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<FundCashAccount4Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl AccountType2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// AccountUsageType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountUsageType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AccountUsageType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl AccountUsageType2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// AccountUsageType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AccountUsageType2Code {
	#[default]
	#[serde(rename = "INVE")]
	CodeINVE,
	#[serde(rename = "ISSP")]
	CodeISSP,
	#[serde(rename = "SETP")]
	CodeSETP,
	#[serde(rename = "TRDP")]
	CodeTRDP,
}

impl AccountUsageType2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AccountingStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountingStatus1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AccountingStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl AccountingStatus1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// AccountingStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AccountingStatus1Code {
	#[default]
	#[serde(rename = "YDOM")]
	CodeYDOM,
	#[serde(rename = "NDOM")]
	CodeNDOM,
}

impl AccountingStatus1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ActiveCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and13_decimal_amount_simple_type: f64,
}

impl ActiveCurrencyAnd13DecimalAmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.active_currency_and13_decimal_amount_simple_type < 0.000000 {
			return false
		}
		return true
	}
}


// ActiveCurrencyAnd13DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveCurrencyAnd13DecimalAmount {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and_amount_simple_type: f64,
}

impl ActiveCurrencyAndAmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.active_currency_and_amount_simple_type < 0.000000 {
			return false
		}
		return true
	}
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveCurrencyAndAmount {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}

impl ActiveCurrencyCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_currency_code) {
			return false
		}
		return true
	}
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_code: String,
}

impl ActiveOrHistoricCurrencyCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_or_historic_currency_code) {
			return false
		}
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


// AdditiononalInformation13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditiononalInformation13 {
	#[serde(rename = "Lmttn", skip_serializing_if = "Option::is_none")]
	pub lmttn: Option<Max350Text>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
	#[serde(rename = "AcctVldtn", skip_serializing_if = "Option::is_none")]
	pub acct_vldtn: Option<Max350Text>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<Max35Text>,
	#[serde(rename = "Rgltr", skip_serializing_if = "Option::is_none")]
	pub rgltr: Option<PartyIdentification125Choice>,
	#[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
	pub sts: Option<RestrictionStatus1Choice>,
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<DateTimePeriod2>,
}

impl AdditiononalInformation13 {
	pub fn validate(&self) -> bool {
		if let Some(ref lmttn_value) = self.lmttn { if !lmttn_value.validate() { return false; } }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if !addtl_inf_value.validate() { return false; } }
		if let Some(ref acct_vldtn_value) = self.acct_vldtn { if !acct_vldtn_value.validate() { return false; } }
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref rgltr_value) = self.rgltr { if !rgltr_value.validate() { return false; } }
		if let Some(ref sts_value) = self.sts { if !sts_value.validate() { return false; } }
		if let Some(ref prd_value) = self.prd { if !prd_value.validate() { return false; } }
		return true
	}
}


// AddressType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AddressType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl AddressType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// AddressType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AddressType1Code {
	#[default]
	#[serde(rename = "HOME")]
	CodeHOME,
	#[serde(rename = "BIZZ")]
	CodeBIZZ,
}

impl AddressType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AddressType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AddressType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl AddressType2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
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


// AlternateSecurityIdentification7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AlternateSecurityIdentification7 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "IdSrc")]
	pub id_src: IdentificationSource1Choice,
}

impl AlternateSecurityIdentification7 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.id_src.validate() { return false }
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


// AustrianBankleitzahlIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AustrianBankleitzahlIdentifier {
	#[serde(rename = "$value")]
	pub austrian_bankleitzahl_identifier: String,
}

impl AustrianBankleitzahlIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("AT[0-9]{5,5}").unwrap();
		if !pattern.is_match(&self.austrian_bankleitzahl_identifier) {
			return false
		}
		return true
	}
}


// BICFIDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "$value")]
	pub bicfi_dec2014_identifier: String,
}

impl BICFIDec2014Identifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.bicfi_dec2014_identifier) {
			return false
		}
		return true
	}
}


// BelgianIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BelgianIdentifier {
	#[serde(rename = "$value")]
	pub belgian_identifier: String,
}

impl BelgianIdentifier {
	pub fn validate(&self) -> bool {
		return true
	}
}


// BlockedHoldingDetails2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockedHoldingDetails2 {
	#[serde(rename = "BlckdHldg")]
	pub blckd_hldg: Holding1Code,
	#[serde(rename = "PrtlHldgUnits", skip_serializing_if = "Option::is_none")]
	pub prtl_hldg_units: Option<f64>,
	#[serde(rename = "HldgCertNb", skip_serializing_if = "Option::is_none")]
	pub hldg_cert_nb: Option<Max35Text>,
}

impl BlockedHoldingDetails2 {
	pub fn validate(&self) -> bool {
		if !self.blckd_hldg.validate() { return false }
		if let Some(ref hldg_cert_nb_value) = self.hldg_cert_nb { if !hldg_cert_nb_value.validate() { return false; } }
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


// Bloomberg2Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Bloomberg2Identifier {
	#[serde(rename = "$value")]
	pub bloomberg2_identifier: String,
}

impl Bloomberg2Identifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("(BBG)[BCDFGHJKLMNPQRSTVWXYZ\\d]{8}\\d").unwrap();
		if !pattern.is_match(&self.bloomberg2_identifier) {
			return false
		}
		return true
	}
}


// BranchData4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchData4 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max35Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress1>,
}

impl BranchData4 {
	pub fn validate(&self) -> bool {
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if !pstl_adr_value.validate() { return false; } }
		return true
	}
}


// CHIPSParticipantIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CHIPSParticipantIdentifier {
	#[serde(rename = "$value")]
	pub chips_participant_identifier: String,
}

impl CHIPSParticipantIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("CP[0-9]{4,4}").unwrap();
		if !pattern.is_match(&self.chips_participant_identifier) {
			return false
		}
		return true
	}
}


// CHIPSUniversalIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CHIPSUniversalIdentifier {
	#[serde(rename = "$value")]
	pub chips_universal_identifier: String,
}

impl CHIPSUniversalIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("CH[0-9]{6,6}").unwrap();
		if !pattern.is_match(&self.chips_universal_identifier) {
			return false
		}
		return true
	}
}


// CRSForm1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CRSForm1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CRSFormType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl CRSForm1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// CRSFormType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CRSFormType1Code {
	#[default]
	#[serde(rename = "CER4")]
	CodeCER4,
	#[serde(rename = "CER3")]
	CodeCER3,
	#[serde(rename = "CER5")]
	CodeCER5,
	#[serde(rename = "CER6")]
	CodeCER6,
	#[serde(rename = "CER8")]
	CodeCER8,
	#[serde(rename = "CER1")]
	CodeCER1,
	#[serde(rename = "CER2")]
	CodeCER2,
	#[serde(rename = "CER7")]
	CodeCER7,
}

impl CRSFormType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CRSSource1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CRSSource1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CRSSourceStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl CRSSource1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// CRSSourceStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CRSSourceStatus1Code {
	#[default]
	#[serde(rename = "CALC")]
	CodeCALC,
	#[serde(rename = "DECL")]
	CodeDECL,
}

impl CRSSourceStatus1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CRSStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CRSStatus1Code {
	#[default]
	#[serde(rename = "C101")]
	CodeC101,
	#[serde(rename = "C102")]
	CodeC102,
	#[serde(rename = "C103")]
	CodeC103,
	#[serde(rename = "C104")]
	CodeC104,
	#[serde(rename = "C105")]
	CodeC105,
	#[serde(rename = "C106")]
	CodeC106,
	#[serde(rename = "C107")]
	CodeC107,
	#[serde(rename = "C108")]
	CodeC108,
	#[serde(rename = "C109")]
	CodeC109,
	#[serde(rename = "C110")]
	CodeC110,
	#[serde(rename = "C111")]
	CodeC111,
	#[serde(rename = "C112")]
	CodeC112,
	#[serde(rename = "C113")]
	CodeC113,
	#[serde(rename = "C114")]
	CodeC114,
}

impl CRSStatus1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CRSStatus3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CRSStatus3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CRSStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl CRSStatus3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// CRSStatus4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CRSStatus4 {
	#[serde(rename = "Tp")]
	pub tp: CRSStatus3Choice,
	#[serde(rename = "Src", skip_serializing_if = "Option::is_none")]
	pub src: Option<CRSSource1Choice>,
	#[serde(rename = "XcptnlRptgCtry", skip_serializing_if = "Option::is_none")]
	pub xcptnl_rptg_ctry: Option<CountryCode>,
}

impl CRSStatus4 {
	pub fn validate(&self) -> bool {
		if !self.tp.validate() { return false }
		if let Some(ref src_value) = self.src { if !src_value.validate() { return false; } }
		if let Some(ref xcptnl_rptg_ctry_value) = self.xcptnl_rptg_ctry { if !xcptnl_rptg_ctry_value.validate() { return false; } }
		return true
	}
}


// CUSIPIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CUSIPIdentifier {
	#[serde(rename = "$value")]
	pub cusip_identifier: String,
}

impl CUSIPIdentifier {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CanadianPaymentsARNIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CanadianPaymentsARNIdentifier {
	#[serde(rename = "$value")]
	pub canadian_payments_arn_identifier: String,
}

impl CanadianPaymentsARNIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("CA[0-9]{9,9}").unwrap();
		if !pattern.is_match(&self.canadian_payments_arn_identifier) {
			return false
		}
		return true
	}
}


// CardType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CardType1Code {
	#[default]
	#[serde(rename = "CRDT")]
	CodeCRDT,
	#[serde(rename = "DBIT")]
	CodeDBIT,
}

impl CardType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CashAccount204 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount204 {
	#[serde(rename = "SttlmCcy")]
	pub sttlm_ccy: ActiveCurrencyCode,
	#[serde(rename = "Id")]
	pub id: AccountIdentificationAndName5,
	#[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
	pub acct_ownr: Option<PartyIdentification125Choice>,
	#[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
	pub acct_svcr: Option<FinancialInstitutionIdentification11Choice>,
	#[serde(rename = "AcctSvcrBrnch", skip_serializing_if = "Option::is_none")]
	pub acct_svcr_brnch: Option<BranchData4>,
	#[serde(rename = "AcctOwnrOthrId", skip_serializing_if = "Option::is_none")]
	pub acct_ownr_othr_id: Option<Vec<GenericIdentification82>>,
	#[serde(rename = "InvstmtAcctTp", skip_serializing_if = "Option::is_none")]
	pub invstmt_acct_tp: Option<AccountType2Choice>,
	#[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt: Option<CreditDebit3Code>,
	#[serde(rename = "SttlmInstrRsn", skip_serializing_if = "Option::is_none")]
	pub sttlm_instr_rsn: Option<SettlementInstructionReason1Choice>,
	#[serde(rename = "CshAcctPurp", skip_serializing_if = "Option::is_none")]
	pub csh_acct_purp: Option<CashAccountType3Choice>,
	#[serde(rename = "CshAcctDsgnt", skip_serializing_if = "Option::is_none")]
	pub csh_acct_dsgnt: Option<AccountDesignation1Choice>,
	#[serde(rename = "DvddPctg", skip_serializing_if = "Option::is_none")]
	pub dvdd_pctg: Option<f64>,
}

impl CashAccount204 {
	pub fn validate(&self) -> bool {
		if !self.sttlm_ccy.validate() { return false }
		if !self.id.validate() { return false }
		if let Some(ref acct_ownr_value) = self.acct_ownr { if !acct_ownr_value.validate() { return false; } }
		if let Some(ref acct_svcr_value) = self.acct_svcr { if !acct_svcr_value.validate() { return false; } }
		if let Some(ref acct_svcr_brnch_value) = self.acct_svcr_brnch { if !acct_svcr_brnch_value.validate() { return false; } }
		if let Some(ref acct_ownr_othr_id_vec) = self.acct_ownr_othr_id { for item in acct_ownr_othr_id_vec { if !item.validate() { return false; } } }
		if let Some(ref invstmt_acct_tp_value) = self.invstmt_acct_tp { if !invstmt_acct_tp_value.validate() { return false; } }
		if let Some(ref cdt_dbt_value) = self.cdt_dbt { if !cdt_dbt_value.validate() { return false; } }
		if let Some(ref sttlm_instr_rsn_value) = self.sttlm_instr_rsn { if !sttlm_instr_rsn_value.validate() { return false; } }
		if let Some(ref csh_acct_purp_value) = self.csh_acct_purp { if !csh_acct_purp_value.validate() { return false; } }
		if let Some(ref csh_acct_dsgnt_value) = self.csh_acct_dsgnt { if !csh_acct_dsgnt_value.validate() { return false; } }
		return true
	}
}


// CashAccountType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CashAccountType5Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl CashAccountType3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// CashAccountType5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CashAccountType5Code {
	#[default]
	#[serde(rename = "LEND")]
	CodeLEND,
	#[serde(rename = "COLL")]
	CodeCOLL,
	#[serde(rename = "SETT")]
	CodeSETT,
	#[serde(rename = "MARR")]
	CodeMARR,
	#[serde(rename = "SEGT")]
	CodeSEGT,
}

impl CashAccountType5Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CashSettlement3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashSettlement3 {
	#[serde(rename = "CshAcctDtls", skip_serializing_if = "Option::is_none")]
	pub csh_acct_dtls: Option<Vec<CashAccount204>>,
	#[serde(rename = "OthrCshSttlmDtls", skip_serializing_if = "Option::is_none")]
	pub othr_csh_sttlm_dtls: Option<Vec<PaymentInstrument17>>,
}

impl CashSettlement3 {
	pub fn validate(&self) -> bool {
		if let Some(ref csh_acct_dtls_vec) = self.csh_acct_dtls { for item in csh_acct_dtls_vec { if !item.validate() { return false; } } }
		if let Some(ref othr_csh_sttlm_dtls_vec) = self.othr_csh_sttlm_dtls { for item in othr_csh_sttlm_dtls_vec { if !item.validate() { return false; } } }
		return true
	}
}


// CertificateType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CertificateType2Code {
	#[default]
	#[serde(rename = "AMLC")]
	CodeAMLC,
	#[serde(rename = "DVLC")]
	CodeDVLC,
	#[serde(rename = "DFOR")]
	CodeDFOR,
	#[serde(rename = "GOST")]
	CodeGOST,
	#[serde(rename = "IDEN")]
	CodeIDEN,
	#[serde(rename = "INCU")]
	CodeINCU,
	#[serde(rename = "LREF")]
	CodeLREF,
	#[serde(rename = "PASS")]
	CodePASS,
	#[serde(rename = "PRAD")]
	CodePRAD,
	#[serde(rename = "PKIC")]
	CodePKIC,
}

impl CertificateType2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CertificationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CertificationType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CertificateType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl CertificationType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// Cheque4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Cheque4 {
	#[serde(rename = "PyeeId")]
	pub pyee_id: NameAndAddress5,
}

impl Cheque4 {
	pub fn validate(&self) -> bool {
		if !self.pyee_id.validate() { return false }
		return true
	}
}


// CitizenshipInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CitizenshipInformation2 {
	#[serde(rename = "Ntlty")]
	pub ntlty: String,
	#[serde(rename = "MnrInd")]
	pub mnr_ind: bool,
}

impl CitizenshipInformation2 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CivilStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CivilStatus1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CivilStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl CivilStatus1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// CivilStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CivilStatus1Code {
	#[default]
	#[serde(rename = "DIVO")]
	CodeDIVO,
	#[serde(rename = "LDIV")]
	CodeLDIV,
	#[serde(rename = "MARR")]
	CodeMARR,
	#[serde(rename = "SEPA")]
	CodeSEPA,
	#[serde(rename = "SING")]
	CodeSING,
	#[serde(rename = "UNIO")]
	CodeUNIO,
	#[serde(rename = "WIDO")]
	CodeWIDO,
}

impl CivilStatus1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ClearingSystemMemberIdentification4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemMemberIdentification4Choice {
	#[serde(rename = "USCHU", skip_serializing_if = "Option::is_none")]
	pub uschu: Option<CHIPSUniversalIdentifier>,
	#[serde(rename = "NZNCC", skip_serializing_if = "Option::is_none")]
	pub nzncc: Option<NewZealandNCCIdentifier>,
	#[serde(rename = "IENSC", skip_serializing_if = "Option::is_none")]
	pub iensc: Option<IrishNSCIdentifier>,
	#[serde(rename = "GBSC", skip_serializing_if = "Option::is_none")]
	pub gbsc: Option<UKDomesticSortCodeIdentifier>,
	#[serde(rename = "USCH", skip_serializing_if = "Option::is_none")]
	pub usch: Option<CHIPSParticipantIdentifier>,
	#[serde(rename = "CHBC", skip_serializing_if = "Option::is_none")]
	pub chbc: Option<SwissBCIdentifier>,
	#[serde(rename = "USFW", skip_serializing_if = "Option::is_none")]
	pub usfw: Option<FedwireRoutingNumberIdentifier>,
	#[serde(rename = "PTNCC", skip_serializing_if = "Option::is_none")]
	pub ptncc: Option<PortugueseNCCIdentifier>,
	#[serde(rename = "RUCB", skip_serializing_if = "Option::is_none")]
	pub rucb: Option<RussianCentralBankIdentificationCodeIdentifier>,
	#[serde(rename = "ITNCC", skip_serializing_if = "Option::is_none")]
	pub itncc: Option<ItalianDomesticIdentifier>,
	#[serde(rename = "ATBLZ", skip_serializing_if = "Option::is_none")]
	pub atblz: Option<AustrianBankleitzahlIdentifier>,
	#[serde(rename = "CACPA", skip_serializing_if = "Option::is_none")]
	pub cacpa: Option<CanadianPaymentsARNIdentifier>,
	#[serde(rename = "CHSIC", skip_serializing_if = "Option::is_none")]
	pub chsic: Option<SwissSICIdentifier>,
	#[serde(rename = "DEBLZ", skip_serializing_if = "Option::is_none")]
	pub deblz: Option<GermanBankleitzahlIdentifier>,
	#[serde(rename = "ESNCC", skip_serializing_if = "Option::is_none")]
	pub esncc: Option<SpanishDomesticInterbankingIdentifier>,
	#[serde(rename = "ZANCC", skip_serializing_if = "Option::is_none")]
	pub zancc: Option<SouthAfricanNCCIdentifier>,
	#[serde(rename = "HKNCC", skip_serializing_if = "Option::is_none")]
	pub hkncc: Option<HongKongBankIdentifier>,
	#[serde(rename = "AUBSBx", skip_serializing_if = "Option::is_none")]
	pub aubs_bx: Option<ExtensiveBranchNetworkIdentifier>,
	#[serde(rename = "AUBSBs", skip_serializing_if = "Option::is_none")]
	pub aubs_bs: Option<SmallNetworkIdentifier>,
}

impl ClearingSystemMemberIdentification4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref uschu_value) = self.uschu { if !uschu_value.validate() { return false; } }
		if let Some(ref nzncc_value) = self.nzncc { if !nzncc_value.validate() { return false; } }
		if let Some(ref iensc_value) = self.iensc { if !iensc_value.validate() { return false; } }
		if let Some(ref gbsc_value) = self.gbsc { if !gbsc_value.validate() { return false; } }
		if let Some(ref usch_value) = self.usch { if !usch_value.validate() { return false; } }
		if let Some(ref chbc_value) = self.chbc { if !chbc_value.validate() { return false; } }
		if let Some(ref usfw_value) = self.usfw { if !usfw_value.validate() { return false; } }
		if let Some(ref ptncc_value) = self.ptncc { if !ptncc_value.validate() { return false; } }
		if let Some(ref rucb_value) = self.rucb { if !rucb_value.validate() { return false; } }
		if let Some(ref itncc_value) = self.itncc { if !itncc_value.validate() { return false; } }
		if let Some(ref atblz_value) = self.atblz { if !atblz_value.validate() { return false; } }
		if let Some(ref cacpa_value) = self.cacpa { if !cacpa_value.validate() { return false; } }
		if let Some(ref chsic_value) = self.chsic { if !chsic_value.validate() { return false; } }
		if let Some(ref deblz_value) = self.deblz { if !deblz_value.validate() { return false; } }
		if let Some(ref esncc_value) = self.esncc { if !esncc_value.validate() { return false; } }
		if let Some(ref zancc_value) = self.zancc { if !zancc_value.validate() { return false; } }
		if let Some(ref hkncc_value) = self.hkncc { if !hkncc_value.validate() { return false; } }
		if let Some(ref aubs_bx_value) = self.aubs_bx { if !aubs_bx_value.validate() { return false; } }
		if let Some(ref aubs_bs_value) = self.aubs_bs { if !aubs_bs_value.validate() { return false; } }
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


// Collateral1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Collateral1Code {
	#[default]
	#[serde(rename = "COLL")]
	CodeCOLL,
	#[serde(rename = "NCOL")]
	CodeNCOL,
}

impl Collateral1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CommunicationAddress6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommunicationAddress6 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType1Choice>,
	#[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
	pub email: Option<Max256Text>,
	#[serde(rename = "Phne", skip_serializing_if = "Option::is_none")]
	pub phne: Option<PhoneNumber>,
	#[serde(rename = "Mob", skip_serializing_if = "Option::is_none")]
	pub mob: Option<PhoneNumber>,
	#[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
	pub fax_nb: Option<PhoneNumber>,
	#[serde(rename = "TlxAdr", skip_serializing_if = "Option::is_none")]
	pub tlx_adr: Option<Max35Text>,
	#[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
	pub url_adr: Option<Max256Text>,
}

impl CommunicationAddress6 {
	pub fn validate(&self) -> bool {
		if let Some(ref adr_tp_value) = self.adr_tp { if !adr_tp_value.validate() { return false; } }
		if let Some(ref email_value) = self.email { if !email_value.validate() { return false; } }
		if let Some(ref phne_value) = self.phne { if !phne_value.validate() { return false; } }
		if let Some(ref mob_value) = self.mob { if !mob_value.validate() { return false; } }
		if let Some(ref fax_nb_value) = self.fax_nb { if !fax_nb_value.validate() { return false; } }
		if let Some(ref tlx_adr_value) = self.tlx_adr { if !tlx_adr_value.validate() { return false; } }
		if let Some(ref url_adr_value) = self.url_adr { if !url_adr_value.validate() { return false; } }
		return true
	}
}


// CommunicationMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CommunicationMethod1Code {
	#[default]
	#[serde(rename = "SWMT")]
	CodeSWMT,
	#[serde(rename = "SWMX")]
	CodeSWMX,
	#[serde(rename = "FAXI")]
	CodeFAXI,
	#[serde(rename = "EMAL")]
	CodeEMAL,
	#[serde(rename = "PROP")]
	CodePROP,
}

impl CommunicationMethod1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CommunicationMethod3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommunicationMethod3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CommunicationMethod1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl CommunicationMethod3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// CompanyLink1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompanyLink1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CompanyLink1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl CompanyLink1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// CompanyLink1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CompanyLink1Code {
	#[default]
	#[serde(rename = "AGEN")]
	CodeAGEN,
	#[serde(rename = "BROK")]
	CodeBROK,
	#[serde(rename = "PART")]
	CodePART,
	#[serde(rename = "MEMB")]
	CodeMEMB,
	#[serde(rename = "PCOM")]
	CodePCOM,
	#[serde(rename = "RELA")]
	CodeRELA,
}

impl CompanyLink1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ConductClassification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ConductClassification1Code {
	#[default]
	#[serde(rename = "NSTA")]
	CodeNSTA,
	#[serde(rename = "RCLT")]
	CodeRCLT,
	#[serde(rename = "STAN")]
	CodeSTAN,
}

impl ConductClassification1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ConfirmationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConfirmationType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AccountManagementType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl ConfirmationType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// ConsolidatedTapeAssociationIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ConsolidatedTapeAssociationIdentifier {
	#[serde(rename = "$value")]
	pub consolidated_tape_association_identifier: String,
}

impl ConsolidatedTapeAssociationIdentifier {
	pub fn validate(&self) -> bool {
		if self.consolidated_tape_association_identifier.chars().count() < 1 {
			return false
		}
		if self.consolidated_tape_association_identifier.chars().count() > 35 {
			return false
		}
		return true
	}
}


// ConsolidationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConsolidationType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ConsolidationType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl ConsolidationType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// ConsolidationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ConsolidationType1Code {
	#[default]
	#[serde(rename = "GENL")]
	CodeGENL,
	#[serde(rename = "PART")]
	CodePART,
}

impl ConsolidationType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CountryAndResidentialStatusType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryAndResidentialStatusType2 {
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
	#[serde(rename = "ResdtlSts")]
	pub resdtl_sts: ResidentialStatus1Code,
}

impl CountryAndResidentialStatusType2 {
	pub fn validate(&self) -> bool {
		if !self.ctry.validate() { return false }
		if !self.resdtl_sts.validate() { return false }
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


// CreditDebit3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CreditDebit3Code {
	#[default]
	#[serde(rename = "CRDT")]
	CodeCRDT,
	#[serde(rename = "DBIT")]
	CodeDBIT,
}

impl CreditDebit3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CustomerConductClassification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CustomerConductClassification1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ConductClassification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl CustomerConductClassification1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// DataBaseCheck1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DataBaseCheck1 {
	#[serde(rename = "DBChck")]
	pub db_chck: bool,
	#[serde(rename = "Id")]
	pub id: Max35Text,
}

impl DataBaseCheck1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		return true
	}
}


// DateAndAmount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndAmount1 {
	#[serde(rename = "Dt")]
	pub dt: String,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
}

impl DateAndAmount1 {
	pub fn validate(&self) -> bool {
		if !self.amt.validate() { return false }
		return true
	}
}


// DateAndDateTime1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime1Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}

impl DateAndDateTime1Choice {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DateTimePeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod2 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none")]
	pub to_dt_tm: Option<String>,
}

impl DateTimePeriod2 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DeMinimus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DeMinimus1Choice {
	#[serde(rename = "DeMnmsAplbl", skip_serializing_if = "Option::is_none")]
	pub de_mnms_aplbl: Option<DeMinimusApplicable1>,
	#[serde(rename = "DeMnmsNotAplbl", skip_serializing_if = "Option::is_none")]
	pub de_mnms_not_aplbl: Option<DeMinimusNotApplicable1>,
}

impl DeMinimus1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref de_mnms_aplbl_value) = self.de_mnms_aplbl { if !de_mnms_aplbl_value.validate() { return false; } }
		if let Some(ref de_mnms_not_aplbl_value) = self.de_mnms_not_aplbl { if !de_mnms_not_aplbl_value.validate() { return false; } }
		return true
	}
}


// DeMinimusApplicable1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DeMinimusApplicable1 {
	#[serde(rename = "NewIssePrmssn")]
	pub new_isse_prmssn: bool,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
}

impl DeMinimusApplicable1 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DeMinimusNotApplicable1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DeMinimusNotApplicable1 {
	#[serde(rename = "RstrctdPrsnRsn")]
	pub rstrctd_prsn_rsn: Max350Text,
}

impl DeMinimusNotApplicable1 {
	pub fn validate(&self) -> bool {
		if !self.rstrctd_prsn_rsn.validate() { return false }
		return true
	}
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}

impl DecimalNumber {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DirectDebitMandate7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DirectDebitMandate7 {
	#[serde(rename = "DbtrAcct")]
	pub dbtr_acct: AccountIdentificationAndName5,
	#[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
	pub dbtr: Option<PartyIdentification125Choice>,
	#[serde(rename = "DbtrTaxIdNb", skip_serializing_if = "Option::is_none")]
	pub dbtr_tax_id_nb: Option<Max35Text>,
	#[serde(rename = "DbtrNtlRegnNb", skip_serializing_if = "Option::is_none")]
	pub dbtr_ntl_regn_nb: Option<Max35Text>,
	#[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
	pub cdtr: Option<PartyIdentification125Choice>,
	#[serde(rename = "DbtrAgt")]
	pub dbtr_agt: FinancialInstitutionIdentification11Choice,
	#[serde(rename = "DbtrAgtBrnch", skip_serializing_if = "Option::is_none")]
	pub dbtr_agt_brnch: Option<BranchData4>,
	#[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
	pub cdtr_agt: Option<FinancialInstitutionIdentification11Choice>,
	#[serde(rename = "CdtrAgtBrnch", skip_serializing_if = "Option::is_none")]
	pub cdtr_agt_brnch: Option<BranchData4>,
	#[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
	pub regn_id: Option<Max35Text>,
	#[serde(rename = "MndtId", skip_serializing_if = "Option::is_none")]
	pub mndt_id: Option<Max35Text>,
}

impl DirectDebitMandate7 {
	pub fn validate(&self) -> bool {
		if !self.dbtr_acct.validate() { return false }
		if let Some(ref dbtr_value) = self.dbtr { if !dbtr_value.validate() { return false; } }
		if let Some(ref dbtr_tax_id_nb_value) = self.dbtr_tax_id_nb { if !dbtr_tax_id_nb_value.validate() { return false; } }
		if let Some(ref dbtr_ntl_regn_nb_value) = self.dbtr_ntl_regn_nb { if !dbtr_ntl_regn_nb_value.validate() { return false; } }
		if let Some(ref cdtr_value) = self.cdtr { if !cdtr_value.validate() { return false; } }
		if !self.dbtr_agt.validate() { return false }
		if let Some(ref dbtr_agt_brnch_value) = self.dbtr_agt_brnch { if !dbtr_agt_brnch_value.validate() { return false; } }
		if let Some(ref cdtr_agt_value) = self.cdtr_agt { if !cdtr_agt_value.validate() { return false; } }
		if let Some(ref cdtr_agt_brnch_value) = self.cdtr_agt_brnch { if !cdtr_agt_brnch_value.validate() { return false; } }
		if let Some(ref regn_id_value) = self.regn_id { if !regn_id_value.validate() { return false; } }
		if let Some(ref mndt_id_value) = self.mndt_id { if !mndt_id_value.validate() { return false; } }
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


// DistributionPolicy1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DistributionPolicy1Code {
	#[default]
	#[serde(rename = "DIST")]
	CodeDIST,
	#[serde(rename = "ACCU")]
	CodeACCU,
}

impl DistributionPolicy1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DocumentToSend4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentToSend4 {
	#[serde(rename = "Tp")]
	pub tp: Max140Text,
	#[serde(rename = "Rcpt")]
	pub rcpt: PartyIdentification125Choice,
	#[serde(rename = "MtdOfTrnsmssn")]
	pub mtd_of_trnsmssn: CommunicationMethod3Choice,
}

impl DocumentToSend4 {
	pub fn validate(&self) -> bool {
		if !self.tp.validate() { return false }
		if !self.rcpt.validate() { return false }
		if !self.mtd_of_trnsmssn.validate() { return false }
		return true
	}
}


// DutchIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DutchIdentifier {
	#[serde(rename = "$value")]
	pub dutch_identifier: String,
}

impl DutchIdentifier {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Eligible1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Eligible1Code {
	#[default]
	#[serde(rename = "ELIG")]
	CodeELIG,
	#[serde(rename = "NELI")]
	CodeNELI,
}

impl Eligible1Code {
	pub fn validate(&self) -> bool {
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


// EuroclearClearstreamIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EuroclearClearstreamIdentifier {
	#[serde(rename = "$value")]
	pub euroclear_clearstream_identifier: String,
}

impl EuroclearClearstreamIdentifier {
	pub fn validate(&self) -> bool {
		if self.euroclear_clearstream_identifier.chars().count() < 1 {
			return false
		}
		if self.euroclear_clearstream_identifier.chars().count() > 12 {
			return false
		}
		return true
	}
}


// EventFrequency10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EventFrequency10Code {
	#[default]
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "ADHO")]
	CodeADHO,
}

impl EventFrequency10Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// EventFrequency1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EventFrequency1Code {
	#[default]
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "SEMI")]
	CodeSEMI,
	#[serde(rename = "QUTR")]
	CodeQUTR,
	#[serde(rename = "TOMN")]
	CodeTOMN,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "TWMN")]
	CodeTWMN,
	#[serde(rename = "TOWK")]
	CodeTOWK,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "ADHO")]
	CodeADHO,
	#[serde(rename = "INDA")]
	CodeINDA,
	#[serde(rename = "OVNG")]
	CodeOVNG,
	#[serde(rename = "ONDE")]
	CodeONDE,
}

impl EventFrequency1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// EventFrequency8Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EventFrequency8Code {
	#[default]
	#[serde(rename = "ADHO")]
	CodeADHO,
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "FOMN")]
	CodeFOMN,
	#[serde(rename = "TOMN")]
	CodeTOMN,
	#[serde(rename = "TOWK")]
	CodeTOWK,
	#[serde(rename = "TYEA")]
	CodeTYEA,
	#[serde(rename = "INDA")]
	CodeINDA,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "ONDE")]
	CodeONDE,
	#[serde(rename = "OVNG")]
	CodeOVNG,
	#[serde(rename = "QUTR")]
	CodeQUTR,
	#[serde(rename = "SEMI")]
	CodeSEMI,
	#[serde(rename = "TWMN")]
	CodeTWMN,
	#[serde(rename = "WEEK")]
	CodeWEEK,
}

impl EventFrequency8Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// EventFrequency9Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EventFrequency9Code {
	#[default]
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "SEMI")]
	CodeSEMI,
	#[serde(rename = "QUTR")]
	CodeQUTR,
	#[serde(rename = "TOMN")]
	CodeTOMN,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "TWMN")]
	CodeTWMN,
	#[serde(rename = "TOWK")]
	CodeTOWK,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "ADHO")]
	CodeADHO,
	#[serde(rename = "INDA")]
	CodeINDA,
	#[serde(rename = "OVNG")]
	CodeOVNG,
	#[serde(rename = "ONDE")]
	CodeONDE,
	#[serde(rename = "NONE")]
	CodeNONE,
}

impl EventFrequency9Code {
	pub fn validate(&self) -> bool {
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


// Extended350Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Extended350Code {
	#[serde(rename = "$value")]
	pub extended350_code: String,
}

impl Extended350Code {
	pub fn validate(&self) -> bool {
		if self.extended350_code.chars().count() < 1 {
			return false
		}
		if self.extended350_code.chars().count() > 350 {
			return false
		}
		return true
	}
}


// ExtendedParty14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExtendedParty14 {
	#[serde(rename = "XtndedPtyRole")]
	pub xtnded_pty_role: Extended350Code,
	#[serde(rename = "OthrPtyDtls")]
	pub othr_pty_dtls: InvestmentAccountOwnershipInformation16,
}

impl ExtendedParty14 {
	pub fn validate(&self) -> bool {
		if !self.xtnded_pty_role.validate() { return false }
		if !self.othr_pty_dtls.validate() { return false }
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


// ExtensiveBranchNetworkIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExtensiveBranchNetworkIdentifier {
	#[serde(rename = "$value")]
	pub extensive_branch_network_identifier: String,
}

impl ExtensiveBranchNetworkIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("AU[0-9]{6,6}").unwrap();
		if !pattern.is_match(&self.extensive_branch_network_identifier) {
			return false
		}
		return true
	}
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "$value")]
	pub external_account_identification1_code: String,
}

impl ExternalAccountIdentification1Code {
	pub fn validate(&self) -> bool {
		if self.external_account_identification1_code.chars().count() < 1 {
			return false
		}
		if self.external_account_identification1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// FATCAForm1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FATCAForm1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<FATCAFormType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl FATCAForm1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// FATCAFormType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FATCAFormType1Code {
	#[default]
	#[serde(rename = "CER5")]
	CodeCER5,
	#[serde(rename = "CER7")]
	CodeCER7,
	#[serde(rename = "CER1")]
	CodeCER1,
	#[serde(rename = "CER2")]
	CodeCER2,
	#[serde(rename = "CER3")]
	CodeCER3,
	#[serde(rename = "CER4")]
	CodeCER4,
	#[serde(rename = "CER6")]
	CodeCER6,
}

impl FATCAFormType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// FATCASource1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FATCASource1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<FATCASourceStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl FATCASource1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// FATCASourceStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FATCASourceStatus1Code {
	#[default]
	#[serde(rename = "CALC")]
	CodeCALC,
	#[serde(rename = "DECL")]
	CodeDECL,
}

impl FATCASourceStatus1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// FATCAStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FATCAStatus1Code {
	#[default]
	#[serde(rename = "F101")]
	CodeF101,
	#[serde(rename = "F102")]
	CodeF102,
	#[serde(rename = "F103")]
	CodeF103,
	#[serde(rename = "F104")]
	CodeF104,
	#[serde(rename = "F105")]
	CodeF105,
	#[serde(rename = "F201")]
	CodeF201,
	#[serde(rename = "F202")]
	CodeF202,
	#[serde(rename = "F203")]
	CodeF203,
	#[serde(rename = "F204")]
	CodeF204,
	#[serde(rename = "F205")]
	CodeF205,
	#[serde(rename = "F206")]
	CodeF206,
}

impl FATCAStatus1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// FATCAStatus2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FATCAStatus2 {
	#[serde(rename = "Tp")]
	pub tp: FATCAStatus2Choice,
	#[serde(rename = "Src", skip_serializing_if = "Option::is_none")]
	pub src: Option<FATCASource1Choice>,
}

impl FATCAStatus2 {
	pub fn validate(&self) -> bool {
		if !self.tp.validate() { return false }
		if let Some(ref src_value) = self.src { if !src_value.validate() { return false; } }
		return true
	}
}


// FATCAStatus2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FATCAStatus2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<FATCAStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl FATCAStatus2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// FedwireRoutingNumberIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FedwireRoutingNumberIdentifier {
	#[serde(rename = "$value")]
	pub fedwire_routing_number_identifier: String,
}

impl FedwireRoutingNumberIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("FW[0-9]{9,9}").unwrap();
		if !pattern.is_match(&self.fedwire_routing_number_identifier) {
			return false
		}
		return true
	}
}


// FinancialInstitutionIdentification11Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionIdentification11Choice {
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
	#[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
	pub bicfi: Option<BICFIDec2014Identifier>,
	#[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification4Choice>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<SimpleIdentificationInformation4>,
}

impl FinancialInstitutionIdentification11Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref nm_and_adr_value) = self.nm_and_adr { if !nm_and_adr_value.validate() { return false; } }
		if let Some(ref bicfi_value) = self.bicfi { if !bicfi_value.validate() { return false; } }
		if let Some(ref clr_sys_mmb_id_value) = self.clr_sys_mmb_id { if !clr_sys_mmb_id_value.validate() { return false; } }
		if let Some(ref prtry_id_value) = self.prtry_id { if !prtry_id_value.validate() { return false; } }
		return true
	}
}


// FinancialInstrument87 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument87 {
	#[serde(rename = "Id")]
	pub id: SecurityIdentification25Choice,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
	#[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
	pub shrt_nm: Option<Max35Text>,
	#[serde(rename = "SplmtryId", skip_serializing_if = "Option::is_none")]
	pub splmtry_id: Option<Max35Text>,
	#[serde(rename = "ClssTp", skip_serializing_if = "Option::is_none")]
	pub clss_tp: Option<Max35Text>,
	#[serde(rename = "SctiesForm", skip_serializing_if = "Option::is_none")]
	pub scties_form: Option<FormOfSecurity1Code>,
	#[serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none")]
	pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
	#[serde(rename = "PdctGrp", skip_serializing_if = "Option::is_none")]
	pub pdct_grp: Option<Max140Text>,
	#[serde(rename = "BlckdHldgDtls", skip_serializing_if = "Option::is_none")]
	pub blckd_hldg_dtls: Option<BlockedHoldingDetails2>,
	#[serde(rename = "Pldgg", skip_serializing_if = "Option::is_none")]
	pub pldgg: Option<Eligible1Code>,
	#[serde(rename = "Coll", skip_serializing_if = "Option::is_none")]
	pub coll: Option<Collateral1Code>,
	#[serde(rename = "ThrdPtyRghts", skip_serializing_if = "Option::is_none")]
	pub thrd_pty_rghts: Option<ThirdPartyRights2>,
	#[serde(rename = "FndOwnrsh", skip_serializing_if = "Option::is_none")]
	pub fnd_ownrsh: Option<FundOwnership1Code>,
	#[serde(rename = "FndIntntn", skip_serializing_if = "Option::is_none")]
	pub fnd_intntn: Option<FundIntention1Code>,
	#[serde(rename = "OprlSts", skip_serializing_if = "Option::is_none")]
	pub oprl_sts: Option<OperationalStatus1Code>,
}

impl FinancialInstrument87 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref shrt_nm_value) = self.shrt_nm { if !shrt_nm_value.validate() { return false; } }
		if let Some(ref splmtry_id_value) = self.splmtry_id { if !splmtry_id_value.validate() { return false; } }
		if let Some(ref clss_tp_value) = self.clss_tp { if !clss_tp_value.validate() { return false; } }
		if let Some(ref scties_form_value) = self.scties_form { if !scties_form_value.validate() { return false; } }
		if let Some(ref dstrbtn_plcy_value) = self.dstrbtn_plcy { if !dstrbtn_plcy_value.validate() { return false; } }
		if let Some(ref pdct_grp_value) = self.pdct_grp { if !pdct_grp_value.validate() { return false; } }
		if let Some(ref blckd_hldg_dtls_value) = self.blckd_hldg_dtls { if !blckd_hldg_dtls_value.validate() { return false; } }
		if let Some(ref pldgg_value) = self.pldgg { if !pldgg_value.validate() { return false; } }
		if let Some(ref coll_value) = self.coll { if !coll_value.validate() { return false; } }
		if let Some(ref thrd_pty_rghts_value) = self.thrd_pty_rghts { if !thrd_pty_rghts_value.validate() { return false; } }
		if let Some(ref fnd_ownrsh_value) = self.fnd_ownrsh { if !fnd_ownrsh_value.validate() { return false; } }
		if let Some(ref fnd_intntn_value) = self.fnd_intntn { if !fnd_intntn_value.validate() { return false; } }
		if let Some(ref oprl_sts_value) = self.oprl_sts { if !oprl_sts_value.validate() { return false; } }
		return true
	}
}


// FiscalYear1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FiscalYear1Choice {
	#[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
	pub start_dt: Option<String>,
	#[serde(rename = "EndDt", skip_serializing_if = "Option::is_none")]
	pub end_dt: Option<String>,
}

impl FiscalYear1Choice {
	pub fn validate(&self) -> bool {
		return true
	}
}


// FormOfSecurity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FormOfSecurity1Code {
	#[default]
	#[serde(rename = "BEAR")]
	CodeBEAR,
	#[serde(rename = "REGD")]
	CodeREGD,
}

impl FormOfSecurity1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Frequency20Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency20Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<EventFrequency8Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl Frequency20Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// FundCashAccount4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FundCashAccount4Code {
	#[default]
	#[serde(rename = "HEDG")]
	CodeHEDG,
	#[serde(rename = "CPFO")]
	CodeCPFO,
	#[serde(rename = "CPFS")]
	CodeCPFS,
	#[serde(rename = "SRSA")]
	CodeSRSA,
	#[serde(rename = "CSDO")]
	CodeCSDO,
	#[serde(rename = "TOFF")]
	CodeTOFF,
	#[serde(rename = "ICSA")]
	CodeICSA,
	#[serde(rename = "CSDM")]
	CodeCSDM,
	#[serde(rename = "CSDP")]
	CodeCSDP,
	#[serde(rename = "PPEN")]
	CodePPEN,
	#[serde(rename = "CPEN")]
	CodeCPEN,
}

impl FundCashAccount4Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// FundIntention1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FundIntention1Code {
	#[default]
	#[serde(rename = "YQUA")]
	CodeYQUA,
	#[serde(rename = "NQUA")]
	CodeNQUA,
}

impl FundIntention1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// FundOwnership1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FundOwnership1Code {
	#[default]
	#[serde(rename = "YALL")]
	CodeYALL,
	#[serde(rename = "NALL")]
	CodeNALL,
}

impl FundOwnership1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// GDPRData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GDPRData1 {
	#[serde(rename = "CnsntTp")]
	pub cnsnt_tp: GDPRDataConsent1Choice,
	#[serde(rename = "CnsntInd")]
	pub cnsnt_ind: bool,
	#[serde(rename = "CnsntDt")]
	pub cnsnt_dt: String,
}

impl GDPRData1 {
	pub fn validate(&self) -> bool {
		if !self.cnsnt_tp.validate() { return false }
		return true
	}
}


// GDPRDataConsent1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GDPRDataConsent1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<GDPRDataConsent1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl GDPRDataConsent1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// GDPRDataConsent1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum GDPRDataConsent1Code {
	#[default]
	#[serde(rename = "DP00")]
	CodeDP00,
	#[serde(rename = "DP03")]
	CodeDP03,
	#[serde(rename = "DP01")]
	CodeDP01,
	#[serde(rename = "DP02")]
	CodeDP02,
}

impl GDPRDataConsent1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Gender1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Gender1Code {
	#[default]
	#[serde(rename = "FEMA")]
	CodeFEMA,
	#[serde(rename = "MALE")]
	CodeMALE,
}

impl Gender1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// GenericAccountIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericAccountIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max34Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<AccountSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericAccountIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
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


// GenericIdentification82 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification82 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Tp")]
	pub tp: OtherIdentification3Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
	#[serde(rename = "IsseDt", skip_serializing_if = "Option::is_none")]
	pub isse_dt: Option<String>,
	#[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
	pub xpry_dt: Option<String>,
	#[serde(rename = "Stat", skip_serializing_if = "Option::is_none")]
	pub stat: Option<Max70Text>,
	#[serde(rename = "IssrCtry", skip_serializing_if = "Option::is_none")]
	pub issr_ctry: Option<CountryCode>,
}

impl GenericIdentification82 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.tp.validate() { return false }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		if let Some(ref stat_value) = self.stat { if !stat_value.validate() { return false; } }
		if let Some(ref issr_ctry_value) = self.issr_ctry { if !issr_ctry_value.validate() { return false; } }
		return true
	}
}


// GermanBankleitzahlIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GermanBankleitzahlIdentifier {
	#[serde(rename = "$value")]
	pub german_bankleitzahl_identifier: String,
}

impl GermanBankleitzahlIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("BL[0-9]{8,8}").unwrap();
		if !pattern.is_match(&self.german_bankleitzahl_identifier) {
			return false
		}
		return true
	}
}


// HighFrequencyTradingProfile1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct HighFrequencyTradingProfile1 {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "SttlmFrqcy", skip_serializing_if = "Option::is_none")]
	pub sttlm_frqcy: Option<SettlementFrequency1Choice>,
	#[serde(rename = "CnsldtnTp", skip_serializing_if = "Option::is_none")]
	pub cnsldtn_tp: Option<ConsolidationType1Choice>,
}

impl HighFrequencyTradingProfile1 {
	pub fn validate(&self) -> bool {
		if let Some(ref sttlm_frqcy_value) = self.sttlm_frqcy { if !sttlm_frqcy_value.validate() { return false; } }
		if let Some(ref cnsldtn_tp_value) = self.cnsldtn_tp { if !cnsldtn_tp_value.validate() { return false; } }
		return true
	}
}


// Holding1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Holding1Code {
	#[default]
	#[serde(rename = "CERT")]
	CodeCERT,
	#[serde(rename = "NPRH")]
	CodeNPRH,
	#[serde(rename = "PRTH")]
	CodePRTH,
}

impl Holding1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// HongKongBankIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HongKongBankIdentifier {
	#[serde(rename = "$value")]
	pub hong_kong_bank_identifier: String,
}

impl HongKongBankIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("HK[0-9]{3,3}").unwrap();
		if !pattern.is_match(&self.hong_kong_bank_identifier) {
			return false
		}
		return true
	}
}


// IBAN2007Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct IBAN2007Identifier {
	#[serde(rename = "$value")]
	pub iban2007_identifier: String,
}

impl IBAN2007Identifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}").unwrap();
		if !pattern.is_match(&self.iban2007_identifier) {
			return false
		}
		return true
	}
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "$value")]
	pub isin_oct2015_identifier: String,
}

impl ISINOct2015Identifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
		if !pattern.is_match(&self.isin_oct2015_identifier) {
			return false
		}
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


// IdentificationSource1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource1Choice {
	#[serde(rename = "Dmst", skip_serializing_if = "Option::is_none")]
	pub dmst: Option<CountryCode>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl IdentificationSource1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref dmst_value) = self.dmst { if !dmst_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// IncomePreference2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum IncomePreference2Code {
	#[default]
	#[serde(rename = "CASH")]
	CodeCASH,
	#[serde(rename = "SECU")]
	CodeSECU,
}

impl IncomePreference2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// IndividualPerson29 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndividualPerson29 {
	#[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
	pub nm_prfx: Option<NamePrefix1Choice>,
	#[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
	pub gvn_nm: Option<Max35Text>,
	#[serde(rename = "MddlNm", skip_serializing_if = "Option::is_none")]
	pub mddl_nm: Option<Max35Text>,
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Vec<PostalAddress21>,
}

impl IndividualPerson29 {
	pub fn validate(&self) -> bool {
		if let Some(ref nm_prfx_value) = self.nm_prfx { if !nm_prfx_value.validate() { return false; } }
		if let Some(ref gvn_nm_value) = self.gvn_nm { if !gvn_nm_value.validate() { return false; } }
		if let Some(ref mddl_nm_value) = self.mddl_nm { if !mddl_nm_value.validate() { return false; } }
		if !self.nm.validate() { return false }
		for item in &self.pstl_adr { if !item.validate() { return false; } }
		return true
	}
}


// IndividualPerson37 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndividualPerson37 {
	#[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
	pub nm_prfx: Option<NamePrefix1Choice>,
	#[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
	pub gvn_nm: Option<Max35Text>,
	#[serde(rename = "MddlNm", skip_serializing_if = "Option::is_none")]
	pub mddl_nm: Option<Max35Text>,
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "NmSfx", skip_serializing_if = "Option::is_none")]
	pub nm_sfx: Option<Max35Text>,
	#[serde(rename = "Gndr", skip_serializing_if = "Option::is_none")]
	pub gndr: Option<Gender1Code>,
	#[serde(rename = "BirthDt", skip_serializing_if = "Option::is_none")]
	pub birth_dt: Option<String>,
	#[serde(rename = "CtryOfBirth", skip_serializing_if = "Option::is_none")]
	pub ctry_of_birth: Option<CountryCode>,
	#[serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none")]
	pub prvc_of_birth: Option<Max35Text>,
	#[serde(rename = "CityOfBirth", skip_serializing_if = "Option::is_none")]
	pub city_of_birth: Option<Max35Text>,
	#[serde(rename = "Prfssn", skip_serializing_if = "Option::is_none")]
	pub prfssn: Option<Max35Text>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Vec<PostalAddress21>,
	#[serde(rename = "Ctznsh", skip_serializing_if = "Option::is_none")]
	pub ctznsh: Option<Vec<CitizenshipInformation2>>,
	#[serde(rename = "EmplngCpny", skip_serializing_if = "Option::is_none")]
	pub emplng_cpny: Option<Max140Text>,
	#[serde(rename = "BizFctn", skip_serializing_if = "Option::is_none")]
	pub biz_fctn: Option<Max35Text>,
	#[serde(rename = "PltclyXpsdPrsn", skip_serializing_if = "Option::is_none")]
	pub pltcly_xpsd_prsn: Option<PoliticallyExposedPerson1>,
	#[serde(rename = "DthDt", skip_serializing_if = "Option::is_none")]
	pub dth_dt: Option<String>,
	#[serde(rename = "CvlSts", skip_serializing_if = "Option::is_none")]
	pub cvl_sts: Option<CivilStatus1Choice>,
	#[serde(rename = "EdctnLvl", skip_serializing_if = "Option::is_none")]
	pub edctn_lvl: Option<Max35Text>,
	#[serde(rename = "FmlyInf", skip_serializing_if = "Option::is_none")]
	pub fmly_inf: Option<PersonalInformation1>,
	#[serde(rename = "GDPRData", skip_serializing_if = "Option::is_none")]
	pub gdpr_data: Option<Vec<GDPRData1>>,
}

impl IndividualPerson37 {
	pub fn validate(&self) -> bool {
		if let Some(ref nm_prfx_value) = self.nm_prfx { if !nm_prfx_value.validate() { return false; } }
		if let Some(ref gvn_nm_value) = self.gvn_nm { if !gvn_nm_value.validate() { return false; } }
		if let Some(ref mddl_nm_value) = self.mddl_nm { if !mddl_nm_value.validate() { return false; } }
		if !self.nm.validate() { return false }
		if let Some(ref nm_sfx_value) = self.nm_sfx { if !nm_sfx_value.validate() { return false; } }
		if let Some(ref gndr_value) = self.gndr { if !gndr_value.validate() { return false; } }
		if let Some(ref ctry_of_birth_value) = self.ctry_of_birth { if !ctry_of_birth_value.validate() { return false; } }
		if let Some(ref prvc_of_birth_value) = self.prvc_of_birth { if !prvc_of_birth_value.validate() { return false; } }
		if let Some(ref city_of_birth_value) = self.city_of_birth { if !city_of_birth_value.validate() { return false; } }
		if let Some(ref prfssn_value) = self.prfssn { if !prfssn_value.validate() { return false; } }
		for item in &self.pstl_adr { if !item.validate() { return false; } }
		if let Some(ref ctznsh_vec) = self.ctznsh { for item in ctznsh_vec { if !item.validate() { return false; } } }
		if let Some(ref emplng_cpny_value) = self.emplng_cpny { if !emplng_cpny_value.validate() { return false; } }
		if let Some(ref biz_fctn_value) = self.biz_fctn { if !biz_fctn_value.validate() { return false; } }
		if let Some(ref pltcly_xpsd_prsn_value) = self.pltcly_xpsd_prsn { if !pltcly_xpsd_prsn_value.validate() { return false; } }
		if let Some(ref cvl_sts_value) = self.cvl_sts { if !cvl_sts_value.validate() { return false; } }
		if let Some(ref edctn_lvl_value) = self.edctn_lvl { if !edctn_lvl_value.validate() { return false; } }
		if let Some(ref fmly_inf_value) = self.fmly_inf { if !fmly_inf_value.validate() { return false; } }
		if let Some(ref gdpr_data_vec) = self.gdpr_data { for item in gdpr_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// InformationDistribution1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InformationDistribution1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InformationDistribution2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl InformationDistribution1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// InformationDistribution2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InformationDistribution2Code {
	#[default]
	#[serde(rename = "ELEC")]
	CodeELEC,
	#[serde(rename = "NONE")]
	CodeNONE,
	#[serde(rename = "PAPR")]
	CodePAPR,
}

impl InformationDistribution2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// InitialAmount1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InitialAmount1Choice {
	#[serde(rename = "InitlNbOfInstlmts", skip_serializing_if = "Option::is_none")]
	pub initl_nb_of_instlmts: Option<f64>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
}

impl InitialAmount1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref amt_value) = self.amt { if !amt_value.validate() { return false; } }
		return true
	}
}


// Insurance1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Insurance1Code {
	#[default]
	#[serde(rename = "LIFE")]
	CodeLIFE,
	#[serde(rename = "PDIS")]
	CodePDIS,
}

impl Insurance1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// InsuranceType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InsuranceType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Insurance1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl InsuranceType2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// Intermediary46 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Intermediary46 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification177Choice,
	#[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
	pub lgl_ntty_idr: Option<LEIIdentifier>,
	#[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
	pub acct: Option<Account32>,
	#[serde(rename = "WvdTrlrComssnInd", skip_serializing_if = "Option::is_none")]
	pub wvd_trlr_comssn_ind: Option<bool>,
	#[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
	pub role: Option<PartyRole2Choice>,
	#[serde(rename = "PmryComAdr", skip_serializing_if = "Option::is_none")]
	pub pmry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[serde(rename = "ScndryComAdr", skip_serializing_if = "Option::is_none")]
	pub scndry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress4>,
}

impl Intermediary46 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref lgl_ntty_idr_value) = self.lgl_ntty_idr { if !lgl_ntty_idr_value.validate() { return false; } }
		if let Some(ref acct_value) = self.acct { if !acct_value.validate() { return false; } }
		if let Some(ref role_value) = self.role { if !role_value.validate() { return false; } }
		if let Some(ref pmry_com_adr_vec) = self.pmry_com_adr { for item in pmry_com_adr_vec { if !item.validate() { return false; } } }
		if let Some(ref scndry_com_adr_vec) = self.scndry_com_adr { for item in scndry_com_adr_vec { if !item.validate() { return false; } } }
		if let Some(ref nm_and_adr_value) = self.nm_and_adr { if !nm_and_adr_value.validate() { return false; } }
		return true
	}
}


// InvestmentAccount74 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentAccount74 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "AcctSts", skip_serializing_if = "Option::is_none")]
	pub acct_sts: Option<AccountStatus2>,
	#[serde(rename = "BlckdSts", skip_serializing_if = "Option::is_none")]
	pub blckd_sts: Option<BlockedStatusReason2Choice>,
	#[serde(rename = "StsDt", skip_serializing_if = "Option::is_none")]
	pub sts_dt: Option<DateAndDateTime1Choice>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max35Text>,
	#[serde(rename = "Dsgnt", skip_serializing_if = "Option::is_none")]
	pub dsgnt: Option<Max35Text>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<AccountType2Choice>,
	#[serde(rename = "OwnrshTp", skip_serializing_if = "Option::is_none")]
	pub ownrsh_tp: Option<OwnershipType2Choice>,
	#[serde(rename = "TaxXmptn", skip_serializing_if = "Option::is_none")]
	pub tax_xmptn: Option<TaxExemptionReason2Choice>,
	#[serde(rename = "StmtFrqcy", skip_serializing_if = "Option::is_none")]
	pub stmt_frqcy: Option<StatementFrequencyReason2Choice>,
	#[serde(rename = "RefCcy", skip_serializing_if = "Option::is_none")]
	pub ref_ccy: Option<ActiveCurrencyCode>,
	#[serde(rename = "Lang", skip_serializing_if = "Option::is_none")]
	pub lang: Option<String>,
	#[serde(rename = "IncmPref", skip_serializing_if = "Option::is_none")]
	pub incm_pref: Option<IncomePreference2Code>,
	#[serde(rename = "RinvstmtDtls", skip_serializing_if = "Option::is_none")]
	pub rinvstmt_dtls: Option<Vec<Reinvestment4>>,
	#[serde(rename = "TaxWhldgMtd", skip_serializing_if = "Option::is_none")]
	pub tax_whldg_mtd: Option<TaxWithholdingMethod3Code>,
	#[serde(rename = "TaxRptg", skip_serializing_if = "Option::is_none")]
	pub tax_rptg: Option<Vec<TaxReporting3>>,
	#[serde(rename = "LttrInttDtls", skip_serializing_if = "Option::is_none")]
	pub lttr_intt_dtls: Option<LetterIntent1>,
	#[serde(rename = "AcmltnRghtRef", skip_serializing_if = "Option::is_none")]
	pub acmltn_rght_ref: Option<Max35Text>,
	#[serde(rename = "ReqrdSgntriesNb", skip_serializing_if = "Option::is_none")]
	pub reqrd_sgntries_nb: Option<f64>,
	#[serde(rename = "FndFmlyNm", skip_serializing_if = "Option::is_none")]
	pub fnd_fmly_nm: Option<Max350Text>,
	#[serde(rename = "FinInstrmDtls", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_dtls: Option<Vec<FinancialInstrument87>>,
	#[serde(rename = "RndgDtls", skip_serializing_if = "Option::is_none")]
	pub rndg_dtls: Option<RoundingParameters1>,
	#[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
	pub acct_svcr: Option<PartyIdentification125Choice>,
	#[serde(rename = "AcctUsgTp", skip_serializing_if = "Option::is_none")]
	pub acct_usg_tp: Option<AccountUsageType2Choice>,
	#[serde(rename = "FrgnStsCertfctn", skip_serializing_if = "Option::is_none")]
	pub frgn_sts_certfctn: Option<Provided1Code>,
	#[serde(rename = "AcctSgntrDtTm", skip_serializing_if = "Option::is_none")]
	pub acct_sgntr_dt_tm: Option<DateAndDateTime1Choice>,
	#[serde(rename = "TxChanlTp", skip_serializing_if = "Option::is_none")]
	pub tx_chanl_tp: Option<TransactionChannelType1Choice>,
	#[serde(rename = "InvstmtAcctCtgy", skip_serializing_if = "Option::is_none")]
	pub invstmt_acct_ctgy: Option<InvestmentAccountCategory1Choice>,
	#[serde(rename = "Pldgg", skip_serializing_if = "Option::is_none")]
	pub pldgg: Option<Eligible1Code>,
	#[serde(rename = "Coll", skip_serializing_if = "Option::is_none")]
	pub coll: Option<Collateral1Code>,
	#[serde(rename = "ThrdPtyRghts", skip_serializing_if = "Option::is_none")]
	pub thrd_pty_rghts: Option<ThirdPartyRights2>,
	#[serde(rename = "PwrOfAttnyLvlOfCtrl", skip_serializing_if = "Option::is_none")]
	pub pwr_of_attny_lvl_of_ctrl: Option<LevelOfControl1Choice>,
	#[serde(rename = "AcctgSts", skip_serializing_if = "Option::is_none")]
	pub acctg_sts: Option<AccountingStatus1Choice>,
	#[serde(rename = "OpngDt", skip_serializing_if = "Option::is_none")]
	pub opng_dt: Option<DateAndDateTime1Choice>,
	#[serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none")]
	pub clsg_dt: Option<DateAndDateTime1Choice>,
	#[serde(rename = "NegInd", skip_serializing_if = "Option::is_none")]
	pub neg_ind: Option<bool>,
	#[serde(rename = "PrcgOrdr", skip_serializing_if = "Option::is_none")]
	pub prcg_ordr: Option<PositionEffect3Code>,
	#[serde(rename = "Lblty", skip_serializing_if = "Option::is_none")]
	pub lblty: Option<Liability1Choice>,
	#[serde(rename = "InvstrPrfl", skip_serializing_if = "Option::is_none")]
	pub invstr_prfl: Option<Vec<InvestorProfile2>>,
	#[serde(rename = "FsclYr", skip_serializing_if = "Option::is_none")]
	pub fscl_yr: Option<FiscalYear1Choice>,
}

impl InvestmentAccount74 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref acct_sts_value) = self.acct_sts { if !acct_sts_value.validate() { return false; } }
		if let Some(ref blckd_sts_value) = self.blckd_sts { if !blckd_sts_value.validate() { return false; } }
		if let Some(ref sts_dt_value) = self.sts_dt { if !sts_dt_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref dsgnt_value) = self.dsgnt { if !dsgnt_value.validate() { return false; } }
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref ownrsh_tp_value) = self.ownrsh_tp { if !ownrsh_tp_value.validate() { return false; } }
		if let Some(ref tax_xmptn_value) = self.tax_xmptn { if !tax_xmptn_value.validate() { return false; } }
		if let Some(ref stmt_frqcy_value) = self.stmt_frqcy { if !stmt_frqcy_value.validate() { return false; } }
		if let Some(ref ref_ccy_value) = self.ref_ccy { if !ref_ccy_value.validate() { return false; } }
		if let Some(ref incm_pref_value) = self.incm_pref { if !incm_pref_value.validate() { return false; } }
		if let Some(ref rinvstmt_dtls_vec) = self.rinvstmt_dtls { for item in rinvstmt_dtls_vec { if !item.validate() { return false; } } }
		if let Some(ref tax_whldg_mtd_value) = self.tax_whldg_mtd { if !tax_whldg_mtd_value.validate() { return false; } }
		if let Some(ref tax_rptg_vec) = self.tax_rptg { for item in tax_rptg_vec { if !item.validate() { return false; } } }
		if let Some(ref lttr_intt_dtls_value) = self.lttr_intt_dtls { if !lttr_intt_dtls_value.validate() { return false; } }
		if let Some(ref acmltn_rght_ref_value) = self.acmltn_rght_ref { if !acmltn_rght_ref_value.validate() { return false; } }
		if let Some(ref fnd_fmly_nm_value) = self.fnd_fmly_nm { if !fnd_fmly_nm_value.validate() { return false; } }
		if let Some(ref fin_instrm_dtls_vec) = self.fin_instrm_dtls { for item in fin_instrm_dtls_vec { if !item.validate() { return false; } } }
		if let Some(ref rndg_dtls_value) = self.rndg_dtls { if !rndg_dtls_value.validate() { return false; } }
		if let Some(ref acct_svcr_value) = self.acct_svcr { if !acct_svcr_value.validate() { return false; } }
		if let Some(ref acct_usg_tp_value) = self.acct_usg_tp { if !acct_usg_tp_value.validate() { return false; } }
		if let Some(ref frgn_sts_certfctn_value) = self.frgn_sts_certfctn { if !frgn_sts_certfctn_value.validate() { return false; } }
		if let Some(ref acct_sgntr_dt_tm_value) = self.acct_sgntr_dt_tm { if !acct_sgntr_dt_tm_value.validate() { return false; } }
		if let Some(ref tx_chanl_tp_value) = self.tx_chanl_tp { if !tx_chanl_tp_value.validate() { return false; } }
		if let Some(ref invstmt_acct_ctgy_value) = self.invstmt_acct_ctgy { if !invstmt_acct_ctgy_value.validate() { return false; } }
		if let Some(ref pldgg_value) = self.pldgg { if !pldgg_value.validate() { return false; } }
		if let Some(ref coll_value) = self.coll { if !coll_value.validate() { return false; } }
		if let Some(ref thrd_pty_rghts_value) = self.thrd_pty_rghts { if !thrd_pty_rghts_value.validate() { return false; } }
		if let Some(ref pwr_of_attny_lvl_of_ctrl_value) = self.pwr_of_attny_lvl_of_ctrl { if !pwr_of_attny_lvl_of_ctrl_value.validate() { return false; } }
		if let Some(ref acctg_sts_value) = self.acctg_sts { if !acctg_sts_value.validate() { return false; } }
		if let Some(ref opng_dt_value) = self.opng_dt { if !opng_dt_value.validate() { return false; } }
		if let Some(ref clsg_dt_value) = self.clsg_dt { if !clsg_dt_value.validate() { return false; } }
		if let Some(ref prcg_ordr_value) = self.prcg_ordr { if !prcg_ordr_value.validate() { return false; } }
		if let Some(ref lblty_value) = self.lblty { if !lblty_value.validate() { return false; } }
		if let Some(ref invstr_prfl_vec) = self.invstr_prfl { for item in invstr_prfl_vec { if !item.validate() { return false; } } }
		if let Some(ref fscl_yr_value) = self.fscl_yr { if !fscl_yr_value.validate() { return false; } }
		return true
	}
}


// InvestmentAccountCategory1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentAccountCategory1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestmentAccountCategory1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl InvestmentAccountCategory1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// InvestmentAccountCategory1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestmentAccountCategory1Code {
	#[default]
	#[serde(rename = "MAND")]
	CodeMAND,
	#[serde(rename = "RETA")]
	CodeRETA,
}

impl InvestmentAccountCategory1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// InvestmentAccountOwnershipInformation16 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentAccountOwnershipInformation16 {
	#[serde(rename = "Pty")]
	pub pty: Party47Choice,
	#[serde(rename = "MnyLndrgChck", skip_serializing_if = "Option::is_none")]
	pub mny_lndrg_chck: Option<MoneyLaunderingCheck1Choice>,
	#[serde(rename = "InvstrPrflVldtn", skip_serializing_if = "Option::is_none")]
	pub invstr_prfl_vldtn: Option<Vec<PartyProfileInformation5>>,
	#[serde(rename = "OwnrshBnfcryRate", skip_serializing_if = "Option::is_none")]
	pub ownrsh_bnfcry_rate: Option<OwnershipBeneficiaryRate1>,
	#[serde(rename = "ClntId", skip_serializing_if = "Option::is_none")]
	pub clnt_id: Option<Max35Text>,
	#[serde(rename = "FsclXmptn", skip_serializing_if = "Option::is_none")]
	pub fscl_xmptn: Option<bool>,
	#[serde(rename = "SgntryRghtInd", skip_serializing_if = "Option::is_none")]
	pub sgntry_rght_ind: Option<bool>,
	#[serde(rename = "MiFIDClssfctn", skip_serializing_if = "Option::is_none")]
	pub mi_fid_clssfctn: Option<MiFIDClassification1>,
	#[serde(rename = "Ntfctn", skip_serializing_if = "Option::is_none")]
	pub ntfctn: Option<Vec<Notification2>>,
	#[serde(rename = "FATCAFormTp", skip_serializing_if = "Option::is_none")]
	pub fatca_form_tp: Option<Vec<FATCAForm1Choice>>,
	#[serde(rename = "FATCASts", skip_serializing_if = "Option::is_none")]
	pub fatca_sts: Option<Vec<FATCAStatus2>>,
	#[serde(rename = "FATCARptgDt", skip_serializing_if = "Option::is_none")]
	pub fatca_rptg_dt: Option<String>,
	#[serde(rename = "CRSFormTp", skip_serializing_if = "Option::is_none")]
	pub crs_form_tp: Option<Vec<CRSForm1Choice>>,
	#[serde(rename = "CRSSts", skip_serializing_if = "Option::is_none")]
	pub crs_sts: Option<Vec<CRSStatus4>>,
	#[serde(rename = "CRSRptgDt", skip_serializing_if = "Option::is_none")]
	pub crs_rptg_dt: Option<String>,
	#[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
	pub othr_id: Option<Vec<GenericIdentification82>>,
	#[serde(rename = "TaxXmptn", skip_serializing_if = "Option::is_none")]
	pub tax_xmptn: Option<TaxExemptionReason2Choice>,
	#[serde(rename = "TaxRptg", skip_serializing_if = "Option::is_none")]
	pub tax_rptg: Option<Vec<TaxReporting3>>,
	#[serde(rename = "Lang", skip_serializing_if = "Option::is_none")]
	pub lang: Option<String>,
	#[serde(rename = "MailTp", skip_serializing_if = "Option::is_none")]
	pub mail_tp: Option<MailType1Choice>,
	#[serde(rename = "CtryAndResdtlSts", skip_serializing_if = "Option::is_none")]
	pub ctry_and_resdtl_sts: Option<CountryAndResidentialStatusType2>,
	#[serde(rename = "MntryWlth", skip_serializing_if = "Option::is_none")]
	pub mntry_wlth: Option<DateAndAmount1>,
	#[serde(rename = "EqtyVal", skip_serializing_if = "Option::is_none")]
	pub eqty_val: Option<DateAndAmount1>,
	#[serde(rename = "WorkgCptl", skip_serializing_if = "Option::is_none")]
	pub workg_cptl: Option<DateAndAmount1>,
	#[serde(rename = "CpnyLk", skip_serializing_if = "Option::is_none")]
	pub cpny_lk: Option<CompanyLink1Choice>,
	#[serde(rename = "ElctrncMlngSvcRef", skip_serializing_if = "Option::is_none")]
	pub elctrnc_mlng_svc_ref: Option<Max350Text>,
	#[serde(rename = "PmryComAdr", skip_serializing_if = "Option::is_none")]
	pub pmry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[serde(rename = "ScndryComAdr", skip_serializing_if = "Option::is_none")]
	pub scndry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[serde(rename = "AddtlRgltryInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rgltry_inf: Option<RegulatoryInformation1>,
	#[serde(rename = "AcctgSts", skip_serializing_if = "Option::is_none")]
	pub acctg_sts: Option<AccountingStatus1Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditiononalInformation13>>,
	#[serde(rename = "CtrlgPty", skip_serializing_if = "Option::is_none")]
	pub ctrlg_pty: Option<bool>,
}

impl InvestmentAccountOwnershipInformation16 {
	pub fn validate(&self) -> bool {
		if !self.pty.validate() { return false }
		if let Some(ref mny_lndrg_chck_value) = self.mny_lndrg_chck { if !mny_lndrg_chck_value.validate() { return false; } }
		if let Some(ref invstr_prfl_vldtn_vec) = self.invstr_prfl_vldtn { for item in invstr_prfl_vldtn_vec { if !item.validate() { return false; } } }
		if let Some(ref ownrsh_bnfcry_rate_value) = self.ownrsh_bnfcry_rate { if !ownrsh_bnfcry_rate_value.validate() { return false; } }
		if let Some(ref clnt_id_value) = self.clnt_id { if !clnt_id_value.validate() { return false; } }
		if let Some(ref mi_fid_clssfctn_value) = self.mi_fid_clssfctn { if !mi_fid_clssfctn_value.validate() { return false; } }
		if let Some(ref ntfctn_vec) = self.ntfctn { for item in ntfctn_vec { if !item.validate() { return false; } } }
		if let Some(ref fatca_form_tp_vec) = self.fatca_form_tp { for item in fatca_form_tp_vec { if !item.validate() { return false; } } }
		if let Some(ref fatca_sts_vec) = self.fatca_sts { for item in fatca_sts_vec { if !item.validate() { return false; } } }
		if let Some(ref crs_form_tp_vec) = self.crs_form_tp { for item in crs_form_tp_vec { if !item.validate() { return false; } } }
		if let Some(ref crs_sts_vec) = self.crs_sts { for item in crs_sts_vec { if !item.validate() { return false; } } }
		if let Some(ref othr_id_vec) = self.othr_id { for item in othr_id_vec { if !item.validate() { return false; } } }
		if let Some(ref tax_xmptn_value) = self.tax_xmptn { if !tax_xmptn_value.validate() { return false; } }
		if let Some(ref tax_rptg_vec) = self.tax_rptg { for item in tax_rptg_vec { if !item.validate() { return false; } } }
		if let Some(ref mail_tp_value) = self.mail_tp { if !mail_tp_value.validate() { return false; } }
		if let Some(ref ctry_and_resdtl_sts_value) = self.ctry_and_resdtl_sts { if !ctry_and_resdtl_sts_value.validate() { return false; } }
		if let Some(ref mntry_wlth_value) = self.mntry_wlth { if !mntry_wlth_value.validate() { return false; } }
		if let Some(ref eqty_val_value) = self.eqty_val { if !eqty_val_value.validate() { return false; } }
		if let Some(ref workg_cptl_value) = self.workg_cptl { if !workg_cptl_value.validate() { return false; } }
		if let Some(ref cpny_lk_value) = self.cpny_lk { if !cpny_lk_value.validate() { return false; } }
		if let Some(ref elctrnc_mlng_svc_ref_value) = self.elctrnc_mlng_svc_ref { if !elctrnc_mlng_svc_ref_value.validate() { return false; } }
		if let Some(ref pmry_com_adr_vec) = self.pmry_com_adr { for item in pmry_com_adr_vec { if !item.validate() { return false; } } }
		if let Some(ref scndry_com_adr_vec) = self.scndry_com_adr { for item in scndry_com_adr_vec { if !item.validate() { return false; } } }
		if let Some(ref addtl_rgltry_inf_value) = self.addtl_rgltry_inf { if !addtl_rgltry_inf_value.validate() { return false; } }
		if let Some(ref acctg_sts_value) = self.acctg_sts { if !acctg_sts_value.validate() { return false; } }
		if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if !item.validate() { return false; } } }
		return true
	}
}


// InvestmentFundOrder4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentFundOrder4 {
	#[serde(rename = "OrdrRef", skip_serializing_if = "Option::is_none")]
	pub ordr_ref: Option<Max35Text>,
	#[serde(rename = "MstrRef", skip_serializing_if = "Option::is_none")]
	pub mstr_ref: Option<Max35Text>,
}

impl InvestmentFundOrder4 {
	pub fn validate(&self) -> bool {
		if let Some(ref ordr_ref_value) = self.ordr_ref { if !ordr_ref_value.validate() { return false; } }
		if let Some(ref mstr_ref_value) = self.mstr_ref { if !mstr_ref_value.validate() { return false; } }
		return true
	}
}


// InvestmentFundRole6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestmentFundRole6Code {
	#[default]
	#[serde(rename = "CACO")]
	CodeCACO,
	#[serde(rename = "CONC")]
	CodeCONC,
	#[serde(rename = "CUST")]
	CodeCUST,
	#[serde(rename = "DATP")]
	CodeDATP,
	#[serde(rename = "DIST")]
	CodeDIST,
	#[serde(rename = "FACT")]
	CodeFACT,
	#[serde(rename = "FIAD")]
	CodeFIAD,
	#[serde(rename = "FIAG")]
	CodeFIAG,
	#[serde(rename = "FMCO")]
	CodeFMCO,
	#[serde(rename = "FNBR")]
	CodeFNBR,
	#[serde(rename = "FTAG")]
	CodeFTAG,
	#[serde(rename = "INTR")]
	CodeINTR,
	#[serde(rename = "INVE")]
	CodeINVE,
	#[serde(rename = "INVS")]
	CodeINVS,
	#[serde(rename = "PAYI")]
	CodePAYI,
	#[serde(rename = "REGI")]
	CodeREGI,
	#[serde(rename = "TRAG")]
	CodeTRAG,
	#[serde(rename = "TRAN")]
	CodeTRAN,
}

impl InvestmentFundRole6Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// InvestmentFundRole7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestmentFundRole7Code {
	#[default]
	#[serde(rename = "CONC")]
	CodeCONC,
	#[serde(rename = "DIST")]
	CodeDIST,
	#[serde(rename = "FMCO")]
	CodeFMCO,
	#[serde(rename = "INTR")]
	CodeINTR,
	#[serde(rename = "PAYI")]
	CodePAYI,
	#[serde(rename = "TRAG")]
	CodeTRAG,
	#[serde(rename = "CUST")]
	CodeCUST,
	#[serde(rename = "CACO")]
	CodeCACO,
	#[serde(rename = "FACT")]
	CodeFACT,
	#[serde(rename = "INVE")]
	CodeINVE,
	#[serde(rename = "INVS")]
	CodeINVS,
}

impl InvestmentFundRole7Code {
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


// InvestmentPlan17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentPlan17 {
	#[serde(rename = "Frqcy")]
	pub frqcy: Frequency20Choice,
	#[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
	pub start_dt: Option<String>,
	#[serde(rename = "EndDt", skip_serializing_if = "Option::is_none")]
	pub end_dt: Option<String>,
	#[serde(rename = "Qty")]
	pub qty: UnitsOrAmount1Choice,
	#[serde(rename = "GrssAmtInd", skip_serializing_if = "Option::is_none")]
	pub grss_amt_ind: Option<bool>,
	#[serde(rename = "IncmPref", skip_serializing_if = "Option::is_none")]
	pub incm_pref: Option<IncomePreference2Code>,
	#[serde(rename = "InitlAmt", skip_serializing_if = "Option::is_none")]
	pub initl_amt: Option<InitialAmount1Choice>,
	#[serde(rename = "TtlNbOfInstlmts", skip_serializing_if = "Option::is_none")]
	pub ttl_nb_of_instlmts: Option<f64>,
	#[serde(rename = "RndgDrctn", skip_serializing_if = "Option::is_none")]
	pub rndg_drctn: Option<RoundingDirection1Code>,
	#[serde(rename = "SctyDtls")]
	pub scty_dtls: Vec<Repartition6>,
	#[serde(rename = "CshSttlm", skip_serializing_if = "Option::is_none")]
	pub csh_sttlm: Option<Vec<CashSettlement3>>,
	#[serde(rename = "CtrctRef", skip_serializing_if = "Option::is_none")]
	pub ctrct_ref: Option<Max35Text>,
	#[serde(rename = "RltdCtrctRef", skip_serializing_if = "Option::is_none")]
	pub rltd_ctrct_ref: Option<Max35Text>,
	#[serde(rename = "PdctId", skip_serializing_if = "Option::is_none")]
	pub pdct_id: Option<Max35Text>,
	#[serde(rename = "SLAChrgAndComssnRef", skip_serializing_if = "Option::is_none")]
	pub sla_chrg_and_comssn_ref: Option<Max35Text>,
	#[serde(rename = "InsrncCover", skip_serializing_if = "Option::is_none")]
	pub insrnc_cover: Option<InsuranceType2Choice>,
	#[serde(rename = "PlanSts", skip_serializing_if = "Option::is_none")]
	pub plan_sts: Option<PlanStatus2Choice>,
	#[serde(rename = "InstlmtMgrRole", skip_serializing_if = "Option::is_none")]
	pub instlmt_mgr_role: Option<PartyRole4Choice>,
}

impl InvestmentPlan17 {
	pub fn validate(&self) -> bool {
		if !self.frqcy.validate() { return false }
		if !self.qty.validate() { return false }
		if let Some(ref incm_pref_value) = self.incm_pref { if !incm_pref_value.validate() { return false; } }
		if let Some(ref initl_amt_value) = self.initl_amt { if !initl_amt_value.validate() { return false; } }
		if let Some(ref rndg_drctn_value) = self.rndg_drctn { if !rndg_drctn_value.validate() { return false; } }
		for item in &self.scty_dtls { if !item.validate() { return false; } }
		if let Some(ref csh_sttlm_vec) = self.csh_sttlm { for item in csh_sttlm_vec { if !item.validate() { return false; } } }
		if let Some(ref ctrct_ref_value) = self.ctrct_ref { if !ctrct_ref_value.validate() { return false; } }
		if let Some(ref rltd_ctrct_ref_value) = self.rltd_ctrct_ref { if !rltd_ctrct_ref_value.validate() { return false; } }
		if let Some(ref pdct_id_value) = self.pdct_id { if !pdct_id_value.validate() { return false; } }
		if let Some(ref sla_chrg_and_comssn_ref_value) = self.sla_chrg_and_comssn_ref { if !sla_chrg_and_comssn_ref_value.validate() { return false; } }
		if let Some(ref insrnc_cover_value) = self.insrnc_cover { if !insrnc_cover_value.validate() { return false; } }
		if let Some(ref plan_sts_value) = self.plan_sts { if !plan_sts_value.validate() { return false; } }
		if let Some(ref instlmt_mgr_role_value) = self.instlmt_mgr_role { if !instlmt_mgr_role_value.validate() { return false; } }
		return true
	}
}


// InvestorProfile2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorProfile2 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ProfileType1Choice>,
	#[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
	pub sts: Option<InvestorProfileStatus1Choice>,
	#[serde(rename = "Trsr", skip_serializing_if = "Option::is_none")]
	pub trsr: Option<TreasuryProfile1>,
	#[serde(rename = "HghFrqcyTradg", skip_serializing_if = "Option::is_none")]
	pub hgh_frqcy_tradg: Option<HighFrequencyTradingProfile1>,
	#[serde(rename = "MktMakr", skip_serializing_if = "Option::is_none")]
	pub mkt_makr: Option<MarketMakerProfile2>,
}

impl InvestorProfile2 {
	pub fn validate(&self) -> bool {
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref sts_value) = self.sts { if !sts_value.validate() { return false; } }
		if let Some(ref trsr_value) = self.trsr { if !trsr_value.validate() { return false; } }
		if let Some(ref hgh_frqcy_tradg_value) = self.hgh_frqcy_tradg { if !hgh_frqcy_tradg_value.validate() { return false; } }
		if let Some(ref mkt_makr_value) = self.mkt_makr { if !mkt_makr_value.validate() { return false; } }
		return true
	}
}


// InvestorProfileStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorProfileStatus1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestorProfileStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl InvestorProfileStatus1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// InvestorProfileStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestorProfileStatus1Code {
	#[default]
	#[serde(rename = "DISA")]
	CodeDISA,
	#[serde(rename = "DISG")]
	CodeDISG,
	#[serde(rename = "ENAB")]
	CodeENAB,
	#[serde(rename = "ENBG")]
	CodeENBG,
	#[serde(rename = "ADMI")]
	CodeADMI,
	#[serde(rename = "ANLY")]
	CodeANLY,
	#[serde(rename = "NAPP")]
	CodeNAPP,
	#[serde(rename = "PSUS")]
	CodePSUS,
	#[serde(rename = "PEND")]
	CodePEND,
	#[serde(rename = "SUPS")]
	CodeSUPS,
}

impl InvestorProfileStatus1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// IrishNSCIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct IrishNSCIdentifier {
	#[serde(rename = "$value")]
	pub irish_nsc_identifier: String,
}

impl IrishNSCIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("IE[0-9]{6,6}").unwrap();
		if !pattern.is_match(&self.irish_nsc_identifier) {
			return false
		}
		return true
	}
}


// ItalianDomesticIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ItalianDomesticIdentifier {
	#[serde(rename = "$value")]
	pub italian_domestic_identifier: String,
}

impl ItalianDomesticIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("IT[0-9]{10,10}").unwrap();
		if !pattern.is_match(&self.italian_domestic_identifier) {
			return false
		}
		return true
	}
}


// KYCCheckType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct KYCCheckType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<KnowYourCustomerCheckType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl KYCCheckType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// KnowYourCustomerCheckType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum KnowYourCustomerCheckType1Code {
	#[default]
	#[serde(rename = "ENHA")]
	CodeENHA,
	#[serde(rename = "ORDN")]
	CodeORDN,
	#[serde(rename = "SIMP")]
	CodeSIMP,
}

impl KnowYourCustomerCheckType1Code {
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


// LanguageCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LanguageCode {
	#[serde(rename = "$value")]
	pub language_code: String,
}

impl LanguageCode {
	pub fn validate(&self) -> bool {
		return true
	}
}


// LetterIntent1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LetterIntent1 {
	#[serde(rename = "LttrInttRef")]
	pub lttr_intt_ref: Max35Text,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
	#[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
	pub start_dt: Option<String>,
	#[serde(rename = "EndDt", skip_serializing_if = "Option::is_none")]
	pub end_dt: Option<String>,
}

impl LetterIntent1 {
	pub fn validate(&self) -> bool {
		if !self.lttr_intt_ref.validate() { return false }
		if let Some(ref amt_value) = self.amt { if !amt_value.validate() { return false; } }
		return true
	}
}


// LevelOfControl1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LevelOfControl1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<LevelOfControl1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl LevelOfControl1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// LevelOfControl1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum LevelOfControl1Code {
	#[default]
	#[serde(rename = "TRAN")]
	CodeTRAN,
	#[serde(rename = "VIEW")]
	CodeVIEW,
}

impl LevelOfControl1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Liability1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Liability1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Liability1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl Liability1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// Liability1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Liability1Code {
	#[default]
	#[serde(rename = "INVE")]
	CodeINVE,
	#[serde(rename = "BROK")]
	CodeBROK,
}

impl Liability1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MICIdentifier {
	#[serde(rename = "$value")]
	pub mic_identifier: String,
}

impl MICIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
		if !pattern.is_match(&self.mic_identifier) {
			return false
		}
		return true
	}
}


// MailType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MailType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<MailType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl MailType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// MailType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum MailType1Code {
	#[default]
	#[serde(rename = "AIRM")]
	CodeAIRM,
	#[serde(rename = "ORDM")]
	CodeORDM,
	#[serde(rename = "REGM")]
	CodeREGM,
}

impl MailType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// MarketMakerProfile2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketMakerProfile2 {
	#[serde(rename = "CtrctPrd", skip_serializing_if = "Option::is_none")]
	pub ctrct_prd: Option<DateTimePeriod2>,
	#[serde(rename = "Cmplc", skip_serializing_if = "Option::is_none")]
	pub cmplc: Option<bool>,
	#[serde(rename = "MaxSprd", skip_serializing_if = "Option::is_none")]
	pub max_sprd: Option<f64>,
	#[serde(rename = "Dscnt", skip_serializing_if = "Option::is_none")]
	pub dscnt: Option<f64>,
}

impl MarketMakerProfile2 {
	pub fn validate(&self) -> bool {
		if let Some(ref ctrct_prd_value) = self.ctrct_prd { if !ctrct_prd_value.validate() { return false; } }
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


// Max10Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max10Text {
	#[serde(rename = "$value")]
	pub max10_text: String,
}

impl Max10Text {
	pub fn validate(&self) -> bool {
		if self.max10_text.chars().count() < 1 {
			return false
		}
		if self.max10_text.chars().count() > 10 {
			return false
		}
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


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max256Text {
	#[serde(rename = "$value")]
	pub max256_text: String,
}

impl Max256Text {
	pub fn validate(&self) -> bool {
		if self.max256_text.chars().count() < 1 {
			return false
		}
		if self.max256_text.chars().count() > 256 {
			return false
		}
		return true
	}
}


// Max34Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max34Text {
	#[serde(rename = "$value")]
	pub max34_text: String,
}

impl Max34Text {
	pub fn validate(&self) -> bool {
		if self.max34_text.chars().count() < 1 {
			return false
		}
		if self.max34_text.chars().count() > 34 {
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


// Max3Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max3Text {
	#[serde(rename = "$value")]
	pub max3_text: String,
}

impl Max3Text {
	pub fn validate(&self) -> bool {
		if self.max3_text.chars().count() < 1 {
			return false
		}
		if self.max3_text.chars().count() > 3 {
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


// MiFIDClassification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MiFIDClassification1 {
	#[serde(rename = "Clssfctn")]
	pub clssfctn: OrderOriginatorEligibility1Code,
	#[serde(rename = "Nrrtv", skip_serializing_if = "Option::is_none")]
	pub nrrtv: Option<Max350Text>,
}

impl MiFIDClassification1 {
	pub fn validate(&self) -> bool {
		if !self.clssfctn.validate() { return false }
		if let Some(ref nrrtv_value) = self.nrrtv { if !nrrtv_value.validate() { return false; } }
		return true
	}
}


// MoneyLaunderingCheck1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MoneyLaunderingCheck1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<MoneyLaunderingCheck1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl MoneyLaunderingCheck1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// MoneyLaunderingCheck1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum MoneyLaunderingCheck1Code {
	#[default]
	#[serde(rename = "PASS")]
	CodePASS,
	#[serde(rename = "NOTC")]
	CodeNOTC,
	#[serde(rename = "EXEM")]
	CodeEXEM,
	#[serde(rename = "CLMO")]
	CodeCLMO,
	#[serde(rename = "AUTH")]
	CodeAUTH,
	#[serde(rename = "POEP")]
	CodePOEP,
}

impl MoneyLaunderingCheck1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// NameAndAddress4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress4 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
	#[serde(rename = "Adr")]
	pub adr: PostalAddress1,
}

impl NameAndAddress4 {
	pub fn validate(&self) -> bool {
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if !self.adr.validate() { return false }
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


// NamePrefix1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NamePrefix1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<NamePrefix1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl NamePrefix1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// NamePrefix1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NamePrefix1Code {
	#[default]
	#[serde(rename = "DOCT")]
	CodeDOCT,
	#[serde(rename = "MIST")]
	CodeMIST,
	#[serde(rename = "MISS")]
	CodeMISS,
	#[serde(rename = "MADM")]
	CodeMADM,
}

impl NamePrefix1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// NationalityCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NationalityCode {
	#[serde(rename = "$value")]
	pub nationality_code: String,
}

impl NationalityCode {
	pub fn validate(&self) -> bool {
		return true
	}
}


// NewIssueAllocation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewIssueAllocation2 {
	#[serde(rename = "Rstrctd")]
	pub rstrctd: bool,
	#[serde(rename = "XmptPrsnRsn", skip_serializing_if = "Option::is_none")]
	pub xmpt_prsn_rsn: Option<Max350Text>,
	#[serde(rename = "DeMnms", skip_serializing_if = "Option::is_none")]
	pub de_mnms: Option<DeMinimus1Choice>,
}

impl NewIssueAllocation2 {
	pub fn validate(&self) -> bool {
		if let Some(ref xmpt_prsn_rsn_value) = self.xmpt_prsn_rsn { if !xmpt_prsn_rsn_value.validate() { return false; } }
		if let Some(ref de_mnms_value) = self.de_mnms { if !de_mnms_value.validate() { return false; } }
		return true
	}
}


// NewZealandNCCIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NewZealandNCCIdentifier {
	#[serde(rename = "$value")]
	pub new_zealand_ncc_identifier: String,
}

impl NewZealandNCCIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("NZ[0-9]{6,6}").unwrap();
		if !pattern.is_match(&self.new_zealand_ncc_identifier) {
			return false
		}
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


// Notification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Notification2 {
	#[serde(rename = "NtfctnTp")]
	pub ntfctn_tp: Max35Text,
	#[serde(rename = "Reqrd")]
	pub reqrd: bool,
	#[serde(rename = "DstrbtnTp", skip_serializing_if = "Option::is_none")]
	pub dstrbtn_tp: Option<InformationDistribution1Choice>,
}

impl Notification2 {
	pub fn validate(&self) -> bool {
		if !self.ntfctn_tp.validate() { return false }
		if let Some(ref dstrbtn_tp_value) = self.dstrbtn_tp { if !dstrbtn_tp_value.validate() { return false; } }
		return true
	}
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}

impl Number {
	pub fn validate(&self) -> bool {
		return true
	}
}


// OperationalStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OperationalStatus1Code {
	#[default]
	#[serde(rename = "ENAB")]
	CodeENAB,
	#[serde(rename = "SPEC")]
	CodeSPEC,
}

impl OperationalStatus1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// OrderOriginatorEligibility1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OrderOriginatorEligibility1Code {
	#[default]
	#[serde(rename = "ELIG")]
	CodeELIG,
	#[serde(rename = "RETL")]
	CodeRETL,
	#[serde(rename = "PROF")]
	CodePROF,
}

impl OrderOriginatorEligibility1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Organisation23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Organisation23 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
	pub shrt_nm: Option<Max35Text>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Vec<PostalAddress21>,
}

impl Organisation23 {
	pub fn validate(&self) -> bool {
		if !self.nm.validate() { return false }
		if let Some(ref shrt_nm_value) = self.shrt_nm { if !shrt_nm_value.validate() { return false; } }
		for item in &self.pstl_adr { if !item.validate() { return false; } }
		return true
	}
}


// Organisation39 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Organisation39 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
	#[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
	pub shrt_nm: Option<Max35Text>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<PartyIdentification177Choice>,
	#[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
	pub lgl_ntty_idr: Option<LEIIdentifier>,
	#[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
	pub purp: Option<Max35Text>,
	#[serde(rename = "RegnCtry", skip_serializing_if = "Option::is_none")]
	pub regn_ctry: Option<CountryCode>,
	#[serde(rename = "RegnDt", skip_serializing_if = "Option::is_none")]
	pub regn_dt: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<Vec<PostalAddress21>>,
	#[serde(rename = "TpOfOrg", skip_serializing_if = "Option::is_none")]
	pub tp_of_org: Option<OrganisationType1Choice>,
	#[serde(rename = "PlcOfListg", skip_serializing_if = "Option::is_none")]
	pub plc_of_listg: Option<Vec<MICIdentifier>>,
}

impl Organisation39 {
	pub fn validate(&self) -> bool {
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref shrt_nm_value) = self.shrt_nm { if !shrt_nm_value.validate() { return false; } }
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref lgl_ntty_idr_value) = self.lgl_ntty_idr { if !lgl_ntty_idr_value.validate() { return false; } }
		if let Some(ref purp_value) = self.purp { if !purp_value.validate() { return false; } }
		if let Some(ref regn_ctry_value) = self.regn_ctry { if !regn_ctry_value.validate() { return false; } }
		if let Some(ref pstl_adr_vec) = self.pstl_adr { for item in pstl_adr_vec { if !item.validate() { return false; } } }
		if let Some(ref tp_of_org_value) = self.tp_of_org { if !tp_of_org_value.validate() { return false; } }
		if let Some(ref plc_of_listg_vec) = self.plc_of_listg { for item in plc_of_listg_vec { if !item.validate() { return false; } } }
		return true
	}
}


// OrganisationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<OrganisationType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl OrganisationType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// OrganisationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OrganisationType1Code {
	#[default]
	#[serde(rename = "IFUN")]
	CodeIFUN,
	#[serde(rename = "PRIV")]
	CodePRIV,
	#[serde(rename = "PUBL")]
	CodePUBL,
	#[serde(rename = "PFUN")]
	CodePFUN,
}

impl OrganisationType1Code {
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


// OwnershipBeneficiaryRate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OwnershipBeneficiaryRate1 {
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "Frctn", skip_serializing_if = "Option::is_none")]
	pub frctn: Option<Max35Text>,
}

impl OwnershipBeneficiaryRate1 {
	pub fn validate(&self) -> bool {
		if let Some(ref frctn_value) = self.frctn { if !frctn_value.validate() { return false; } }
		return true
	}
}


// OwnershipType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OwnershipType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AccountOwnershipType4Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl OwnershipType2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// Party47Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party47Choice {
	#[serde(rename = "Org", skip_serializing_if = "Option::is_none")]
	pub org: Option<Organisation39>,
	#[serde(rename = "IndvPrsn", skip_serializing_if = "Option::is_none")]
	pub indv_prsn: Option<IndividualPerson37>,
}

impl Party47Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref org_value) = self.org { if !org_value.validate() { return false; } }
		if let Some(ref indv_prsn_value) = self.indv_prsn { if !indv_prsn_value.validate() { return false; } }
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


// PartyIdentification177Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification177Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification1>,
}

impl PartyIdentification177Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref any_bic_value) = self.any_bic { if !any_bic_value.validate() { return false; } }
		if let Some(ref prtry_id_value) = self.prtry_id { if !prtry_id_value.validate() { return false; } }
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


// PartyProfileInformation5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyProfileInformation5 {
	#[serde(rename = "CertfctnInd", skip_serializing_if = "Option::is_none")]
	pub certfctn_ind: Option<bool>,
	#[serde(rename = "VldtngPty", skip_serializing_if = "Option::is_none")]
	pub vldtng_pty: Option<Max140Text>,
	#[serde(rename = "ChckngPty", skip_serializing_if = "Option::is_none")]
	pub chckng_pty: Option<Max140Text>,
	#[serde(rename = "RspnsblPty", skip_serializing_if = "Option::is_none")]
	pub rspnsbl_pty: Option<Max140Text>,
	#[serde(rename = "CertTp", skip_serializing_if = "Option::is_none")]
	pub cert_tp: Option<CertificationType1Choice>,
	#[serde(rename = "ChckngDt", skip_serializing_if = "Option::is_none")]
	pub chckng_dt: Option<String>,
	#[serde(rename = "ChckngFrqcy", skip_serializing_if = "Option::is_none")]
	pub chckng_frqcy: Option<EventFrequency1Code>,
	#[serde(rename = "NxtRvsnDt", skip_serializing_if = "Option::is_none")]
	pub nxt_rvsn_dt: Option<String>,
	#[serde(rename = "SlryRg", skip_serializing_if = "Option::is_none")]
	pub slry_rg: Option<Max35Text>,
	#[serde(rename = "SrcOfWlth", skip_serializing_if = "Option::is_none")]
	pub src_of_wlth: Option<Max140Text>,
	#[serde(rename = "CstmrCndctClssfctn", skip_serializing_if = "Option::is_none")]
	pub cstmr_cndct_clssfctn: Option<CustomerConductClassification1Choice>,
	#[serde(rename = "RskLvl", skip_serializing_if = "Option::is_none")]
	pub rsk_lvl: Option<RiskLevel2Choice>,
	#[serde(rename = "KnowYourCstmrChckTp", skip_serializing_if = "Option::is_none")]
	pub know_your_cstmr_chck_tp: Option<KYCCheckType1Choice>,
	#[serde(rename = "KnowYourCstmrDBChck", skip_serializing_if = "Option::is_none")]
	pub know_your_cstmr_db_chck: Option<DataBaseCheck1>,
}

impl PartyProfileInformation5 {
	pub fn validate(&self) -> bool {
		if let Some(ref vldtng_pty_value) = self.vldtng_pty { if !vldtng_pty_value.validate() { return false; } }
		if let Some(ref chckng_pty_value) = self.chckng_pty { if !chckng_pty_value.validate() { return false; } }
		if let Some(ref rspnsbl_pty_value) = self.rspnsbl_pty { if !rspnsbl_pty_value.validate() { return false; } }
		if let Some(ref cert_tp_value) = self.cert_tp { if !cert_tp_value.validate() { return false; } }
		if let Some(ref chckng_frqcy_value) = self.chckng_frqcy { if !chckng_frqcy_value.validate() { return false; } }
		if let Some(ref slry_rg_value) = self.slry_rg { if !slry_rg_value.validate() { return false; } }
		if let Some(ref src_of_wlth_value) = self.src_of_wlth { if !src_of_wlth_value.validate() { return false; } }
		if let Some(ref cstmr_cndct_clssfctn_value) = self.cstmr_cndct_clssfctn { if !cstmr_cndct_clssfctn_value.validate() { return false; } }
		if let Some(ref rsk_lvl_value) = self.rsk_lvl { if !rsk_lvl_value.validate() { return false; } }
		if let Some(ref know_your_cstmr_chck_tp_value) = self.know_your_cstmr_chck_tp { if !know_your_cstmr_chck_tp_value.validate() { return false; } }
		if let Some(ref know_your_cstmr_db_chck_value) = self.know_your_cstmr_db_chck { if !know_your_cstmr_db_chck_value.validate() { return false; } }
		return true
	}
}


// PartyRole1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PartyRole1Code {
	#[default]
	#[serde(rename = "CUST")]
	CodeCUST,
	#[serde(rename = "INVS")]
	CodeINVS,
}

impl PartyRole1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PartyRole2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyRole2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestmentFundRole6Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl PartyRole2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// PartyRole4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyRole4Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestmentFundRole7Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl PartyRole4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// PartyRole5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyRole5Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PartyRole1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl PartyRole5Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// PaymentCard29 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentCard29 {
	#[serde(rename = "Tp")]
	pub tp: CardType1Code,
	#[serde(rename = "Nb")]
	pub nb: Max35Text,
	#[serde(rename = "HldrNm")]
	pub hldr_nm: Max35Text,
	#[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
	pub start_dt: Option<String>,
	#[serde(rename = "XpryDt")]
	pub xpry_dt: String,
	#[serde(rename = "CardIssrNm", skip_serializing_if = "Option::is_none")]
	pub card_issr_nm: Option<Max35Text>,
	#[serde(rename = "CardIssrId", skip_serializing_if = "Option::is_none")]
	pub card_issr_id: Option<PartyIdentification125Choice>,
	#[serde(rename = "SctyCd", skip_serializing_if = "Option::is_none")]
	pub scty_cd: Option<Max35Text>,
	#[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
	pub seq_nb: Option<Max3Text>,
}

impl PaymentCard29 {
	pub fn validate(&self) -> bool {
		if !self.tp.validate() { return false }
		if !self.nb.validate() { return false }
		if !self.hldr_nm.validate() { return false }
		if let Some(ref card_issr_nm_value) = self.card_issr_nm { if !card_issr_nm_value.validate() { return false; } }
		if let Some(ref card_issr_id_value) = self.card_issr_id { if !card_issr_id_value.validate() { return false; } }
		if let Some(ref scty_cd_value) = self.scty_cd { if !scty_cd_value.validate() { return false; } }
		if let Some(ref seq_nb_value) = self.seq_nb { if !seq_nb_value.validate() { return false; } }
		return true
	}
}


// PaymentInstrument17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentInstrument17 {
	#[serde(rename = "SttlmCcy")]
	pub sttlm_ccy: ActiveCurrencyCode,
	#[serde(rename = "DvddPctg", skip_serializing_if = "Option::is_none")]
	pub dvdd_pctg: Option<f64>,
	#[serde(rename = "SbcptPmtInstrm", skip_serializing_if = "Option::is_none")]
	pub sbcpt_pmt_instrm: Option<PaymentInstrument24Choice>,
	#[serde(rename = "RedPmtInstrm", skip_serializing_if = "Option::is_none")]
	pub red_pmt_instrm: Option<PaymentInstrument19Choice>,
	#[serde(rename = "DvddPmtInstrm", skip_serializing_if = "Option::is_none")]
	pub dvdd_pmt_instrm: Option<PaymentInstrument19Choice>,
	#[serde(rename = "SvgsPlanPmtInstrm", skip_serializing_if = "Option::is_none")]
	pub svgs_plan_pmt_instrm: Option<PaymentInstrument24Choice>,
	#[serde(rename = "IntrstPmtInstrm", skip_serializing_if = "Option::is_none")]
	pub intrst_pmt_instrm: Option<PaymentInstrument19Choice>,
}

impl PaymentInstrument17 {
	pub fn validate(&self) -> bool {
		if !self.sttlm_ccy.validate() { return false }
		if let Some(ref sbcpt_pmt_instrm_value) = self.sbcpt_pmt_instrm { if !sbcpt_pmt_instrm_value.validate() { return false; } }
		if let Some(ref red_pmt_instrm_value) = self.red_pmt_instrm { if !red_pmt_instrm_value.validate() { return false; } }
		if let Some(ref dvdd_pmt_instrm_value) = self.dvdd_pmt_instrm { if !dvdd_pmt_instrm_value.validate() { return false; } }
		if let Some(ref svgs_plan_pmt_instrm_value) = self.svgs_plan_pmt_instrm { if !svgs_plan_pmt_instrm_value.validate() { return false; } }
		if let Some(ref intrst_pmt_instrm_value) = self.intrst_pmt_instrm { if !intrst_pmt_instrm_value.validate() { return false; } }
		return true
	}
}


// PaymentInstrument19Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentInstrument19Choice {
	#[serde(rename = "ChqDtls", skip_serializing_if = "Option::is_none")]
	pub chq_dtls: Option<Cheque4>,
	#[serde(rename = "BkrsDrftDtls", skip_serializing_if = "Option::is_none")]
	pub bkrs_drft_dtls: Option<Cheque4>,
}

impl PaymentInstrument19Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref chq_dtls_value) = self.chq_dtls { if !chq_dtls_value.validate() { return false; } }
		if let Some(ref bkrs_drft_dtls_value) = self.bkrs_drft_dtls { if !bkrs_drft_dtls_value.validate() { return false; } }
		return true
	}
}


// PaymentInstrument24Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentInstrument24Choice {
	#[serde(rename = "PmtCardDtls", skip_serializing_if = "Option::is_none")]
	pub pmt_card_dtls: Option<PaymentCard29>,
	#[serde(rename = "DrctDbtDtls", skip_serializing_if = "Option::is_none")]
	pub drct_dbt_dtls: Option<DirectDebitMandate7>,
	#[serde(rename = "Chq", skip_serializing_if = "Option::is_none")]
	pub chq: Option<bool>,
	#[serde(rename = "BkrsDrft", skip_serializing_if = "Option::is_none")]
	pub bkrs_drft: Option<bool>,
}

impl PaymentInstrument24Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref pmt_card_dtls_value) = self.pmt_card_dtls { if !pmt_card_dtls_value.validate() { return false; } }
		if let Some(ref drct_dbt_dtls_value) = self.drct_dbt_dtls { if !drct_dbt_dtls_value.validate() { return false; } }
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


// PercentageBoundedRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PercentageBoundedRate {
	#[serde(rename = "$value")]
	pub percentage_bounded_rate: f64,
}

impl PercentageBoundedRate {
	pub fn validate(&self) -> bool {
		if self.percentage_bounded_rate < 0.000000 {
			return false
		}
		if self.percentage_bounded_rate > 100.000000 {
			return false
		}
		return true
	}
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
}

impl PercentageRate {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PersonalInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonalInformation1 {
	#[serde(rename = "NmOfFthr", skip_serializing_if = "Option::is_none")]
	pub nm_of_fthr: Option<Max35Text>,
	#[serde(rename = "MdnNmOfMthr", skip_serializing_if = "Option::is_none")]
	pub mdn_nm_of_mthr: Option<Max35Text>,
	#[serde(rename = "NmOfPrtnr", skip_serializing_if = "Option::is_none")]
	pub nm_of_prtnr: Option<Max35Text>,
}

impl PersonalInformation1 {
	pub fn validate(&self) -> bool {
		if let Some(ref nm_of_fthr_value) = self.nm_of_fthr { if !nm_of_fthr_value.validate() { return false; } }
		if let Some(ref mdn_nm_of_mthr_value) = self.mdn_nm_of_mthr { if !mdn_nm_of_mthr_value.validate() { return false; } }
		if let Some(ref nm_of_prtnr_value) = self.nm_of_prtnr { if !nm_of_prtnr_value.validate() { return false; } }
		return true
	}
}


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PhoneNumber {
	#[serde(rename = "$value")]
	pub phone_number: String,
}

impl PhoneNumber {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
		if !pattern.is_match(&self.phone_number) {
			return false
		}
		return true
	}
}


// PlanStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PlanStatus1Code {
	#[default]
	#[serde(rename = "ACTV")]
	CodeACTV,
	#[serde(rename = "CLOS")]
	CodeCLOS,
	#[serde(rename = "SUSP")]
	CodeSUSP,
}

impl PlanStatus1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PlanStatus2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlanStatus2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PlanStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl PlanStatus2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// PoliticalExposureType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PoliticalExposureType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PoliticalExposureType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl PoliticalExposureType2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// PoliticalExposureType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PoliticalExposureType2Code {
	#[default]
	#[serde(rename = "NPEX")]
	CodeNPEX,
	#[serde(rename = "YPEX")]
	CodeYPEX,
	#[serde(rename = "PEXD")]
	CodePEXD,
	#[serde(rename = "PEXF")]
	CodePEXF,
}

impl PoliticalExposureType2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PoliticallyExposedPerson1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PoliticallyExposedPerson1 {
	#[serde(rename = "PltclyXpsdPrsnTp")]
	pub pltcly_xpsd_prsn_tp: PoliticalExposureType2Choice,
	#[serde(rename = "PltclyXpsdPrsnSts", skip_serializing_if = "Option::is_none")]
	pub pltcly_xpsd_prsn_sts: Option<PoliticallyExposedPersonStatus1Choice>,
}

impl PoliticallyExposedPerson1 {
	pub fn validate(&self) -> bool {
		if !self.pltcly_xpsd_prsn_tp.validate() { return false }
		if let Some(ref pltcly_xpsd_prsn_sts_value) = self.pltcly_xpsd_prsn_sts { if !pltcly_xpsd_prsn_sts_value.validate() { return false; } }
		return true
	}
}


// PoliticallyExposedPersonStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PoliticallyExposedPersonStatus1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PoliticallyExposedPersonStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl PoliticallyExposedPersonStatus1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// PoliticallyExposedPersonStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PoliticallyExposedPersonStatus1Code {
	#[default]
	#[serde(rename = "PE03")]
	CodePE03,
	#[serde(rename = "PE01")]
	CodePE01,
	#[serde(rename = "PE02")]
	CodePE02,
}

impl PoliticallyExposedPersonStatus1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PortugueseNCCIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PortugueseNCCIdentifier {
	#[serde(rename = "$value")]
	pub portuguese_ncc_identifier: String,
}

impl PortugueseNCCIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("PT[0-9]{8,8}").unwrap();
		if !pattern.is_match(&self.portuguese_ncc_identifier) {
			return false
		}
		return true
	}
}


// PositionEffect3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PositionEffect3Code {
	#[default]
	#[serde(rename = "FIFO")]
	CodeFIFO,
	#[serde(rename = "LIFO")]
	CodeLIFO,
}

impl PositionEffect3Code {
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


// PostalAddress21 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress21 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType2Choice>,
	#[serde(rename = "MlngInd", skip_serializing_if = "Option::is_none")]
	pub mlng_ind: Option<bool>,
	#[serde(rename = "RegnAdrInd", skip_serializing_if = "Option::is_none")]
	pub regn_adr_ind: Option<bool>,
	#[serde(rename = "CareOf", skip_serializing_if = "Option::is_none")]
	pub care_of: Option<Max70Text>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<Max70Text>>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max70Text>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<Max16Text>,
	#[serde(rename = "BldgNm", skip_serializing_if = "Option::is_none")]
	pub bldg_nm: Option<Max35Text>,
	#[serde(rename = "PstBx", skip_serializing_if = "Option::is_none")]
	pub pst_bx: Option<Max10Text>,
	#[serde(rename = "SdInBldg", skip_serializing_if = "Option::is_none")]
	pub sd_in_bldg: Option<Max35Text>,
	#[serde(rename = "Flr", skip_serializing_if = "Option::is_none")]
	pub flr: Option<Max70Text>,
	#[serde(rename = "SuiteId", skip_serializing_if = "Option::is_none")]
	pub suite_id: Option<Max10Text>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<Max16Text>,
	#[serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none")]
	pub dstrct_nm: Option<Max35Text>,
	#[serde(rename = "Vllg", skip_serializing_if = "Option::is_none")]
	pub vllg: Option<Max70Text>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<Max35Text>,
	#[serde(rename = "Stat", skip_serializing_if = "Option::is_none")]
	pub stat: Option<Max70Text>,
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
}

impl PostalAddress21 {
	pub fn validate(&self) -> bool {
		if let Some(ref adr_tp_value) = self.adr_tp { if !adr_tp_value.validate() { return false; } }
		if let Some(ref care_of_value) = self.care_of { if !care_of_value.validate() { return false; } }
		if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if !item.validate() { return false; } } }
		if let Some(ref strt_nm_value) = self.strt_nm { if !strt_nm_value.validate() { return false; } }
		if let Some(ref bldg_nb_value) = self.bldg_nb { if !bldg_nb_value.validate() { return false; } }
		if let Some(ref bldg_nm_value) = self.bldg_nm { if !bldg_nm_value.validate() { return false; } }
		if let Some(ref pst_bx_value) = self.pst_bx { if !pst_bx_value.validate() { return false; } }
		if let Some(ref sd_in_bldg_value) = self.sd_in_bldg { if !sd_in_bldg_value.validate() { return false; } }
		if let Some(ref flr_value) = self.flr { if !flr_value.validate() { return false; } }
		if let Some(ref suite_id_value) = self.suite_id { if !suite_id_value.validate() { return false; } }
		if let Some(ref pst_cd_value) = self.pst_cd { if !pst_cd_value.validate() { return false; } }
		if let Some(ref dstrct_nm_value) = self.dstrct_nm { if !dstrct_nm_value.validate() { return false; } }
		if let Some(ref vllg_value) = self.vllg { if !vllg_value.validate() { return false; } }
		if let Some(ref twn_nm_value) = self.twn_nm { if !twn_nm_value.validate() { return false; } }
		if let Some(ref stat_value) = self.stat { if !stat_value.validate() { return false; } }
		if !self.ctry.validate() { return false }
		return true
	}
}


// ProfileType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProfileType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ProfileType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl ProfileType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// ProfileType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ProfileType1Code {
	#[default]
	#[serde(rename = "HEDG")]
	CodeHEDG,
	#[serde(rename = "HFTR")]
	CodeHFTR,
	#[serde(rename = "MAKE")]
	CodeMAKE,
	#[serde(rename = "TREA")]
	CodeTREA,
}

impl ProfileType1Code {
	pub fn validate(&self) -> bool {
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


// Provided1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Provided1Code {
	#[default]
	#[serde(rename = "NPRO")]
	CodeNPRO,
	#[serde(rename = "PROV")]
	CodePROV,
}

impl Provided1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// QUICKIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct QUICKIdentifier {
	#[serde(rename = "$value")]
	pub quick_identifier: String,
}

impl QUICKIdentifier {
	pub fn validate(&self) -> bool {
		return true
	}
}


// RICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RICIdentifier {
	#[serde(rename = "$value")]
	pub ric_identifier: String,
}

impl RICIdentifier {
	pub fn validate(&self) -> bool {
		if self.ric_identifier.chars().count() < 1 {
			return false
		}
		if self.ric_identifier.chars().count() > 35 {
			return false
		}
		return true
	}
}


// Rank1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Rank1Code {
	#[default]
	#[serde(rename = "PRIM")]
	CodePRIM,
	#[serde(rename = "SECO")]
	CodeSECO,
}

impl Rank1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Referred1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Referred1Code {
	#[default]
	#[serde(rename = "REFR")]
	CodeREFR,
	#[serde(rename = "NRFR")]
	CodeNRFR,
	#[serde(rename = "UKNW")]
	CodeUKNW,
}

impl Referred1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ReferredAgent3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferredAgent3 {
	#[serde(rename = "Rfrd")]
	pub rfrd: Referred1Code,
	#[serde(rename = "RfrdPlcmntAgt", skip_serializing_if = "Option::is_none")]
	pub rfrd_plcmnt_agt: Option<PartyIdentification125Choice>,
}

impl ReferredAgent3 {
	pub fn validate(&self) -> bool {
		if !self.rfrd.validate() { return false }
		if let Some(ref rfrd_plcmnt_agt_value) = self.rfrd_plcmnt_agt { if !rfrd_plcmnt_agt_value.validate() { return false; } }
		return true
	}
}


// RegisteredShareholderName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegisteredShareholderName1Choice {
	#[serde(rename = "IndvPrsn", skip_serializing_if = "Option::is_none")]
	pub indv_prsn: Option<IndividualPerson29>,
	#[serde(rename = "Org", skip_serializing_if = "Option::is_none")]
	pub org: Option<Organisation23>,
}

impl RegisteredShareholderName1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref indv_prsn_value) = self.indv_prsn { if !indv_prsn_value.validate() { return false; } }
		if let Some(ref org_value) = self.org { if !org_value.validate() { return false; } }
		return true
	}
}


// RegulatoryInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegulatoryInformation1 {
	#[serde(rename = "Sctr", skip_serializing_if = "Option::is_none")]
	pub sctr: Option<Max35Text>,
	#[serde(rename = "Brnch", skip_serializing_if = "Option::is_none")]
	pub brnch: Option<Max35Text>,
	#[serde(rename = "Grp", skip_serializing_if = "Option::is_none")]
	pub grp: Option<Max35Text>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Max35Text>,
}

impl RegulatoryInformation1 {
	pub fn validate(&self) -> bool {
		if let Some(ref sctr_value) = self.sctr { if !sctr_value.validate() { return false; } }
		if let Some(ref brnch_value) = self.brnch { if !brnch_value.validate() { return false; } }
		if let Some(ref grp_value) = self.grp { if !grp_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// Reinvestment4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Reinvestment4 {
	#[serde(rename = "FinInstrmDtls")]
	pub fin_instrm_dtls: FinancialInstrument87,
	#[serde(rename = "ReqdNAVCcy", skip_serializing_if = "Option::is_none")]
	pub reqd_nav_ccy: Option<ActiveCurrencyCode>,
	#[serde(rename = "RinvstmtPctg")]
	pub rinvstmt_pctg: f64,
}

impl Reinvestment4 {
	pub fn validate(&self) -> bool {
		if !self.fin_instrm_dtls.validate() { return false }
		if let Some(ref reqd_nav_ccy_value) = self.reqd_nav_ccy { if !reqd_nav_ccy_value.validate() { return false; } }
		return true
	}
}


// Repartition6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Repartition6 {
	#[serde(rename = "Qty")]
	pub qty: UnitsOrAmountOrPercentage1Choice,
	#[serde(rename = "FinInstrm")]
	pub fin_instrm: FinancialInstrument87,
	#[serde(rename = "CcyOfPlan", skip_serializing_if = "Option::is_none")]
	pub ccy_of_plan: Option<ActiveOrHistoricCurrencyCode>,
}

impl Repartition6 {
	pub fn validate(&self) -> bool {
		if !self.qty.validate() { return false }
		if !self.fin_instrm.validate() { return false }
		if let Some(ref ccy_of_plan_value) = self.ccy_of_plan { if !ccy_of_plan_value.validate() { return false; } }
		return true
	}
}


// ResidentialStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ResidentialStatus1Code {
	#[default]
	#[serde(rename = "RESI")]
	CodeRESI,
	#[serde(rename = "PRES")]
	CodePRES,
	#[serde(rename = "NRES")]
	CodeNRES,
}

impl ResidentialStatus1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// RestrictionStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RestrictionStatus1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<RestrictionStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl RestrictionStatus1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// RestrictionStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RestrictionStatus1Code {
	#[default]
	#[serde(rename = "ACTV")]
	CodeACTV,
	#[serde(rename = "INAC")]
	CodeINAC,
}

impl RestrictionStatus1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// RiskLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RiskLevel1Code {
	#[default]
	#[serde(rename = "HIGH")]
	CodeHIGH,
	#[serde(rename = "LOWW")]
	CodeLOWW,
	#[serde(rename = "MEDM")]
	CodeMEDM,
}

impl RiskLevel1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// RiskLevel2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RiskLevel2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<RiskLevel1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl RiskLevel2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// RoundingDirection1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RoundingDirection1Code {
	#[default]
	#[serde(rename = "RDUP")]
	CodeRDUP,
	#[serde(rename = "RDWN")]
	CodeRDWN,
	#[serde(rename = "STAN")]
	CodeSTAN,
	#[serde(rename = "DIST")]
	CodeDIST,
}

impl RoundingDirection1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// RoundingParameters1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RoundingParameters1 {
	#[serde(rename = "RndgMdlus", skip_serializing_if = "Option::is_none")]
	pub rndg_mdlus: Option<f64>,
	#[serde(rename = "RndgDrctn")]
	pub rndg_drctn: RoundingDirection1Code,
}

impl RoundingParameters1 {
	pub fn validate(&self) -> bool {
		if !self.rndg_drctn.validate() { return false }
		return true
	}
}


// RussianCentralBankIdentificationCodeIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RussianCentralBankIdentificationCodeIdentifier {
	#[serde(rename = "$value")]
	pub russian_central_bank_identification_code_identifier: String,
}

impl RussianCentralBankIdentificationCodeIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("RU[0-9]{9,9}").unwrap();
		if !pattern.is_match(&self.russian_central_bank_identification_code_identifier) {
			return false
		}
		return true
	}
}


// SEDOLIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SEDOLIdentifier {
	#[serde(rename = "$value")]
	pub sedol_identifier: String,
}

impl SEDOLIdentifier {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SecurityIdentification25Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification25Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "SEDOL", skip_serializing_if = "Option::is_none")]
	pub sedol: Option<String>,
	#[serde(rename = "CUSIP", skip_serializing_if = "Option::is_none")]
	pub cusip: Option<String>,
	#[serde(rename = "RIC", skip_serializing_if = "Option::is_none")]
	pub ric: Option<RICIdentifier>,
	#[serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none")]
	pub tckr_symb: Option<TickerIdentifier>,
	#[serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none")]
	pub blmbrg: Option<Bloomberg2Identifier>,
	#[serde(rename = "CTA", skip_serializing_if = "Option::is_none")]
	pub cta: Option<ConsolidatedTapeAssociationIdentifier>,
	#[serde(rename = "QUICK", skip_serializing_if = "Option::is_none")]
	pub quick: Option<String>,
	#[serde(rename = "Wrtppr", skip_serializing_if = "Option::is_none")]
	pub wrtppr: Option<String>,
	#[serde(rename = "Dtch", skip_serializing_if = "Option::is_none")]
	pub dtch: Option<String>,
	#[serde(rename = "Vlrn", skip_serializing_if = "Option::is_none")]
	pub vlrn: Option<String>,
	#[serde(rename = "SCVM", skip_serializing_if = "Option::is_none")]
	pub scvm: Option<String>,
	#[serde(rename = "Belgn", skip_serializing_if = "Option::is_none")]
	pub belgn: Option<String>,
	#[serde(rename = "Cmon", skip_serializing_if = "Option::is_none")]
	pub cmon: Option<EuroclearClearstreamIdentifier>,
	#[serde(rename = "OthrPrtryId", skip_serializing_if = "Option::is_none")]
	pub othr_prtry_id: Option<AlternateSecurityIdentification7>,
}

impl SecurityIdentification25Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref isin_value) = self.isin { if !isin_value.validate() { return false; } }
		if let Some(ref ric_value) = self.ric { if !ric_value.validate() { return false; } }
		if let Some(ref tckr_symb_value) = self.tckr_symb { if !tckr_symb_value.validate() { return false; } }
		if let Some(ref blmbrg_value) = self.blmbrg { if !blmbrg_value.validate() { return false; } }
		if let Some(ref cta_value) = self.cta { if !cta_value.validate() { return false; } }
		if let Some(ref cmon_value) = self.cmon { if !cmon_value.validate() { return false; } }
		if let Some(ref othr_prtry_id_value) = self.othr_prtry_id { if !othr_prtry_id_value.validate() { return false; } }
		return true
	}
}


// SettlementFrequency1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFrequency1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<EventFrequency10Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl SettlementFrequency1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// SettlementInstructionReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementInstructionReason1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<SettlementInstructionReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl SettlementInstructionReason1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// SettlementInstructionReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SettlementInstructionReason1Code {
	#[default]
	#[serde(rename = "CSHI")]
	CodeCSHI,
	#[serde(rename = "ALLL")]
	CodeALLL,
	#[serde(rename = "CSHO")]
	CodeCSHO,
	#[serde(rename = "CHAR")]
	CodeCHAR,
	#[serde(rename = "DIVI")]
	CodeDIVI,
	#[serde(rename = "INTE")]
	CodeINTE,
	#[serde(rename = "SAVP")]
	CodeSAVP,
	#[serde(rename = "REDM")]
	CodeREDM,
	#[serde(rename = "SAVE")]
	CodeSAVE,
	#[serde(rename = "BUYI")]
	CodeBUYI,
	#[serde(rename = "SELL")]
	CodeSELL,
	#[serde(rename = "SUBS")]
	CodeSUBS,
	#[serde(rename = "WTHP")]
	CodeWTHP,
	#[serde(rename = "CORP")]
	CodeCORP,
}

impl SettlementInstructionReason1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SicovamIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SicovamIdentifier {
	#[serde(rename = "$value")]
	pub sicovam_identifier: String,
}

impl SicovamIdentifier {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SimpleIdentificationInformation4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SimpleIdentificationInformation4 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
}

impl SimpleIdentificationInformation4 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		return true
	}
}


// SmallNetworkIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SmallNetworkIdentifier {
	#[serde(rename = "$value")]
	pub small_network_identifier: String,
}

impl SmallNetworkIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("AU[0-9]{6,6}").unwrap();
		if !pattern.is_match(&self.small_network_identifier) {
			return false
		}
		return true
	}
}


// SouthAfricanNCCIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SouthAfricanNCCIdentifier {
	#[serde(rename = "$value")]
	pub south_african_ncc_identifier: String,
}

impl SouthAfricanNCCIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("ZA[0-9]{6,6}").unwrap();
		if !pattern.is_match(&self.south_african_ncc_identifier) {
			return false
		}
		return true
	}
}


// SpanishDomesticInterbankingIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SpanishDomesticInterbankingIdentifier {
	#[serde(rename = "$value")]
	pub spanish_domestic_interbanking_identifier: String,
}

impl SpanishDomesticInterbankingIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("ES[0-9]{8,9}").unwrap();
		if !pattern.is_match(&self.spanish_domestic_interbanking_identifier) {
			return false
		}
		return true
	}
}


// StatementFrequencyReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatementFrequencyReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<EventFrequency9Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl StatementFrequencyReason2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// SwissBCIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SwissBCIdentifier {
	#[serde(rename = "$value")]
	pub swiss_bc_identifier: String,
}

impl SwissBCIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("SW[0-9]{3,5}").unwrap();
		if !pattern.is_match(&self.swiss_bc_identifier) {
			return false
		}
		return true
	}
}


// SwissSICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SwissSICIdentifier {
	#[serde(rename = "$value")]
	pub swiss_sic_identifier: String,
}

impl SwissSICIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("SW[0-9]{6,6}").unwrap();
		if !pattern.is_match(&self.swiss_sic_identifier) {
			return false
		}
		return true
	}
}


// TaxExemptReason3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TaxExemptReason3Code {
	#[default]
	#[serde(rename = "NONE")]
	CodeNONE,
	#[serde(rename = "MASA")]
	CodeMASA,
	#[serde(rename = "MISA")]
	CodeMISA,
	#[serde(rename = "SISA")]
	CodeSISA,
	#[serde(rename = "IISA")]
	CodeIISA,
	#[serde(rename = "CUYP")]
	CodeCUYP,
	#[serde(rename = "PRYP")]
	CodePRYP,
	#[serde(rename = "ASTR")]
	CodeASTR,
	#[serde(rename = "EMPY")]
	CodeEMPY,
	#[serde(rename = "EMCY")]
	CodeEMCY,
	#[serde(rename = "EPRY")]
	CodeEPRY,
	#[serde(rename = "ECYE")]
	CodeECYE,
	#[serde(rename = "NFPI")]
	CodeNFPI,
	#[serde(rename = "NFQP")]
	CodeNFQP,
	#[serde(rename = "DECP")]
	CodeDECP,
	#[serde(rename = "IRAC")]
	CodeIRAC,
	#[serde(rename = "IRAR")]
	CodeIRAR,
	#[serde(rename = "KEOG")]
	CodeKEOG,
	#[serde(rename = "PFSP")]
	CodePFSP,
	#[serde(rename = "401K")]
	Code401K,
	#[serde(rename = "SIRA")]
	CodeSIRA,
	#[serde(rename = "403B")]
	Code403B,
	#[serde(rename = "457X")]
	Code457X,
	#[serde(rename = "RIRA")]
	CodeRIRA,
	#[serde(rename = "RIAN")]
	CodeRIAN,
	#[serde(rename = "RCRF")]
	CodeRCRF,
	#[serde(rename = "RCIP")]
	CodeRCIP,
	#[serde(rename = "EIFP")]
	CodeEIFP,
	#[serde(rename = "EIOP")]
	CodeEIOP,
	#[serde(rename = "FORE")]
	CodeFORE,
	#[serde(rename = "INCA")]
	CodeINCA,
	#[serde(rename = "MINO")]
	CodeMINO,
	#[serde(rename = "ASSO")]
	CodeASSO,
	#[serde(rename = "DIPL")]
	CodeDIPL,
	#[serde(rename = "DOME")]
	CodeDOME,
	#[serde(rename = "FORP")]
	CodeFORP,
	#[serde(rename = "ORDR")]
	CodeORDR,
	#[serde(rename = "PENF")]
	CodePENF,
	#[serde(rename = "REFU")]
	CodeREFU,
	#[serde(rename = "RIHO")]
	CodeRIHO,
	#[serde(rename = "ADMI")]
	CodeADMI,
	#[serde(rename = "TANR")]
	CodeTANR,
	#[serde(rename = "OANR")]
	CodeOANR,
}

impl TaxExemptReason3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TaxExemptionReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxExemptionReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<TaxExemptReason3Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl TaxExemptionReason2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// TaxReporting3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxReporting3 {
	#[serde(rename = "TaxtnCtry")]
	pub taxtn_ctry: CountryCode,
	#[serde(rename = "TaxRate", skip_serializing_if = "Option::is_none")]
	pub tax_rate: Option<f64>,
	#[serde(rename = "TaxPyer", skip_serializing_if = "Option::is_none")]
	pub tax_pyer: Option<PartyIdentification125Choice>,
	#[serde(rename = "TaxRcpt", skip_serializing_if = "Option::is_none")]
	pub tax_rcpt: Option<PartyIdentification125Choice>,
	#[serde(rename = "CshAcctDtls", skip_serializing_if = "Option::is_none")]
	pub csh_acct_dtls: Option<CashAccount204>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max350Text>,
}

impl TaxReporting3 {
	pub fn validate(&self) -> bool {
		if !self.taxtn_ctry.validate() { return false }
		if let Some(ref tax_pyer_value) = self.tax_pyer { if !tax_pyer_value.validate() { return false; } }
		if let Some(ref tax_rcpt_value) = self.tax_rcpt { if !tax_rcpt_value.validate() { return false; } }
		if let Some(ref csh_acct_dtls_value) = self.csh_acct_dtls { if !csh_acct_dtls_value.validate() { return false; } }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
		return true
	}
}


// TaxWithholdingMethod3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TaxWithholdingMethod3Code {
	#[default]
	#[serde(rename = "MITX")]
	CodeMITX,
	#[serde(rename = "INVE")]
	CodeINVE,
	#[serde(rename = "ACCT")]
	CodeACCT,
	#[serde(rename = "EXMT")]
	CodeEXMT,
	#[serde(rename = "REPT")]
	CodeREPT,
	#[serde(rename = "CRTF")]
	CodeCRTF,
	#[serde(rename = "WHCO")]
	CodeWHCO,
	#[serde(rename = "WTHD")]
	CodeWTHD,
	#[serde(rename = "WTRE")]
	CodeWTRE,
}

impl TaxWithholdingMethod3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ThirdPartyRights2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ThirdPartyRights2 {
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
	#[serde(rename = "Hldr", skip_serializing_if = "Option::is_none")]
	pub hldr: Option<PartyIdentification125Choice>,
	#[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
	pub lgl_ntty_idr: Option<LEIIdentifier>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max350Text>,
}

impl ThirdPartyRights2 {
	pub fn validate(&self) -> bool {
		if !self.tp.validate() { return false }
		if let Some(ref hldr_value) = self.hldr { if !hldr_value.validate() { return false; } }
		if let Some(ref lgl_ntty_idr_value) = self.lgl_ntty_idr { if !lgl_ntty_idr_value.validate() { return false; } }
		if let Some(ref amt_value) = self.amt { if !amt_value.validate() { return false; } }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
		return true
	}
}


// TickerIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TickerIdentifier {
	#[serde(rename = "$value")]
	pub ticker_identifier: String,
}

impl TickerIdentifier {
	pub fn validate(&self) -> bool {
		if self.ticker_identifier.chars().count() < 1 {
			return false
		}
		if self.ticker_identifier.chars().count() > 35 {
			return false
		}
		return true
	}
}


// TransactionChannel2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionChannel2Code {
	#[default]
	#[serde(rename = "FIAD")]
	CodeFIAD,
	#[serde(rename = "HOBA")]
	CodeHOBA,
	#[serde(rename = "BRAN")]
	CodeBRAN,
}

impl TransactionChannel2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TransactionChannelType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionChannelType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<TransactionChannel2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl TransactionChannelType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
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


// TreasuryProfile1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TreasuryProfile1 {
	#[serde(rename = "Dt")]
	pub dt: String,
	#[serde(rename = "TradrTp")]
	pub tradr_tp: PartyRole5Choice,
	#[serde(rename = "Rate")]
	pub rate: f64,
}

impl TreasuryProfile1 {
	pub fn validate(&self) -> bool {
		if !self.tradr_tp.validate() { return false }
		return true
	}
}


// UKDomesticSortCodeIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UKDomesticSortCodeIdentifier {
	#[serde(rename = "$value")]
	pub uk_domestic_sort_code_identifier: String,
}

impl UKDomesticSortCodeIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("SC[0-9]{6,6}").unwrap();
		if !pattern.is_match(&self.uk_domestic_sort_code_identifier) {
			return false
		}
		return true
	}
}


// UnitsOrAmount1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitsOrAmount1Choice {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
}

impl UnitsOrAmount1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref amt_value) = self.amt { if !amt_value.validate() { return false; } }
		return true
	}
}


// UnitsOrAmountOrPercentage1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitsOrAmountOrPercentage1Choice {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
}

impl UnitsOrAmountOrPercentage1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref amt_value) = self.amt { if !amt_value.validate() { return false; } }
		return true
	}
}


// ValorenIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ValorenIdentifier {
	#[serde(rename = "$value")]
	pub valoren_identifier: String,
}

impl ValorenIdentifier {
	pub fn validate(&self) -> bool {
		return true
	}
}


// WertpapierIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WertpapierIdentifier {
	#[serde(rename = "$value")]
	pub wertpapier_identifier: String,
}

impl WertpapierIdentifier {
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
