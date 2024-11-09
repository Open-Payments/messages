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
#[cfg(feature = "derive_samplify")]
use samplify_rs::Sampleable;

// FedNowParticipantFile1: This is the participant profile of the FedNow participant and contains the participant's identification, name and the FedNow services the participant has enrolled for.
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FedNowParticipantFile1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizDay") )]
	pub biz_day: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtcptPrfl") )]
	pub ptcpt_prfl: Vec<FedNowParticipantProfile1>,
}

impl FedNowParticipantFile1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.ptcpt_prfl { item.validate()? }
		Ok(())
	}
}


// FedNowParticipantProfile1: This specifies the FedNow services the FedNow participant has enrolled for.
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FedNowParticipantProfile1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Svcs") )]
	pub svcs: Vec<ServicesFedNow1>,
}

impl FedNowParticipantProfile1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{9,9}").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 140 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
		}
		for item in &self.svcs { item.validate()? }
		Ok(())
	}
}


// Services_FedNow_1: This indicates a FedNow participant is enabled to receive request for payment messages.
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum ServicesFedNow1 {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CTSR") )]
	CodeCTSR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CTRO") )]
	CodeCTRO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RFPR") )]
	CodeRFPR,
}

impl ServicesFedNow1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Admi998SuplDataV01: This is the FedNow participant file and contains the FedNow Service funds-transfer business day and the FedNow participants with their FedNow Service profile.
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Admi998SuplDataV01 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtcptFile") )]
	pub ptcpt_file: FedNowParticipantFile1,
}

impl Admi998SuplDataV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.ptcpt_file.validate()?;
		Ok(())
	}
}
