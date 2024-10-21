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

#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub mod iso20022 {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	
	
	// ApplicationSpecifics1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ApplicationSpecifics1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SysUsr", skip_serializing_if = "Option::is_none") )]
		pub sys_usr: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sgntr", skip_serializing_if = "Option::is_none") )]
		pub sgntr: Option<SignatureEnvelope>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfDocs") )]
		pub ttl_nb_of_docs: f64,
	}
	
	impl ApplicationSpecifics1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref sys_usr_value) = self.sys_usr { if let Err(e) = sys_usr_value.validate() { return Err(e); } }
			if let Some(ref sgntr_value) = self.sgntr { if let Err(e) = sgntr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BusinessFileHeaderV01 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BusinessFileHeaderV01 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PyldDesc") )]
		pub pyld_desc: PayloadDescription2,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pyld", skip_serializing_if = "Option::is_none") )]
		pub pyld: Option<Vec<LaxPayload>>,
	}
	
	impl BusinessFileHeaderV01 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pyld_desc.validate() { return Err(e); }
			if let Some(ref pyld_vec) = self.pyld { for item in pyld_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ISODateTime ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// LaxPayload ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct LaxPayload {
	}
	
	impl LaxPayload {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ManifestData2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ManifestData2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DocTp") )]
		pub doc_tp: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfDocs") )]
		pub nb_of_docs: f64,
	}
	
	impl ManifestData2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.doc_tp.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// Max140Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max140Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max140_text: String,
	}
	
	impl Max140Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max140_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max140_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max140_text.chars().count() > 140 {
				return Err(ValidationError::new(1002, "max140_text exceeds the maximum length of 140".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max256Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max256Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max256_text: String,
	}
	
	impl Max256Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max256_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max256_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max256_text.chars().count() > 256 {
				return Err(ValidationError::new(1002, "max256_text exceeds the maximum length of 256".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max35Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// Number ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Number {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub number: f64,
	}
	
	impl Number {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PayloadData2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PayloadData2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PyldIdr") )]
		pub pyld_idr: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtAndTm") )]
		pub cre_dt_and_tm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PssblDplctFlg", skip_serializing_if = "Option::is_none") )]
		pub pssbl_dplct_flg: Option<bool>,
	}
	
	impl PayloadData2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pyld_idr.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// PayloadDescription2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PayloadDescription2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PyldData") )]
		pub pyld_data: PayloadData2,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ApplSpcfcs", skip_serializing_if = "Option::is_none") )]
		pub appl_spcfcs: Option<ApplicationSpecifics1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PyldTp") )]
		pub pyld_tp: Max256Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MnfstData", skip_serializing_if = "Option::is_none") )]
		pub mnfst_data: Option<Vec<ManifestData2>>,
	}
	
	impl PayloadDescription2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pyld_data.validate() { return Err(e); }
			if let Some(ref appl_spcfcs_value) = self.appl_spcfcs { if let Err(e) = appl_spcfcs_value.validate() { return Err(e); } }
			if let Err(e) = self.pyld_tp.validate() { return Err(e); }
			if let Some(ref mnfst_data_vec) = self.mnfst_data { for item in mnfst_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// SignatureEnvelope ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SignatureEnvelope {
	}
	
	impl SignatureEnvelope {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TrueFalseIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct TrueFalseIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub true_false_indicator: bool,
	}
	
	impl TrueFalseIndicator {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
}