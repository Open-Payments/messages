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


use regex::Regex;
use crate::common::*;
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};


// MessageReference ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MessageReference {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
	pub ref_attr: String,
}

impl MessageReference {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.ref_attr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "ref_attr is shorter than the minimum length of 1".to_string()));
		}
		if self.ref_attr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "ref_attr exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// RejectionReason2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RejectionReason2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RjctgPtyRsn") )]
	pub rjctg_pty_rsn: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RjctnDtTm", skip_serializing_if = "Option::is_none") )]
	pub rjctn_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ErrLctn", skip_serializing_if = "Option::is_none") )]
	pub err_lctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RsnDesc", skip_serializing_if = "Option::is_none") )]
	pub rsn_desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlData", skip_serializing_if = "Option::is_none") )]
	pub addtl_data: Option<String>,
}

impl RejectionReason2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.rjctg_pty_rsn.chars().count() < 1 {
			return Err(ValidationError::new(1001, "rjctg_pty_rsn is shorter than the minimum length of 1".to_string()));
		}
		if self.rjctg_pty_rsn.chars().count() > 35 {
			return Err(ValidationError::new(1002, "rjctg_pty_rsn exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.err_lctn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "err_lctn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "err_lctn exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.rsn_desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rsn_desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "rsn_desc exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.addtl_data {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_data is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 20000 {
				return Err(ValidationError::new(1002, "addtl_data exceeds the maximum length of 20000".to_string()));
			}
		}
		Ok(())
	}
}


// Admi00200101 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Admi00200101 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdRef") )]
	pub rltd_ref: MessageReference,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: RejectionReason2,
}

impl Admi00200101 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rltd_ref.validate()?;
		self.rsn.validate()?;
		Ok(())
	}
}
