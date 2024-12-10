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

// FundDetailedConfirmedCashForecastReportCancellationV03 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FundDetailedConfirmedCashForecastReportCancellationV03 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PoolRef", skip_serializing_if = "Option::is_none") )]
	pub pool_ref: Option<AdditionalReference3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none") )]
	pub prvs_ref: Option<AdditionalReference3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdRef", skip_serializing_if = "Option::is_none") )]
	pub rltd_ref: Option<Vec<AdditionalReference3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgPgntn") )]
	pub msg_pgntn: Pagination,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshFcstRptToBeCanc", skip_serializing_if = "Option::is_none") )]
	pub csh_fcst_rpt_to_be_canc: Option<FundDetailedConfirmedCashForecastReport3>,
}

impl FundDetailedConfirmedCashForecastReportCancellationV03 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.msg_id.validate()?;
		if let Some(ref val) = self.pool_ref { val.validate()? }
		if let Some(ref val) = self.prvs_ref { val.validate()? }
		if let Some(ref vec) = self.rltd_ref { for item in vec { item.validate()? } }
		self.msg_pgntn.validate()?;
		if let Some(ref val) = self.csh_fcst_rpt_to_be_canc { val.validate()? }
		Ok(())
	}
}
