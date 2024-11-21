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


#![allow(unused_imports)]
use regex::Regex;
use crate::common::*;
use open_payments_common::ValidationError;
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};

// AccountModificationInstructionV08 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AccountModificationInstructionV08 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none") )]
	pub prvs_ref: Option<AdditionalReference13>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrDtls", skip_serializing_if = "Option::is_none") )]
	pub instr_dtls: Option<InvestmentAccountModification4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtAcctSelctn") )]
	pub invstmt_acct_selctn: AccountSelection3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfdInvstmtAcct", skip_serializing_if = "Option::is_none") )]
	pub modfd_invstmt_acct: Option<InvestmentAccount75>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfdAcctPties", skip_serializing_if = "Option::is_none") )]
	pub modfd_acct_pties: Option<Vec<AccountParties18>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfdIntrmies", skip_serializing_if = "Option::is_none") )]
	pub modfd_intrmies: Option<Vec<ModificationScope40>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfdPlcmnt", skip_serializing_if = "Option::is_none") )]
	pub modfd_plcmnt: Option<ModificationScope43>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfdIsseAllcn", skip_serializing_if = "Option::is_none") )]
	pub modfd_isse_allcn: Option<ModificationScope21>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfdSvgsInvstmtPlan", skip_serializing_if = "Option::is_none") )]
	pub modfd_svgs_invstmt_plan: Option<Vec<ModificationScope41>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfdWdrwlInvstmtPlan", skip_serializing_if = "Option::is_none") )]
	pub modfd_wdrwl_invstmt_plan: Option<Vec<ModificationScope41>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfdCshSttlm", skip_serializing_if = "Option::is_none") )]
	pub modfd_csh_sttlm: Option<Vec<CashSettlement4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfdSvcLvlAgrmt", skip_serializing_if = "Option::is_none") )]
	pub modfd_svc_lvl_agrmt: Option<Vec<ModificationScope44>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfdAddtlInf", skip_serializing_if = "Option::is_none") )]
	pub modfd_addtl_inf: Option<Vec<ModificationScope45>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktPrctcVrsn", skip_serializing_if = "Option::is_none") )]
	pub mkt_prctc_vrsn: Option<MarketPracticeVersion1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Xtnsn", skip_serializing_if = "Option::is_none") )]
	pub xtnsn: Option<Vec<Extension1>>,
}

impl AccountModificationInstructionV08 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.msg_id.validate()?;
		if let Some(ref val) = self.prvs_ref { val.validate()? }
		if let Some(ref val) = self.instr_dtls { val.validate()? }
		self.invstmt_acct_selctn.validate()?;
		if let Some(ref val) = self.modfd_invstmt_acct { val.validate()? }
		if let Some(ref vec) = self.modfd_acct_pties { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.modfd_intrmies { for item in vec { item.validate()? } }
		if let Some(ref val) = self.modfd_plcmnt { val.validate()? }
		if let Some(ref val) = self.modfd_isse_allcn { val.validate()? }
		if let Some(ref vec) = self.modfd_svgs_invstmt_plan { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.modfd_wdrwl_invstmt_plan { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.modfd_csh_sttlm { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.modfd_svc_lvl_agrmt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.modfd_addtl_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.mkt_prctc_vrsn { val.validate()? }
		if let Some(ref vec) = self.xtnsn { for item in vec { item.validate()? } }
		Ok(())
	}
}
