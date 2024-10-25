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


// AddressType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum AddressType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ADDR") )]
	CodeADDR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PBOX") )]
	CodePBOX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HOME") )]
	CodeHOME,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BIZZ") )]
	CodeBIZZ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MLTO") )]
	CodeMLTO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DLVY") )]
	CodeDLVY,
}

impl AddressType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AddressType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AddressType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AddressType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl AddressType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// CodeOrProprietary1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CodeOrProprietary1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification13>,
}

impl CodeOrProprietary1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// Contact14 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Contact14 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none") )]
	pub nm_prfx: Option<NamePrefix2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PhneNb", skip_serializing_if = "Option::is_none") )]
	pub phne_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MobNb", skip_serializing_if = "Option::is_none") )]
	pub mob_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaxNb", skip_serializing_if = "Option::is_none") )]
	pub fax_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "URLAdr", skip_serializing_if = "Option::is_none") )]
	pub url_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none") )]
	pub email_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmailPurp", skip_serializing_if = "Option::is_none") )]
	pub email_purp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "JobTitl", skip_serializing_if = "Option::is_none") )]
	pub job_titl: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rspnsblty", skip_serializing_if = "Option::is_none") )]
	pub rspnsblty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dept", skip_serializing_if = "Option::is_none") )]
	pub dept: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<OtherContact1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrefrdMtd", skip_serializing_if = "Option::is_none") )]
	pub prefrd_mtd: Option<PreferredContactMethod2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldFr", skip_serializing_if = "Option::is_none") )]
	pub vld_fr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldTo", skip_serializing_if = "Option::is_none") )]
	pub vld_to: Option<String>,
}

impl Contact14 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm_prfx { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.phne_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "phne_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.mob_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "mob_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.fax_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "fax_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.url_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "url_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "url_adr exceeds the maximum length of 2048".to_string()));
			}
		}
		if let Some(ref val) = self.email_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "email_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "email_adr exceeds the maximum length of 256".to_string()));
			}
		}
		if let Some(ref val) = self.email_purp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "email_purp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "email_purp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.job_titl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "job_titl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "job_titl exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.rspnsblty {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rspnsblty is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rspnsblty exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.dept {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dept is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "dept exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref vec) = self.othr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.prefrd_mtd { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// DatePeriod2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DatePeriod2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt") )]
	pub fr_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt") )]
	pub to_dt: String,
}

impl DatePeriod2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DatePeriod3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DatePeriod3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt", skip_serializing_if = "Option::is_none") )]
	pub fr_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt", skip_serializing_if = "Option::is_none") )]
	pub to_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDt", skip_serializing_if = "Option::is_none") )]
	pub fr_to_dt: Option<DatePeriod2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
}

impl DatePeriod3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fr_to_dt { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// ErrorHandling3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ErrorHandling3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ErrorHandling3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ErrorHandling5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ErrorHandling5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Err") )]
	pub err: ErrorHandling3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
}

impl ErrorHandling5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.err.validate() { return Err(e); }
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// GenericIdentification13 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification13 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
	pub issr: String,
}

impl GenericIdentification13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 4 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 4".to_string()));
		}
		let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if self.issr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
		}
		if self.issr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// GenericIdentification30 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification30 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
	pub issr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
}

impl GenericIdentification30 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		if self.issr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
		}
		if self.issr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GenericIdentification36 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification36 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
	pub issr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
}

impl GenericIdentification36 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if self.issr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
		}
		if self.issr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// LockStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum LockStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "LOCK") )]
	CodeLOCK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ULCK") )]
	CodeULCK,
}

impl LockStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// MarketSpecificAttribute1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MarketSpecificAttribute1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: String,
}

impl MarketSpecificAttribute1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 35 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 35".to_string()));
		}
		if self.val.chars().count() < 1 {
			return Err(ValidationError::new(1001, "val is shorter than the minimum length of 1".to_string()));
		}
		if self.val.chars().count() > 350 {
			return Err(ValidationError::new(1002, "val exceeds the maximum length of 350".to_string()));
		}
		Ok(())
	}
}


