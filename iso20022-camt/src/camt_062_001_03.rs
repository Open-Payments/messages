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
use open_payments_common::{common::*, ValidationError};
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};

// PayInScheduleV03 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PayInScheduleV03 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId") )]
	pub pty_id: PartyIdentification73Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptData") )]
	pub rpt_data: ReportData4,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PayInSchdlLngBal", skip_serializing_if = "Option::is_none") )]
	pub pay_in_schdl_lng_bal: Option<Vec<BalanceStatus2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PayInSchdlItm", skip_serializing_if = "Option::is_none") )]
	pub pay_in_schdl_itm: Option<Vec<PayInScheduleItems1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PayInFctrs", skip_serializing_if = "Option::is_none") )]
	pub pay_in_fctrs: Option<PayInFactors1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl PayInScheduleV03 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty_id.validate()?;
		self.rpt_data.validate()?;
		if let Some(ref vec) = self.pay_in_schdl_lng_bal { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.pay_in_schdl_itm { for item in vec { item.validate()? } }
		if let Some(ref val) = self.pay_in_fctrs { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}
