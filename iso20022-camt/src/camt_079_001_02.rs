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

// IntraBalanceMovementQueryResponseV02 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct IntraBalanceMovementQueryResponseV02 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<DocumentIdentification51>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pgntn") )]
	pub pgntn: Pagination1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptGnlDtls") )]
	pub rpt_gnl_dtls: MovementReport1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptOrErr", skip_serializing_if = "Option::is_none") )]
	pub rpt_or_err: Option<IntraBalanceOrOperationalError11Choice>,
}

impl IntraBalanceMovementQueryResponseV02 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id { val.validate()? }
		self.pgntn.validate()?;
		self.rpt_gnl_dtls.validate()?;
		if let Some(ref val) = self.rpt_or_err { val.validate()? }
		Ok(())
	}
}