// MessageHeader12 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MessageHeader12 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBizInstr", skip_serializing_if = "Option::is_none") )]
	pub orgnl_biz_instr: Option<OriginalBusinessInstruction1>,
}

impl MessageHeader12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.orgnl_biz_instr { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// NameAndAddress5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NameAndAddress5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adr", skip_serializing_if = "Option::is_none") )]
	pub adr: Option<PostalAddress1>,
}

impl NameAndAddress5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
		}
		if let Some(ref val) = self.adr { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// NamePrefix2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum NamePrefix2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DOCT") )]
	CodeDOCT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MADM") )]
	CodeMADM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MISS") )]
	CodeMISS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIST") )]
	CodeMIST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIKS") )]
	CodeMIKS,
}

impl NamePrefix2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OriginalBusinessInstruction1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OriginalBusinessInstruction1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none") )]
	pub msg_nm_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
}

impl OriginalBusinessInstruction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.msg_nm_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_nm_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_nm_id exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// OtherContact1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OtherContact1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChanlTp") )]
	pub chanl_tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
}

impl OtherContact1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.chanl_tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "chanl_tp is shorter than the minimum length of 1".to_string()));
		}
		if self.chanl_tp.chars().count() > 4 {
			return Err(ValidationError::new(1002, "chanl_tp exceeds the maximum length of 4".to_string()));
		}
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 128 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 128".to_string()));
			}
		}
		Ok(())
	}
}


// PartyAuditTrail2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyAuditTrail2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rcrd") )]
	pub rcrd: Vec<UpdateLogPartyRecord2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OprTmStmp") )]
	pub opr_tm_stmp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgUsr") )]
	pub instg_usr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ApprvgUsr", skip_serializing_if = "Option::is_none") )]
	pub apprvg_usr: Option<String>,
}

impl PartyAuditTrail2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.rcrd { if let Err(e) = item.validate() { return Err(e); } }
		if self.instg_usr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "instg_usr is shorter than the minimum length of 1".to_string()));
		}
		if self.instg_usr.chars().count() > 256 {
			return Err(ValidationError::new(1002, "instg_usr exceeds the maximum length of 256".to_string()));
		}
		if let Some(ref val) = self.apprvg_usr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "apprvg_usr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "apprvg_usr exceeds the maximum length of 256".to_string()));
			}
		}
		Ok(())
	}
}


// PartyAuditTrailOrError3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyAuditTrailOrError3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyAudtTrlRpt", skip_serializing_if = "Option::is_none") )]
	pub pty_audt_trl_rpt: Option<Vec<PartyAuditTrailReport4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OprlErr", skip_serializing_if = "Option::is_none") )]
	pub oprl_err: Option<Vec<ErrorHandling5>>,
}

impl PartyAuditTrailOrError3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.pty_audt_trl_rpt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref vec) = self.oprl_err { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// PartyAuditTrailOrError4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyAuditTrailOrError4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AudtTrl", skip_serializing_if = "Option::is_none") )]
	pub audt_trl: Option<Vec<PartyAuditTrail2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizErr", skip_serializing_if = "Option::is_none") )]
	pub biz_err: Option<Vec<ErrorHandling5>>,
}

impl PartyAuditTrailOrError4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.audt_trl { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref vec) = self.biz_err { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// PartyAuditTrailReport4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyAuditTrailReport4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyAudtTrlOrErr") )]
	pub pty_audt_trl_or_err: PartyAuditTrailOrError4Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtPrd", skip_serializing_if = "Option::is_none") )]
	pub dt_prd: Option<DatePeriod3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId") )]
	pub pty_id: SystemPartyIdentification8,
}

impl PartyAuditTrailReport4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pty_audt_trl_or_err.validate() { return Err(e); }
		if let Some(ref val) = self.dt_prd { if let Err(e) = val.validate() { return Err(e); } }
		if let Err(e) = self.pty_id.validate() { return Err(e); }
		Ok(())
	}
}


