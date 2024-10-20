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

pub mod iso20022 {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	
	
	// ISODateTime ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max20000Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max20000Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max20000_text: String,
	}
	
	impl Max20000Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max20000_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max20000_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max20000_text.chars().count() > 20000 {
				return Err(ValidationError::new(1002, "max20000_text exceeds the maximum length of 20000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max350Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// MessageReference ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MessageReference {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
		pub ref_attr: Max35Text,
	}
	
	impl MessageReference {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ref_attr.validate() { return Err(e); }
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
		pub rjctg_pty_rsn: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RjctnDtTm", skip_serializing_if = "Option::is_none") )]
		pub rjctn_dt_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ErrLctn", skip_serializing_if = "Option::is_none") )]
		pub err_lctn: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RsnDesc", skip_serializing_if = "Option::is_none") )]
		pub rsn_desc: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlData", skip_serializing_if = "Option::is_none") )]
		pub addtl_data: Option<Max20000Text>,
	}
	
	impl RejectionReason2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rjctg_pty_rsn.validate() { return Err(e); }
			if let Some(ref err_lctn_value) = self.err_lctn { if let Err(e) = err_lctn_value.validate() { return Err(e); } }
			if let Some(ref rsn_desc_value) = self.rsn_desc { if let Err(e) = rsn_desc_value.validate() { return Err(e); } }
			if let Some(ref addtl_data_value) = self.addtl_data { if let Err(e) = addtl_data_value.validate() { return Err(e); } }
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
			if let Err(e) = self.rltd_ref.validate() { return Err(e); }
			if let Err(e) = self.rsn.validate() { return Err(e); }
			Ok(())
		}
	}
	
}