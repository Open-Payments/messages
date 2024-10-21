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

#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub mod iso20022 {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	
	
	// AcceptedStatusReason1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AcceptedStatusReason1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<AcceptedStatusReason1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification36>,
	}
	
	impl AcceptedStatusReason1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AcceptedStatusReason1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AcceptedStatusReason1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PLAC") )]
		CodePLAC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SECT") )]
		CodeSECT,
	}
	
	impl AcceptedStatusReason1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Account23 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// AccountManagementStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AccountManagementStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "RECE") )]
		CodeRECE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACCP") )]
		CodeACCP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EXEC") )]
		CodeEXEC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "STNP") )]
		CodeSTNP,
	}
	
	impl AccountManagementStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AccountManagementStatusAndReason5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AccountManagementStatusAndReason5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
		pub sts: Status25Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StsRsn", skip_serializing_if = "Option::is_none") )]
		pub sts_rsn: Option<Vec<AcceptedStatusReason1Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctApplId", skip_serializing_if = "Option::is_none") )]
		pub acct_appl_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ExstgAcctId", skip_serializing_if = "Option::is_none") )]
		pub exstg_acct_id: Option<Vec<Account23>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctId", skip_serializing_if = "Option::is_none") )]
		pub acct_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSts", skip_serializing_if = "Option::is_none") )]
		pub acct_sts: Option<AccountStatus2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BlckdSts", skip_serializing_if = "Option::is_none") )]
		pub blckd_sts: Option<BlockedStatusReason2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FATCARptgDt", skip_serializing_if = "Option::is_none") )]
		pub fatca_rptg_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CRSRptgDt", skip_serializing_if = "Option::is_none") )]
		pub crs_rptg_dt: Option<String>,
	}
	
	impl AccountManagementStatusAndReason5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.sts.validate() { return Err(e); }
			if let Some(ref sts_rsn_vec) = self.sts_rsn { for item in sts_rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref acct_appl_id_value) = self.acct_appl_id { if let Err(e) = acct_appl_id_value.validate() { return Err(e); } }
			if let Some(ref exstg_acct_id_vec) = self.exstg_acct_id { for item in exstg_acct_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref acct_id_value) = self.acct_id { if let Err(e) = acct_id_value.validate() { return Err(e); } }
			if let Some(ref acct_sts_value) = self.acct_sts { if let Err(e) = acct_sts_value.validate() { return Err(e); } }
			if let Some(ref blckd_sts_value) = self.blckd_sts { if let Err(e) = blckd_sts_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AccountManagementStatusReportV07 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AccountManagementStatusReportV07 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
		pub msg_id: MessageIdentification1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdRef") )]
		pub rltd_ref: Vec<AdditionalReference13>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StsRpt") )]
		pub sts_rpt: AccountManagementStatusAndReason5,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MktPrctcVrsn", skip_serializing_if = "Option::is_none") )]
		pub mkt_prctc_vrsn: Option<MarketPracticeVersion1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Xtnsn", skip_serializing_if = "Option::is_none") )]
		pub xtnsn: Option<Vec<Extension1>>,
	}
	
	impl AccountManagementStatusReportV07 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.msg_id.validate() { return Err(e); }
			for item in &self.rltd_ref { if let Err(e) = item.validate() { return Err(e); } }
			if let Err(e) = self.sts_rpt.validate() { return Err(e); }
			if let Some(ref mkt_prctc_vrsn_value) = self.mkt_prctc_vrsn { if let Err(e) = mkt_prctc_vrsn_value.validate() { return Err(e); } }
			if let Some(ref xtnsn_vec) = self.xtnsn { for item in xtnsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// AccountStatus2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// AdditionalReference13 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// AddressType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// AnyBICDec2014Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// BlockedReason2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// ClosedStatusReason1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// CountryCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// DisabledReason2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// EnabledStatusReason1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// Exact4AlphaNumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// Extension1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// GenericIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// ISODate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// InvestmentFundTransactionType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// MarketPracticeVersion1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// Max16Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// Max350Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// Max4AlphaNumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// NameAndAddress5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// NoReasonCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// OtherAccountStatus1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// PartyIdentification125Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// PendingOpeningStatusReason1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// PostalAddress1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// ProformaStatusReason1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// RejectedReason16Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct RejectedReason16Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<RejectedStatusReason6Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification36>,
	}
	
	impl RejectedReason16Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RejectedStatusReason6Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum RejectedStatusReason6Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "SAFE") )]
		CodeSAFE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NSLA") )]
		CodeNSLA,
	}
	
	impl RejectedStatusReason6Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// RejectionReason31 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct RejectionReason31 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
		pub rsn: RejectedReason16Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_rsn_inf: Option<Max350Text>,
	}
	
	impl RejectionReason31 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rsn.validate() { return Err(e); }
			if let Some(ref addtl_rsn_inf_value) = self.addtl_rsn_inf { if let Err(e) = addtl_rsn_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Status25Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Status25Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
		pub sts: Option<AccountManagementStatus1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rjctd", skip_serializing_if = "Option::is_none") )]
		pub rjctd: Option<Vec<RejectionReason31>>,
	}
	
	impl Status25Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref sts_value) = self.sts { if let Err(e) = sts_value.validate() { return Err(e); } }
			if let Some(ref rjctd_vec) = self.rjctd { for item in rjctd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TransactionType5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// YesNoIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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