// PartyAuditTrailReportV02 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyAuditTrailReportV02 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgHdr", skip_serializing_if = "Option::is_none") )]
	pub msg_hdr: Option<MessageHeader12>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptOrErr") )]
	pub rpt_or_err: PartyAuditTrailOrError3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl PartyAuditTrailReportV02 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.msg_hdr { if let Err(e) = val.validate() { return Err(e); } }
		if let Err(e) = self.rpt_or_err.validate() { return Err(e); }
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// PartyIdentification120Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification120Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
	pub prtry_id: Option<GenericIdentification36>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
	pub nm_and_adr: Option<NameAndAddress5>,
}

impl PartyIdentification120Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry_id { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.nm_and_adr { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyIdentification136 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification136 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: PartyIdentification120Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
}

impl PartyIdentification136 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PartyLockStatus1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyLockStatus1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldFr", skip_serializing_if = "Option::is_none") )]
	pub vld_fr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
	pub sts: LockStatus1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LckRsn", skip_serializing_if = "Option::is_none") )]
	pub lck_rsn: Option<Vec<String>>,
}

impl PartyLockStatus1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.sts.validate() { return Err(e); }
		if let Some(ref vec) = self.lck_rsn {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "lck_rsn is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "lck_rsn exceeds the maximum length of 35".to_string()));
				}
			}
		}
		Ok(())
	}
}


// PartyName4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyName4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldFr", skip_serializing_if = "Option::is_none") )]
	pub vld_fr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
	pub shrt_nm: Option<String>,
}

impl PartyName4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
		}
		if let Some(ref val) = self.shrt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "shrt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "shrt_nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// PostalAddress1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PostalAddress1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
	pub adr_tp: Option<AddressType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
	pub adr_line: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
	pub strt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
	pub bldg_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
	pub pst_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
	pub twn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
	pub ctry_sub_dvsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
	pub ctry: String,
}

impl PostalAddress1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.adr_tp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.adr_line {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "adr_line is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 70 {
					return Err(ValidationError::new(1002, "adr_line exceeds the maximum length of 70".to_string()));
				}
			}
		}
		if let Some(ref val) = self.strt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "strt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "strt_nm exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.bldg_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bldg_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "bldg_nb exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.pst_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pst_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "pst_cd exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.twn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "twn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "twn_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctry_sub_dvsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctry_sub_dvsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctry_sub_dvsn exceeds the maximum length of 35".to_string()));
			}
		}
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry) {
			return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// PostalAddress28 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PostalAddress28 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
	pub adr_tp: Option<AddressType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CareOf", skip_serializing_if = "Option::is_none") )]
	pub care_of: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dept", skip_serializing_if = "Option::is_none") )]
	pub dept: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubDept", skip_serializing_if = "Option::is_none") )]
	pub sub_dept: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
	pub strt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
	pub bldg_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNm", skip_serializing_if = "Option::is_none") )]
	pub bldg_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Flr", skip_serializing_if = "Option::is_none") )]
	pub flr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitNb", skip_serializing_if = "Option::is_none") )]
	pub unit_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstBx", skip_serializing_if = "Option::is_none") )]
	pub pst_bx: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Room", skip_serializing_if = "Option::is_none") )]
	pub room: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
	pub pst_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
	pub twn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none") )]
	pub twn_lctn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none") )]
	pub dstrct_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
	pub ctry_sub_dvsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
	pub adr_line: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldFr", skip_serializing_if = "Option::is_none") )]
	pub vld_fr: Option<String>,
}

