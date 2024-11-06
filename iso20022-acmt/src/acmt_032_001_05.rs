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
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};

// AccountSwitchBalanceTransferAcknowledgementV05 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AccountSwitchBalanceTransferAcknowledgementV05 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSwtchDtls") )]
	pub acct_swtch_dtls: AccountSwitchDetails1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OdAcct") )]
	pub od_acct: CashAccount43,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OdAcctBal") )]
	pub od_acct_bal: AmountAndDirection5,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BalTrf", skip_serializing_if = "Option::is_none") )]
	pub bal_trf: Option<Vec<BalanceTransfer5>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl AccountSwitchBalanceTransferAcknowledgementV05 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.msg_id.validate()?;
		self.acct_swtch_dtls.validate()?;
		self.od_acct.validate()?;
		self.od_acct_bal.validate()?;
		if let Some(ref vec) = self.bal_trf { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}
