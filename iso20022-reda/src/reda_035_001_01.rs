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
	
	
	// MessageHeader1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MessageHeader1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
		pub msg_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
		pub cre_dt_tm: Option<String>,
	}
	
	impl MessageHeader1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.msg_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
			}
			if self.msg_id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Pagination1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Pagination1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PgNb") )]
		pub pg_nb: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LastPgInd") )]
		pub last_pg_ind: bool,
	}
	
	impl Pagination1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{1,5}").unwrap();
			if !pattern.is_match(&self.pg_nb) {
				return Err(ValidationError::new(1005, "pg_nb does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// SecuritiesAccount19 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecuritiesAccount19 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<GenericIdentification30>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<String>,
	}
	
	impl SecuritiesAccount19 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 70 {
					return Err(ValidationError::new(1002, "nm exceeds the maximum length of 70".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// SecuritiesAccountActivityAdviceV01 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecuritiesAccountActivityAdviceV01 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgHdr", skip_serializing_if = "Option::is_none") )]
		pub msg_hdr: Option<MessageHeader1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pgntn") )]
		pub pgntn: Pagination1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcctActvty") )]
		pub scties_acct_actvty: SecuritiesAccountStatement2,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl SecuritiesAccountActivityAdviceV01 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.msg_hdr { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.pgntn.validate() { return Err(e); }
			if let Err(e) = self.scties_acct_actvty.validate() { return Err(e); }
			if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// SecuritiesAccountReferenceDataChange2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecuritiesAccountReferenceDataChange2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcctId") )]
		pub scties_acct_id: SecuritiesAccount19,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FldNm") )]
		pub fld_nm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OdFldVal") )]
		pub od_fld_val: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NewFldVal") )]
		pub new_fld_val: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OprTmStmp") )]
		pub opr_tm_stmp: String,
	}
	
	impl SecuritiesAccountReferenceDataChange2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.scties_acct_id.validate() { return Err(e); }
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
	
	
	// SecuritiesAccountStatement2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecuritiesAccountStatement2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SysDt") )]
		pub sys_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Chng", skip_serializing_if = "Option::is_none") )]
		pub chng: Option<Vec<SecuritiesAccountReferenceDataChange2>>,
	}
	
	impl SecuritiesAccountStatement2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref vec) = self.chng { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	
}