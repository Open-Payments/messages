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

// PriceReportCancellationV04 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PriceReportCancellationV04 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PoolRef", skip_serializing_if = "Option::is_none") )]
	pub pool_ref: Option<AdditionalReference3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none") )]
	pub prvs_ref: Option<AdditionalReference3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgPgntn") )]
	pub msg_pgntn: Pagination,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricRptId") )]
	pub pric_rpt_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlId") )]
	pub cxl_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlRsn", skip_serializing_if = "Option::is_none") )]
	pub cxl_rsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XpctdPricCrrctnDt", skip_serializing_if = "Option::is_none") )]
	pub xpctd_pric_crrctn_dt: Option<DateAndDateTime1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CmpltPricCxl") )]
	pub cmplt_pric_cxl: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CancPricValtnDtls", skip_serializing_if = "Option::is_none") )]
	pub canc_pric_valtn_dtls: Option<Vec<PriceReport3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Xtnsn", skip_serializing_if = "Option::is_none") )]
	pub xtnsn: Option<Vec<Extension1>>,
}

impl PriceReportCancellationV04 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.msg_id.validate()?;
		if let Some(ref val) = self.pool_ref { val.validate()? }
		if let Some(ref val) = self.prvs_ref { val.validate()? }
		self.msg_pgntn.validate()?;
		if self.pric_rpt_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "pric_rpt_id is shorter than the minimum length of 1".to_string()));
		}
		if self.pric_rpt_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "pric_rpt_id exceeds the maximum length of 35".to_string()));
		}
		if self.cxl_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "cxl_id is shorter than the minimum length of 1".to_string()));
		}
		if self.cxl_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "cxl_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.cxl_rsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cxl_rsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "cxl_rsn exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.xpctd_pric_crrctn_dt { val.validate()? }
		if let Some(ref vec) = self.canc_pric_valtn_dtls { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.xtnsn { for item in vec { item.validate()? } }
		Ok(())
	}
}
