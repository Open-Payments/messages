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
		pub acct_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdAcctDtls", skip_serializing_if = "Option::is_none") )]
		pub rltd_acct_dtls: Option<GenericIdentification1>,
	}
	
	impl Account23 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.acct_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_id is shorter than the minimum length of 1".to_string()));
			}
			if self.acct_id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_id exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.rltd_acct_dtls { if let Err(e) = val.validate() { return Err(e); } }
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
		pub id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcr") )]
		pub acct_svcr: PartyIdentification125Choice,
	}
	
	impl Account32 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
				}
			}
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.ordr_ref { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rltd_ref { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.conf_dtls.validate() { return Err(e); }
			if let Some(ref val) = self.invstmt_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.acct_pties { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.intrmies { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.plcmnt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.new_isse_allcn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.svgs_invstmt_plan { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.wdrwl_invstmt_plan { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.csh_sttlm { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.svc_lvl_agrmt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.addtl_inf { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.mkt_prctc_vrsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.xtnsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
		pub iban: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<GenericAccountIdentification1>,
	}
	
	impl AccountIdentification4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.iban {
				let pattern = Regex::new("[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "iban does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.othr { if let Err(e) = val.validate() { return Err(e); } }
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
		pub nm: Option<String>,
	}
	
	impl AccountIdentificationAndName5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref val) = self.nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "nm exceeds the maximum length of 35".to_string()));
				}
			}
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
		pub acct_appl_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClntRef", skip_serializing_if = "Option::is_none") )]
		pub clnt_ref: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyRef", skip_serializing_if = "Option::is_none") )]
		pub ctr_pty_ref: Option<AdditionalReference13>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ExstgAcctId", skip_serializing_if = "Option::is_none") )]
		pub exstg_acct_id: Option<Vec<Account23>>,
	}
	
	impl AccountManagementConfirmation5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.conf_tp.validate() { return Err(e); }
			if let Some(ref val) = self.acct_appl_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "acct_appl_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "acct_appl_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.clnt_ref {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "clnt_ref is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "clnt_ref exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.ctr_pty_ref { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.exstg_acct_id { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
			if let Some(ref val) = self.pmry_ownr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.trstee { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.nmnee { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.jnt_ownr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
			if let Some(ref vec) = self.scndry_ownr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.bnfcry { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.pwr_of_attny { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.lgl_guardn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.ctdn_for_mnr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.sucssr_on_dth { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.admstr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.othr_pty { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.grntr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.sttlr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.snr_mgg_offcl { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.prtctr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.regd_shrhldr_nm { if let Err(e) = val.validate() { return Err(e); } }
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
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl AccountSchemeName1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 4 {
					return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
				}
			}
			if let Some(ref val) = self.prtry {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
				}
			}
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
			if let Some(ref val) = self.nbld { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dsbld { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.pdg { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.pdg_opng { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.profrm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.clsd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.clsr_pdg { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.othr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// AdditionalReference13 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AdditionalReference13 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
		pub ref_attr: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefIssr", skip_serializing_if = "Option::is_none") )]
		pub ref_issr: Option<PartyIdentification125Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNm", skip_serializing_if = "Option::is_none") )]
		pub msg_nm: Option<String>,
	}
	
	impl AdditionalReference13 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.ref_attr.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ref_attr is shorter than the minimum length of 1".to_string()));
			}
			if self.ref_attr.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ref_attr exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.ref_issr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.msg_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "msg_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "msg_nm exceeds the maximum length of 35".to_string()));
				}
			}
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
		pub lmttn: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctVldtn", skip_serializing_if = "Option::is_none") )]
		pub acct_vldtn: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rgltr", skip_serializing_if = "Option::is_none") )]
		pub rgltr: Option<PartyIdentification125Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
		pub sts: Option<RestrictionStatus1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
		pub prd: Option<DateTimePeriod2>,
	}
	
	impl AdditiononalInformation13 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.lmttn {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "lmttn is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "lmttn exceeds the maximum length of 350".to_string()));
				}
			}
			if let Some(ref val) = self.addtl_inf {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
				}
			}
			if let Some(ref val) = self.acct_vldtn {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "acct_vldtn is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "acct_vldtn exceeds the maximum length of 350".to_string()));
				}
			}
			if let Some(ref val) = self.tp {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.rgltr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.sts { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prd { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IdSrc") )]
		pub id_src: IdentificationSource1Choice,
	}
	
	impl AlternateSecurityIdentification7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
			if let Err(e) = self.id_src.validate() { return Err(e); }
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
		pub hldg_cert_nb: Option<String>,
	}
	
	impl BlockedHoldingDetails2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.blckd_hldg.validate() { return Err(e); }
			if let Some(ref val) = self.hldg_cert_nb {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "hldg_cert_nb is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "hldg_cert_nb exceeds the maximum length of 35".to_string()));
				}
			}
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub addtl_inf: String,
	}
	
	impl BlockedStatusReason2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tx_tp.validate() { return Err(e); }
			if let Some(ref vec) = self.rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if self.addtl_inf.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if self.addtl_inf.chars().count() > 350 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
			}
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
			if let Some(ref val) = self.no_spcfd_rsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
		pub id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress1>,
	}
	
	impl BranchData4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "nm exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.pstl_adr { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub xcptnl_rptg_ctry: Option<String>,
	}
	
	impl CRSStatus4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Some(ref val) = self.src { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.xcptnl_rptg_ctry {
				let pattern = Regex::new("[A-Z]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "xcptnl_rptg_ctry does not match the required pattern".to_string()));
				}
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
		pub sttlm_ccy: String,
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
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.sttlm_ccy) {
				return Err(ValidationError::new(1005, "sttlm_ccy does not match the required pattern".to_string()));
			}
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref val) = self.acct_ownr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.acct_svcr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.acct_svcr_brnch { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.acct_ownr_othr_id { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.invstmt_acct_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.cdt_dbt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.sttlm_instr_rsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.csh_acct_purp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.csh_acct_dsgnt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dvdd_pctg {
				if *val < 0.000000 {
					return Err(ValidationError::new(1003, "dvdd_pctg is less than the minimum value of 0.000000".to_string()));
				}
				if *val > 100.000000 {
					return Err(ValidationError::new(1004, "dvdd_pctg exceeds the maximum value of 100.000000".to_string()));
				}
			}
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref vec) = self.csh_acct_dtls { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.othr_csh_sttlm_dtls { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub uschu: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NZNCC", skip_serializing_if = "Option::is_none") )]
		pub nzncc: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IENSC", skip_serializing_if = "Option::is_none") )]
		pub iensc: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GBSC", skip_serializing_if = "Option::is_none") )]
		pub gbsc: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "USCH", skip_serializing_if = "Option::is_none") )]
		pub usch: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CHBC", skip_serializing_if = "Option::is_none") )]
		pub chbc: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "USFW", skip_serializing_if = "Option::is_none") )]
		pub usfw: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PTNCC", skip_serializing_if = "Option::is_none") )]
		pub ptncc: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RUCB", skip_serializing_if = "Option::is_none") )]
		pub rucb: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ITNCC", skip_serializing_if = "Option::is_none") )]
		pub itncc: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ATBLZ", skip_serializing_if = "Option::is_none") )]
		pub atblz: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CACPA", skip_serializing_if = "Option::is_none") )]
		pub cacpa: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CHSIC", skip_serializing_if = "Option::is_none") )]
		pub chsic: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DEBLZ", skip_serializing_if = "Option::is_none") )]
		pub deblz: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ESNCC", skip_serializing_if = "Option::is_none") )]
		pub esncc: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ZANCC", skip_serializing_if = "Option::is_none") )]
		pub zancc: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HKNCC", skip_serializing_if = "Option::is_none") )]
		pub hkncc: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AUBSBx", skip_serializing_if = "Option::is_none") )]
		pub aubs_bx: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AUBSBs", skip_serializing_if = "Option::is_none") )]
		pub aubs_bs: Option<String>,
	}
	
	impl ClearingSystemMemberIdentification4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.uschu {
				let pattern = Regex::new("CH[0-9]{6,6}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "uschu does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.nzncc {
				let pattern = Regex::new("NZ[0-9]{6,6}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "nzncc does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.iensc {
				let pattern = Regex::new("IE[0-9]{6,6}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "iensc does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.gbsc {
				let pattern = Regex::new("SC[0-9]{6,6}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "gbsc does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.usch {
				let pattern = Regex::new("CP[0-9]{4,4}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "usch does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.chbc {
				let pattern = Regex::new("SW[0-9]{3,5}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "chbc does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.usfw {
				let pattern = Regex::new("FW[0-9]{9,9}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "usfw does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.ptncc {
				let pattern = Regex::new("PT[0-9]{8,8}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "ptncc does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.rucb {
				let pattern = Regex::new("RU[0-9]{9,9}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "rucb does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.itncc {
				let pattern = Regex::new("IT[0-9]{10,10}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "itncc does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.atblz {
				let pattern = Regex::new("AT[0-9]{5,5}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "atblz does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.cacpa {
				let pattern = Regex::new("CA[0-9]{9,9}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "cacpa does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.chsic {
				let pattern = Regex::new("SW[0-9]{6,6}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "chsic does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.deblz {
				let pattern = Regex::new("BL[0-9]{8,8}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "deblz does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.esncc {
				let pattern = Regex::new("ES[0-9]{8,9}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "esncc does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.zancc {
				let pattern = Regex::new("ZA[0-9]{6,6}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "zancc does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.hkncc {
				let pattern = Regex::new("HK[0-9]{3,3}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "hkncc does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.aubs_bx {
				let pattern = Regex::new("AU[0-9]{6,6}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "aubs_bx does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.aubs_bs {
				let pattern = Regex::new("AU[0-9]{6,6}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "aubs_bs does not match the required pattern".to_string()));
				}
			}
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
		pub addtl_inf: Option<String>,
	}
	
	impl ClosedStatusReason1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref val) = self.addtl_inf {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
				}
			}
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
			if let Some(ref val) = self.no_spcfd_rsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub addtl_inf: Option<String>,
	}
	
	impl ClosurePendingStatusReason1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref val) = self.addtl_inf {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
				}
			}
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
			if let Some(ref val) = self.no_spcfd_rsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub email: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Phne", skip_serializing_if = "Option::is_none") )]
		pub phne: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mob", skip_serializing_if = "Option::is_none") )]
		pub mob: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FaxNb", skip_serializing_if = "Option::is_none") )]
		pub fax_nb: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TlxAdr", skip_serializing_if = "Option::is_none") )]
		pub tlx_adr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "URLAdr", skip_serializing_if = "Option::is_none") )]
		pub url_adr: Option<String>,
	}
	
	impl CommunicationAddress6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.adr_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.email {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "email is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 256 {
					return Err(ValidationError::new(1002, "email exceeds the maximum length of 256".to_string()));
				}
			}
			if let Some(ref val) = self.phne {
				let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "phne does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.mob {
				let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "mob does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.fax_nb {
				let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "fax_nb does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.tlx_adr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tlx_adr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "tlx_adr exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.url_adr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "url_adr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 256 {
					return Err(ValidationError::new(1002, "url_adr exceeds the maximum length of 256".to_string()));
				}
			}
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub ctry: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ResdtlSts") )]
		pub resdtl_sts: ResidentialStatus1Code,
	}
	
	impl CountryAndResidentialStatusType2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&self.ctry) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
			if let Err(e) = self.resdtl_sts.validate() { return Err(e); }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub id: String,
	}
	
	impl DataBaseCheck1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
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
			if let Some(ref val) = self.de_mnms_aplbl { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.de_mnms_not_aplbl { if let Err(e) = val.validate() { return Err(e); } }
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
		pub rstrctd_prsn_rsn: String,
	}
	
	impl DeMinimusNotApplicable1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.rstrctd_prsn_rsn.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rstrctd_prsn_rsn is shorter than the minimum length of 1".to_string()));
			}
			if self.rstrctd_prsn_rsn.chars().count() > 350 {
				return Err(ValidationError::new(1002, "rstrctd_prsn_rsn exceeds the maximum length of 350".to_string()));
			}
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
		pub dbtr_tax_id_nb: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrNtlRegnNb", skip_serializing_if = "Option::is_none") )]
		pub dbtr_ntl_regn_nb: Option<String>,
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
		pub regn_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MndtId", skip_serializing_if = "Option::is_none") )]
		pub mndt_id: Option<String>,
	}
	
	impl DirectDebitMandate7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.dbtr_acct.validate() { return Err(e); }
			if let Some(ref val) = self.dbtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dbtr_tax_id_nb {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "dbtr_tax_id_nb is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "dbtr_tax_id_nb exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.dbtr_ntl_regn_nb {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "dbtr_ntl_regn_nb is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "dbtr_ntl_regn_nb exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.cdtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.dbtr_agt.validate() { return Err(e); }
			if let Some(ref val) = self.dbtr_agt_brnch { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.cdtr_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.cdtr_agt_brnch { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.regn_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "regn_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "regn_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.mndt_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "mndt_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "mndt_id exceeds the maximum length of 35".to_string()));
				}
			}
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
		pub addtl_inf: Option<String>,
	}
	
	impl DisabledStatusReason1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref val) = self.addtl_inf {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
				}
			}
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
			if let Some(ref val) = self.no_spcfd_rsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub tp: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rcpt") )]
		pub rcpt: PartyIdentification125Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MtdOfTrnsmssn") )]
		pub mtd_of_trnsmssn: CommunicationMethod3Choice,
	}
	
	impl DocumentToSend4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.tp.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
			}
			if self.tp.chars().count() > 140 {
				return Err(ValidationError::new(1002, "tp exceeds the maximum length of 140".to_string()));
			}
			if let Err(e) = self.rcpt.validate() { return Err(e); }
			if let Err(e) = self.mtd_of_trnsmssn.validate() { return Err(e); }
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
		pub addtl_inf: Option<String>,
	}
	
	impl EnabledStatusReason1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref val) = self.addtl_inf {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
				}
			}
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
			if let Some(ref val) = self.no_spcfd_rsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// ExtendedParty14 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ExtendedParty14 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "XtndedPtyRole") )]
		pub xtnded_pty_role: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPtyDtls") )]
		pub othr_pty_dtls: InvestmentAccountOwnershipInformation16,
	}
	
	impl ExtendedParty14 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.xtnded_pty_role.chars().count() < 1 {
				return Err(ValidationError::new(1001, "xtnded_pty_role is shorter than the minimum length of 1".to_string()));
			}
			if self.xtnded_pty_role.chars().count() > 350 {
				return Err(ValidationError::new(1002, "xtnded_pty_role exceeds the maximum length of 350".to_string()));
			}
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
		pub plc_and_nm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Txt") )]
		pub txt: String,
	}
	
	impl Extension1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.plc_and_nm.chars().count() < 1 {
				return Err(ValidationError::new(1001, "plc_and_nm is shorter than the minimum length of 1".to_string()));
			}
			if self.plc_and_nm.chars().count() > 350 {
				return Err(ValidationError::new(1002, "plc_and_nm exceeds the maximum length of 350".to_string()));
			}
			if self.txt.chars().count() < 1 {
				return Err(ValidationError::new(1001, "txt is shorter than the minimum length of 1".to_string()));
			}
			if self.txt.chars().count() > 350 {
				return Err(ValidationError::new(1002, "txt exceeds the maximum length of 350".to_string()));
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.src { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub bicfi: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none") )]
		pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<SimpleIdentificationInformation4>,
	}
	
	impl FinancialInstitutionIdentification11Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.nm_and_adr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.bicfi {
				let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "bicfi does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.clr_sys_mmb_id { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry_id { if let Err(e) = val.validate() { return Err(e); } }
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
		pub nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
		pub shrt_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryId", skip_serializing_if = "Option::is_none") )]
		pub splmtry_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClssTp", skip_serializing_if = "Option::is_none") )]
		pub clss_tp: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesForm", skip_serializing_if = "Option::is_none") )]
		pub scties_form: Option<FormOfSecurity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none") )]
		pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdctGrp", skip_serializing_if = "Option::is_none") )]
		pub pdct_grp: Option<String>,
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
			if let Some(ref val) = self.nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
				}
			}
			if let Some(ref val) = self.shrt_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "shrt_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "shrt_nm exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.splmtry_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "splmtry_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "splmtry_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.clss_tp {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "clss_tp is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "clss_tp exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.scties_form { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dstrbtn_plcy { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.pdct_grp {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "pdct_grp is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "pdct_grp exceeds the maximum length of 140".to_string()));
				}
			}
			if let Some(ref val) = self.blckd_hldg_dtls { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.pldgg { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.coll { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.thrd_pty_rghts { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fnd_ownrsh { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fnd_intntn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.oprl_sts { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<AccountSchemeName1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<String>,
	}
	
	impl GenericAccountIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 34 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 34".to_string()));
			}
			if let Some(ref val) = self.schme_nm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.issr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
				}
			}
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
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<String>,
	}
	
	impl GenericIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.schme_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.issr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
				}
			}
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
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<String>,
	}
	
	impl GenericIdentification36 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
			if self.issr.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if self.issr.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.schme_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
				}
			}
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
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<String>,
	}
	
	impl GenericIdentification47 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
			if !pattern.is_match(&self.id) {
				return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
			}
			if self.issr.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if self.issr.chars().count() > 4 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 4".to_string()));
			}
			let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
			if !pattern.is_match(&self.issr) {
				return Err(ValidationError::new(1005, "issr does not match the required pattern".to_string()));
			}
			if let Some(ref val) = self.schme_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 4 {
					return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 4".to_string()));
				}
				let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "schme_nm does not match the required pattern".to_string()));
				}
			}
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
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: OtherIdentification3Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IsseDt", skip_serializing_if = "Option::is_none") )]
		pub isse_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt", skip_serializing_if = "Option::is_none") )]
		pub xpry_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Stat", skip_serializing_if = "Option::is_none") )]
		pub stat: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IssrCtry", skip_serializing_if = "Option::is_none") )]
		pub issr_ctry: Option<String>,
	}
	
	impl GenericIdentification82 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Some(ref val) = self.issr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.stat {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "stat is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 70 {
					return Err(ValidationError::new(1002, "stat exceeds the maximum length of 70".to_string()));
				}
			}
			if let Some(ref val) = self.issr_ctry {
				let pattern = Regex::new("[A-Z]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "issr_ctry does not match the required pattern".to_string()));
				}
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
			if let Some(ref val) = self.sttlm_frqcy { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.cnsldtn_tp { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// IdentificationSource1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IdentificationSource1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dmst", skip_serializing_if = "Option::is_none") )]
		pub dmst: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl IdentificationSource1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.dmst {
				let pattern = Regex::new("[A-Z]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "dmst does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.prtry {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
				}
			}
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
		pub gvn_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MddlNm", skip_serializing_if = "Option::is_none") )]
		pub mddl_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr") )]
		pub pstl_adr: Vec<PostalAddress21>,
	}
	
	impl IndividualPerson29 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.nm_prfx { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.gvn_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "gvn_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "gvn_nm exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.mddl_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "mddl_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "mddl_nm exceeds the maximum length of 35".to_string()));
				}
			}
			if self.nm.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if self.nm.chars().count() > 350 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
			}
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
		pub gvn_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MddlNm", skip_serializing_if = "Option::is_none") )]
		pub mddl_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmSfx", skip_serializing_if = "Option::is_none") )]
		pub nm_sfx: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Gndr", skip_serializing_if = "Option::is_none") )]
		pub gndr: Option<Gender1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BirthDt", skip_serializing_if = "Option::is_none") )]
		pub birth_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfBirth", skip_serializing_if = "Option::is_none") )]
		pub ctry_of_birth: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none") )]
		pub prvc_of_birth: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CityOfBirth", skip_serializing_if = "Option::is_none") )]
		pub city_of_birth: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prfssn", skip_serializing_if = "Option::is_none") )]
		pub prfssn: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr") )]
		pub pstl_adr: Vec<PostalAddress21>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctznsh", skip_serializing_if = "Option::is_none") )]
		pub ctznsh: Option<Vec<CitizenshipInformation2>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EmplngCpny", skip_serializing_if = "Option::is_none") )]
		pub emplng_cpny: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BizFctn", skip_serializing_if = "Option::is_none") )]
		pub biz_fctn: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PltclyXpsdPrsn", skip_serializing_if = "Option::is_none") )]
		pub pltcly_xpsd_prsn: Option<PoliticallyExposedPerson1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DthDt", skip_serializing_if = "Option::is_none") )]
		pub dth_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CvlSts", skip_serializing_if = "Option::is_none") )]
		pub cvl_sts: Option<CivilStatus1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EdctnLvl", skip_serializing_if = "Option::is_none") )]
		pub edctn_lvl: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FmlyInf", skip_serializing_if = "Option::is_none") )]
		pub fmly_inf: Option<PersonalInformation1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GDPRData", skip_serializing_if = "Option::is_none") )]
		pub gdpr_data: Option<Vec<GDPRData1>>,
	}
	
	impl IndividualPerson37 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.nm_prfx { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.gvn_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "gvn_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "gvn_nm exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.mddl_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "mddl_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "mddl_nm exceeds the maximum length of 35".to_string()));
				}
			}
			if self.nm.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if self.nm.chars().count() > 350 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
			}
			if let Some(ref val) = self.nm_sfx {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nm_sfx is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "nm_sfx exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.gndr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ctry_of_birth {
				let pattern = Regex::new("[A-Z]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "ctry_of_birth does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.prvc_of_birth {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "prvc_of_birth is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "prvc_of_birth exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.city_of_birth {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "city_of_birth is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "city_of_birth exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.prfssn {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "prfssn is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "prfssn exceeds the maximum length of 35".to_string()));
				}
			}
			for item in &self.pstl_adr { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref vec) = self.ctznsh { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.emplng_cpny {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "emplng_cpny is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "emplng_cpny exceeds the maximum length of 140".to_string()));
				}
			}
			if let Some(ref val) = self.biz_fctn {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "biz_fctn is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "biz_fctn exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.pltcly_xpsd_prsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.cvl_sts { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.edctn_lvl {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "edctn_lvl is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "edctn_lvl exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.fmly_inf { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.gdpr_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.amt { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub lgl_ntty_idr: Option<String>,
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
			if let Some(ref val) = self.lgl_ntty_idr {
				let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "lgl_ntty_idr does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.role { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.pmry_com_adr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.scndry_com_adr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.nm_and_adr { if let Err(e) = val.validate() { return Err(e); } }
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
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSts", skip_serializing_if = "Option::is_none") )]
		pub acct_sts: Option<AccountStatus2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BlckdSts", skip_serializing_if = "Option::is_none") )]
		pub blckd_sts: Option<BlockedStatusReason2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StsDt", skip_serializing_if = "Option::is_none") )]
		pub sts_dt: Option<DateAndDateTime1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dsgnt", skip_serializing_if = "Option::is_none") )]
		pub dsgnt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<AccountType2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OwnrshTp", skip_serializing_if = "Option::is_none") )]
		pub ownrsh_tp: Option<OwnershipType2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxXmptn", skip_serializing_if = "Option::is_none") )]
		pub tax_xmptn: Option<TaxExemptionReason2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StmtFrqcy", skip_serializing_if = "Option::is_none") )]
		pub stmt_frqcy: Option<StatementFrequencyReason2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefCcy", skip_serializing_if = "Option::is_none") )]
		pub ref_ccy: Option<String>,
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
		pub acmltn_rght_ref: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ReqrdSgntriesNb", skip_serializing_if = "Option::is_none") )]
		pub reqrd_sgntries_nb: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FndFmlyNm", skip_serializing_if = "Option::is_none") )]
		pub fnd_fmly_nm: Option<String>,
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
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.acct_sts { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.blckd_sts { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.sts_dt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "nm exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.dsgnt {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "dsgnt is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "dsgnt exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ownrsh_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.tax_xmptn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.stmt_frqcy { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ref_ccy {
				let pattern = Regex::new("[A-Z]{3,3}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "ref_ccy does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.incm_pref { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rinvstmt_dtls { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.tax_whldg_mtd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.tax_rptg { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.lttr_intt_dtls { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.acmltn_rght_ref {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "acmltn_rght_ref is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "acmltn_rght_ref exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.fnd_fmly_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "fnd_fmly_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "fnd_fmly_nm exceeds the maximum length of 350".to_string()));
				}
			}
			if let Some(ref vec) = self.fin_instrm_dtls { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.rndg_dtls { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.acct_svcr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.acct_usg_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.frgn_sts_certfctn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.acct_sgntr_dt_tm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.tx_chanl_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.invstmt_acct_ctgy { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.pldgg { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.coll { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.thrd_pty_rghts { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.pwr_of_attny_lvl_of_ctrl { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.acctg_sts { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.opng_dt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.clsg_dt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prcg_ordr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.lblty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.invstr_prfl { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.fscl_yr { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub clnt_id: Option<String>,
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
		pub elctrnc_mlng_svc_ref: Option<String>,
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
			if let Some(ref val) = self.mny_lndrg_chck { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.invstr_prfl_vldtn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.ownrsh_bnfcry_rate { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.clnt_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "clnt_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "clnt_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.mi_fid_clssfctn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.ntfctn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.fatca_form_tp { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.fatca_sts { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.crs_form_tp { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.crs_sts { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.othr_id { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.tax_xmptn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.tax_rptg { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.mail_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ctry_and_resdtl_sts { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.mntry_wlth { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.eqty_val { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.workg_cptl { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.cpny_lk { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.elctrnc_mlng_svc_ref {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "elctrnc_mlng_svc_ref is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "elctrnc_mlng_svc_ref exceeds the maximum length of 350".to_string()));
				}
			}
			if let Some(ref vec) = self.pmry_com_adr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.scndry_com_adr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.addtl_rgltry_inf { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.acctg_sts { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.addtl_inf { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
		pub ordr_ref: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MstrRef", skip_serializing_if = "Option::is_none") )]
		pub mstr_ref: Option<String>,
	}
	
	impl InvestmentFundOrder4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.ordr_ref {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "ordr_ref is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "ordr_ref exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.mstr_ref {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "mstr_ref is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "mstr_ref exceeds the maximum length of 35".to_string()));
				}
			}
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
		pub ctrct_ref: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdCtrctRef", skip_serializing_if = "Option::is_none") )]
		pub rltd_ctrct_ref: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdctId", skip_serializing_if = "Option::is_none") )]
		pub pdct_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SLAChrgAndComssnRef", skip_serializing_if = "Option::is_none") )]
		pub sla_chrg_and_comssn_ref: Option<String>,
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
			if let Some(ref val) = self.incm_pref { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.initl_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rndg_drctn { if let Err(e) = val.validate() { return Err(e); } }
			for item in &self.scty_dtls { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref vec) = self.csh_sttlm { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.ctrct_ref {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "ctrct_ref is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "ctrct_ref exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.rltd_ctrct_ref {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "rltd_ctrct_ref is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "rltd_ctrct_ref exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.pdct_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "pdct_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "pdct_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.sla_chrg_and_comssn_ref {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "sla_chrg_and_comssn_ref is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "sla_chrg_and_comssn_ref exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.insrnc_cover { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.plan_sts { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.instlmt_mgr_role { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.sts { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.trsr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.hgh_frqcy_tradg { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.mkt_makr { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// LetterIntent1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct LetterIntent1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "LttrInttRef") )]
		pub lttr_intt_ref: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
		pub start_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt", skip_serializing_if = "Option::is_none") )]
		pub end_dt: Option<String>,
	}
	
	impl LetterIntent1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.lttr_intt_ref.chars().count() < 1 {
				return Err(ValidationError::new(1001, "lttr_intt_ref is shorter than the minimum length of 1".to_string()));
			}
			if self.lttr_intt_ref.chars().count() > 35 {
				return Err(ValidationError::new(1002, "lttr_intt_ref exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.amt { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.ctrct_prd { if let Err(e) = val.validate() { return Err(e); } }
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
		pub nm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
		pub dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nb", skip_serializing_if = "Option::is_none") )]
		pub nb: Option<String>,
	}
	
	impl MarketPracticeVersion1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.nm.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if self.nm.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.nb {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nb is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "nb exceeds the maximum length of 35".to_string()));
				}
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
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
		pub cre_dt_tm: String,
	}
	
	impl MessageIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
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
		pub nrrtv: Option<String>,
	}
	
	impl MiFIDClassification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.clssfctn.validate() { return Err(e); }
			if let Some(ref val) = self.nrrtv {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nrrtv is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "nrrtv exceeds the maximum length of 350".to_string()));
				}
			}
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Adr") )]
		pub adr: PostalAddress1,
	}
	
	impl NameAndAddress4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
				}
			}
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
		pub nm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Adr", skip_serializing_if = "Option::is_none") )]
		pub adr: Option<PostalAddress1>,
	}
	
	impl NameAndAddress5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.nm.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if self.nm.chars().count() > 350 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
			}
			if let Some(ref val) = self.adr { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub xmpt_prsn_rsn: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DeMnms", skip_serializing_if = "Option::is_none") )]
		pub de_mnms: Option<DeMinimus1Choice>,
	}
	
	impl NewIssueAllocation2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.xmpt_prsn_rsn {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "xmpt_prsn_rsn is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "xmpt_prsn_rsn exceeds the maximum length of 350".to_string()));
				}
			}
			if let Some(ref val) = self.de_mnms { if let Err(e) = val.validate() { return Err(e); } }
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
		pub ntfctn_tp: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Reqrd") )]
		pub reqrd: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DstrbtnTp", skip_serializing_if = "Option::is_none") )]
		pub dstrbtn_tp: Option<InformationDistribution1Choice>,
	}
	
	impl Notification2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.ntfctn_tp.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ntfctn_tp is shorter than the minimum length of 1".to_string()));
			}
			if self.ntfctn_tp.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ntfctn_tp exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.dstrbtn_tp { if let Err(e) = val.validate() { return Err(e); } }
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
		pub nm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
		pub shrt_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr") )]
		pub pstl_adr: Vec<PostalAddress21>,
	}
	
	impl Organisation23 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.nm.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if self.nm.chars().count() > 350 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
			}
			if let Some(ref val) = self.shrt_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "shrt_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "shrt_nm exceeds the maximum length of 35".to_string()));
				}
			}
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
		pub nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
		pub shrt_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<PartyIdentification177Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none") )]
		pub lgl_ntty_idr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
		pub purp: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RegnCtry", skip_serializing_if = "Option::is_none") )]
		pub regn_ctry: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RegnDt", skip_serializing_if = "Option::is_none") )]
		pub regn_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<Vec<PostalAddress21>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TpOfOrg", skip_serializing_if = "Option::is_none") )]
		pub tp_of_org: Option<OrganisationType1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PlcOfListg", skip_serializing_if = "Option::is_none") )]
		pub plc_of_listg: Option<Vec<String>>,
	}
	
	impl Organisation39 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
				}
			}
			if let Some(ref val) = self.shrt_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "shrt_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "shrt_nm exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.id { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.lgl_ntty_idr {
				let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "lgl_ntty_idr does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.purp {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "purp is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "purp exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.regn_ctry {
				let pattern = Regex::new("[A-Z]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "regn_ctry does not match the required pattern".to_string()));
				}
			}
			if let Some(ref vec) = self.pstl_adr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.tp_of_org { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.plc_of_listg {
				for item in vec {
					let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
					if !pattern.is_match(&item) {
						return Err(ValidationError::new(1005, "plc_of_listg does not match the required pattern".to_string()));
					}
				}
			}
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.rsn { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub frctn: Option<String>,
	}
	
	impl OwnershipBeneficiaryRate1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.frctn {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "frctn is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "frctn exceeds the maximum length of 35".to_string()));
				}
			}
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.org { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.indv_prsn { if let Err(e) = val.validate() { return Err(e); } }
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
		pub any_bic: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<GenericIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
		pub nm_and_adr: Option<NameAndAddress5>,
	}
	
	impl PartyIdentification125Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.any_bic {
				let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.prtry_id { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.nm_and_adr { if let Err(e) = val.validate() { return Err(e); } }
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
		pub any_bic: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<GenericIdentification1>,
	}
	
	impl PartyIdentification177Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.any_bic {
				let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.prtry_id { if let Err(e) = val.validate() { return Err(e); } }
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
		pub vldtng_pty: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ChckngPty", skip_serializing_if = "Option::is_none") )]
		pub chckng_pty: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RspnsblPty", skip_serializing_if = "Option::is_none") )]
		pub rspnsbl_pty: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CertTp", skip_serializing_if = "Option::is_none") )]
		pub cert_tp: Option<CertificationType1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ChckngDt", skip_serializing_if = "Option::is_none") )]
		pub chckng_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ChckngFrqcy", skip_serializing_if = "Option::is_none") )]
		pub chckng_frqcy: Option<EventFrequency1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NxtRvsnDt", skip_serializing_if = "Option::is_none") )]
		pub nxt_rvsn_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SlryRg", skip_serializing_if = "Option::is_none") )]
		pub slry_rg: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SrcOfWlth", skip_serializing_if = "Option::is_none") )]
		pub src_of_wlth: Option<String>,
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
			if let Some(ref val) = self.vldtng_pty {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "vldtng_pty is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "vldtng_pty exceeds the maximum length of 140".to_string()));
				}
			}
			if let Some(ref val) = self.chckng_pty {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "chckng_pty is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "chckng_pty exceeds the maximum length of 140".to_string()));
				}
			}
			if let Some(ref val) = self.rspnsbl_pty {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "rspnsbl_pty is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "rspnsbl_pty exceeds the maximum length of 140".to_string()));
				}
			}
			if let Some(ref val) = self.cert_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.chckng_frqcy { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.slry_rg {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "slry_rg is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "slry_rg exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.src_of_wlth {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "src_of_wlth is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "src_of_wlth exceeds the maximum length of 140".to_string()));
				}
			}
			if let Some(ref val) = self.cstmr_cndct_clssfctn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rsk_lvl { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.know_your_cstmr_chck_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.know_your_cstmr_db_chck { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub nb: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HldrNm") )]
		pub hldr_nm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
		pub start_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt") )]
		pub xpry_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CardIssrNm", skip_serializing_if = "Option::is_none") )]
		pub card_issr_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CardIssrId", skip_serializing_if = "Option::is_none") )]
		pub card_issr_id: Option<PartyIdentification125Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SctyCd", skip_serializing_if = "Option::is_none") )]
		pub scty_cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SeqNb", skip_serializing_if = "Option::is_none") )]
		pub seq_nb: Option<String>,
	}
	
	impl PaymentCard29 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if self.nb.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nb is shorter than the minimum length of 1".to_string()));
			}
			if self.nb.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nb exceeds the maximum length of 35".to_string()));
			}
			if self.hldr_nm.chars().count() < 1 {
				return Err(ValidationError::new(1001, "hldr_nm is shorter than the minimum length of 1".to_string()));
			}
			if self.hldr_nm.chars().count() > 35 {
				return Err(ValidationError::new(1002, "hldr_nm exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.card_issr_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "card_issr_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "card_issr_nm exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.card_issr_id { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.scty_cd {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "scty_cd is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "scty_cd exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.seq_nb {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "seq_nb is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 3 {
					return Err(ValidationError::new(1002, "seq_nb exceeds the maximum length of 3".to_string()));
				}
			}
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
		pub sttlm_ccy: String,
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
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.sttlm_ccy) {
				return Err(ValidationError::new(1005, "sttlm_ccy does not match the required pattern".to_string()));
			}
			if let Some(ref val) = self.dvdd_pctg {
				if *val < 0.000000 {
					return Err(ValidationError::new(1003, "dvdd_pctg is less than the minimum value of 0.000000".to_string()));
				}
				if *val > 100.000000 {
					return Err(ValidationError::new(1004, "dvdd_pctg exceeds the maximum value of 100.000000".to_string()));
				}
			}
			if let Some(ref val) = self.sbcpt_pmt_instrm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.red_pmt_instrm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dvdd_pmt_instrm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.svgs_plan_pmt_instrm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.intrst_pmt_instrm { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.chq_dtls { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.bkrs_drft_dtls { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.pmt_card_dtls { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.drct_dbt_dtls { if let Err(e) = val.validate() { return Err(e); } }
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
		pub addtl_inf: Option<String>,
	}
	
	impl PendingOpeningStatusReason1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref val) = self.addtl_inf {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
				}
			}
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
			if let Some(ref val) = self.no_spcfd_rsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub addtl_inf: Option<String>,
	}
	
	impl PendingStatusReason14 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref val) = self.addtl_inf {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
				}
			}
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
			if let Some(ref val) = self.no_spcfd_rsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub nm_of_fthr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MdnNmOfMthr", skip_serializing_if = "Option::is_none") )]
		pub mdn_nm_of_mthr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmOfPrtnr", skip_serializing_if = "Option::is_none") )]
		pub nm_of_prtnr: Option<String>,
	}
	
	impl PersonalInformation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.nm_of_fthr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nm_of_fthr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "nm_of_fthr exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.mdn_nm_of_mthr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "mdn_nm_of_mthr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "mdn_nm_of_mthr exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.nm_of_prtnr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nm_of_prtnr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "nm_of_prtnr exceeds the maximum length of 35".to_string()));
				}
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.pltcly_xpsd_prsn_sts { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub adr_line: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
		pub strt_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
		pub bldg_nb: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
		pub pst_cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
		pub twn_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
		pub ctry_sub_dvsn: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
		pub ctry: String,
	}
	
	impl PostalAddress1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.adr_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.adr_line {
				for item in vec {
					if item.chars().count() < 1 {
						return Err(ValidationError::new(1001, "adr_line is shorter than the minimum length of 1".to_string()));
					}
					if item.chars().count() > 70 {
						return Err(ValidationError::new(1002, "adr_line exceeds the maximum length of 70".to_string()));
					}
				}
			}
			if let Some(ref val) = self.strt_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "strt_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 70 {
					return Err(ValidationError::new(1002, "strt_nm exceeds the maximum length of 70".to_string()));
				}
			}
			if let Some(ref val) = self.bldg_nb {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "bldg_nb is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 16 {
					return Err(ValidationError::new(1002, "bldg_nb exceeds the maximum length of 16".to_string()));
				}
			}
			if let Some(ref val) = self.pst_cd {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "pst_cd is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 16 {
					return Err(ValidationError::new(1002, "pst_cd exceeds the maximum length of 16".to_string()));
				}
			}
			if let Some(ref val) = self.twn_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "twn_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "twn_nm exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.ctry_sub_dvsn {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "ctry_sub_dvsn is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "ctry_sub_dvsn exceeds the maximum length of 35".to_string()));
				}
			}
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&self.ctry) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
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
		pub care_of: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
		pub adr_line: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
		pub strt_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
		pub bldg_nb: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNm", skip_serializing_if = "Option::is_none") )]
		pub bldg_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstBx", skip_serializing_if = "Option::is_none") )]
		pub pst_bx: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SdInBldg", skip_serializing_if = "Option::is_none") )]
		pub sd_in_bldg: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Flr", skip_serializing_if = "Option::is_none") )]
		pub flr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SuiteId", skip_serializing_if = "Option::is_none") )]
		pub suite_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
		pub pst_cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none") )]
		pub dstrct_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Vllg", skip_serializing_if = "Option::is_none") )]
		pub vllg: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
		pub twn_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Stat", skip_serializing_if = "Option::is_none") )]
		pub stat: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
		pub ctry: String,
	}
	
	impl PostalAddress21 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.adr_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.care_of {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "care_of is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 70 {
					return Err(ValidationError::new(1002, "care_of exceeds the maximum length of 70".to_string()));
				}
			}
			if let Some(ref vec) = self.adr_line {
				for item in vec {
					if item.chars().count() < 1 {
						return Err(ValidationError::new(1001, "adr_line is shorter than the minimum length of 1".to_string()));
					}
					if item.chars().count() > 70 {
						return Err(ValidationError::new(1002, "adr_line exceeds the maximum length of 70".to_string()));
					}
				}
			}
			if let Some(ref val) = self.strt_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "strt_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 70 {
					return Err(ValidationError::new(1002, "strt_nm exceeds the maximum length of 70".to_string()));
				}
			}
			if let Some(ref val) = self.bldg_nb {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "bldg_nb is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 16 {
					return Err(ValidationError::new(1002, "bldg_nb exceeds the maximum length of 16".to_string()));
				}
			}
			if let Some(ref val) = self.bldg_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "bldg_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "bldg_nm exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.pst_bx {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "pst_bx is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 10 {
					return Err(ValidationError::new(1002, "pst_bx exceeds the maximum length of 10".to_string()));
				}
			}
			if let Some(ref val) = self.sd_in_bldg {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "sd_in_bldg is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "sd_in_bldg exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.flr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "flr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 70 {
					return Err(ValidationError::new(1002, "flr exceeds the maximum length of 70".to_string()));
				}
			}
			if let Some(ref val) = self.suite_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "suite_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 10 {
					return Err(ValidationError::new(1002, "suite_id exceeds the maximum length of 10".to_string()));
				}
			}
			if let Some(ref val) = self.pst_cd {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "pst_cd is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 16 {
					return Err(ValidationError::new(1002, "pst_cd exceeds the maximum length of 16".to_string()));
				}
			}
			if let Some(ref val) = self.dstrct_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "dstrct_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "dstrct_nm exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.vllg {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "vllg is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 70 {
					return Err(ValidationError::new(1002, "vllg exceeds the maximum length of 70".to_string()));
				}
			}
			if let Some(ref val) = self.twn_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "twn_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "twn_nm exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.stat {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "stat is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 70 {
					return Err(ValidationError::new(1002, "stat exceeds the maximum length of 70".to_string()));
				}
			}
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&self.ctry) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub addtl_inf: Option<String>,
	}
	
	impl ProformaStatusReason1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref val) = self.addtl_inf {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
				}
			}
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
			if let Some(ref val) = self.no_spcfd_rsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.rfrd_plcmnt_agt { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.indv_prsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.org { if let Err(e) = val.validate() { return Err(e); } }
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
		pub sctr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Brnch", skip_serializing_if = "Option::is_none") )]
		pub brnch: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Grp", skip_serializing_if = "Option::is_none") )]
		pub grp: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<String>,
	}
	
	impl RegulatoryInformation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.sctr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "sctr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "sctr exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.brnch {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "brnch is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "brnch exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.grp {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "grp is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "grp exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.othr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "othr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "othr exceeds the maximum length of 35".to_string()));
				}
			}
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
		pub reqd_nav_ccy: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RinvstmtPctg") )]
		pub rinvstmt_pctg: f64,
	}
	
	impl Reinvestment4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.fin_instrm_dtls.validate() { return Err(e); }
			if let Some(ref val) = self.reqd_nav_ccy {
				let pattern = Regex::new("[A-Z]{3,3}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "reqd_nav_ccy does not match the required pattern".to_string()));
				}
			}
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
		pub ccy_of_plan: Option<String>,
	}
	
	impl Repartition6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.qty.validate() { return Err(e); }
			if let Err(e) = self.fin_instrm.validate() { return Err(e); }
			if let Some(ref val) = self.ccy_of_plan {
				let pattern = Regex::new("[A-Z]{3,3}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "ccy_of_plan does not match the required pattern".to_string()));
				}
			}
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// SecurityIdentification25Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecurityIdentification25Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
		pub isin: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SEDOL", skip_serializing_if = "Option::is_none") )]
		pub sedol: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CUSIP", skip_serializing_if = "Option::is_none") )]
		pub cusip: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RIC", skip_serializing_if = "Option::is_none") )]
		pub ric: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none") )]
		pub tckr_symb: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none") )]
		pub blmbrg: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CTA", skip_serializing_if = "Option::is_none") )]
		pub cta: Option<String>,
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
		pub cmon: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPrtryId", skip_serializing_if = "Option::is_none") )]
		pub othr_prtry_id: Option<AlternateSecurityIdentification7>,
	}
	
	impl SecurityIdentification25Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.isin {
				let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.ric {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "ric is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "ric exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.tckr_symb {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tckr_symb is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "tckr_symb exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.blmbrg {
				let pattern = Regex::new("(BBG)[BCDFGHJKLMNPQRSTVWXYZ\\d]{8}\\d").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "blmbrg does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.cta {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "cta is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "cta exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.cmon {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "cmon is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 12 {
					return Err(ValidationError::new(1002, "cmon exceeds the maximum length of 12".to_string()));
				}
			}
			if let Some(ref val) = self.othr_prtry_id { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// SimpleIdentificationInformation4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SimpleIdentificationInformation4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
	}
	
	impl SimpleIdentificationInformation4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
		pub taxtn_ctry: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRate", skip_serializing_if = "Option::is_none") )]
		pub tax_rate: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxPyer", skip_serializing_if = "Option::is_none") )]
		pub tax_pyer: Option<PartyIdentification125Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRcpt", skip_serializing_if = "Option::is_none") )]
		pub tax_rcpt: Option<PartyIdentification125Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctDtls", skip_serializing_if = "Option::is_none") )]
		pub csh_acct_dtls: Option<CashAccount204>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<String>,
	}
	
	impl TaxReporting3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&self.taxtn_ctry) {
				return Err(ValidationError::new(1005, "taxtn_ctry does not match the required pattern".to_string()));
			}
			if let Some(ref val) = self.tax_pyer { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.tax_rcpt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.csh_acct_dtls { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.desc {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "desc exceeds the maximum length of 350".to_string()));
				}
			}
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
		pub tp: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtTm", skip_serializing_if = "Option::is_none") )]
		pub dt_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Hldr", skip_serializing_if = "Option::is_none") )]
		pub hldr: Option<PartyIdentification125Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none") )]
		pub lgl_ntty_idr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<ActiveCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<String>,
	}
	
	impl ThirdPartyRights2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.tp.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
			}
			if self.tp.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.hldr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.lgl_ntty_idr {
				let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "lgl_ntty_idr does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.desc {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "desc exceeds the maximum length of 350".to_string()));
				}
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.amt { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.amt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
}