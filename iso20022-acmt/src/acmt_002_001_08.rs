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

pub mod iso20022 {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	
	
	// Account23 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Account23 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctId") )]
		pub acct_id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdAcctDtls", skip_serializing_if = "Option::is_none") )]
		pub rltd_acct_dtls: Option<GenericIdentification1>,
	}
	
	impl Account23 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.acct_id.validate() { return Err(e); }
			if let Some(ref rltd_acct_dtls_value) = self.rltd_acct_dtls { if let Err(e) = rltd_acct_dtls_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Account32 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Account32 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcr") )]
		pub acct_svcr: PartyIdentification125Choice,
	}
	
	impl Account32 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Err(e) = self.acct_svcr.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AccountDesignation1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountDesignation1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<Rank1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl AccountDesignation1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AccountDetailsConfirmationV08 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountDetailsConfirmationV08 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
		pub msg_id: MessageIdentification1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrRef", skip_serializing_if = "Option::is_none") )]
		pub ordr_ref: Option<InvestmentFundOrder4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdRef", skip_serializing_if = "Option::is_none") )]
		pub rltd_ref: Option<AdditionalReference13>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ConfDtls") )]
		pub conf_dtls: AccountManagementConfirmation5,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtAcct", skip_serializing_if = "Option::is_none") )]
		pub invstmt_acct: Option<InvestmentAccount74>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctPties", skip_serializing_if = "Option::is_none") )]
		pub acct_pties: Option<AccountParties17>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Intrmies", skip_serializing_if = "Option::is_none") )]
		pub intrmies: Option<Vec<Intermediary46>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Plcmnt", skip_serializing_if = "Option::is_none") )]
		pub plcmnt: Option<ReferredAgent3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NewIsseAllcn", skip_serializing_if = "Option::is_none") )]
		pub new_isse_allcn: Option<NewIssueAllocation2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvgsInvstmtPlan", skip_serializing_if = "Option::is_none") )]
		pub svgs_invstmt_plan: Option<Vec<InvestmentPlan17>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WdrwlInvstmtPlan", skip_serializing_if = "Option::is_none") )]
		pub wdrwl_invstmt_plan: Option<Vec<InvestmentPlan17>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshSttlm", skip_serializing_if = "Option::is_none") )]
		pub csh_sttlm: Option<Vec<CashSettlement3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvcLvlAgrmt", skip_serializing_if = "Option::is_none") )]
		pub svc_lvl_agrmt: Option<Vec<DocumentToSend4>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Vec<AdditiononalInformation13>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MktPrctcVrsn", skip_serializing_if = "Option::is_none") )]
		pub mkt_prctc_vrsn: Option<MarketPracticeVersion1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Xtnsn", skip_serializing_if = "Option::is_none") )]
		pub xtnsn: Option<Vec<Extension1>>,
	}
	
	impl AccountDetailsConfirmationV08 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.msg_id.validate() { return Err(e); }
			if let Some(ref ordr_ref_value) = self.ordr_ref { if let Err(e) = ordr_ref_value.validate() { return Err(e); } }
			if let Some(ref rltd_ref_value) = self.rltd_ref { if let Err(e) = rltd_ref_value.validate() { return Err(e); } }
			if let Err(e) = self.conf_dtls.validate() { return Err(e); }
			if let Some(ref invstmt_acct_value) = self.invstmt_acct { if let Err(e) = invstmt_acct_value.validate() { return Err(e); } }
			if let Some(ref acct_pties_value) = self.acct_pties { if let Err(e) = acct_pties_value.validate() { return Err(e); } }
			if let Some(ref intrmies_vec) = self.intrmies { for item in intrmies_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref plcmnt_value) = self.plcmnt { if let Err(e) = plcmnt_value.validate() { return Err(e); } }
			if let Some(ref new_isse_allcn_value) = self.new_isse_allcn { if let Err(e) = new_isse_allcn_value.validate() { return Err(e); } }
			if let Some(ref svgs_invstmt_plan_vec) = self.svgs_invstmt_plan { for item in svgs_invstmt_plan_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref wdrwl_invstmt_plan_vec) = self.wdrwl_invstmt_plan { for item in wdrwl_invstmt_plan_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref csh_sttlm_vec) = self.csh_sttlm { for item in csh_sttlm_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref svc_lvl_agrmt_vec) = self.svc_lvl_agrmt { for item in svc_lvl_agrmt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref mkt_prctc_vrsn_value) = self.mkt_prctc_vrsn { if let Err(e) = mkt_prctc_vrsn_value.validate() { return Err(e); } }
			if let Some(ref xtnsn_vec) = self.xtnsn { for item in xtnsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// AccountIdentification4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountIdentification4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "IBAN", skip_serializing_if = "Option::is_none") )]
		pub iban: Option<IBAN2007Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<GenericAccountIdentification1>,
	}
	
	impl AccountIdentification4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref iban_value) = self.iban { if let Err(e) = iban_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AccountIdentificationAndName5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountIdentificationAndName5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: AccountIdentification4Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max35Text>,
	}
	
	impl AccountIdentificationAndName5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AccountManagementConfirmation5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountManagementConfirmation5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ConfTp") )]
		pub conf_tp: ConfirmationType1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctApplId", skip_serializing_if = "Option::is_none") )]
		pub acct_appl_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClntRef", skip_serializing_if = "Option::is_none") )]
		pub clnt_ref: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyRef", skip_serializing_if = "Option::is_none") )]
		pub ctr_pty_ref: Option<AdditionalReference13>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ExstgAcctId", skip_serializing_if = "Option::is_none") )]
		pub exstg_acct_id: Option<Vec<Account23>>,
	}
	
	impl AccountManagementConfirmation5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.conf_tp.validate() { return Err(e); }
			if let Some(ref acct_appl_id_value) = self.acct_appl_id { if let Err(e) = acct_appl_id_value.validate() { return Err(e); } }
			if let Some(ref clnt_ref_value) = self.clnt_ref { if let Err(e) = clnt_ref_value.validate() { return Err(e); } }
			if let Some(ref ctr_pty_ref_value) = self.ctr_pty_ref { if let Err(e) = ctr_pty_ref_value.validate() { return Err(e); } }
			if let Some(ref exstg_acct_id_vec) = self.exstg_acct_id { for item in exstg_acct_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// AccountManagementType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum AccountManagementType2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACCO") )]
		CodeACCO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACCM") )]
		CodeACCM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GACC") )]
		CodeGACC,
	}
	
	impl AccountManagementType2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AccountOwnershipType4Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum AccountOwnershipType4Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "UNCO") )]
		CodeUNCO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LIPA") )]
		CodeLIPA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ENTR") )]
		CodeENTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CORP") )]
		CodeCORP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CUST") )]
		CodeCUST,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EURE") )]
		CodeEURE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PART") )]
		CodePART,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRUS") )]
		CodeTRUS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GOVO") )]
		CodeGOVO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "JOIT") )]
		CodeJOIT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COMO") )]
		CodeCOMO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "JOIN") )]
		CodeJOIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LLCO") )]
		CodeLLCO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOMI") )]
		CodeNOMI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NFPO") )]
		CodeNFPO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ONIS") )]
		CodeONIS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RGIC") )]
		CodeRGIC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SIGL") )]
		CodeSIGL,
	}
	
	impl AccountOwnershipType4Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AccountParties12Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountParties12Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmryOwnr", skip_serializing_if = "Option::is_none") )]
		pub pmry_ownr: Option<InvestmentAccountOwnershipInformation16>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Trstee", skip_serializing_if = "Option::is_none") )]
		pub trstee: Option<Vec<InvestmentAccountOwnershipInformation16>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nmnee", skip_serializing_if = "Option::is_none") )]
		pub nmnee: Option<InvestmentAccountOwnershipInformation16>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "JntOwnr", skip_serializing_if = "Option::is_none") )]
		pub jnt_ownr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	}
	
	impl AccountParties12Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref pmry_ownr_value) = self.pmry_ownr { if let Err(e) = pmry_ownr_value.validate() { return Err(e); } }
			if let Some(ref trstee_vec) = self.trstee { for item in trstee_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref nmnee_value) = self.nmnee { if let Err(e) = nmnee_value.validate() { return Err(e); } }
			if let Some(ref jnt_ownr_vec) = self.jnt_ownr { for item in jnt_ownr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// AccountParties17 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountParties17 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrncplAcctPty") )]
		pub prncpl_acct_pty: AccountParties12Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ScndryOwnr", skip_serializing_if = "Option::is_none") )]
		pub scndry_ownr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none") )]
		pub bnfcry: Option<Vec<InvestmentAccountOwnershipInformation16>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PwrOfAttny", skip_serializing_if = "Option::is_none") )]
		pub pwr_of_attny: Option<Vec<InvestmentAccountOwnershipInformation16>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LglGuardn", skip_serializing_if = "Option::is_none") )]
		pub lgl_guardn: Option<Vec<InvestmentAccountOwnershipInformation16>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtdnForMnr", skip_serializing_if = "Option::is_none") )]
		pub ctdn_for_mnr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SucssrOnDth", skip_serializing_if = "Option::is_none") )]
		pub sucssr_on_dth: Option<Vec<InvestmentAccountOwnershipInformation16>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Admstr", skip_serializing_if = "Option::is_none") )]
		pub admstr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPty", skip_serializing_if = "Option::is_none") )]
		pub othr_pty: Option<Vec<ExtendedParty14>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Grntr", skip_serializing_if = "Option::is_none") )]
		pub grntr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sttlr", skip_serializing_if = "Option::is_none") )]
		pub sttlr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SnrMggOffcl", skip_serializing_if = "Option::is_none") )]
		pub snr_mgg_offcl: Option<Vec<InvestmentAccountOwnershipInformation16>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtctr", skip_serializing_if = "Option::is_none") )]
		pub prtctr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RegdShrhldrNm", skip_serializing_if = "Option::is_none") )]
		pub regd_shrhldr_nm: Option<RegisteredShareholderName1Choice>,
	}
	
	impl AccountParties17 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.prncpl_acct_pty.validate() { return Err(e); }
			if let Some(ref scndry_ownr_vec) = self.scndry_ownr { for item in scndry_ownr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref bnfcry_vec) = self.bnfcry { for item in bnfcry_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref pwr_of_attny_vec) = self.pwr_of_attny { for item in pwr_of_attny_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref lgl_guardn_vec) = self.lgl_guardn { for item in lgl_guardn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ctdn_for_mnr_vec) = self.ctdn_for_mnr { for item in ctdn_for_mnr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref sucssr_on_dth_vec) = self.sucssr_on_dth { for item in sucssr_on_dth_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref admstr_vec) = self.admstr { for item in admstr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref othr_pty_vec) = self.othr_pty { for item in othr_pty_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref grntr_vec) = self.grntr { for item in grntr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref sttlr_vec) = self.sttlr { for item in sttlr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref snr_mgg_offcl_vec) = self.snr_mgg_offcl { for item in snr_mgg_offcl_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref prtctr_vec) = self.prtctr { for item in prtctr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref regd_shrhldr_nm_value) = self.regd_shrhldr_nm { if let Err(e) = regd_shrhldr_nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AccountSchemeName1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountSchemeName1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalAccountIdentification1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl AccountSchemeName1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AccountStatus2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountStatus2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nbld", skip_serializing_if = "Option::is_none") )]
		pub nbld: Option<EnabledStatusReason1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dsbld", skip_serializing_if = "Option::is_none") )]
		pub dsbld: Option<DisabledStatusReason1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pdg", skip_serializing_if = "Option::is_none") )]
		pub pdg: Option<PendingStatusReason1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdgOpng", skip_serializing_if = "Option::is_none") )]
		pub pdg_opng: Option<PendingOpeningStatusReason1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Profrm", skip_serializing_if = "Option::is_none") )]
		pub profrm: Option<ProformaStatusReason1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Clsd", skip_serializing_if = "Option::is_none") )]
		pub clsd: Option<ClosedStatusReason1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClsrPdg", skip_serializing_if = "Option::is_none") )]
		pub clsr_pdg: Option<ClosurePendingStatusReason1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<Vec<OtherAccountStatus1>>,
	}
	
	impl AccountStatus2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nbld_value) = self.nbld { if let Err(e) = nbld_value.validate() { return Err(e); } }
			if let Some(ref dsbld_value) = self.dsbld { if let Err(e) = dsbld_value.validate() { return Err(e); } }
			if let Some(ref pdg_value) = self.pdg { if let Err(e) = pdg_value.validate() { return Err(e); } }
			if let Some(ref pdg_opng_value) = self.pdg_opng { if let Err(e) = pdg_opng_value.validate() { return Err(e); } }
			if let Some(ref profrm_value) = self.profrm { if let Err(e) = profrm_value.validate() { return Err(e); } }
			if let Some(ref clsd_value) = self.clsd { if let Err(e) = clsd_value.validate() { return Err(e); } }
			if let Some(ref clsr_pdg_value) = self.clsr_pdg { if let Err(e) = clsr_pdg_value.validate() { return Err(e); } }
			if let Some(ref othr_vec) = self.othr { for item in othr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// AccountType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<FundCashAccount4Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl AccountType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AccountUsageType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountUsageType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<AccountUsageType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl AccountUsageType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AccountUsageType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum AccountUsageType2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVE") )]
		CodeINVE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISSP") )]
		CodeISSP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SETP") )]
		CodeSETP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRDP") )]
		CodeTRDP,
	}
	
	impl AccountUsageType2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AccountingStatus1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountingStatus1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<AccountingStatus1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl AccountingStatus1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AccountingStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum AccountingStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "YDOM") )]
		CodeYDOM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NDOM") )]
		CodeNDOM,
	}
	
	impl AccountingStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ActiveCurrencyAnd13DecimalAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_currency_and13_decimal_amount_simple_type: f64,
	}
	
	impl ActiveCurrencyAnd13DecimalAmountSimpleType {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.active_currency_and13_decimal_amount_simple_type < 0.000000 {
				return Err(ValidationError::new(1003, "active_currency_and13_decimal_amount_simple_type is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ActiveCurrencyAnd13DecimalAmount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ActiveCurrencyAnd13DecimalAmount {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
		pub ccy: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub value: f64,
	}
	
	impl ActiveCurrencyAnd13DecimalAmount {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ActiveCurrencyAndAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveCurrencyAndAmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_currency_and_amount_simple_type: f64,
	}
	
	impl ActiveCurrencyAndAmountSimpleType {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.active_currency_and_amount_simple_type < 0.000000 {
				return Err(ValidationError::new(1003, "active_currency_and_amount_simple_type is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ActiveCurrencyAndAmount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ActiveCurrencyAndAmount {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
		pub ccy: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub value: f64,
	}
	
	impl ActiveCurrencyAndAmount {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ActiveCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveCurrencyCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_currency_code: String,
	}
	
	impl ActiveCurrencyCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.active_currency_code) {
				return Err(ValidationError::new(1005, "active_currency_code does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ActiveOrHistoricCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveOrHistoricCurrencyCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_or_historic_currency_code: String,
	}
	
	impl ActiveOrHistoricCurrencyCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.active_or_historic_currency_code) {
				return Err(ValidationError::new(1005, "active_or_historic_currency_code does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// AdditionalReference13 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AdditionalReference13 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
		pub ref_attr: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefIssr", skip_serializing_if = "Option::is_none") )]
		pub ref_issr: Option<PartyIdentification125Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNm", skip_serializing_if = "Option::is_none") )]
		pub msg_nm: Option<Max35Text>,
	}
	
	impl AdditionalReference13 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ref_attr.validate() { return Err(e); }
			if let Some(ref ref_issr_value) = self.ref_issr { if let Err(e) = ref_issr_value.validate() { return Err(e); } }
			if let Some(ref msg_nm_value) = self.msg_nm { if let Err(e) = msg_nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AdditiononalInformation13 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AdditiononalInformation13 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Lmttn", skip_serializing_if = "Option::is_none") )]
		pub lmttn: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctVldtn", skip_serializing_if = "Option::is_none") )]
		pub acct_vldtn: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rgltr", skip_serializing_if = "Option::is_none") )]
		pub rgltr: Option<PartyIdentification125Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
		pub sts: Option<RestrictionStatus1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
		pub prd: Option<DateTimePeriod2>,
	}
	
	impl AdditiononalInformation13 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref lmttn_value) = self.lmttn { if let Err(e) = lmttn_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			if let Some(ref acct_vldtn_value) = self.acct_vldtn { if let Err(e) = acct_vldtn_value.validate() { return Err(e); } }
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref rgltr_value) = self.rgltr { if let Err(e) = rgltr_value.validate() { return Err(e); } }
			if let Some(ref sts_value) = self.sts { if let Err(e) = sts_value.validate() { return Err(e); } }
			if let Some(ref prd_value) = self.prd { if let Err(e) = prd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AddressType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AddressType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<AddressType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl AddressType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AddressType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum AddressType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "HOME") )]
		CodeHOME,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BIZZ") )]
		CodeBIZZ,
	}
	
	impl AddressType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AddressType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AddressType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<AddressType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl AddressType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AddressType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum AddressType2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADDR") )]
		CodeADDR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PBOX") )]
		CodePBOX,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HOME") )]
		CodeHOME,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BIZZ") )]
		CodeBIZZ,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MLTO") )]
		CodeMLTO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DLVY") )]
		CodeDLVY,
	}
	
	impl AddressType2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AlternateSecurityIdentification7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AlternateSecurityIdentification7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IdSrc") )]
		pub id_src: IdentificationSource1Choice,
	}
	
	impl AlternateSecurityIdentification7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.id_src.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AnyBICDec2014Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct AnyBICDec2014Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub any_bic_dec2014_identifier: String,
	}
	
	impl AnyBICDec2014Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(&self.any_bic_dec2014_identifier) {
				return Err(ValidationError::new(1005, "any_bic_dec2014_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// AustrianBankleitzahlIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct AustrianBankleitzahlIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub austrian_bankleitzahl_identifier: String,
	}
	
	impl AustrianBankleitzahlIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("AT[0-9]{5,5}").unwrap();
			if !pattern.is_match(&self.austrian_bankleitzahl_identifier) {
				return Err(ValidationError::new(1005, "austrian_bankleitzahl_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// BICFIDec2014Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct BICFIDec2014Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub bicfi_dec2014_identifier: String,
	}
	
	impl BICFIDec2014Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(&self.bicfi_dec2014_identifier) {
				return Err(ValidationError::new(1005, "bicfi_dec2014_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// BelgianIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct BelgianIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub belgian_identifier: String,
	}
	
	impl BelgianIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BlockedHoldingDetails2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BlockedHoldingDetails2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BlckdHldg") )]
		pub blckd_hldg: Holding1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtlHldgUnits", skip_serializing_if = "Option::is_none") )]
		pub prtl_hldg_units: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HldgCertNb", skip_serializing_if = "Option::is_none") )]
		pub hldg_cert_nb: Option<Max35Text>,
	}
	
	impl BlockedHoldingDetails2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.blckd_hldg.validate() { return Err(e); }
			if let Some(ref hldg_cert_nb_value) = self.hldg_cert_nb { if let Err(e) = hldg_cert_nb_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BlockedReason2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BlockedReason2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<BlockedReason2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl BlockedReason2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BlockedReason2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum BlockedReason2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "BKRP") )]
		CodeBKRP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CMMT") )]
		CodeCMMT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CNFS") )]
		CodeCNFS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MORT") )]
		CodeMORT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PCOM") )]
		CodePCOM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PLDG") )]
		CodePLDG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRPE") )]
		CodeTRPE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SANC") )]
		CodeSANC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRAN") )]
		CodeTRAN,
	}
	
	impl BlockedReason2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BlockedStatusReason2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BlockedStatusReason2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxTp") )]
		pub tx_tp: TransactionType5Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Blckd") )]
		pub blckd: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Vec<BlockedReason2Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf") )]
		pub addtl_inf: Max350Text,
	}
	
	impl BlockedStatusReason2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tx_tp.validate() { return Err(e); }
			if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Err(e) = self.addtl_inf.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BlockedStatusReason2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BlockedStatusReason2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
		pub no_spcfd_rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Vec<BlockedStatusReason2>>,
	}
	
	impl BlockedStatusReason2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if let Err(e) = no_spcfd_rsn_value.validate() { return Err(e); } }
			if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// Bloomberg2Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Bloomberg2Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub bloomberg2_identifier: String,
	}
	
	impl Bloomberg2Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("(BBG)[BCDFGHJKLMNPQRSTVWXYZ\\d]{8}\\d").unwrap();
			if !pattern.is_match(&self.bloomberg2_identifier) {
				return Err(ValidationError::new(1005, "bloomberg2_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// BranchData4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BranchData4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress1>,
	}
	
	impl BranchData4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CHIPSParticipantIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct CHIPSParticipantIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub chips_participant_identifier: String,
	}
	
	impl CHIPSParticipantIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("CP[0-9]{4,4}").unwrap();
			if !pattern.is_match(&self.chips_participant_identifier) {
				return Err(ValidationError::new(1005, "chips_participant_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// CHIPSUniversalIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct CHIPSUniversalIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub chips_universal_identifier: String,
	}
	
	impl CHIPSUniversalIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("CH[0-9]{6,6}").unwrap();
			if !pattern.is_match(&self.chips_universal_identifier) {
				return Err(ValidationError::new(1005, "chips_universal_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// CRSForm1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CRSForm1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<CRSFormType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl CRSForm1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CRSFormType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CRSFormType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CER4") )]
		CodeCER4,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CER3") )]
		CodeCER3,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CER5") )]
		CodeCER5,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CER6") )]
		CodeCER6,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CER8") )]
		CodeCER8,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CER1") )]
		CodeCER1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CER2") )]
		CodeCER2,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CER7") )]
		CodeCER7,
	}
	
	impl CRSFormType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CRSSource1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CRSSource1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<CRSSourceStatus1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl CRSSource1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CRSSourceStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CRSSourceStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CALC") )]
		CodeCALC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DECL") )]
		CodeDECL,
	}
	
	impl CRSSourceStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CRSStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CRSStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "C101") )]
		CodeC101,
		#[cfg_attr( feature = "derive_serde", serde(rename = "C102") )]
		CodeC102,
		#[cfg_attr( feature = "derive_serde", serde(rename = "C103") )]
		CodeC103,
		#[cfg_attr( feature = "derive_serde", serde(rename = "C104") )]
		CodeC104,
		#[cfg_attr( feature = "derive_serde", serde(rename = "C105") )]
		CodeC105,
		#[cfg_attr( feature = "derive_serde", serde(rename = "C106") )]
		CodeC106,
		#[cfg_attr( feature = "derive_serde", serde(rename = "C107") )]
		CodeC107,
		#[cfg_attr( feature = "derive_serde", serde(rename = "C108") )]
		CodeC108,
		#[cfg_attr( feature = "derive_serde", serde(rename = "C109") )]
		CodeC109,
		#[cfg_attr( feature = "derive_serde", serde(rename = "C110") )]
		CodeC110,
		#[cfg_attr( feature = "derive_serde", serde(rename = "C111") )]
		CodeC111,
		#[cfg_attr( feature = "derive_serde", serde(rename = "C112") )]
		CodeC112,
		#[cfg_attr( feature = "derive_serde", serde(rename = "C113") )]
		CodeC113,
		#[cfg_attr( feature = "derive_serde", serde(rename = "C114") )]
		CodeC114,
	}
	
	impl CRSStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CRSStatus3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CRSStatus3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<CRSStatus1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl CRSStatus3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CRSStatus4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CRSStatus4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: CRSStatus3Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Src", skip_serializing_if = "Option::is_none") )]
		pub src: Option<CRSSource1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XcptnlRptgCtry", skip_serializing_if = "Option::is_none") )]
		pub xcptnl_rptg_ctry: Option<CountryCode>,
	}
	
	impl CRSStatus4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Some(ref src_value) = self.src { if let Err(e) = src_value.validate() { return Err(e); } }
			if let Some(ref xcptnl_rptg_ctry_value) = self.xcptnl_rptg_ctry { if let Err(e) = xcptnl_rptg_ctry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CUSIPIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct CUSIPIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub cusip_identifier: String,
	}
	
	impl CUSIPIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CanadianPaymentsARNIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct CanadianPaymentsARNIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub canadian_payments_arn_identifier: String,
	}
	
	impl CanadianPaymentsARNIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("CA[0-9]{9,9}").unwrap();
			if !pattern.is_match(&self.canadian_payments_arn_identifier) {
				return Err(ValidationError::new(1005, "canadian_payments_arn_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// CardType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CardType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CRDT") )]
		CodeCRDT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DBIT") )]
		CodeDBIT,
	}
	
	impl CardType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CashAccount204 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CashAccount204 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCcy") )]
		pub sttlm_ccy: ActiveCurrencyCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: AccountIdentificationAndName5,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none") )]
		pub acct_ownr: Option<PartyIdentification125Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none") )]
		pub acct_svcr: Option<FinancialInstitutionIdentification11Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrBrnch", skip_serializing_if = "Option::is_none") )]
		pub acct_svcr_brnch: Option<BranchData4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOwnrOthrId", skip_serializing_if = "Option::is_none") )]
		pub acct_ownr_othr_id: Option<Vec<GenericIdentification82>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtAcctTp", skip_serializing_if = "Option::is_none") )]
		pub invstmt_acct_tp: Option<AccountType2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none") )]
		pub cdt_dbt: Option<CreditDebit3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmInstrRsn", skip_serializing_if = "Option::is_none") )]
		pub sttlm_instr_rsn: Option<SettlementInstructionReason1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctPurp", skip_serializing_if = "Option::is_none") )]
		pub csh_acct_purp: Option<CashAccountType3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctDsgnt", skip_serializing_if = "Option::is_none") )]
		pub csh_acct_dsgnt: Option<AccountDesignation1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DvddPctg", skip_serializing_if = "Option::is_none") )]
		pub dvdd_pctg: Option<f64>,
	}
	
	impl CashAccount204 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.sttlm_ccy.validate() { return Err(e); }
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref acct_ownr_value) = self.acct_ownr { if let Err(e) = acct_ownr_value.validate() { return Err(e); } }
			if let Some(ref acct_svcr_value) = self.acct_svcr { if let Err(e) = acct_svcr_value.validate() { return Err(e); } }
			if let Some(ref acct_svcr_brnch_value) = self.acct_svcr_brnch { if let Err(e) = acct_svcr_brnch_value.validate() { return Err(e); } }
			if let Some(ref acct_ownr_othr_id_vec) = self.acct_ownr_othr_id { for item in acct_ownr_othr_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref invstmt_acct_tp_value) = self.invstmt_acct_tp { if let Err(e) = invstmt_acct_tp_value.validate() { return Err(e); } }
			if let Some(ref cdt_dbt_value) = self.cdt_dbt { if let Err(e) = cdt_dbt_value.validate() { return Err(e); } }
			if let Some(ref sttlm_instr_rsn_value) = self.sttlm_instr_rsn { if let Err(e) = sttlm_instr_rsn_value.validate() { return Err(e); } }
			if let Some(ref csh_acct_purp_value) = self.csh_acct_purp { if let Err(e) = csh_acct_purp_value.validate() { return Err(e); } }
			if let Some(ref csh_acct_dsgnt_value) = self.csh_acct_dsgnt { if let Err(e) = csh_acct_dsgnt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CashAccountType3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CashAccountType3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<CashAccountType5Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl CashAccountType3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CashAccountType5Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CashAccountType5Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEND") )]
		CodeLEND,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COLL") )]
		CodeCOLL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SETT") )]
		CodeSETT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MARR") )]
		CodeMARR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SEGT") )]
		CodeSEGT,
	}
	
	impl CashAccountType5Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CashSettlement3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CashSettlement3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctDtls", skip_serializing_if = "Option::is_none") )]
		pub csh_acct_dtls: Option<Vec<CashAccount204>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCshSttlmDtls", skip_serializing_if = "Option::is_none") )]
		pub othr_csh_sttlm_dtls: Option<Vec<PaymentInstrument17>>,
	}
	
	impl CashSettlement3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref csh_acct_dtls_vec) = self.csh_acct_dtls { for item in csh_acct_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref othr_csh_sttlm_dtls_vec) = self.othr_csh_sttlm_dtls { for item in othr_csh_sttlm_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// CertificateType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CertificateType2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "AMLC") )]
		CodeAMLC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DVLC") )]
		CodeDVLC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DFOR") )]
		CodeDFOR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GOST") )]
		CodeGOST,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IDEN") )]
		CodeIDEN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INCU") )]
		CodeINCU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LREF") )]
		CodeLREF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PASS") )]
		CodePASS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRAD") )]
		CodePRAD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PKIC") )]
		CodePKIC,
	}
	
	impl CertificateType2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CertificationType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CertificationType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<CertificateType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl CertificationType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Cheque4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Cheque4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PyeeId") )]
		pub pyee_id: NameAndAddress5,
	}
	
	impl Cheque4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pyee_id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// CitizenshipInformation2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CitizenshipInformation2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ntlty") )]
		pub ntlty: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MnrInd") )]
		pub mnr_ind: bool,
	}
	
	impl CitizenshipInformation2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CivilStatus1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CivilStatus1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<CivilStatus1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl CivilStatus1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CivilStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CivilStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DIVO") )]
		CodeDIVO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LDIV") )]
		CodeLDIV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MARR") )]
		CodeMARR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SEPA") )]
		CodeSEPA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SING") )]
		CodeSING,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UNIO") )]
		CodeUNIO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WIDO") )]
		CodeWIDO,
	}
	
	impl CivilStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ClearingSystemMemberIdentification4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ClearingSystemMemberIdentification4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "USCHU", skip_serializing_if = "Option::is_none") )]
		pub uschu: Option<CHIPSUniversalIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NZNCC", skip_serializing_if = "Option::is_none") )]
		pub nzncc: Option<NewZealandNCCIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IENSC", skip_serializing_if = "Option::is_none") )]
		pub iensc: Option<IrishNSCIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GBSC", skip_serializing_if = "Option::is_none") )]
		pub gbsc: Option<UKDomesticSortCodeIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "USCH", skip_serializing_if = "Option::is_none") )]
		pub usch: Option<CHIPSParticipantIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CHBC", skip_serializing_if = "Option::is_none") )]
		pub chbc: Option<SwissBCIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "USFW", skip_serializing_if = "Option::is_none") )]
		pub usfw: Option<FedwireRoutingNumberIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PTNCC", skip_serializing_if = "Option::is_none") )]
		pub ptncc: Option<PortugueseNCCIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RUCB", skip_serializing_if = "Option::is_none") )]
		pub rucb: Option<RussianCentralBankIdentificationCodeIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ITNCC", skip_serializing_if = "Option::is_none") )]
		pub itncc: Option<ItalianDomesticIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ATBLZ", skip_serializing_if = "Option::is_none") )]
		pub atblz: Option<AustrianBankleitzahlIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CACPA", skip_serializing_if = "Option::is_none") )]
		pub cacpa: Option<CanadianPaymentsARNIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CHSIC", skip_serializing_if = "Option::is_none") )]
		pub chsic: Option<SwissSICIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DEBLZ", skip_serializing_if = "Option::is_none") )]
		pub deblz: Option<GermanBankleitzahlIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ESNCC", skip_serializing_if = "Option::is_none") )]
		pub esncc: Option<SpanishDomesticInterbankingIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ZANCC", skip_serializing_if = "Option::is_none") )]
		pub zancc: Option<SouthAfricanNCCIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HKNCC", skip_serializing_if = "Option::is_none") )]
		pub hkncc: Option<HongKongBankIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AUBSBx", skip_serializing_if = "Option::is_none") )]
		pub aubs_bx: Option<ExtensiveBranchNetworkIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AUBSBs", skip_serializing_if = "Option::is_none") )]
		pub aubs_bs: Option<SmallNetworkIdentifier>,
	}
	
	impl ClearingSystemMemberIdentification4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref uschu_value) = self.uschu { if let Err(e) = uschu_value.validate() { return Err(e); } }
			if let Some(ref nzncc_value) = self.nzncc { if let Err(e) = nzncc_value.validate() { return Err(e); } }
			if let Some(ref iensc_value) = self.iensc { if let Err(e) = iensc_value.validate() { return Err(e); } }
			if let Some(ref gbsc_value) = self.gbsc { if let Err(e) = gbsc_value.validate() { return Err(e); } }
			if let Some(ref usch_value) = self.usch { if let Err(e) = usch_value.validate() { return Err(e); } }
			if let Some(ref chbc_value) = self.chbc { if let Err(e) = chbc_value.validate() { return Err(e); } }
			if let Some(ref usfw_value) = self.usfw { if let Err(e) = usfw_value.validate() { return Err(e); } }
			if let Some(ref ptncc_value) = self.ptncc { if let Err(e) = ptncc_value.validate() { return Err(e); } }
			if let Some(ref rucb_value) = self.rucb { if let Err(e) = rucb_value.validate() { return Err(e); } }
			if let Some(ref itncc_value) = self.itncc { if let Err(e) = itncc_value.validate() { return Err(e); } }
			if let Some(ref atblz_value) = self.atblz { if let Err(e) = atblz_value.validate() { return Err(e); } }
			if let Some(ref cacpa_value) = self.cacpa { if let Err(e) = cacpa_value.validate() { return Err(e); } }
			if let Some(ref chsic_value) = self.chsic { if let Err(e) = chsic_value.validate() { return Err(e); } }
			if let Some(ref deblz_value) = self.deblz { if let Err(e) = deblz_value.validate() { return Err(e); } }
			if let Some(ref esncc_value) = self.esncc { if let Err(e) = esncc_value.validate() { return Err(e); } }
			if let Some(ref zancc_value) = self.zancc { if let Err(e) = zancc_value.validate() { return Err(e); } }
			if let Some(ref hkncc_value) = self.hkncc { if let Err(e) = hkncc_value.validate() { return Err(e); } }
			if let Some(ref aubs_bx_value) = self.aubs_bx { if let Err(e) = aubs_bx_value.validate() { return Err(e); } }
			if let Some(ref aubs_bs_value) = self.aubs_bs { if let Err(e) = aubs_bs_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ClosedStatusReason1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ClosedStatusReason1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: ClosedStatusReason2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Max350Text>,
	}
	
	impl ClosedStatusReason1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ClosedStatusReason1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ClosedStatusReason1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
		pub no_spcfd_rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Vec<ClosedStatusReason1>>,
	}
	
	impl ClosedStatusReason1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if let Err(e) = no_spcfd_rsn_value.validate() { return Err(e); } }
			if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ClosedStatusReason1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ClosedStatusReason1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ASIN") )]
		CodeASIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLIN") )]
		CodeCLIN,
	}
	
	impl ClosedStatusReason1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ClosedStatusReason2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ClosedStatusReason2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ClosedStatusReason1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification36>,
	}
	
	impl ClosedStatusReason2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ClosurePendingStatusReason1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ClosurePendingStatusReason1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: ClosurePendingStatusReason2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Max350Text>,
	}
	
	impl ClosurePendingStatusReason1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ClosurePendingStatusReason1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ClosurePendingStatusReason1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
		pub no_spcfd_rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Vec<ClosurePendingStatusReason1>>,
	}
	
	impl ClosurePendingStatusReason1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if let Err(e) = no_spcfd_rsn_value.validate() { return Err(e); } }
			if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ClosurePendingStatusReason1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ClosurePendingStatusReason1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLOS") )]
		CodeCLOS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PEND") )]
		CodePEND,
	}
	
	impl ClosurePendingStatusReason1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ClosurePendingStatusReason2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ClosurePendingStatusReason2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ClosurePendingStatusReason1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification36>,
	}
	
	impl ClosurePendingStatusReason2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Collateral1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Collateral1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "COLL") )]
		CodeCOLL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NCOL") )]
		CodeNCOL,
	}
	
	impl Collateral1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CommunicationAddress6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CommunicationAddress6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
		pub adr_tp: Option<AddressType1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Email", skip_serializing_if = "Option::is_none") )]
		pub email: Option<Max256Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Phne", skip_serializing_if = "Option::is_none") )]
		pub phne: Option<PhoneNumber>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mob", skip_serializing_if = "Option::is_none") )]
		pub mob: Option<PhoneNumber>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FaxNb", skip_serializing_if = "Option::is_none") )]
		pub fax_nb: Option<PhoneNumber>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TlxAdr", skip_serializing_if = "Option::is_none") )]
		pub tlx_adr: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "URLAdr", skip_serializing_if = "Option::is_none") )]
		pub url_adr: Option<Max256Text>,
	}
	
	impl CommunicationAddress6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref adr_tp_value) = self.adr_tp { if let Err(e) = adr_tp_value.validate() { return Err(e); } }
			if let Some(ref email_value) = self.email { if let Err(e) = email_value.validate() { return Err(e); } }
			if let Some(ref phne_value) = self.phne { if let Err(e) = phne_value.validate() { return Err(e); } }
			if let Some(ref mob_value) = self.mob { if let Err(e) = mob_value.validate() { return Err(e); } }
			if let Some(ref fax_nb_value) = self.fax_nb { if let Err(e) = fax_nb_value.validate() { return Err(e); } }
			if let Some(ref tlx_adr_value) = self.tlx_adr { if let Err(e) = tlx_adr_value.validate() { return Err(e); } }
			if let Some(ref url_adr_value) = self.url_adr { if let Err(e) = url_adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CommunicationMethod1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CommunicationMethod1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "SWMT") )]
		CodeSWMT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SWMX") )]
		CodeSWMX,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FAXI") )]
		CodeFAXI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EMAL") )]
		CodeEMAL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PROP") )]
		CodePROP,
	}
	
	impl CommunicationMethod1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CommunicationMethod3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CommunicationMethod3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<CommunicationMethod1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl CommunicationMethod3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompanyLink1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompanyLink1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<CompanyLink1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl CompanyLink1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompanyLink1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CompanyLink1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "AGEN") )]
		CodeAGEN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BROK") )]
		CodeBROK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PART") )]
		CodePART,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MEMB") )]
		CodeMEMB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PCOM") )]
		CodePCOM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RELA") )]
		CodeRELA,
	}
	
	impl CompanyLink1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ConductClassification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ConductClassification1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NSTA") )]
		CodeNSTA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RCLT") )]
		CodeRCLT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "STAN") )]
		CodeSTAN,
	}
	
	impl ConductClassification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ConfirmationType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ConfirmationType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<AccountManagementType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl ConfirmationType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ConsolidatedTapeAssociationIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ConsolidatedTapeAssociationIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub consolidated_tape_association_identifier: String,
	}
	
	impl ConsolidatedTapeAssociationIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.consolidated_tape_association_identifier.chars().count() < 1 {
			return Err(ValidationError::new(1001, "consolidated_tape_association_identifier is shorter than the minimum length of 1".to_string()));
			}
			if self.consolidated_tape_association_identifier.chars().count() > 35 {
				return Err(ValidationError::new(1002, "consolidated_tape_association_identifier exceeds the maximum length of 35".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ConsolidationType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ConsolidationType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ConsolidationType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl ConsolidationType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ConsolidationType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ConsolidationType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "GENL") )]
		CodeGENL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PART") )]
		CodePART,
	}
	
	impl ConsolidationType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CountryAndResidentialStatusType2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CountryAndResidentialStatusType2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
		pub ctry: CountryCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ResdtlSts") )]
		pub resdtl_sts: ResidentialStatus1Code,
	}
	
	impl CountryAndResidentialStatusType2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ctry.validate() { return Err(e); }
			if let Err(e) = self.resdtl_sts.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// CountryCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct CountryCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub country_code: String,
	}
	
	impl CountryCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&self.country_code) {
				return Err(ValidationError::new(1005, "country_code does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// CreditDebit3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CreditDebit3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CRDT") )]
		CodeCRDT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DBIT") )]
		CodeDBIT,
	}
	
	impl CreditDebit3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CustomerConductClassification1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CustomerConductClassification1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ConductClassification1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl CustomerConductClassification1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DataBaseCheck1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DataBaseCheck1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DBChck") )]
		pub db_chck: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
	}
	
	impl DataBaseCheck1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// DateAndAmount1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DateAndAmount1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt") )]
		pub dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveCurrencyAndAmount,
	}
	
	impl DateAndAmount1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// DateAndDateTime1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DateAndDateTime1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
		pub dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtTm", skip_serializing_if = "Option::is_none") )]
		pub dt_tm: Option<String>,
	}
	
	impl DateAndDateTime1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// DateTimePeriod2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DateTimePeriod2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtTm") )]
		pub fr_dt_tm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none") )]
		pub to_dt_tm: Option<String>,
	}
	
	impl DateTimePeriod2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// DeMinimus1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DeMinimus1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DeMnmsAplbl", skip_serializing_if = "Option::is_none") )]
		pub de_mnms_aplbl: Option<DeMinimusApplicable1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DeMnmsNotAplbl", skip_serializing_if = "Option::is_none") )]
		pub de_mnms_not_aplbl: Option<DeMinimusNotApplicable1>,
	}
	
	impl DeMinimus1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref de_mnms_aplbl_value) = self.de_mnms_aplbl { if let Err(e) = de_mnms_aplbl_value.validate() { return Err(e); } }
			if let Some(ref de_mnms_not_aplbl_value) = self.de_mnms_not_aplbl { if let Err(e) = de_mnms_not_aplbl_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DeMinimusApplicable1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DeMinimusApplicable1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NewIssePrmssn") )]
		pub new_isse_prmssn: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pctg", skip_serializing_if = "Option::is_none") )]
		pub pctg: Option<f64>,
	}
	
	impl DeMinimusApplicable1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// DeMinimusNotApplicable1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DeMinimusNotApplicable1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RstrctdPrsnRsn") )]
		pub rstrctd_prsn_rsn: Max350Text,
	}
	
	impl DeMinimusNotApplicable1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rstrctd_prsn_rsn.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// DecimalNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct DecimalNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub decimal_number: f64,
	}
	
	impl DecimalNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// DirectDebitMandate7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DirectDebitMandate7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct") )]
		pub dbtr_acct: AccountIdentificationAndName5,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
		pub dbtr: Option<PartyIdentification125Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrTaxIdNb", skip_serializing_if = "Option::is_none") )]
		pub dbtr_tax_id_nb: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrNtlRegnNb", skip_serializing_if = "Option::is_none") )]
		pub dbtr_ntl_regn_nb: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
		pub cdtr: Option<PartyIdentification125Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt") )]
		pub dbtr_agt: FinancialInstitutionIdentification11Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtBrnch", skip_serializing_if = "Option::is_none") )]
		pub dbtr_agt_brnch: Option<BranchData4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
		pub cdtr_agt: Option<FinancialInstitutionIdentification11Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtBrnch", skip_serializing_if = "Option::is_none") )]
		pub cdtr_agt_brnch: Option<BranchData4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RegnId", skip_serializing_if = "Option::is_none") )]
		pub regn_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MndtId", skip_serializing_if = "Option::is_none") )]
		pub mndt_id: Option<Max35Text>,
	}
	
	impl DirectDebitMandate7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.dbtr_acct.validate() { return Err(e); }
			if let Some(ref dbtr_value) = self.dbtr { if let Err(e) = dbtr_value.validate() { return Err(e); } }
			if let Some(ref dbtr_tax_id_nb_value) = self.dbtr_tax_id_nb { if let Err(e) = dbtr_tax_id_nb_value.validate() { return Err(e); } }
			if let Some(ref dbtr_ntl_regn_nb_value) = self.dbtr_ntl_regn_nb { if let Err(e) = dbtr_ntl_regn_nb_value.validate() { return Err(e); } }
			if let Some(ref cdtr_value) = self.cdtr { if let Err(e) = cdtr_value.validate() { return Err(e); } }
			if let Err(e) = self.dbtr_agt.validate() { return Err(e); }
			if let Some(ref dbtr_agt_brnch_value) = self.dbtr_agt_brnch { if let Err(e) = dbtr_agt_brnch_value.validate() { return Err(e); } }
			if let Some(ref cdtr_agt_value) = self.cdtr_agt { if let Err(e) = cdtr_agt_value.validate() { return Err(e); } }
			if let Some(ref cdtr_agt_brnch_value) = self.cdtr_agt_brnch { if let Err(e) = cdtr_agt_brnch_value.validate() { return Err(e); } }
			if let Some(ref regn_id_value) = self.regn_id { if let Err(e) = regn_id_value.validate() { return Err(e); } }
			if let Some(ref mndt_id_value) = self.mndt_id { if let Err(e) = mndt_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DisabledReason2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum DisabledReason2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLOS") )]
		CodeCLOS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BKRP") )]
		CodeBKRP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CMMT") )]
		CodeCMMT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CNFS") )]
		CodeCNFS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MORT") )]
		CodeMORT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PCOM") )]
		CodePCOM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PLDG") )]
		CodePLDG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRPE") )]
		CodeTRPE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SANC") )]
		CodeSANC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRAN") )]
		CodeTRAN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REJT") )]
		CodeREJT,
	}
	
	impl DisabledReason2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// DisabledStatusReason1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DisabledStatusReason1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: DisabledStatusReason2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Max350Text>,
	}
	
	impl DisabledStatusReason1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DisabledStatusReason1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DisabledStatusReason1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
		pub no_spcfd_rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Vec<DisabledStatusReason1>>,
	}
	
	impl DisabledStatusReason1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if let Err(e) = no_spcfd_rsn_value.validate() { return Err(e); } }
			if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// DisabledStatusReason2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DisabledStatusReason2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<DisabledReason2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification36>,
	}
	
	impl DisabledStatusReason2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DistributionPolicy1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum DistributionPolicy1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DIST") )]
		CodeDIST,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACCU") )]
		CodeACCU,
	}
	
	impl DistributionPolicy1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// DocumentToSend4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentToSend4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: Max140Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rcpt") )]
		pub rcpt: PartyIdentification125Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MtdOfTrnsmssn") )]
		pub mtd_of_trnsmssn: CommunicationMethod3Choice,
	}
	
	impl DocumentToSend4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Err(e) = self.rcpt.validate() { return Err(e); }
			if let Err(e) = self.mtd_of_trnsmssn.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// DutchIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct DutchIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub dutch_identifier: String,
	}
	
	impl DutchIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Eligible1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Eligible1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ELIG") )]
		CodeELIG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NELI") )]
		CodeNELI,
	}
	
	impl Eligible1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// EnabledStatusReason1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EnabledStatusReason1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: EnabledStatusReason2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Max350Text>,
	}
	
	impl EnabledStatusReason1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EnabledStatusReason1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EnabledStatusReason1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
		pub no_spcfd_rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Vec<EnabledStatusReason1>>,
	}
	
	impl EnabledStatusReason1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if let Err(e) = no_spcfd_rsn_value.validate() { return Err(e); } }
			if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// EnabledStatusReason1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum EnabledStatusReason1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "MODI") )]
		CodeMODI,
	}
	
	impl EnabledStatusReason1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// EnabledStatusReason2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EnabledStatusReason2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<EnabledStatusReason1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification36>,
	}
	
	impl EnabledStatusReason2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EuroclearClearstreamIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct EuroclearClearstreamIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub euroclear_clearstream_identifier: String,
	}
	
	impl EuroclearClearstreamIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.euroclear_clearstream_identifier.chars().count() < 1 {
			return Err(ValidationError::new(1001, "euroclear_clearstream_identifier is shorter than the minimum length of 1".to_string()));
			}
			if self.euroclear_clearstream_identifier.chars().count() > 12 {
				return Err(ValidationError::new(1002, "euroclear_clearstream_identifier exceeds the maximum length of 12".to_string()));
			}
			Ok(())
		}
	}
	
	
	// EventFrequency10Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum EventFrequency10Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
		CodeDAIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADHO") )]
		CodeADHO,
	}
	
	impl EventFrequency10Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// EventFrequency1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum EventFrequency1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
		CodeYEAR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SEMI") )]
		CodeSEMI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QUTR") )]
		CodeQUTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TOMN") )]
		CodeTOMN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
		CodeMNTH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TWMN") )]
		CodeTWMN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TOWK") )]
		CodeTOWK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
		CodeWEEK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
		CodeDAIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADHO") )]
		CodeADHO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INDA") )]
		CodeINDA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OVNG") )]
		CodeOVNG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ONDE") )]
		CodeONDE,
	}
	
	impl EventFrequency1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// EventFrequency8Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum EventFrequency8Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADHO") )]
		CodeADHO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
		CodeYEAR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
		CodeDAIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FOMN") )]
		CodeFOMN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TOMN") )]
		CodeTOMN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TOWK") )]
		CodeTOWK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TYEA") )]
		CodeTYEA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INDA") )]
		CodeINDA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
		CodeMNTH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ONDE") )]
		CodeONDE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OVNG") )]
		CodeOVNG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QUTR") )]
		CodeQUTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SEMI") )]
		CodeSEMI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TWMN") )]
		CodeTWMN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
		CodeWEEK,
	}
	
	impl EventFrequency8Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// EventFrequency9Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum EventFrequency9Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
		CodeYEAR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SEMI") )]
		CodeSEMI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QUTR") )]
		CodeQUTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TOMN") )]
		CodeTOMN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
		CodeMNTH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TWMN") )]
		CodeTWMN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TOWK") )]
		CodeTOWK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
		CodeWEEK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
		CodeDAIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADHO") )]
		CodeADHO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INDA") )]
		CodeINDA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OVNG") )]
		CodeOVNG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ONDE") )]
		CodeONDE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NONE") )]
		CodeNONE,
	}
	
	impl EventFrequency9Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Exact4AlphaNumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Exact4AlphaNumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub exact4_alpha_numeric_text: String,
	}
	
	impl Exact4AlphaNumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
			if !pattern.is_match(&self.exact4_alpha_numeric_text) {
				return Err(ValidationError::new(1005, "exact4_alpha_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Extended350Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Extended350Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub extended350_code: String,
	}
	
	impl Extended350Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.extended350_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "extended350_code is shorter than the minimum length of 1".to_string()));
			}
			if self.extended350_code.chars().count() > 350 {
				return Err(ValidationError::new(1002, "extended350_code exceeds the maximum length of 350".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExtendedParty14 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ExtendedParty14 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "XtndedPtyRole") )]
		pub xtnded_pty_role: Extended350Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPtyDtls") )]
		pub othr_pty_dtls: InvestmentAccountOwnershipInformation16,
	}
	
	impl ExtendedParty14 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.xtnded_pty_role.validate() { return Err(e); }
			if let Err(e) = self.othr_pty_dtls.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// Extension1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Extension1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PlcAndNm") )]
		pub plc_and_nm: Max350Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Txt") )]
		pub txt: Max350Text,
	}
	
	impl Extension1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.plc_and_nm.validate() { return Err(e); }
			if let Err(e) = self.txt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ExtensiveBranchNetworkIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExtensiveBranchNetworkIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub extensive_branch_network_identifier: String,
	}
	
	impl ExtensiveBranchNetworkIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("AU[0-9]{6,6}").unwrap();
			if !pattern.is_match(&self.extensive_branch_network_identifier) {
				return Err(ValidationError::new(1005, "extensive_branch_network_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalAccountIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalAccountIdentification1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_account_identification1_code: String,
	}
	
	impl ExternalAccountIdentification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_account_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_account_identification1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_account_identification1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_account_identification1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// FATCAForm1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FATCAForm1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<FATCAFormType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl FATCAForm1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FATCAFormType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum FATCAFormType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CER5") )]
		CodeCER5,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CER7") )]
		CodeCER7,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CER1") )]
		CodeCER1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CER2") )]
		CodeCER2,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CER3") )]
		CodeCER3,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CER4") )]
		CodeCER4,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CER6") )]
		CodeCER6,
	}
	
	impl FATCAFormType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// FATCASource1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FATCASource1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<FATCASourceStatus1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl FATCASource1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FATCASourceStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum FATCASourceStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CALC") )]
		CodeCALC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DECL") )]
		CodeDECL,
	}
	
	impl FATCASourceStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// FATCAStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum FATCAStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "F101") )]
		CodeF101,
		#[cfg_attr( feature = "derive_serde", serde(rename = "F102") )]
		CodeF102,
		#[cfg_attr( feature = "derive_serde", serde(rename = "F103") )]
		CodeF103,
		#[cfg_attr( feature = "derive_serde", serde(rename = "F104") )]
		CodeF104,
		#[cfg_attr( feature = "derive_serde", serde(rename = "F105") )]
		CodeF105,
		#[cfg_attr( feature = "derive_serde", serde(rename = "F201") )]
		CodeF201,
		#[cfg_attr( feature = "derive_serde", serde(rename = "F202") )]
		CodeF202,
		#[cfg_attr( feature = "derive_serde", serde(rename = "F203") )]
		CodeF203,
		#[cfg_attr( feature = "derive_serde", serde(rename = "F204") )]
		CodeF204,
		#[cfg_attr( feature = "derive_serde", serde(rename = "F205") )]
		CodeF205,
		#[cfg_attr( feature = "derive_serde", serde(rename = "F206") )]
		CodeF206,
	}
	
	impl FATCAStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// FATCAStatus2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FATCAStatus2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: FATCAStatus2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Src", skip_serializing_if = "Option::is_none") )]
		pub src: Option<FATCASource1Choice>,
	}
	
	impl FATCAStatus2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Some(ref src_value) = self.src { if let Err(e) = src_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FATCAStatus2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FATCAStatus2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<FATCAStatus1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl FATCAStatus2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FedwireRoutingNumberIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct FedwireRoutingNumberIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub fedwire_routing_number_identifier: String,
	}
	
	impl FedwireRoutingNumberIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("FW[0-9]{9,9}").unwrap();
			if !pattern.is_match(&self.fedwire_routing_number_identifier) {
				return Err(ValidationError::new(1005, "fedwire_routing_number_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// FinancialInstitutionIdentification11Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FinancialInstitutionIdentification11Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
		pub nm_and_adr: Option<NameAndAddress5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BICFI", skip_serializing_if = "Option::is_none") )]
		pub bicfi: Option<BICFIDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none") )]
		pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<SimpleIdentificationInformation4>,
	}
	
	impl FinancialInstitutionIdentification11Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nm_and_adr_value) = self.nm_and_adr { if let Err(e) = nm_and_adr_value.validate() { return Err(e); } }
			if let Some(ref bicfi_value) = self.bicfi { if let Err(e) = bicfi_value.validate() { return Err(e); } }
			if let Some(ref clr_sys_mmb_id_value) = self.clr_sys_mmb_id { if let Err(e) = clr_sys_mmb_id_value.validate() { return Err(e); } }
			if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FinancialInstrument87 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FinancialInstrument87 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: SecurityIdentification25Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
		pub shrt_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryId", skip_serializing_if = "Option::is_none") )]
		pub splmtry_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClssTp", skip_serializing_if = "Option::is_none") )]
		pub clss_tp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesForm", skip_serializing_if = "Option::is_none") )]
		pub scties_form: Option<FormOfSecurity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none") )]
		pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdctGrp", skip_serializing_if = "Option::is_none") )]
		pub pdct_grp: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BlckdHldgDtls", skip_serializing_if = "Option::is_none") )]
		pub blckd_hldg_dtls: Option<BlockedHoldingDetails2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pldgg", skip_serializing_if = "Option::is_none") )]
		pub pldgg: Option<Eligible1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Coll", skip_serializing_if = "Option::is_none") )]
		pub coll: Option<Collateral1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ThrdPtyRghts", skip_serializing_if = "Option::is_none") )]
		pub thrd_pty_rghts: Option<ThirdPartyRights2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FndOwnrsh", skip_serializing_if = "Option::is_none") )]
		pub fnd_ownrsh: Option<FundOwnership1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FndIntntn", skip_serializing_if = "Option::is_none") )]
		pub fnd_intntn: Option<FundIntention1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OprlSts", skip_serializing_if = "Option::is_none") )]
		pub oprl_sts: Option<OperationalStatus1Code>,
	}
	
	impl FinancialInstrument87 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref shrt_nm_value) = self.shrt_nm { if let Err(e) = shrt_nm_value.validate() { return Err(e); } }
			if let Some(ref splmtry_id_value) = self.splmtry_id { if let Err(e) = splmtry_id_value.validate() { return Err(e); } }
			if let Some(ref clss_tp_value) = self.clss_tp { if let Err(e) = clss_tp_value.validate() { return Err(e); } }
			if let Some(ref scties_form_value) = self.scties_form { if let Err(e) = scties_form_value.validate() { return Err(e); } }
			if let Some(ref dstrbtn_plcy_value) = self.dstrbtn_plcy { if let Err(e) = dstrbtn_plcy_value.validate() { return Err(e); } }
			if let Some(ref pdct_grp_value) = self.pdct_grp { if let Err(e) = pdct_grp_value.validate() { return Err(e); } }
			if let Some(ref blckd_hldg_dtls_value) = self.blckd_hldg_dtls { if let Err(e) = blckd_hldg_dtls_value.validate() { return Err(e); } }
			if let Some(ref pldgg_value) = self.pldgg { if let Err(e) = pldgg_value.validate() { return Err(e); } }
			if let Some(ref coll_value) = self.coll { if let Err(e) = coll_value.validate() { return Err(e); } }
			if let Some(ref thrd_pty_rghts_value) = self.thrd_pty_rghts { if let Err(e) = thrd_pty_rghts_value.validate() { return Err(e); } }
			if let Some(ref fnd_ownrsh_value) = self.fnd_ownrsh { if let Err(e) = fnd_ownrsh_value.validate() { return Err(e); } }
			if let Some(ref fnd_intntn_value) = self.fnd_intntn { if let Err(e) = fnd_intntn_value.validate() { return Err(e); } }
			if let Some(ref oprl_sts_value) = self.oprl_sts { if let Err(e) = oprl_sts_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FiscalYear1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FiscalYear1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
		pub start_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt", skip_serializing_if = "Option::is_none") )]
		pub end_dt: Option<String>,
	}
	
	impl FiscalYear1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// FormOfSecurity1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum FormOfSecurity1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "BEAR") )]
		CodeBEAR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REGD") )]
		CodeREGD,
	}
	
	impl FormOfSecurity1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Frequency20Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Frequency20Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<EventFrequency8Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl Frequency20Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FundCashAccount4Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum FundCashAccount4Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "HEDG") )]
		CodeHEDG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CPFO") )]
		CodeCPFO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CPFS") )]
		CodeCPFS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SRSA") )]
		CodeSRSA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CSDO") )]
		CodeCSDO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TOFF") )]
		CodeTOFF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ICSA") )]
		CodeICSA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CSDM") )]
		CodeCSDM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CSDP") )]
		CodeCSDP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PPEN") )]
		CodePPEN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CPEN") )]
		CodeCPEN,
	}
	
	impl FundCashAccount4Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// FundIntention1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum FundIntention1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "YQUA") )]
		CodeYQUA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NQUA") )]
		CodeNQUA,
	}
	
	impl FundIntention1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// FundOwnership1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum FundOwnership1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "YALL") )]
		CodeYALL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NALL") )]
		CodeNALL,
	}
	
	impl FundOwnership1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// GDPRData1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GDPRData1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CnsntTp") )]
		pub cnsnt_tp: GDPRDataConsent1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CnsntInd") )]
		pub cnsnt_ind: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CnsntDt") )]
		pub cnsnt_dt: String,
	}
	
	impl GDPRData1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cnsnt_tp.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// GDPRDataConsent1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GDPRDataConsent1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<GDPRDataConsent1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl GDPRDataConsent1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GDPRDataConsent1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum GDPRDataConsent1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DP00") )]
		CodeDP00,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DP03") )]
		CodeDP03,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DP01") )]
		CodeDP01,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DP02") )]
		CodeDP02,
	}
	
	impl GDPRDataConsent1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Gender1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Gender1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "FEMA") )]
		CodeFEMA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MALE") )]
		CodeMALE,
	}
	
	impl Gender1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// GenericAccountIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericAccountIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max34Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<AccountSchemeName1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericAccountIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericIdentification36 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericIdentification36 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
	}
	
	impl GenericIdentification36 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.issr.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericIdentification47 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericIdentification47 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Exact4AlphaNumericText,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: Max4AlphaNumericText,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max4AlphaNumericText>,
	}
	
	impl GenericIdentification47 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.issr.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericIdentification82 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericIdentification82 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: OtherIdentification3Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IsseDt", skip_serializing_if = "Option::is_none") )]
		pub isse_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt", skip_serializing_if = "Option::is_none") )]
		pub xpry_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Stat", skip_serializing_if = "Option::is_none") )]
		pub stat: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IssrCtry", skip_serializing_if = "Option::is_none") )]
		pub issr_ctry: Option<CountryCode>,
	}
	
	impl GenericIdentification82 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			if let Some(ref stat_value) = self.stat { if let Err(e) = stat_value.validate() { return Err(e); } }
			if let Some(ref issr_ctry_value) = self.issr_ctry { if let Err(e) = issr_ctry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GermanBankleitzahlIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct GermanBankleitzahlIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub german_bankleitzahl_identifier: String,
	}
	
	impl GermanBankleitzahlIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("BL[0-9]{8,8}").unwrap();
			if !pattern.is_match(&self.german_bankleitzahl_identifier) {
				return Err(ValidationError::new(1005, "german_bankleitzahl_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// HighFrequencyTradingProfile1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct HighFrequencyTradingProfile1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
		pub dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmFrqcy", skip_serializing_if = "Option::is_none") )]
		pub sttlm_frqcy: Option<SettlementFrequency1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CnsldtnTp", skip_serializing_if = "Option::is_none") )]
		pub cnsldtn_tp: Option<ConsolidationType1Choice>,
	}
	
	impl HighFrequencyTradingProfile1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref sttlm_frqcy_value) = self.sttlm_frqcy { if let Err(e) = sttlm_frqcy_value.validate() { return Err(e); } }
			if let Some(ref cnsldtn_tp_value) = self.cnsldtn_tp { if let Err(e) = cnsldtn_tp_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Holding1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Holding1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CERT") )]
		CodeCERT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NPRH") )]
		CodeNPRH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRTH") )]
		CodePRTH,
	}
	
	impl Holding1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// HongKongBankIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct HongKongBankIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub hong_kong_bank_identifier: String,
	}
	
	impl HongKongBankIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("HK[0-9]{3,3}").unwrap();
			if !pattern.is_match(&self.hong_kong_bank_identifier) {
				return Err(ValidationError::new(1005, "hong_kong_bank_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// IBAN2007Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct IBAN2007Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iban2007_identifier: String,
	}
	
	impl IBAN2007Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}").unwrap();
			if !pattern.is_match(&self.iban2007_identifier) {
				return Err(ValidationError::new(1005, "iban2007_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ISINOct2015Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISINOct2015Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub isin_oct2015_identifier: String,
	}
	
	impl ISINOct2015Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(&self.isin_oct2015_identifier) {
				return Err(ValidationError::new(1005, "isin_oct2015_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ISODate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISODate {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_date: String,
	}
	
	impl ISODate {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ISODateTime ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISODateTime {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_date_time: String,
	}
	
	impl ISODateTime {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ISOYearMonth ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISOYearMonth {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_year_month: String,
	}
	
	impl ISOYearMonth {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// IdentificationSource1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IdentificationSource1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dmst", skip_serializing_if = "Option::is_none") )]
		pub dmst: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl IdentificationSource1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref dmst_value) = self.dmst { if let Err(e) = dmst_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// IncomePreference2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum IncomePreference2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CASH") )]
		CodeCASH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SECU") )]
		CodeSECU,
	}
	
	impl IncomePreference2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// IndividualPerson29 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IndividualPerson29 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none") )]
		pub nm_prfx: Option<NamePrefix1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GvnNm", skip_serializing_if = "Option::is_none") )]
		pub gvn_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MddlNm", skip_serializing_if = "Option::is_none") )]
		pub mddl_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: Max350Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr") )]
		pub pstl_adr: Vec<PostalAddress21>,
	}
	
	impl IndividualPerson29 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nm_prfx_value) = self.nm_prfx { if let Err(e) = nm_prfx_value.validate() { return Err(e); } }
			if let Some(ref gvn_nm_value) = self.gvn_nm { if let Err(e) = gvn_nm_value.validate() { return Err(e); } }
			if let Some(ref mddl_nm_value) = self.mddl_nm { if let Err(e) = mddl_nm_value.validate() { return Err(e); } }
			if let Err(e) = self.nm.validate() { return Err(e); }
			for item in &self.pstl_adr { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// IndividualPerson37 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IndividualPerson37 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none") )]
		pub nm_prfx: Option<NamePrefix1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GvnNm", skip_serializing_if = "Option::is_none") )]
		pub gvn_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MddlNm", skip_serializing_if = "Option::is_none") )]
		pub mddl_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: Max350Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmSfx", skip_serializing_if = "Option::is_none") )]
		pub nm_sfx: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Gndr", skip_serializing_if = "Option::is_none") )]
		pub gndr: Option<Gender1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BirthDt", skip_serializing_if = "Option::is_none") )]
		pub birth_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfBirth", skip_serializing_if = "Option::is_none") )]
		pub ctry_of_birth: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none") )]
		pub prvc_of_birth: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CityOfBirth", skip_serializing_if = "Option::is_none") )]
		pub city_of_birth: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prfssn", skip_serializing_if = "Option::is_none") )]
		pub prfssn: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr") )]
		pub pstl_adr: Vec<PostalAddress21>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctznsh", skip_serializing_if = "Option::is_none") )]
		pub ctznsh: Option<Vec<CitizenshipInformation2>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EmplngCpny", skip_serializing_if = "Option::is_none") )]
		pub emplng_cpny: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BizFctn", skip_serializing_if = "Option::is_none") )]
		pub biz_fctn: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PltclyXpsdPrsn", skip_serializing_if = "Option::is_none") )]
		pub pltcly_xpsd_prsn: Option<PoliticallyExposedPerson1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DthDt", skip_serializing_if = "Option::is_none") )]
		pub dth_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CvlSts", skip_serializing_if = "Option::is_none") )]
		pub cvl_sts: Option<CivilStatus1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EdctnLvl", skip_serializing_if = "Option::is_none") )]
		pub edctn_lvl: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FmlyInf", skip_serializing_if = "Option::is_none") )]
		pub fmly_inf: Option<PersonalInformation1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GDPRData", skip_serializing_if = "Option::is_none") )]
		pub gdpr_data: Option<Vec<GDPRData1>>,
	}
	
	impl IndividualPerson37 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nm_prfx_value) = self.nm_prfx { if let Err(e) = nm_prfx_value.validate() { return Err(e); } }
			if let Some(ref gvn_nm_value) = self.gvn_nm { if let Err(e) = gvn_nm_value.validate() { return Err(e); } }
			if let Some(ref mddl_nm_value) = self.mddl_nm { if let Err(e) = mddl_nm_value.validate() { return Err(e); } }
			if let Err(e) = self.nm.validate() { return Err(e); }
			if let Some(ref nm_sfx_value) = self.nm_sfx { if let Err(e) = nm_sfx_value.validate() { return Err(e); } }
			if let Some(ref gndr_value) = self.gndr { if let Err(e) = gndr_value.validate() { return Err(e); } }
			if let Some(ref ctry_of_birth_value) = self.ctry_of_birth { if let Err(e) = ctry_of_birth_value.validate() { return Err(e); } }
			if let Some(ref prvc_of_birth_value) = self.prvc_of_birth { if let Err(e) = prvc_of_birth_value.validate() { return Err(e); } }
			if let Some(ref city_of_birth_value) = self.city_of_birth { if let Err(e) = city_of_birth_value.validate() { return Err(e); } }
			if let Some(ref prfssn_value) = self.prfssn { if let Err(e) = prfssn_value.validate() { return Err(e); } }
			for item in &self.pstl_adr { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref ctznsh_vec) = self.ctznsh { for item in ctznsh_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref emplng_cpny_value) = self.emplng_cpny { if let Err(e) = emplng_cpny_value.validate() { return Err(e); } }
			if let Some(ref biz_fctn_value) = self.biz_fctn { if let Err(e) = biz_fctn_value.validate() { return Err(e); } }
			if let Some(ref pltcly_xpsd_prsn_value) = self.pltcly_xpsd_prsn { if let Err(e) = pltcly_xpsd_prsn_value.validate() { return Err(e); } }
			if let Some(ref cvl_sts_value) = self.cvl_sts { if let Err(e) = cvl_sts_value.validate() { return Err(e); } }
			if let Some(ref edctn_lvl_value) = self.edctn_lvl { if let Err(e) = edctn_lvl_value.validate() { return Err(e); } }
			if let Some(ref fmly_inf_value) = self.fmly_inf { if let Err(e) = fmly_inf_value.validate() { return Err(e); } }
			if let Some(ref gdpr_data_vec) = self.gdpr_data { for item in gdpr_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// InformationDistribution1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InformationDistribution1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<InformationDistribution2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl InformationDistribution1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InformationDistribution2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum InformationDistribution2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ELEC") )]
		CodeELEC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NONE") )]
		CodeNONE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PAPR") )]
		CodePAPR,
	}
	
	impl InformationDistribution2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// InitialAmount1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InitialAmount1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InitlNbOfInstlmts", skip_serializing_if = "Option::is_none") )]
		pub initl_nb_of_instlmts: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<ActiveCurrencyAndAmount>,
	}
	
	impl InitialAmount1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Insurance1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Insurance1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "LIFE") )]
		CodeLIFE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PDIS") )]
		CodePDIS,
	}
	
	impl Insurance1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// InsuranceType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InsuranceType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<Insurance1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl InsuranceType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Intermediary46 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Intermediary46 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: PartyIdentification177Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none") )]
		pub lgl_ntty_idr: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Acct", skip_serializing_if = "Option::is_none") )]
		pub acct: Option<Account32>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WvdTrlrComssnInd", skip_serializing_if = "Option::is_none") )]
		pub wvd_trlr_comssn_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Role", skip_serializing_if = "Option::is_none") )]
		pub role: Option<PartyRole2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmryComAdr", skip_serializing_if = "Option::is_none") )]
		pub pmry_com_adr: Option<Vec<CommunicationAddress6>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ScndryComAdr", skip_serializing_if = "Option::is_none") )]
		pub scndry_com_adr: Option<Vec<CommunicationAddress6>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
		pub nm_and_adr: Option<NameAndAddress4>,
	}
	
	impl Intermediary46 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref lgl_ntty_idr_value) = self.lgl_ntty_idr { if let Err(e) = lgl_ntty_idr_value.validate() { return Err(e); } }
			if let Some(ref acct_value) = self.acct { if let Err(e) = acct_value.validate() { return Err(e); } }
			if let Some(ref role_value) = self.role { if let Err(e) = role_value.validate() { return Err(e); } }
			if let Some(ref pmry_com_adr_vec) = self.pmry_com_adr { for item in pmry_com_adr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref scndry_com_adr_vec) = self.scndry_com_adr { for item in scndry_com_adr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref nm_and_adr_value) = self.nm_and_adr { if let Err(e) = nm_and_adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InvestmentAccount74 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InvestmentAccount74 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSts", skip_serializing_if = "Option::is_none") )]
		pub acct_sts: Option<AccountStatus2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BlckdSts", skip_serializing_if = "Option::is_none") )]
		pub blckd_sts: Option<BlockedStatusReason2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StsDt", skip_serializing_if = "Option::is_none") )]
		pub sts_dt: Option<DateAndDateTime1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dsgnt", skip_serializing_if = "Option::is_none") )]
		pub dsgnt: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<AccountType2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OwnrshTp", skip_serializing_if = "Option::is_none") )]
		pub ownrsh_tp: Option<OwnershipType2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxXmptn", skip_serializing_if = "Option::is_none") )]
		pub tax_xmptn: Option<TaxExemptionReason2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StmtFrqcy", skip_serializing_if = "Option::is_none") )]
		pub stmt_frqcy: Option<StatementFrequencyReason2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefCcy", skip_serializing_if = "Option::is_none") )]
		pub ref_ccy: Option<ActiveCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Lang", skip_serializing_if = "Option::is_none") )]
		pub lang: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IncmPref", skip_serializing_if = "Option::is_none") )]
		pub incm_pref: Option<IncomePreference2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RinvstmtDtls", skip_serializing_if = "Option::is_none") )]
		pub rinvstmt_dtls: Option<Vec<Reinvestment4>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxWhldgMtd", skip_serializing_if = "Option::is_none") )]
		pub tax_whldg_mtd: Option<TaxWithholdingMethod3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRptg", skip_serializing_if = "Option::is_none") )]
		pub tax_rptg: Option<Vec<TaxReporting3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LttrInttDtls", skip_serializing_if = "Option::is_none") )]
		pub lttr_intt_dtls: Option<LetterIntent1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcmltnRghtRef", skip_serializing_if = "Option::is_none") )]
		pub acmltn_rght_ref: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ReqrdSgntriesNb", skip_serializing_if = "Option::is_none") )]
		pub reqrd_sgntries_nb: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FndFmlyNm", skip_serializing_if = "Option::is_none") )]
		pub fnd_fmly_nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmDtls", skip_serializing_if = "Option::is_none") )]
		pub fin_instrm_dtls: Option<Vec<FinancialInstrument87>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RndgDtls", skip_serializing_if = "Option::is_none") )]
		pub rndg_dtls: Option<RoundingParameters1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none") )]
		pub acct_svcr: Option<PartyIdentification125Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctUsgTp", skip_serializing_if = "Option::is_none") )]
		pub acct_usg_tp: Option<AccountUsageType2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrgnStsCertfctn", skip_serializing_if = "Option::is_none") )]
		pub frgn_sts_certfctn: Option<Provided1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSgntrDtTm", skip_serializing_if = "Option::is_none") )]
		pub acct_sgntr_dt_tm: Option<DateAndDateTime1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxChanlTp", skip_serializing_if = "Option::is_none") )]
		pub tx_chanl_tp: Option<TransactionChannelType1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtAcctCtgy", skip_serializing_if = "Option::is_none") )]
		pub invstmt_acct_ctgy: Option<InvestmentAccountCategory1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pldgg", skip_serializing_if = "Option::is_none") )]
		pub pldgg: Option<Eligible1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Coll", skip_serializing_if = "Option::is_none") )]
		pub coll: Option<Collateral1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ThrdPtyRghts", skip_serializing_if = "Option::is_none") )]
		pub thrd_pty_rghts: Option<ThirdPartyRights2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PwrOfAttnyLvlOfCtrl", skip_serializing_if = "Option::is_none") )]
		pub pwr_of_attny_lvl_of_ctrl: Option<LevelOfControl1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctgSts", skip_serializing_if = "Option::is_none") )]
		pub acctg_sts: Option<AccountingStatus1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OpngDt", skip_serializing_if = "Option::is_none") )]
		pub opng_dt: Option<DateAndDateTime1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
		pub clsg_dt: Option<DateAndDateTime1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NegInd", skip_serializing_if = "Option::is_none") )]
		pub neg_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgOrdr", skip_serializing_if = "Option::is_none") )]
		pub prcg_ordr: Option<PositionEffect3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Lblty", skip_serializing_if = "Option::is_none") )]
		pub lblty: Option<Liability1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrPrfl", skip_serializing_if = "Option::is_none") )]
		pub invstr_prfl: Option<Vec<InvestorProfile2>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FsclYr", skip_serializing_if = "Option::is_none") )]
		pub fscl_yr: Option<FiscalYear1Choice>,
	}
	
	impl InvestmentAccount74 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref acct_sts_value) = self.acct_sts { if let Err(e) = acct_sts_value.validate() { return Err(e); } }
			if let Some(ref blckd_sts_value) = self.blckd_sts { if let Err(e) = blckd_sts_value.validate() { return Err(e); } }
			if let Some(ref sts_dt_value) = self.sts_dt { if let Err(e) = sts_dt_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref dsgnt_value) = self.dsgnt { if let Err(e) = dsgnt_value.validate() { return Err(e); } }
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref ownrsh_tp_value) = self.ownrsh_tp { if let Err(e) = ownrsh_tp_value.validate() { return Err(e); } }
			if let Some(ref tax_xmptn_value) = self.tax_xmptn { if let Err(e) = tax_xmptn_value.validate() { return Err(e); } }
			if let Some(ref stmt_frqcy_value) = self.stmt_frqcy { if let Err(e) = stmt_frqcy_value.validate() { return Err(e); } }
			if let Some(ref ref_ccy_value) = self.ref_ccy { if let Err(e) = ref_ccy_value.validate() { return Err(e); } }
			if let Some(ref incm_pref_value) = self.incm_pref { if let Err(e) = incm_pref_value.validate() { return Err(e); } }
			if let Some(ref rinvstmt_dtls_vec) = self.rinvstmt_dtls { for item in rinvstmt_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref tax_whldg_mtd_value) = self.tax_whldg_mtd { if let Err(e) = tax_whldg_mtd_value.validate() { return Err(e); } }
			if let Some(ref tax_rptg_vec) = self.tax_rptg { for item in tax_rptg_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref lttr_intt_dtls_value) = self.lttr_intt_dtls { if let Err(e) = lttr_intt_dtls_value.validate() { return Err(e); } }
			if let Some(ref acmltn_rght_ref_value) = self.acmltn_rght_ref { if let Err(e) = acmltn_rght_ref_value.validate() { return Err(e); } }
			if let Some(ref fnd_fmly_nm_value) = self.fnd_fmly_nm { if let Err(e) = fnd_fmly_nm_value.validate() { return Err(e); } }
			if let Some(ref fin_instrm_dtls_vec) = self.fin_instrm_dtls { for item in fin_instrm_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref rndg_dtls_value) = self.rndg_dtls { if let Err(e) = rndg_dtls_value.validate() { return Err(e); } }
			if let Some(ref acct_svcr_value) = self.acct_svcr { if let Err(e) = acct_svcr_value.validate() { return Err(e); } }
			if let Some(ref acct_usg_tp_value) = self.acct_usg_tp { if let Err(e) = acct_usg_tp_value.validate() { return Err(e); } }
			if let Some(ref frgn_sts_certfctn_value) = self.frgn_sts_certfctn { if let Err(e) = frgn_sts_certfctn_value.validate() { return Err(e); } }
			if let Some(ref acct_sgntr_dt_tm_value) = self.acct_sgntr_dt_tm { if let Err(e) = acct_sgntr_dt_tm_value.validate() { return Err(e); } }
			if let Some(ref tx_chanl_tp_value) = self.tx_chanl_tp { if let Err(e) = tx_chanl_tp_value.validate() { return Err(e); } }
			if let Some(ref invstmt_acct_ctgy_value) = self.invstmt_acct_ctgy { if let Err(e) = invstmt_acct_ctgy_value.validate() { return Err(e); } }
			if let Some(ref pldgg_value) = self.pldgg { if let Err(e) = pldgg_value.validate() { return Err(e); } }
			if let Some(ref coll_value) = self.coll { if let Err(e) = coll_value.validate() { return Err(e); } }
			if let Some(ref thrd_pty_rghts_value) = self.thrd_pty_rghts { if let Err(e) = thrd_pty_rghts_value.validate() { return Err(e); } }
			if let Some(ref pwr_of_attny_lvl_of_ctrl_value) = self.pwr_of_attny_lvl_of_ctrl { if let Err(e) = pwr_of_attny_lvl_of_ctrl_value.validate() { return Err(e); } }
			if let Some(ref acctg_sts_value) = self.acctg_sts { if let Err(e) = acctg_sts_value.validate() { return Err(e); } }
			if let Some(ref opng_dt_value) = self.opng_dt { if let Err(e) = opng_dt_value.validate() { return Err(e); } }
			if let Some(ref clsg_dt_value) = self.clsg_dt { if let Err(e) = clsg_dt_value.validate() { return Err(e); } }
			if let Some(ref prcg_ordr_value) = self.prcg_ordr { if let Err(e) = prcg_ordr_value.validate() { return Err(e); } }
			if let Some(ref lblty_value) = self.lblty { if let Err(e) = lblty_value.validate() { return Err(e); } }
			if let Some(ref invstr_prfl_vec) = self.invstr_prfl { for item in invstr_prfl_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref fscl_yr_value) = self.fscl_yr { if let Err(e) = fscl_yr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InvestmentAccountCategory1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InvestmentAccountCategory1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<InvestmentAccountCategory1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl InvestmentAccountCategory1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InvestmentAccountCategory1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum InvestmentAccountCategory1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "MAND") )]
		CodeMAND,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RETA") )]
		CodeRETA,
	}
	
	impl InvestmentAccountCategory1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// InvestmentAccountOwnershipInformation16 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InvestmentAccountOwnershipInformation16 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pty") )]
		pub pty: Party47Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MnyLndrgChck", skip_serializing_if = "Option::is_none") )]
		pub mny_lndrg_chck: Option<MoneyLaunderingCheck1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrPrflVldtn", skip_serializing_if = "Option::is_none") )]
		pub invstr_prfl_vldtn: Option<Vec<PartyProfileInformation5>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OwnrshBnfcryRate", skip_serializing_if = "Option::is_none") )]
		pub ownrsh_bnfcry_rate: Option<OwnershipBeneficiaryRate1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClntId", skip_serializing_if = "Option::is_none") )]
		pub clnt_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FsclXmptn", skip_serializing_if = "Option::is_none") )]
		pub fscl_xmptn: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SgntryRghtInd", skip_serializing_if = "Option::is_none") )]
		pub sgntry_rght_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MiFIDClssfctn", skip_serializing_if = "Option::is_none") )]
		pub mi_fid_clssfctn: Option<MiFIDClassification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ntfctn", skip_serializing_if = "Option::is_none") )]
		pub ntfctn: Option<Vec<Notification2>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FATCAFormTp", skip_serializing_if = "Option::is_none") )]
		pub fatca_form_tp: Option<Vec<FATCAForm1Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FATCASts", skip_serializing_if = "Option::is_none") )]
		pub fatca_sts: Option<Vec<FATCAStatus2>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FATCARptgDt", skip_serializing_if = "Option::is_none") )]
		pub fatca_rptg_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CRSFormTp", skip_serializing_if = "Option::is_none") )]
		pub crs_form_tp: Option<Vec<CRSForm1Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CRSSts", skip_serializing_if = "Option::is_none") )]
		pub crs_sts: Option<Vec<CRSStatus4>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CRSRptgDt", skip_serializing_if = "Option::is_none") )]
		pub crs_rptg_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
		pub othr_id: Option<Vec<GenericIdentification82>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxXmptn", skip_serializing_if = "Option::is_none") )]
		pub tax_xmptn: Option<TaxExemptionReason2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRptg", skip_serializing_if = "Option::is_none") )]
		pub tax_rptg: Option<Vec<TaxReporting3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Lang", skip_serializing_if = "Option::is_none") )]
		pub lang: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MailTp", skip_serializing_if = "Option::is_none") )]
		pub mail_tp: Option<MailType1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryAndResdtlSts", skip_serializing_if = "Option::is_none") )]
		pub ctry_and_resdtl_sts: Option<CountryAndResidentialStatusType2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MntryWlth", skip_serializing_if = "Option::is_none") )]
		pub mntry_wlth: Option<DateAndAmount1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EqtyVal", skip_serializing_if = "Option::is_none") )]
		pub eqty_val: Option<DateAndAmount1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WorkgCptl", skip_serializing_if = "Option::is_none") )]
		pub workg_cptl: Option<DateAndAmount1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CpnyLk", skip_serializing_if = "Option::is_none") )]
		pub cpny_lk: Option<CompanyLink1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ElctrncMlngSvcRef", skip_serializing_if = "Option::is_none") )]
		pub elctrnc_mlng_svc_ref: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmryComAdr", skip_serializing_if = "Option::is_none") )]
		pub pmry_com_adr: Option<Vec<CommunicationAddress6>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ScndryComAdr", skip_serializing_if = "Option::is_none") )]
		pub scndry_com_adr: Option<Vec<CommunicationAddress6>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRgltryInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_rgltry_inf: Option<RegulatoryInformation1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctgSts", skip_serializing_if = "Option::is_none") )]
		pub acctg_sts: Option<AccountingStatus1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Vec<AdditiononalInformation13>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrlgPty", skip_serializing_if = "Option::is_none") )]
		pub ctrlg_pty: Option<bool>,
	}
	
	impl InvestmentAccountOwnershipInformation16 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pty.validate() { return Err(e); }
			if let Some(ref mny_lndrg_chck_value) = self.mny_lndrg_chck { if let Err(e) = mny_lndrg_chck_value.validate() { return Err(e); } }
			if let Some(ref invstr_prfl_vldtn_vec) = self.invstr_prfl_vldtn { for item in invstr_prfl_vldtn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ownrsh_bnfcry_rate_value) = self.ownrsh_bnfcry_rate { if let Err(e) = ownrsh_bnfcry_rate_value.validate() { return Err(e); } }
			if let Some(ref clnt_id_value) = self.clnt_id { if let Err(e) = clnt_id_value.validate() { return Err(e); } }
			if let Some(ref mi_fid_clssfctn_value) = self.mi_fid_clssfctn { if let Err(e) = mi_fid_clssfctn_value.validate() { return Err(e); } }
			if let Some(ref ntfctn_vec) = self.ntfctn { for item in ntfctn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref fatca_form_tp_vec) = self.fatca_form_tp { for item in fatca_form_tp_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref fatca_sts_vec) = self.fatca_sts { for item in fatca_sts_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref crs_form_tp_vec) = self.crs_form_tp { for item in crs_form_tp_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref crs_sts_vec) = self.crs_sts { for item in crs_sts_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref othr_id_vec) = self.othr_id { for item in othr_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref tax_xmptn_value) = self.tax_xmptn { if let Err(e) = tax_xmptn_value.validate() { return Err(e); } }
			if let Some(ref tax_rptg_vec) = self.tax_rptg { for item in tax_rptg_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref mail_tp_value) = self.mail_tp { if let Err(e) = mail_tp_value.validate() { return Err(e); } }
			if let Some(ref ctry_and_resdtl_sts_value) = self.ctry_and_resdtl_sts { if let Err(e) = ctry_and_resdtl_sts_value.validate() { return Err(e); } }
			if let Some(ref mntry_wlth_value) = self.mntry_wlth { if let Err(e) = mntry_wlth_value.validate() { return Err(e); } }
			if let Some(ref eqty_val_value) = self.eqty_val { if let Err(e) = eqty_val_value.validate() { return Err(e); } }
			if let Some(ref workg_cptl_value) = self.workg_cptl { if let Err(e) = workg_cptl_value.validate() { return Err(e); } }
			if let Some(ref cpny_lk_value) = self.cpny_lk { if let Err(e) = cpny_lk_value.validate() { return Err(e); } }
			if let Some(ref elctrnc_mlng_svc_ref_value) = self.elctrnc_mlng_svc_ref { if let Err(e) = elctrnc_mlng_svc_ref_value.validate() { return Err(e); } }
			if let Some(ref pmry_com_adr_vec) = self.pmry_com_adr { for item in pmry_com_adr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref scndry_com_adr_vec) = self.scndry_com_adr { for item in scndry_com_adr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref addtl_rgltry_inf_value) = self.addtl_rgltry_inf { if let Err(e) = addtl_rgltry_inf_value.validate() { return Err(e); } }
			if let Some(ref acctg_sts_value) = self.acctg_sts { if let Err(e) = acctg_sts_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// InvestmentFundOrder4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InvestmentFundOrder4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrRef", skip_serializing_if = "Option::is_none") )]
		pub ordr_ref: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MstrRef", skip_serializing_if = "Option::is_none") )]
		pub mstr_ref: Option<Max35Text>,
	}
	
	impl InvestmentFundOrder4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ordr_ref_value) = self.ordr_ref { if let Err(e) = ordr_ref_value.validate() { return Err(e); } }
			if let Some(ref mstr_ref_value) = self.mstr_ref { if let Err(e) = mstr_ref_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InvestmentFundRole6Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum InvestmentFundRole6Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CACO") )]
		CodeCACO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CONC") )]
		CodeCONC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CUST") )]
		CodeCUST,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DATP") )]
		CodeDATP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DIST") )]
		CodeDIST,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FACT") )]
		CodeFACT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FIAD") )]
		CodeFIAD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FIAG") )]
		CodeFIAG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FMCO") )]
		CodeFMCO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FNBR") )]
		CodeFNBR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FTAG") )]
		CodeFTAG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INTR") )]
		CodeINTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVE") )]
		CodeINVE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVS") )]
		CodeINVS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PAYI") )]
		CodePAYI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REGI") )]
		CodeREGI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRAG") )]
		CodeTRAG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRAN") )]
		CodeTRAN,
	}
	
	impl InvestmentFundRole6Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// InvestmentFundRole7Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum InvestmentFundRole7Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CONC") )]
		CodeCONC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DIST") )]
		CodeDIST,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FMCO") )]
		CodeFMCO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INTR") )]
		CodeINTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PAYI") )]
		CodePAYI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRAG") )]
		CodeTRAG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CUST") )]
		CodeCUST,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CACO") )]
		CodeCACO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FACT") )]
		CodeFACT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVE") )]
		CodeINVE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVS") )]
		CodeINVS,
	}
	
	impl InvestmentFundRole7Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// InvestmentFundTransactionType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum InvestmentFundTransactionType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ALLL") )]
		CodeALLL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SELL") )]
		CodeSELL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BUYI") )]
		CodeBUYI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SWIO") )]
		CodeSWIO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRIN") )]
		CodeTRIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TOUT") )]
		CodeTOUT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SUBS") )]
		CodeSUBS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REDM") )]
		CodeREDM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CDEP") )]
		CodeCDEP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CWIT") )]
		CodeCWIT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DIVP") )]
		CodeDIVP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CAEV") )]
		CodeCAEV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CROI") )]
		CodeCROI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CROO") )]
		CodeCROO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DIVI") )]
		CodeDIVI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INSP") )]
		CodeINSP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REAA") )]
		CodeREAA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RWPL") )]
		CodeRWPL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RDIV") )]
		CodeRDIV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SSPL") )]
		CodeSSPL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SUAA") )]
		CodeSUAA,
	}
	
	impl InvestmentFundTransactionType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// InvestmentPlan17 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InvestmentPlan17 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy") )]
		pub frqcy: Frequency20Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
		pub start_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt", skip_serializing_if = "Option::is_none") )]
		pub end_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Qty") )]
		pub qty: UnitsOrAmount1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GrssAmtInd", skip_serializing_if = "Option::is_none") )]
		pub grss_amt_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IncmPref", skip_serializing_if = "Option::is_none") )]
		pub incm_pref: Option<IncomePreference2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InitlAmt", skip_serializing_if = "Option::is_none") )]
		pub initl_amt: Option<InitialAmount1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfInstlmts", skip_serializing_if = "Option::is_none") )]
		pub ttl_nb_of_instlmts: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RndgDrctn", skip_serializing_if = "Option::is_none") )]
		pub rndg_drctn: Option<RoundingDirection1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SctyDtls") )]
		pub scty_dtls: Vec<Repartition6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshSttlm", skip_serializing_if = "Option::is_none") )]
		pub csh_sttlm: Option<Vec<CashSettlement3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctRef", skip_serializing_if = "Option::is_none") )]
		pub ctrct_ref: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdCtrctRef", skip_serializing_if = "Option::is_none") )]
		pub rltd_ctrct_ref: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdctId", skip_serializing_if = "Option::is_none") )]
		pub pdct_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SLAChrgAndComssnRef", skip_serializing_if = "Option::is_none") )]
		pub sla_chrg_and_comssn_ref: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InsrncCover", skip_serializing_if = "Option::is_none") )]
		pub insrnc_cover: Option<InsuranceType2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PlanSts", skip_serializing_if = "Option::is_none") )]
		pub plan_sts: Option<PlanStatus2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstlmtMgrRole", skip_serializing_if = "Option::is_none") )]
		pub instlmt_mgr_role: Option<PartyRole4Choice>,
	}
	
	impl InvestmentPlan17 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.frqcy.validate() { return Err(e); }
			if let Err(e) = self.qty.validate() { return Err(e); }
			if let Some(ref incm_pref_value) = self.incm_pref { if let Err(e) = incm_pref_value.validate() { return Err(e); } }
			if let Some(ref initl_amt_value) = self.initl_amt { if let Err(e) = initl_amt_value.validate() { return Err(e); } }
			if let Some(ref rndg_drctn_value) = self.rndg_drctn { if let Err(e) = rndg_drctn_value.validate() { return Err(e); } }
			for item in &self.scty_dtls { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref csh_sttlm_vec) = self.csh_sttlm { for item in csh_sttlm_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ctrct_ref_value) = self.ctrct_ref { if let Err(e) = ctrct_ref_value.validate() { return Err(e); } }
			if let Some(ref rltd_ctrct_ref_value) = self.rltd_ctrct_ref { if let Err(e) = rltd_ctrct_ref_value.validate() { return Err(e); } }
			if let Some(ref pdct_id_value) = self.pdct_id { if let Err(e) = pdct_id_value.validate() { return Err(e); } }
			if let Some(ref sla_chrg_and_comssn_ref_value) = self.sla_chrg_and_comssn_ref { if let Err(e) = sla_chrg_and_comssn_ref_value.validate() { return Err(e); } }
			if let Some(ref insrnc_cover_value) = self.insrnc_cover { if let Err(e) = insrnc_cover_value.validate() { return Err(e); } }
			if let Some(ref plan_sts_value) = self.plan_sts { if let Err(e) = plan_sts_value.validate() { return Err(e); } }
			if let Some(ref instlmt_mgr_role_value) = self.instlmt_mgr_role { if let Err(e) = instlmt_mgr_role_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InvestorProfile2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InvestorProfile2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<ProfileType1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
		pub sts: Option<InvestorProfileStatus1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Trsr", skip_serializing_if = "Option::is_none") )]
		pub trsr: Option<TreasuryProfile1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HghFrqcyTradg", skip_serializing_if = "Option::is_none") )]
		pub hgh_frqcy_tradg: Option<HighFrequencyTradingProfile1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MktMakr", skip_serializing_if = "Option::is_none") )]
		pub mkt_makr: Option<MarketMakerProfile2>,
	}
	
	impl InvestorProfile2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref sts_value) = self.sts { if let Err(e) = sts_value.validate() { return Err(e); } }
			if let Some(ref trsr_value) = self.trsr { if let Err(e) = trsr_value.validate() { return Err(e); } }
			if let Some(ref hgh_frqcy_tradg_value) = self.hgh_frqcy_tradg { if let Err(e) = hgh_frqcy_tradg_value.validate() { return Err(e); } }
			if let Some(ref mkt_makr_value) = self.mkt_makr { if let Err(e) = mkt_makr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InvestorProfileStatus1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InvestorProfileStatus1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<InvestorProfileStatus1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl InvestorProfileStatus1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InvestorProfileStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum InvestorProfileStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DISA") )]
		CodeDISA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DISG") )]
		CodeDISG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ENAB") )]
		CodeENAB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ENBG") )]
		CodeENBG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADMI") )]
		CodeADMI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ANLY") )]
		CodeANLY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NAPP") )]
		CodeNAPP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PSUS") )]
		CodePSUS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PEND") )]
		CodePEND,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SUPS") )]
		CodeSUPS,
	}
	
	impl InvestorProfileStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// IrishNSCIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct IrishNSCIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub irish_nsc_identifier: String,
	}
	
	impl IrishNSCIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("IE[0-9]{6,6}").unwrap();
			if !pattern.is_match(&self.irish_nsc_identifier) {
				return Err(ValidationError::new(1005, "irish_nsc_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ItalianDomesticIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ItalianDomesticIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub italian_domestic_identifier: String,
	}
	
	impl ItalianDomesticIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("IT[0-9]{10,10}").unwrap();
			if !pattern.is_match(&self.italian_domestic_identifier) {
				return Err(ValidationError::new(1005, "italian_domestic_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// KYCCheckType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct KYCCheckType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<KnowYourCustomerCheckType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl KYCCheckType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// KnowYourCustomerCheckType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum KnowYourCustomerCheckType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ENHA") )]
		CodeENHA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ORDN") )]
		CodeORDN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SIMP") )]
		CodeSIMP,
	}
	
	impl KnowYourCustomerCheckType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// LEIIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct LEIIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub lei_identifier: String,
	}
	
	impl LEIIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&self.lei_identifier) {
				return Err(ValidationError::new(1005, "lei_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// LanguageCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct LanguageCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub language_code: String,
	}
	
	impl LanguageCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// LetterIntent1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct LetterIntent1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "LttrInttRef") )]
		pub lttr_intt_ref: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
		pub start_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt", skip_serializing_if = "Option::is_none") )]
		pub end_dt: Option<String>,
	}
	
	impl LetterIntent1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.lttr_intt_ref.validate() { return Err(e); }
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// LevelOfControl1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct LevelOfControl1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<LevelOfControl1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl LevelOfControl1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// LevelOfControl1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum LevelOfControl1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRAN") )]
		CodeTRAN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VIEW") )]
		CodeVIEW,
	}
	
	impl LevelOfControl1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Liability1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Liability1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<Liability1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl Liability1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Liability1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Liability1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVE") )]
		CodeINVE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BROK") )]
		CodeBROK,
	}
	
	impl Liability1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// MICIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct MICIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub mic_identifier: String,
	}
	
	impl MICIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(&self.mic_identifier) {
				return Err(ValidationError::new(1005, "mic_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// MailType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MailType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<MailType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl MailType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// MailType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum MailType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "AIRM") )]
		CodeAIRM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ORDM") )]
		CodeORDM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REGM") )]
		CodeREGM,
	}
	
	impl MailType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// MarketMakerProfile2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MarketMakerProfile2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctPrd", skip_serializing_if = "Option::is_none") )]
		pub ctrct_prd: Option<DateTimePeriod2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cmplc", skip_serializing_if = "Option::is_none") )]
		pub cmplc: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MaxSprd", skip_serializing_if = "Option::is_none") )]
		pub max_sprd: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dscnt", skip_serializing_if = "Option::is_none") )]
		pub dscnt: Option<f64>,
	}
	
	impl MarketMakerProfile2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ctrct_prd_value) = self.ctrct_prd { if let Err(e) = ctrct_prd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// MarketPracticeVersion1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MarketPracticeVersion1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
		pub dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nb", skip_serializing_if = "Option::is_none") )]
		pub nb: Option<Max35Text>,
	}
	
	impl MarketPracticeVersion1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.nm.validate() { return Err(e); }
			if let Some(ref nb_value) = self.nb { if let Err(e) = nb_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Max10Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max10Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max10_text: String,
	}
	
	impl Max10Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max10_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max10_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max10_text.chars().count() > 10 {
				return Err(ValidationError::new(1002, "max10_text exceeds the maximum length of 10".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max140Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max140Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max140_text: String,
	}
	
	impl Max140Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max140_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max140_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max140_text.chars().count() > 140 {
				return Err(ValidationError::new(1002, "max140_text exceeds the maximum length of 140".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max16Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max16Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max16_text: String,
	}
	
	impl Max16Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max16_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max16_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max16_text.chars().count() > 16 {
				return Err(ValidationError::new(1002, "max16_text exceeds the maximum length of 16".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max256Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max256Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max256_text: String,
	}
	
	impl Max256Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max256_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max256_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max256_text.chars().count() > 256 {
				return Err(ValidationError::new(1002, "max256_text exceeds the maximum length of 256".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max34Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max34Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max34_text: String,
	}
	
	impl Max34Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max34_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max34_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max34_text.chars().count() > 34 {
				return Err(ValidationError::new(1002, "max34_text exceeds the maximum length of 34".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max350Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max350Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max350_text: String,
	}
	
	impl Max350Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max350_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max350_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max350_text.chars().count() > 350 {
				return Err(ValidationError::new(1002, "max350_text exceeds the maximum length of 350".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max35Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max35Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max35_text: String,
	}
	
	impl Max35Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max35_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max35_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max35_text.chars().count() > 35 {
				return Err(ValidationError::new(1002, "max35_text exceeds the maximum length of 35".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max3Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max3Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max3_text: String,
	}
	
	impl Max3Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max3_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max3_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max3_text.chars().count() > 3 {
				return Err(ValidationError::new(1002, "max3_text exceeds the maximum length of 3".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max4AlphaNumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max4AlphaNumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max4_alpha_numeric_text: String,
	}
	
	impl Max4AlphaNumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max4_alpha_numeric_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max4_alpha_numeric_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max4_alpha_numeric_text.chars().count() > 4 {
				return Err(ValidationError::new(1002, "max4_alpha_numeric_text exceeds the maximum length of 4".to_string()));
			}
			let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
			if !pattern.is_match(&self.max4_alpha_numeric_text) {
				return Err(ValidationError::new(1005, "max4_alpha_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max70Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max70Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max70_text: String,
	}
	
	impl Max70Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max70_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max70_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max70_text.chars().count() > 70 {
				return Err(ValidationError::new(1002, "max70_text exceeds the maximum length of 70".to_string()));
			}
			Ok(())
		}
	}
	
	
	// MessageIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MessageIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
		pub cre_dt_tm: String,
	}
	
	impl MessageIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// MiFIDClassification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MiFIDClassification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Clssfctn") )]
		pub clssfctn: OrderOriginatorEligibility1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nrrtv", skip_serializing_if = "Option::is_none") )]
		pub nrrtv: Option<Max350Text>,
	}
	
	impl MiFIDClassification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.clssfctn.validate() { return Err(e); }
			if let Some(ref nrrtv_value) = self.nrrtv { if let Err(e) = nrrtv_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// MoneyLaunderingCheck1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MoneyLaunderingCheck1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<MoneyLaunderingCheck1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl MoneyLaunderingCheck1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// MoneyLaunderingCheck1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum MoneyLaunderingCheck1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PASS") )]
		CodePASS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOTC") )]
		CodeNOTC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EXEM") )]
		CodeEXEM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLMO") )]
		CodeCLMO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AUTH") )]
		CodeAUTH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "POEP") )]
		CodePOEP,
	}
	
	impl MoneyLaunderingCheck1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// NameAndAddress4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct NameAndAddress4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Adr") )]
		pub adr: PostalAddress1,
	}
	
	impl NameAndAddress4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Err(e) = self.adr.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// NameAndAddress5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct NameAndAddress5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: Max350Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Adr", skip_serializing_if = "Option::is_none") )]
		pub adr: Option<PostalAddress1>,
	}
	
	impl NameAndAddress5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.nm.validate() { return Err(e); }
			if let Some(ref adr_value) = self.adr { if let Err(e) = adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// NamePrefix1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct NamePrefix1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<NamePrefix1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl NamePrefix1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// NamePrefix1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum NamePrefix1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DOCT") )]
		CodeDOCT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MIST") )]
		CodeMIST,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MISS") )]
		CodeMISS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MADM") )]
		CodeMADM,
	}
	
	impl NamePrefix1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// NationalityCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct NationalityCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub nationality_code: String,
	}
	
	impl NationalityCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// NewIssueAllocation2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct NewIssueAllocation2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rstrctd") )]
		pub rstrctd: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XmptPrsnRsn", skip_serializing_if = "Option::is_none") )]
		pub xmpt_prsn_rsn: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DeMnms", skip_serializing_if = "Option::is_none") )]
		pub de_mnms: Option<DeMinimus1Choice>,
	}
	
	impl NewIssueAllocation2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref xmpt_prsn_rsn_value) = self.xmpt_prsn_rsn { if let Err(e) = xmpt_prsn_rsn_value.validate() { return Err(e); } }
			if let Some(ref de_mnms_value) = self.de_mnms { if let Err(e) = de_mnms_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// NewZealandNCCIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct NewZealandNCCIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub new_zealand_ncc_identifier: String,
	}
	
	impl NewZealandNCCIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("NZ[0-9]{6,6}").unwrap();
			if !pattern.is_match(&self.new_zealand_ncc_identifier) {
				return Err(ValidationError::new(1005, "new_zealand_ncc_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// NoReasonCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum NoReasonCode {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NORE") )]
		CodeNORE,
	}
	
	impl NoReasonCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Notification2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Notification2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtfctnTp") )]
		pub ntfctn_tp: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Reqrd") )]
		pub reqrd: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DstrbtnTp", skip_serializing_if = "Option::is_none") )]
		pub dstrbtn_tp: Option<InformationDistribution1Choice>,
	}
	
	impl Notification2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ntfctn_tp.validate() { return Err(e); }
			if let Some(ref dstrbtn_tp_value) = self.dstrbtn_tp { if let Err(e) = dstrbtn_tp_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Number ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Number {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub number: f64,
	}
	
	impl Number {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// OperationalStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum OperationalStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ENAB") )]
		CodeENAB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SPEC") )]
		CodeSPEC,
	}
	
	impl OperationalStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// OrderOriginatorEligibility1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum OrderOriginatorEligibility1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ELIG") )]
		CodeELIG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RETL") )]
		CodeRETL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PROF") )]
		CodePROF,
	}
	
	impl OrderOriginatorEligibility1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Organisation23 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Organisation23 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: Max350Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
		pub shrt_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr") )]
		pub pstl_adr: Vec<PostalAddress21>,
	}
	
	impl Organisation23 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.nm.validate() { return Err(e); }
			if let Some(ref shrt_nm_value) = self.shrt_nm { if let Err(e) = shrt_nm_value.validate() { return Err(e); } }
			for item in &self.pstl_adr { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Organisation39 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Organisation39 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
		pub shrt_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<PartyIdentification177Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none") )]
		pub lgl_ntty_idr: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
		pub purp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RegnCtry", skip_serializing_if = "Option::is_none") )]
		pub regn_ctry: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RegnDt", skip_serializing_if = "Option::is_none") )]
		pub regn_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<Vec<PostalAddress21>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TpOfOrg", skip_serializing_if = "Option::is_none") )]
		pub tp_of_org: Option<OrganisationType1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PlcOfListg", skip_serializing_if = "Option::is_none") )]
		pub plc_of_listg: Option<Vec<MICIdentifier>>,
	}
	
	impl Organisation39 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref shrt_nm_value) = self.shrt_nm { if let Err(e) = shrt_nm_value.validate() { return Err(e); } }
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref lgl_ntty_idr_value) = self.lgl_ntty_idr { if let Err(e) = lgl_ntty_idr_value.validate() { return Err(e); } }
			if let Some(ref purp_value) = self.purp { if let Err(e) = purp_value.validate() { return Err(e); } }
			if let Some(ref regn_ctry_value) = self.regn_ctry { if let Err(e) = regn_ctry_value.validate() { return Err(e); } }
			if let Some(ref pstl_adr_vec) = self.pstl_adr { for item in pstl_adr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref tp_of_org_value) = self.tp_of_org { if let Err(e) = tp_of_org_value.validate() { return Err(e); } }
			if let Some(ref plc_of_listg_vec) = self.plc_of_listg { for item in plc_of_listg_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// OrganisationType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OrganisationType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<OrganisationType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl OrganisationType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OrganisationType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum OrganisationType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "IFUN") )]
		CodeIFUN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRIV") )]
		CodePRIV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PUBL") )]
		CodePUBL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PFUN") )]
		CodePFUN,
	}
	
	impl OrganisationType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// OtherAccountStatus1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OtherAccountStatus1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
		pub sts: GenericIdentification36,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<GenericIdentification36>,
	}
	
	impl OtherAccountStatus1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.sts.validate() { return Err(e); }
			if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OtherIdentification3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OtherIdentification3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<PartyIdentificationType7Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl OtherIdentification3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OwnershipBeneficiaryRate1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OwnershipBeneficiaryRate1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
		pub rate: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Frctn", skip_serializing_if = "Option::is_none") )]
		pub frctn: Option<Max35Text>,
	}
	
	impl OwnershipBeneficiaryRate1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref frctn_value) = self.frctn { if let Err(e) = frctn_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OwnershipType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OwnershipType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<AccountOwnershipType4Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl OwnershipType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Party47Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Party47Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Org", skip_serializing_if = "Option::is_none") )]
		pub org: Option<Organisation39>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IndvPrsn", skip_serializing_if = "Option::is_none") )]
		pub indv_prsn: Option<IndividualPerson37>,
	}
	
	impl Party47Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref org_value) = self.org { if let Err(e) = org_value.validate() { return Err(e); } }
			if let Some(ref indv_prsn_value) = self.indv_prsn { if let Err(e) = indv_prsn_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyIdentification125Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PartyIdentification125Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<AnyBICDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<GenericIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
		pub nm_and_adr: Option<NameAndAddress5>,
	}
	
	impl PartyIdentification125Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
			if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
			if let Some(ref nm_and_adr_value) = self.nm_and_adr { if let Err(e) = nm_and_adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyIdentification177Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PartyIdentification177Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<AnyBICDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<GenericIdentification1>,
	}
	
	impl PartyIdentification177Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
			if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyIdentificationType7Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PartyIdentificationType7Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ATIN") )]
		CodeATIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IDCD") )]
		CodeIDCD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NRIN") )]
		CodeNRIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PASS") )]
		CodePASS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "POCD") )]
		CodePOCD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SOCS") )]
		CodeSOCS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SRSA") )]
		CodeSRSA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GUNL") )]
		CodeGUNL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GTIN") )]
		CodeGTIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ITIN") )]
		CodeITIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CPFA") )]
		CodeCPFA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AREG") )]
		CodeAREG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DRLC") )]
		CodeDRLC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EMID") )]
		CodeEMID,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NINV") )]
		CodeNINV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INCL") )]
		CodeINCL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GIIN") )]
		CodeGIIN,
	}
	
	impl PartyIdentificationType7Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PartyProfileInformation5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PartyProfileInformation5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CertfctnInd", skip_serializing_if = "Option::is_none") )]
		pub certfctn_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VldtngPty", skip_serializing_if = "Option::is_none") )]
		pub vldtng_pty: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ChckngPty", skip_serializing_if = "Option::is_none") )]
		pub chckng_pty: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RspnsblPty", skip_serializing_if = "Option::is_none") )]
		pub rspnsbl_pty: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CertTp", skip_serializing_if = "Option::is_none") )]
		pub cert_tp: Option<CertificationType1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ChckngDt", skip_serializing_if = "Option::is_none") )]
		pub chckng_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ChckngFrqcy", skip_serializing_if = "Option::is_none") )]
		pub chckng_frqcy: Option<EventFrequency1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NxtRvsnDt", skip_serializing_if = "Option::is_none") )]
		pub nxt_rvsn_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SlryRg", skip_serializing_if = "Option::is_none") )]
		pub slry_rg: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SrcOfWlth", skip_serializing_if = "Option::is_none") )]
		pub src_of_wlth: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CstmrCndctClssfctn", skip_serializing_if = "Option::is_none") )]
		pub cstmr_cndct_clssfctn: Option<CustomerConductClassification1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RskLvl", skip_serializing_if = "Option::is_none") )]
		pub rsk_lvl: Option<RiskLevel2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KnowYourCstmrChckTp", skip_serializing_if = "Option::is_none") )]
		pub know_your_cstmr_chck_tp: Option<KYCCheckType1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KnowYourCstmrDBChck", skip_serializing_if = "Option::is_none") )]
		pub know_your_cstmr_db_chck: Option<DataBaseCheck1>,
	}
	
	impl PartyProfileInformation5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref vldtng_pty_value) = self.vldtng_pty { if let Err(e) = vldtng_pty_value.validate() { return Err(e); } }
			if let Some(ref chckng_pty_value) = self.chckng_pty { if let Err(e) = chckng_pty_value.validate() { return Err(e); } }
			if let Some(ref rspnsbl_pty_value) = self.rspnsbl_pty { if let Err(e) = rspnsbl_pty_value.validate() { return Err(e); } }
			if let Some(ref cert_tp_value) = self.cert_tp { if let Err(e) = cert_tp_value.validate() { return Err(e); } }
			if let Some(ref chckng_frqcy_value) = self.chckng_frqcy { if let Err(e) = chckng_frqcy_value.validate() { return Err(e); } }
			if let Some(ref slry_rg_value) = self.slry_rg { if let Err(e) = slry_rg_value.validate() { return Err(e); } }
			if let Some(ref src_of_wlth_value) = self.src_of_wlth { if let Err(e) = src_of_wlth_value.validate() { return Err(e); } }
			if let Some(ref cstmr_cndct_clssfctn_value) = self.cstmr_cndct_clssfctn { if let Err(e) = cstmr_cndct_clssfctn_value.validate() { return Err(e); } }
			if let Some(ref rsk_lvl_value) = self.rsk_lvl { if let Err(e) = rsk_lvl_value.validate() { return Err(e); } }
			if let Some(ref know_your_cstmr_chck_tp_value) = self.know_your_cstmr_chck_tp { if let Err(e) = know_your_cstmr_chck_tp_value.validate() { return Err(e); } }
			if let Some(ref know_your_cstmr_db_chck_value) = self.know_your_cstmr_db_chck { if let Err(e) = know_your_cstmr_db_chck_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyRole1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PartyRole1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CUST") )]
		CodeCUST,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVS") )]
		CodeINVS,
	}
	
	impl PartyRole1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PartyRole2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PartyRole2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<InvestmentFundRole6Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl PartyRole2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyRole4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PartyRole4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<InvestmentFundRole7Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl PartyRole4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyRole5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PartyRole5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<PartyRole1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl PartyRole5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaymentCard29 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentCard29 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: CardType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nb") )]
		pub nb: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HldrNm") )]
		pub hldr_nm: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
		pub start_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt") )]
		pub xpry_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CardIssrNm", skip_serializing_if = "Option::is_none") )]
		pub card_issr_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CardIssrId", skip_serializing_if = "Option::is_none") )]
		pub card_issr_id: Option<PartyIdentification125Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SctyCd", skip_serializing_if = "Option::is_none") )]
		pub scty_cd: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SeqNb", skip_serializing_if = "Option::is_none") )]
		pub seq_nb: Option<Max3Text>,
	}
	
	impl PaymentCard29 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Err(e) = self.nb.validate() { return Err(e); }
			if let Err(e) = self.hldr_nm.validate() { return Err(e); }
			if let Some(ref card_issr_nm_value) = self.card_issr_nm { if let Err(e) = card_issr_nm_value.validate() { return Err(e); } }
			if let Some(ref card_issr_id_value) = self.card_issr_id { if let Err(e) = card_issr_id_value.validate() { return Err(e); } }
			if let Some(ref scty_cd_value) = self.scty_cd { if let Err(e) = scty_cd_value.validate() { return Err(e); } }
			if let Some(ref seq_nb_value) = self.seq_nb { if let Err(e) = seq_nb_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaymentInstrument17 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentInstrument17 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCcy") )]
		pub sttlm_ccy: ActiveCurrencyCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DvddPctg", skip_serializing_if = "Option::is_none") )]
		pub dvdd_pctg: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SbcptPmtInstrm", skip_serializing_if = "Option::is_none") )]
		pub sbcpt_pmt_instrm: Option<PaymentInstrument24Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RedPmtInstrm", skip_serializing_if = "Option::is_none") )]
		pub red_pmt_instrm: Option<PaymentInstrument19Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DvddPmtInstrm", skip_serializing_if = "Option::is_none") )]
		pub dvdd_pmt_instrm: Option<PaymentInstrument19Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvgsPlanPmtInstrm", skip_serializing_if = "Option::is_none") )]
		pub svgs_plan_pmt_instrm: Option<PaymentInstrument24Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstPmtInstrm", skip_serializing_if = "Option::is_none") )]
		pub intrst_pmt_instrm: Option<PaymentInstrument19Choice>,
	}
	
	impl PaymentInstrument17 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.sttlm_ccy.validate() { return Err(e); }
			if let Some(ref sbcpt_pmt_instrm_value) = self.sbcpt_pmt_instrm { if let Err(e) = sbcpt_pmt_instrm_value.validate() { return Err(e); } }
			if let Some(ref red_pmt_instrm_value) = self.red_pmt_instrm { if let Err(e) = red_pmt_instrm_value.validate() { return Err(e); } }
			if let Some(ref dvdd_pmt_instrm_value) = self.dvdd_pmt_instrm { if let Err(e) = dvdd_pmt_instrm_value.validate() { return Err(e); } }
			if let Some(ref svgs_plan_pmt_instrm_value) = self.svgs_plan_pmt_instrm { if let Err(e) = svgs_plan_pmt_instrm_value.validate() { return Err(e); } }
			if let Some(ref intrst_pmt_instrm_value) = self.intrst_pmt_instrm { if let Err(e) = intrst_pmt_instrm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaymentInstrument19Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentInstrument19Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ChqDtls", skip_serializing_if = "Option::is_none") )]
		pub chq_dtls: Option<Cheque4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BkrsDrftDtls", skip_serializing_if = "Option::is_none") )]
		pub bkrs_drft_dtls: Option<Cheque4>,
	}
	
	impl PaymentInstrument19Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref chq_dtls_value) = self.chq_dtls { if let Err(e) = chq_dtls_value.validate() { return Err(e); } }
			if let Some(ref bkrs_drft_dtls_value) = self.bkrs_drft_dtls { if let Err(e) = bkrs_drft_dtls_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaymentInstrument24Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentInstrument24Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCardDtls", skip_serializing_if = "Option::is_none") )]
		pub pmt_card_dtls: Option<PaymentCard29>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DrctDbtDtls", skip_serializing_if = "Option::is_none") )]
		pub drct_dbt_dtls: Option<DirectDebitMandate7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Chq", skip_serializing_if = "Option::is_none") )]
		pub chq: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BkrsDrft", skip_serializing_if = "Option::is_none") )]
		pub bkrs_drft: Option<bool>,
	}
	
	impl PaymentInstrument24Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref pmt_card_dtls_value) = self.pmt_card_dtls { if let Err(e) = pmt_card_dtls_value.validate() { return Err(e); } }
			if let Some(ref drct_dbt_dtls_value) = self.drct_dbt_dtls { if let Err(e) = drct_dbt_dtls_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PendingOpeningStatusReason1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PendingOpeningStatusReason1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: PendingOpeningStatusReason2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Max350Text>,
	}
	
	impl PendingOpeningStatusReason1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PendingOpeningStatusReason1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PendingOpeningStatusReason1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
		pub no_spcfd_rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Vec<PendingOpeningStatusReason1>>,
	}
	
	impl PendingOpeningStatusReason1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if let Err(e) = no_spcfd_rsn_value.validate() { return Err(e); } }
			if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// PendingOpeningStatusReason1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PendingOpeningStatusReason1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ATHR") )]
		CodeATHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ATHP") )]
		CodeATHP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FRDM") )]
		CodeFRDM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KYCM") )]
		CodeKYCM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOTO") )]
		CodeNOTO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REST") )]
		CodeREST,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RIGH") )]
		CodeRIGH,
	}
	
	impl PendingOpeningStatusReason1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PendingOpeningStatusReason2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PendingOpeningStatusReason2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<PendingOpeningStatusReason1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification36>,
	}
	
	impl PendingOpeningStatusReason2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PendingStatusReason14 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PendingStatusReason14 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: PendingStatusReason2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Max350Text>,
	}
	
	impl PendingStatusReason14 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PendingStatusReason1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PendingStatusReason1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
		pub no_spcfd_rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Vec<PendingStatusReason14>>,
	}
	
	impl PendingStatusReason1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if let Err(e) = no_spcfd_rsn_value.validate() { return Err(e); } }
			if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// PendingStatusReason1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PendingStatusReason1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "KYCM") )]
		CodeKYCM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FRDM") )]
		CodeFRDM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RIGH") )]
		CodeRIGH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ATHR") )]
		CodeATHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ATHP") )]
		CodeATHP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MODI") )]
		CodeMODI,
	}
	
	impl PendingStatusReason1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PendingStatusReason2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PendingStatusReason2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<PendingStatusReason1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification36>,
	}
	
	impl PendingStatusReason2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PercentageBoundedRate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct PercentageBoundedRate {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub percentage_bounded_rate: f64,
	}
	
	impl PercentageBoundedRate {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.percentage_bounded_rate < 0.000000 {
				return Err(ValidationError::new(1003, "percentage_bounded_rate is less than the minimum value of 0.000000".to_string()));
			}
			if self.percentage_bounded_rate > 100.000000 {
				return Err(ValidationError::new(1004, "percentage_bounded_rate exceeds the maximum value of 100.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// PercentageRate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct PercentageRate {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub percentage_rate: f64,
	}
	
	impl PercentageRate {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PersonalInformation1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PersonalInformation1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmOfFthr", skip_serializing_if = "Option::is_none") )]
		pub nm_of_fthr: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MdnNmOfMthr", skip_serializing_if = "Option::is_none") )]
		pub mdn_nm_of_mthr: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmOfPrtnr", skip_serializing_if = "Option::is_none") )]
		pub nm_of_prtnr: Option<Max35Text>,
	}
	
	impl PersonalInformation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nm_of_fthr_value) = self.nm_of_fthr { if let Err(e) = nm_of_fthr_value.validate() { return Err(e); } }
			if let Some(ref mdn_nm_of_mthr_value) = self.mdn_nm_of_mthr { if let Err(e) = mdn_nm_of_mthr_value.validate() { return Err(e); } }
			if let Some(ref nm_of_prtnr_value) = self.nm_of_prtnr { if let Err(e) = nm_of_prtnr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PhoneNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct PhoneNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub phone_number: String,
	}
	
	impl PhoneNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(&self.phone_number) {
				return Err(ValidationError::new(1005, "phone_number does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// PlanStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PlanStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACTV") )]
		CodeACTV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLOS") )]
		CodeCLOS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SUSP") )]
		CodeSUSP,
	}
	
	impl PlanStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PlanStatus2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PlanStatus2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<PlanStatus1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl PlanStatus2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PoliticalExposureType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PoliticalExposureType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<PoliticalExposureType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl PoliticalExposureType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PoliticalExposureType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PoliticalExposureType2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NPEX") )]
		CodeNPEX,
		#[cfg_attr( feature = "derive_serde", serde(rename = "YPEX") )]
		CodeYPEX,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PEXD") )]
		CodePEXD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PEXF") )]
		CodePEXF,
	}
	
	impl PoliticalExposureType2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PoliticallyExposedPerson1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PoliticallyExposedPerson1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PltclyXpsdPrsnTp") )]
		pub pltcly_xpsd_prsn_tp: PoliticalExposureType2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PltclyXpsdPrsnSts", skip_serializing_if = "Option::is_none") )]
		pub pltcly_xpsd_prsn_sts: Option<PoliticallyExposedPersonStatus1Choice>,
	}
	
	impl PoliticallyExposedPerson1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pltcly_xpsd_prsn_tp.validate() { return Err(e); }
			if let Some(ref pltcly_xpsd_prsn_sts_value) = self.pltcly_xpsd_prsn_sts { if let Err(e) = pltcly_xpsd_prsn_sts_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PoliticallyExposedPersonStatus1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PoliticallyExposedPersonStatus1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<PoliticallyExposedPersonStatus1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl PoliticallyExposedPersonStatus1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PoliticallyExposedPersonStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PoliticallyExposedPersonStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PE03") )]
		CodePE03,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PE01") )]
		CodePE01,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PE02") )]
		CodePE02,
	}
	
	impl PoliticallyExposedPersonStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PortugueseNCCIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct PortugueseNCCIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub portuguese_ncc_identifier: String,
	}
	
	impl PortugueseNCCIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("PT[0-9]{8,8}").unwrap();
			if !pattern.is_match(&self.portuguese_ncc_identifier) {
				return Err(ValidationError::new(1005, "portuguese_ncc_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// PositionEffect3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PositionEffect3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "FIFO") )]
		CodeFIFO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LIFO") )]
		CodeLIFO,
	}
	
	impl PositionEffect3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PostalAddress1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PostalAddress1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
		pub adr_tp: Option<AddressType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
		pub adr_line: Option<Vec<Max70Text>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
		pub strt_nm: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
		pub bldg_nb: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
		pub pst_cd: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
		pub twn_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
		pub ctry_sub_dvsn: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
		pub ctry: CountryCode,
	}
	
	impl PostalAddress1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref adr_tp_value) = self.adr_tp { if let Err(e) = adr_tp_value.validate() { return Err(e); } }
			if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref strt_nm_value) = self.strt_nm { if let Err(e) = strt_nm_value.validate() { return Err(e); } }
			if let Some(ref bldg_nb_value) = self.bldg_nb { if let Err(e) = bldg_nb_value.validate() { return Err(e); } }
			if let Some(ref pst_cd_value) = self.pst_cd { if let Err(e) = pst_cd_value.validate() { return Err(e); } }
			if let Some(ref twn_nm_value) = self.twn_nm { if let Err(e) = twn_nm_value.validate() { return Err(e); } }
			if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if let Err(e) = ctry_sub_dvsn_value.validate() { return Err(e); } }
			if let Err(e) = self.ctry.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// PostalAddress21 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PostalAddress21 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
		pub adr_tp: Option<AddressType2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MlngInd", skip_serializing_if = "Option::is_none") )]
		pub mlng_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RegnAdrInd", skip_serializing_if = "Option::is_none") )]
		pub regn_adr_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CareOf", skip_serializing_if = "Option::is_none") )]
		pub care_of: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
		pub adr_line: Option<Vec<Max70Text>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
		pub strt_nm: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
		pub bldg_nb: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNm", skip_serializing_if = "Option::is_none") )]
		pub bldg_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstBx", skip_serializing_if = "Option::is_none") )]
		pub pst_bx: Option<Max10Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SdInBldg", skip_serializing_if = "Option::is_none") )]
		pub sd_in_bldg: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Flr", skip_serializing_if = "Option::is_none") )]
		pub flr: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SuiteId", skip_serializing_if = "Option::is_none") )]
		pub suite_id: Option<Max10Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
		pub pst_cd: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none") )]
		pub dstrct_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Vllg", skip_serializing_if = "Option::is_none") )]
		pub vllg: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
		pub twn_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Stat", skip_serializing_if = "Option::is_none") )]
		pub stat: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
		pub ctry: CountryCode,
	}
	
	impl PostalAddress21 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref adr_tp_value) = self.adr_tp { if let Err(e) = adr_tp_value.validate() { return Err(e); } }
			if let Some(ref care_of_value) = self.care_of { if let Err(e) = care_of_value.validate() { return Err(e); } }
			if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref strt_nm_value) = self.strt_nm { if let Err(e) = strt_nm_value.validate() { return Err(e); } }
			if let Some(ref bldg_nb_value) = self.bldg_nb { if let Err(e) = bldg_nb_value.validate() { return Err(e); } }
			if let Some(ref bldg_nm_value) = self.bldg_nm { if let Err(e) = bldg_nm_value.validate() { return Err(e); } }
			if let Some(ref pst_bx_value) = self.pst_bx { if let Err(e) = pst_bx_value.validate() { return Err(e); } }
			if let Some(ref sd_in_bldg_value) = self.sd_in_bldg { if let Err(e) = sd_in_bldg_value.validate() { return Err(e); } }
			if let Some(ref flr_value) = self.flr { if let Err(e) = flr_value.validate() { return Err(e); } }
			if let Some(ref suite_id_value) = self.suite_id { if let Err(e) = suite_id_value.validate() { return Err(e); } }
			if let Some(ref pst_cd_value) = self.pst_cd { if let Err(e) = pst_cd_value.validate() { return Err(e); } }
			if let Some(ref dstrct_nm_value) = self.dstrct_nm { if let Err(e) = dstrct_nm_value.validate() { return Err(e); } }
			if let Some(ref vllg_value) = self.vllg { if let Err(e) = vllg_value.validate() { return Err(e); } }
			if let Some(ref twn_nm_value) = self.twn_nm { if let Err(e) = twn_nm_value.validate() { return Err(e); } }
			if let Some(ref stat_value) = self.stat { if let Err(e) = stat_value.validate() { return Err(e); } }
			if let Err(e) = self.ctry.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ProfileType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ProfileType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ProfileType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl ProfileType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ProfileType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ProfileType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "HEDG") )]
		CodeHEDG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HFTR") )]
		CodeHFTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MAKE") )]
		CodeMAKE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TREA") )]
		CodeTREA,
	}
	
	impl ProfileType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ProformaStatusReason1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ProformaStatusReason1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: ProformaStatusReason2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Max350Text>,
	}
	
	impl ProformaStatusReason1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ProformaStatusReason1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ProformaStatusReason1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
		pub no_spcfd_rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Vec<ProformaStatusReason1>>,
	}
	
	impl ProformaStatusReason1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if let Err(e) = no_spcfd_rsn_value.validate() { return Err(e); } }
			if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ProformaStatusReason1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ProformaStatusReason1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "MODI") )]
		CodeMODI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RIGH") )]
		CodeRIGH,
	}
	
	impl ProformaStatusReason1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ProformaStatusReason2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ProformaStatusReason2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ProformaStatusReason1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification36>,
	}
	
	impl ProformaStatusReason2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Provided1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Provided1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NPRO") )]
		CodeNPRO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PROV") )]
		CodePROV,
	}
	
	impl Provided1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// QUICKIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct QUICKIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub quick_identifier: String,
	}
	
	impl QUICKIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// RICIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct RICIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub ric_identifier: String,
	}
	
	impl RICIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.ric_identifier.chars().count() < 1 {
			return Err(ValidationError::new(1001, "ric_identifier is shorter than the minimum length of 1".to_string()));
			}
			if self.ric_identifier.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ric_identifier exceeds the maximum length of 35".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Rank1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Rank1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRIM") )]
		CodePRIM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SECO") )]
		CodeSECO,
	}
	
	impl Rank1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Referred1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Referred1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "REFR") )]
		CodeREFR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NRFR") )]
		CodeNRFR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UKNW") )]
		CodeUKNW,
	}
	
	impl Referred1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ReferredAgent3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReferredAgent3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rfrd") )]
		pub rfrd: Referred1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RfrdPlcmntAgt", skip_serializing_if = "Option::is_none") )]
		pub rfrd_plcmnt_agt: Option<PartyIdentification125Choice>,
	}
	
	impl ReferredAgent3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rfrd.validate() { return Err(e); }
			if let Some(ref rfrd_plcmnt_agt_value) = self.rfrd_plcmnt_agt { if let Err(e) = rfrd_plcmnt_agt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RegisteredShareholderName1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RegisteredShareholderName1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "IndvPrsn", skip_serializing_if = "Option::is_none") )]
		pub indv_prsn: Option<IndividualPerson29>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Org", skip_serializing_if = "Option::is_none") )]
		pub org: Option<Organisation23>,
	}
	
	impl RegisteredShareholderName1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref indv_prsn_value) = self.indv_prsn { if let Err(e) = indv_prsn_value.validate() { return Err(e); } }
			if let Some(ref org_value) = self.org { if let Err(e) = org_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RegulatoryInformation1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RegulatoryInformation1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sctr", skip_serializing_if = "Option::is_none") )]
		pub sctr: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Brnch", skip_serializing_if = "Option::is_none") )]
		pub brnch: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Grp", skip_serializing_if = "Option::is_none") )]
		pub grp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<Max35Text>,
	}
	
	impl RegulatoryInformation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref sctr_value) = self.sctr { if let Err(e) = sctr_value.validate() { return Err(e); } }
			if let Some(ref brnch_value) = self.brnch { if let Err(e) = brnch_value.validate() { return Err(e); } }
			if let Some(ref grp_value) = self.grp { if let Err(e) = grp_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Reinvestment4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Reinvestment4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmDtls") )]
		pub fin_instrm_dtls: FinancialInstrument87,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdNAVCcy", skip_serializing_if = "Option::is_none") )]
		pub reqd_nav_ccy: Option<ActiveCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RinvstmtPctg") )]
		pub rinvstmt_pctg: f64,
	}
	
	impl Reinvestment4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.fin_instrm_dtls.validate() { return Err(e); }
			if let Some(ref reqd_nav_ccy_value) = self.reqd_nav_ccy { if let Err(e) = reqd_nav_ccy_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Repartition6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Repartition6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Qty") )]
		pub qty: UnitsOrAmountOrPercentage1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrm") )]
		pub fin_instrm: FinancialInstrument87,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CcyOfPlan", skip_serializing_if = "Option::is_none") )]
		pub ccy_of_plan: Option<ActiveOrHistoricCurrencyCode>,
	}
	
	impl Repartition6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.qty.validate() { return Err(e); }
			if let Err(e) = self.fin_instrm.validate() { return Err(e); }
			if let Some(ref ccy_of_plan_value) = self.ccy_of_plan { if let Err(e) = ccy_of_plan_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ResidentialStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ResidentialStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "RESI") )]
		CodeRESI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRES") )]
		CodePRES,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NRES") )]
		CodeNRES,
	}
	
	impl ResidentialStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// RestrictionStatus1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RestrictionStatus1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<RestrictionStatus1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl RestrictionStatus1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RestrictionStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum RestrictionStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACTV") )]
		CodeACTV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INAC") )]
		CodeINAC,
	}
	
	impl RestrictionStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// RiskLevel1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum RiskLevel1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "HIGH") )]
		CodeHIGH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LOWW") )]
		CodeLOWW,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MEDM") )]
		CodeMEDM,
	}
	
	impl RiskLevel1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// RiskLevel2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RiskLevel2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<RiskLevel1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl RiskLevel2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RoundingDirection1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum RoundingDirection1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "RDUP") )]
		CodeRDUP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RDWN") )]
		CodeRDWN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "STAN") )]
		CodeSTAN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DIST") )]
		CodeDIST,
	}
	
	impl RoundingDirection1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// RoundingParameters1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RoundingParameters1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RndgMdlus", skip_serializing_if = "Option::is_none") )]
		pub rndg_mdlus: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RndgDrctn") )]
		pub rndg_drctn: RoundingDirection1Code,
	}
	
	impl RoundingParameters1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rndg_drctn.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// RussianCentralBankIdentificationCodeIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct RussianCentralBankIdentificationCodeIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub russian_central_bank_identification_code_identifier: String,
	}
	
	impl RussianCentralBankIdentificationCodeIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("RU[0-9]{9,9}").unwrap();
			if !pattern.is_match(&self.russian_central_bank_identification_code_identifier) {
				return Err(ValidationError::new(1005, "russian_central_bank_identification_code_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// SEDOLIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct SEDOLIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub sedol_identifier: String,
	}
	
	impl SEDOLIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// SecurityIdentification25Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecurityIdentification25Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
		pub isin: Option<ISINOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SEDOL", skip_serializing_if = "Option::is_none") )]
		pub sedol: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CUSIP", skip_serializing_if = "Option::is_none") )]
		pub cusip: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RIC", skip_serializing_if = "Option::is_none") )]
		pub ric: Option<RICIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none") )]
		pub tckr_symb: Option<TickerIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none") )]
		pub blmbrg: Option<Bloomberg2Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CTA", skip_serializing_if = "Option::is_none") )]
		pub cta: Option<ConsolidatedTapeAssociationIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QUICK", skip_serializing_if = "Option::is_none") )]
		pub quick: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Wrtppr", skip_serializing_if = "Option::is_none") )]
		pub wrtppr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dtch", skip_serializing_if = "Option::is_none") )]
		pub dtch: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Vlrn", skip_serializing_if = "Option::is_none") )]
		pub vlrn: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SCVM", skip_serializing_if = "Option::is_none") )]
		pub scvm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Belgn", skip_serializing_if = "Option::is_none") )]
		pub belgn: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cmon", skip_serializing_if = "Option::is_none") )]
		pub cmon: Option<EuroclearClearstreamIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPrtryId", skip_serializing_if = "Option::is_none") )]
		pub othr_prtry_id: Option<AlternateSecurityIdentification7>,
	}
	
	impl SecurityIdentification25Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
			if let Some(ref ric_value) = self.ric { if let Err(e) = ric_value.validate() { return Err(e); } }
			if let Some(ref tckr_symb_value) = self.tckr_symb { if let Err(e) = tckr_symb_value.validate() { return Err(e); } }
			if let Some(ref blmbrg_value) = self.blmbrg { if let Err(e) = blmbrg_value.validate() { return Err(e); } }
			if let Some(ref cta_value) = self.cta { if let Err(e) = cta_value.validate() { return Err(e); } }
			if let Some(ref cmon_value) = self.cmon { if let Err(e) = cmon_value.validate() { return Err(e); } }
			if let Some(ref othr_prtry_id_value) = self.othr_prtry_id { if let Err(e) = othr_prtry_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SettlementFrequency1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SettlementFrequency1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<EventFrequency10Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl SettlementFrequency1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SettlementInstructionReason1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SettlementInstructionReason1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<SettlementInstructionReason1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl SettlementInstructionReason1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SettlementInstructionReason1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum SettlementInstructionReason1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CSHI") )]
		CodeCSHI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ALLL") )]
		CodeALLL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CSHO") )]
		CodeCSHO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CHAR") )]
		CodeCHAR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DIVI") )]
		CodeDIVI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INTE") )]
		CodeINTE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SAVP") )]
		CodeSAVP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REDM") )]
		CodeREDM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SAVE") )]
		CodeSAVE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BUYI") )]
		CodeBUYI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SELL") )]
		CodeSELL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SUBS") )]
		CodeSUBS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WTHP") )]
		CodeWTHP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CORP") )]
		CodeCORP,
	}
	
	impl SettlementInstructionReason1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// SicovamIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct SicovamIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub sicovam_identifier: String,
	}
	
	impl SicovamIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// SimpleIdentificationInformation4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SimpleIdentificationInformation4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
	}
	
	impl SimpleIdentificationInformation4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SmallNetworkIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct SmallNetworkIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub small_network_identifier: String,
	}
	
	impl SmallNetworkIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("AU[0-9]{6,6}").unwrap();
			if !pattern.is_match(&self.small_network_identifier) {
				return Err(ValidationError::new(1005, "small_network_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// SouthAfricanNCCIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct SouthAfricanNCCIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub south_african_ncc_identifier: String,
	}
	
	impl SouthAfricanNCCIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("ZA[0-9]{6,6}").unwrap();
			if !pattern.is_match(&self.south_african_ncc_identifier) {
				return Err(ValidationError::new(1005, "south_african_ncc_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// SpanishDomesticInterbankingIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct SpanishDomesticInterbankingIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub spanish_domestic_interbanking_identifier: String,
	}
	
	impl SpanishDomesticInterbankingIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("ES[0-9]{8,9}").unwrap();
			if !pattern.is_match(&self.spanish_domestic_interbanking_identifier) {
				return Err(ValidationError::new(1005, "spanish_domestic_interbanking_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// StatementFrequencyReason2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct StatementFrequencyReason2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<EventFrequency9Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl StatementFrequencyReason2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SwissBCIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct SwissBCIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub swiss_bc_identifier: String,
	}
	
	impl SwissBCIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("SW[0-9]{3,5}").unwrap();
			if !pattern.is_match(&self.swiss_bc_identifier) {
				return Err(ValidationError::new(1005, "swiss_bc_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// SwissSICIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct SwissSICIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub swiss_sic_identifier: String,
	}
	
	impl SwissSICIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("SW[0-9]{6,6}").unwrap();
			if !pattern.is_match(&self.swiss_sic_identifier) {
				return Err(ValidationError::new(1005, "swiss_sic_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// TaxExemptReason3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum TaxExemptReason3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NONE") )]
		CodeNONE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MASA") )]
		CodeMASA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MISA") )]
		CodeMISA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SISA") )]
		CodeSISA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IISA") )]
		CodeIISA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CUYP") )]
		CodeCUYP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRYP") )]
		CodePRYP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ASTR") )]
		CodeASTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EMPY") )]
		CodeEMPY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EMCY") )]
		CodeEMCY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EPRY") )]
		CodeEPRY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ECYE") )]
		CodeECYE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NFPI") )]
		CodeNFPI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NFQP") )]
		CodeNFQP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DECP") )]
		CodeDECP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IRAC") )]
		CodeIRAC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IRAR") )]
		CodeIRAR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KEOG") )]
		CodeKEOG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PFSP") )]
		CodePFSP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "401K") )]
		Code401K,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SIRA") )]
		CodeSIRA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "403B") )]
		Code403B,
		#[cfg_attr( feature = "derive_serde", serde(rename = "457X") )]
		Code457X,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RIRA") )]
		CodeRIRA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RIAN") )]
		CodeRIAN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RCRF") )]
		CodeRCRF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RCIP") )]
		CodeRCIP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EIFP") )]
		CodeEIFP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EIOP") )]
		CodeEIOP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FORE") )]
		CodeFORE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INCA") )]
		CodeINCA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MINO") )]
		CodeMINO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ASSO") )]
		CodeASSO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DIPL") )]
		CodeDIPL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DOME") )]
		CodeDOME,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FORP") )]
		CodeFORP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ORDR") )]
		CodeORDR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PENF") )]
		CodePENF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REFU") )]
		CodeREFU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RIHO") )]
		CodeRIHO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADMI") )]
		CodeADMI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TANR") )]
		CodeTANR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OANR") )]
		CodeOANR,
	}
	
	impl TaxExemptReason3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TaxExemptionReason2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxExemptionReason2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<TaxExemptReason3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl TaxExemptionReason2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TaxReporting3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxReporting3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxtnCtry") )]
		pub taxtn_ctry: CountryCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRate", skip_serializing_if = "Option::is_none") )]
		pub tax_rate: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxPyer", skip_serializing_if = "Option::is_none") )]
		pub tax_pyer: Option<PartyIdentification125Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRcpt", skip_serializing_if = "Option::is_none") )]
		pub tax_rcpt: Option<PartyIdentification125Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctDtls", skip_serializing_if = "Option::is_none") )]
		pub csh_acct_dtls: Option<CashAccount204>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max350Text>,
	}
	
	impl TaxReporting3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.taxtn_ctry.validate() { return Err(e); }
			if let Some(ref tax_pyer_value) = self.tax_pyer { if let Err(e) = tax_pyer_value.validate() { return Err(e); } }
			if let Some(ref tax_rcpt_value) = self.tax_rcpt { if let Err(e) = tax_rcpt_value.validate() { return Err(e); } }
			if let Some(ref csh_acct_dtls_value) = self.csh_acct_dtls { if let Err(e) = csh_acct_dtls_value.validate() { return Err(e); } }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TaxWithholdingMethod3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum TaxWithholdingMethod3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "MITX") )]
		CodeMITX,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVE") )]
		CodeINVE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACCT") )]
		CodeACCT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EXMT") )]
		CodeEXMT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REPT") )]
		CodeREPT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CRTF") )]
		CodeCRTF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WHCO") )]
		CodeWHCO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WTHD") )]
		CodeWTHD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WTRE") )]
		CodeWTRE,
	}
	
	impl TaxWithholdingMethod3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ThirdPartyRights2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ThirdPartyRights2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtTm", skip_serializing_if = "Option::is_none") )]
		pub dt_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Hldr", skip_serializing_if = "Option::is_none") )]
		pub hldr: Option<PartyIdentification125Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none") )]
		pub lgl_ntty_idr: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<ActiveCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max350Text>,
	}
	
	impl ThirdPartyRights2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Some(ref hldr_value) = self.hldr { if let Err(e) = hldr_value.validate() { return Err(e); } }
			if let Some(ref lgl_ntty_idr_value) = self.lgl_ntty_idr { if let Err(e) = lgl_ntty_idr_value.validate() { return Err(e); } }
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TickerIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct TickerIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub ticker_identifier: String,
	}
	
	impl TickerIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.ticker_identifier.chars().count() < 1 {
			return Err(ValidationError::new(1001, "ticker_identifier is shorter than the minimum length of 1".to_string()));
			}
			if self.ticker_identifier.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ticker_identifier exceeds the maximum length of 35".to_string()));
			}
			Ok(())
		}
	}
	
	
	// TransactionChannel2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum TransactionChannel2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "FIAD") )]
		CodeFIAD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HOBA") )]
		CodeHOBA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BRAN") )]
		CodeBRAN,
	}
	
	impl TransactionChannel2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TransactionChannelType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TransactionChannelType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<TransactionChannel2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl TransactionChannelType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TransactionType5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TransactionType5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<InvestmentFundTransactionType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl TransactionType5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TreasuryProfile1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TreasuryProfile1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt") )]
		pub dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradrTp") )]
		pub tradr_tp: PartyRole5Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rate") )]
		pub rate: f64,
	}
	
	impl TreasuryProfile1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tradr_tp.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// UKDomesticSortCodeIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct UKDomesticSortCodeIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub uk_domestic_sort_code_identifier: String,
	}
	
	impl UKDomesticSortCodeIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("SC[0-9]{6,6}").unwrap();
			if !pattern.is_match(&self.uk_domestic_sort_code_identifier) {
				return Err(ValidationError::new(1005, "uk_domestic_sort_code_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// UnitsOrAmount1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct UnitsOrAmount1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<ActiveCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
		pub unit: Option<f64>,
	}
	
	impl UnitsOrAmount1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// UnitsOrAmountOrPercentage1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct UnitsOrAmountOrPercentage1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<ActiveCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
		pub unit: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pctg", skip_serializing_if = "Option::is_none") )]
		pub pctg: Option<f64>,
	}
	
	impl UnitsOrAmountOrPercentage1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ValorenIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ValorenIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub valoren_identifier: String,
	}
	
	impl ValorenIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// WertpapierIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct WertpapierIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub wertpapier_identifier: String,
	}
	
	impl WertpapierIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// YesNoIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct YesNoIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub yes_no_indicator: bool,
	}
	
	impl YesNoIndicator {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
}