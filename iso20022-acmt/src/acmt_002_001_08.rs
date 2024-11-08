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

// AccountDetailsConfirmationV08 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
		self.msg_id.validate()?;
		if let Some(ref val) = self.ordr_ref { val.validate()? }
		if let Some(ref val) = self.rltd_ref { val.validate()? }
		self.conf_dtls.validate()?;
		if let Some(ref val) = self.invstmt_acct { val.validate()? }
		if let Some(ref val) = self.acct_pties { val.validate()? }
		if let Some(ref vec) = self.intrmies { for item in vec { item.validate()? } }
		if let Some(ref val) = self.plcmnt { val.validate()? }
		if let Some(ref val) = self.new_isse_allcn { val.validate()? }
		if let Some(ref vec) = self.svgs_invstmt_plan { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.wdrwl_invstmt_plan { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.csh_sttlm { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.svc_lvl_agrmt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.addtl_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.mkt_prctc_vrsn { val.validate()? }
		if let Some(ref vec) = self.xtnsn { for item in vec { item.validate()? } }
		Ok(())
	}
}
