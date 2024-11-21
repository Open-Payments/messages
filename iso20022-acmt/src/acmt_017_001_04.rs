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

// AccountMandateMaintenanceRequestV04 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AccountMandateMaintenanceRequestV04 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Refs") )]
	pub refs: References4,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Fr", skip_serializing_if = "Option::is_none") )]
	pub fr: Option<OrganisationIdentification39>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctDts", skip_serializing_if = "Option::is_none") )]
	pub ctrct_dts: Option<AccountContract2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygMstrAgrmt", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_mstr_agrmt: Option<ContractDocument1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctId") )]
	pub acct_id: Vec<AccountForAction1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrId") )]
	pub acct_svcr_id: BranchAndFinancialInstitutionIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgId") )]
	pub org_id: Organisation44,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mndt") )]
	pub mndt: Vec<OperationMandate6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Grp", skip_serializing_if = "Option::is_none") )]
	pub grp: Option<Vec<Group5>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlMsgInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_msg_inf: Option<AdditionalInformation5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DgtlSgntr", skip_serializing_if = "Option::is_none") )]
	pub dgtl_sgntr: Option<Vec<PartyAndSignature4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl AccountMandateMaintenanceRequestV04 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.refs.validate()?;
		if let Some(ref val) = self.fr { val.validate()? }
		if let Some(ref val) = self.ctrct_dts { val.validate()? }
		if let Some(ref val) = self.undrlyg_mstr_agrmt { val.validate()? }
		for item in &self.acct_id { item.validate()? }
		self.acct_svcr_id.validate()?;
		self.org_id.validate()?;
		for item in &self.mndt { item.validate()? }
		if let Some(ref vec) = self.grp { for item in vec { item.validate()? } }
		if let Some(ref val) = self.addtl_msg_inf { val.validate()? }
		if let Some(ref vec) = self.dgtl_sgntr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}