impl PostalAddress28 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.adr_tp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.care_of {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "care_of is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "care_of exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.dept {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dept is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "dept exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.sub_dept {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sub_dept is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "sub_dept exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.strt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "strt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "strt_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.bldg_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bldg_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "bldg_nb exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.bldg_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bldg_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "bldg_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.flr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "flr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "flr exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.unit_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "unit_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "unit_nb exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.pst_bx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pst_bx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "pst_bx exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.room {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "room is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "room exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.pst_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pst_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "pst_cd exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.twn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "twn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "twn_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.twn_lctn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "twn_lctn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "twn_lctn_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.dstrct_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dstrct_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "dstrct_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.ctry_sub_dvsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctry_sub_dvsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctry_sub_dvsn exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.adr_line {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "adr_line is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 70 {
					return Err(ValidationError::new(1002, "adr_line exceeds the maximum length of 70".to_string()));
				}
			}
		}
		Ok(())
	}
}


// PreferredContactMethod2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PreferredContactMethod2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MAIL") )]
	CodeMAIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FAXX") )]
	CodeFAXX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LETT") )]
	CodeLETT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CELL") )]
	CodeCELL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ONLI") )]
	CodeONLI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PHON") )]
	CodePHON,
}

impl PreferredContactMethod2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ResidenceType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ResidenceType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DMST") )]
	CodeDMST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FRGN") )]
	CodeFRGN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MXED") )]
	CodeMXED,
}

impl ResidenceType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Restriction1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Restriction1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RstrctnTp") )]
	pub rstrctn_tp: CodeOrProprietary1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldFr") )]
	pub vld_fr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldUntil", skip_serializing_if = "Option::is_none") )]
	pub vld_until: Option<String>,
}

impl Restriction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.rstrctn_tp.validate() { return Err(e); }
		Ok(())
	}
}


// SupplementaryData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SupplementaryData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none") )]
	pub plc_and_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Envlp") )]
	pub envlp: SupplementaryDataEnvelope1,
}

impl SupplementaryData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.plc_and_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "plc_and_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "plc_and_nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Err(e) = self.envlp.validate() { return Err(e); }
		Ok(())
	}
}


// SupplementaryDataEnvelope1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SystemPartyIdentification8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SystemPartyIdentification8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: PartyIdentification136,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RspnsblPtyId", skip_serializing_if = "Option::is_none") )]
	pub rspnsbl_pty_id: Option<PartyIdentification136>,
}

impl SystemPartyIdentification8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref val) = self.rspnsbl_pty_id { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// SystemPartyType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SystemPartyType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl SystemPartyType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// TechnicalIdentification2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TechnicalIdentification2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BICFI", skip_serializing_if = "Option::is_none") )]
	pub bicfi: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechAdr", skip_serializing_if = "Option::is_none") )]
	pub tech_adr: Option<String>,
}

impl TechnicalIdentification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.bicfi {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "bicfi does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tech_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tech_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "tech_adr exceeds the maximum length of 256".to_string()));
			}
		}
		Ok(())
	}
}


// UpdateLogAddress2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UpdateLogAddress2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: PostalAddress28,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: PostalAddress28,
}

impl UpdateLogAddress2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.od.validate() { return Err(e); }
		if let Err(e) = self.new.validate() { return Err(e); }
		Ok(())
	}
}


// UpdateLogContact2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UpdateLogContact2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: Contact14,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: Contact14,
}

impl UpdateLogContact2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.od.validate() { return Err(e); }
		if let Err(e) = self.new.validate() { return Err(e); }
		Ok(())
	}
}


// UpdateLogDate1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UpdateLogDate1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: String,
}

impl UpdateLogDate1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UpdateLogMarketSpecificAttribute1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UpdateLogMarketSpecificAttribute1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: MarketSpecificAttribute1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: MarketSpecificAttribute1,
}

impl UpdateLogMarketSpecificAttribute1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.od.validate() { return Err(e); }
		if let Err(e) = self.new.validate() { return Err(e); }
		Ok(())
	}
}


// UpdateLogPartyLockStatus1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UpdateLogPartyLockStatus1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: PartyLockStatus1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: PartyLockStatus1,
}

impl UpdateLogPartyLockStatus1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.od.validate() { return Err(e); }
		if let Err(e) = self.new.validate() { return Err(e); }
		Ok(())
	}
}


// UpdateLogPartyName1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UpdateLogPartyName1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: PartyName4,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: PartyName4,
}

