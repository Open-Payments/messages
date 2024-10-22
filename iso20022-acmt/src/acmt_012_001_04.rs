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
	
	
	// AccountAdditionalInformationRequestV04 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountAdditionalInformationRequestV04 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Refs") )]
		pub refs: References3,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Fr", skip_serializing_if = "Option::is_none") )]
		pub fr: Option<OrganisationIdentification39>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgId") )]
		pub org_id: OrganisationIdentification39,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrId") )]
		pub acct_svcr_id: BranchAndFinancialInstitutionIdentification8,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctId") )]
		pub acct_id: Vec<AccountForAction1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DgtlSgntr", skip_serializing_if = "Option::is_none") )]
		pub dgtl_sgntr: Option<Vec<PartyAndSignature4>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl AccountAdditionalInformationRequestV04 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.refs.validate() { return Err(e); }
			if let Some(ref val) = self.fr { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.org_id.validate() { return Err(e); }
			if let Err(e) = self.acct_svcr_id.validate() { return Err(e); }
			for item in &self.acct_id { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref vec) = self.dgtl_sgntr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// AccountForAction1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountForAction1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: AccountIdentification4Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
		pub ccy: String,
	}
	
	impl AccountForAction1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.ccy) {
				return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// AccountIdentification4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountIdentification4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "IBAN", skip_serializing_if = "Option::is_none") )]
		pub iban: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<GenericAccountIdentification1>,
	}
	
	impl AccountIdentification4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.iban {
				let pattern = Regex::new("[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "iban does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.othr { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AccountSchemeName1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountSchemeName1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl AccountSchemeName1Choice {
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
	
	
	// BranchAndFinancialInstitutionIdentification8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BranchAndFinancialInstitutionIdentification8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstnId") )]
		pub fin_instn_id: FinancialInstitutionIdentification23,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BrnchId", skip_serializing_if = "Option::is_none") )]
		pub brnch_id: Option<BranchData5>,
	}
	
	impl BranchAndFinancialInstitutionIdentification8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.fin_instn_id.validate() { return Err(e); }
			if let Some(ref val) = self.brnch_id { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BranchData5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BranchData5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress27>,
	}
	
	impl BranchData5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.lei {
				let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
				}
			}
			if let Some(ref val) = self.pstl_adr { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ClearingSystemIdentification2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ClearingSystemIdentification2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl ClearingSystemIdentification2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 5 {
					return Err(ValidationError::new(1002, "cd exceeds the maximum length of 5".to_string()));
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
	
	
	// ClearingSystemMemberIdentification2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ClearingSystemMemberIdentification2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none") )]
		pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MmbId") )]
		pub mmb_id: String,
	}
	
	impl ClearingSystemMemberIdentification2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.clr_sys_id { if let Err(e) = val.validate() { return Err(e); } }
			if self.mmb_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mmb_id is shorter than the minimum length of 1".to_string()));
			}
			if self.mmb_id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mmb_id exceeds the maximum length of 35".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Contact13 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Contact13 {
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
	}
	
	impl Contact13 {
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
	
	
	// DateAndPlaceOfBirth1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DateAndPlaceOfBirth1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BirthDt") )]
		pub birth_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none") )]
		pub prvc_of_birth: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CityOfBirth") )]
		pub city_of_birth: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfBirth") )]
		pub ctry_of_birth: String,
	}
	
	impl DateAndPlaceOfBirth1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.prvc_of_birth {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "prvc_of_birth is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "prvc_of_birth exceeds the maximum length of 35".to_string()));
				}
			}
			if self.city_of_birth.chars().count() < 1 {
				return Err(ValidationError::new(1001, "city_of_birth is shorter than the minimum length of 1".to_string()));
			}
			if self.city_of_birth.chars().count() > 35 {
				return Err(ValidationError::new(1002, "city_of_birth exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&self.ctry_of_birth) {
				return Err(ValidationError::new(1005, "ctry_of_birth does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// FinancialIdentificationSchemeName1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FinancialIdentificationSchemeName1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl FinancialIdentificationSchemeName1Choice {
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
	
	
	// FinancialInstitutionIdentification23 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FinancialInstitutionIdentification23 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BICFI", skip_serializing_if = "Option::is_none") )]
		pub bicfi: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none") )]
		pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress27>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<GenericFinancialIdentification1>,
	}
	
	impl FinancialInstitutionIdentification23 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.bicfi {
				let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "bicfi does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.clr_sys_mmb_id { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.lei {
				let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
				}
			}
			if let Some(ref val) = self.pstl_adr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.othr { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericAccountIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericAccountIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<AccountSchemeName1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<String>,
	}
	
	impl GenericAccountIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 34 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 34".to_string()));
			}
			if let Some(ref val) = self.schme_nm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.issr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// GenericFinancialIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericFinancialIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<String>,
	}
	
	impl GenericFinancialIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.schme_nm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.issr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
				}
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
	
	
	// GenericOrganisationIdentification3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericOrganisationIdentification3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<String>,
	}
	
	impl GenericOrganisationIdentification3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 256 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 256".to_string()));
			}
			if let Some(ref val) = self.schme_nm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.issr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// GenericPersonIdentification2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericPersonIdentification2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<String>,
	}
	
	impl GenericPersonIdentification2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 256 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 256".to_string()));
			}
			if let Some(ref val) = self.schme_nm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.issr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// MessageIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MessageIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
		pub cre_dt_tm: String,
	}
	
	impl MessageIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
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
	
	
	// OrganisationIdentification39 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OrganisationIdentification39 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<Vec<GenericOrganisationIdentification3>>,
	}
	
	impl OrganisationIdentification39 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.any_bic {
				let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.lei {
				let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
				}
			}
			if let Some(ref vec) = self.othr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// OrganisationIdentificationSchemeName1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OrganisationIdentificationSchemeName1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl OrganisationIdentificationSchemeName1Choice {
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
	
	
	// Party52Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Party52Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgId", skip_serializing_if = "Option::is_none") )]
		pub org_id: Option<OrganisationIdentification39>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvtId", skip_serializing_if = "Option::is_none") )]
		pub prvt_id: Option<PersonIdentification18>,
	}
	
	impl Party52Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.org_id { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prvt_id { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyAndSignature4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PartyAndSignature4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pty") )]
		pub pty: PartyIdentification272,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sgntr") )]
		pub sgntr: SkipPayload,
	}
	
	impl PartyAndSignature4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pty.validate() { return Err(e); }
			if let Err(e) = self.sgntr.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// PartyIdentification272 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PartyIdentification272 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress27>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Party52Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none") )]
		pub ctry_of_res: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none") )]
		pub ctct_dtls: Option<Contact13>,
	}
	
	impl PartyIdentification272 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
				}
			}
			if let Some(ref val) = self.pstl_adr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.id { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ctry_of_res {
				let pattern = Regex::new("[A-Z]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "ctry_of_res does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.ctct_dtls { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PersonIdentification18 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PersonIdentification18 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none") )]
		pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<Vec<GenericPersonIdentification2>>,
	}
	
	impl PersonIdentification18 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.dt_and_plc_of_birth { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.othr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// PersonIdentificationSchemeName1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PersonIdentificationSchemeName1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl PersonIdentificationSchemeName1Choice {
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
	
	
	// PostalAddress27 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PostalAddress27 {
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
	}
	
	impl PostalAddress27 {
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
	
	
	// References3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct References3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
		pub msg_id: MessageIdentification1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ReqToBeCmpltdId") )]
		pub req_to_be_cmpltd_id: MessageIdentification1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrcId") )]
		pub prc_id: MessageIdentification1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ReqRsn") )]
		pub req_rsn: Vec<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AttchdDocNm", skip_serializing_if = "Option::is_none") )]
		pub attchd_doc_nm: Option<Vec<String>>,
	}
	
	impl References3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.msg_id.validate() { return Err(e); }
			if let Err(e) = self.req_to_be_cmpltd_id.validate() { return Err(e); }
			if let Err(e) = self.prc_id.validate() { return Err(e); }
			for item in &self.req_rsn {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "req_rsn is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "req_rsn exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref vec) = self.attchd_doc_nm {
				for item in vec {
					if item.chars().count() < 1 {
						return Err(ValidationError::new(1001, "attchd_doc_nm is shorter than the minimum length of 1".to_string()));
					}
					if item.chars().count() > 70 {
						return Err(ValidationError::new(1002, "attchd_doc_nm exceeds the maximum length of 70".to_string()));
					}
				}
			}
			Ok(())
		}
	}
	
	
	// SkipPayload ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SkipPayload {
	}
	
	impl SkipPayload {
		pub fn validate(&self) -> Result<(), ValidationError> {
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