impl UpdateLogPartyName1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.od.validate() { return Err(e); }
		if let Err(e) = self.new.validate() { return Err(e); }
		Ok(())
	}
}


// UpdateLogPartyRecord2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UpdateLogPartyRecord2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adr", skip_serializing_if = "Option::is_none") )]
	pub adr: Option<UpdateLogAddress2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none") )]
	pub ctct_dtls: Option<UpdateLogContact2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OpngDt", skip_serializing_if = "Option::is_none") )]
	pub opng_dt: Option<UpdateLogDate1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
	pub clsg_dt: Option<UpdateLogDate1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<UpdateLogSystemPartyType1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechAdr", skip_serializing_if = "Option::is_none") )]
	pub tech_adr: Option<UpdateLogTechnicalAddress1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktSpcfcAttr", skip_serializing_if = "Option::is_none") )]
	pub mkt_spcfc_attr: Option<UpdateLogMarketSpecificAttribute1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<UpdateLogPartyName1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ResTp", skip_serializing_if = "Option::is_none") )]
	pub res_tp: Option<UpdateLogResidenceType1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LckSts", skip_serializing_if = "Option::is_none") )]
	pub lck_sts: Option<UpdateLogPartyLockStatus1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none") )]
	pub rstrctn: Option<UpdateLogRestriction1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<UpdateLogProprietary1>>,
}

impl UpdateLogPartyRecord2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.adr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ctct_dtls { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.opng_dt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.clsg_dt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tech_adr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.mkt_spcfc_attr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.nm { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.res_tp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.lck_sts { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.rstrctn { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.othr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// UpdateLogProprietary1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UpdateLogProprietary1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FldNm") )]
	pub fld_nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OdFldVal") )]
	pub od_fld_val: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NewFldVal") )]
	pub new_fld_val: String,
}

impl UpdateLogProprietary1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.fld_nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "fld_nm is shorter than the minimum length of 1".to_string()));
		}
		if self.fld_nm.chars().count() > 35 {
			return Err(ValidationError::new(1002, "fld_nm exceeds the maximum length of 35".to_string()));
		}
		if self.od_fld_val.chars().count() < 1 {
			return Err(ValidationError::new(1001, "od_fld_val is shorter than the minimum length of 1".to_string()));
		}
		if self.od_fld_val.chars().count() > 350 {
			return Err(ValidationError::new(1002, "od_fld_val exceeds the maximum length of 350".to_string()));
		}
		if self.new_fld_val.chars().count() < 1 {
			return Err(ValidationError::new(1001, "new_fld_val is shorter than the minimum length of 1".to_string()));
		}
		if self.new_fld_val.chars().count() > 350 {
			return Err(ValidationError::new(1002, "new_fld_val exceeds the maximum length of 350".to_string()));
		}
		Ok(())
	}
}


// UpdateLogResidenceType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UpdateLogResidenceType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: ResidenceType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: ResidenceType1Code,
}

impl UpdateLogResidenceType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.od.validate() { return Err(e); }
		if let Err(e) = self.new.validate() { return Err(e); }
		Ok(())
	}
}


// UpdateLogRestriction1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UpdateLogRestriction1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: Restriction1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: Restriction1,
}

impl UpdateLogRestriction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.od.validate() { return Err(e); }
		if let Err(e) = self.new.validate() { return Err(e); }
		Ok(())
	}
}


// UpdateLogSystemPartyType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UpdateLogSystemPartyType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: SystemPartyType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: SystemPartyType1Choice,
}

impl UpdateLogSystemPartyType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.od.validate() { return Err(e); }
		if let Err(e) = self.new.validate() { return Err(e); }
		Ok(())
	}
}


// UpdateLogTechnicalAddress1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UpdateLogTechnicalAddress1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: TechnicalIdentification2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: TechnicalIdentification2Choice,
}

impl UpdateLogTechnicalAddress1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.od.validate() { return Err(e); }
		if let Err(e) = self.new.validate() { return Err(e); }
		Ok(())
	}
}
