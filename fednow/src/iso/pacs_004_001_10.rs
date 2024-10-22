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

pub mod fednow {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	
	
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
	
	
	// ActiveCurrencyAndAmount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ActiveCurrencyAndAmount {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
		pub ccy: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub value: f64,
	}
	
	impl ActiveCurrencyAndAmount {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ActiveOrHistoricCurrencyAndAmount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ActiveOrHistoricCurrencyAndAmount {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
		pub ccy: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub value: f64,
	}
	
	impl ActiveOrHistoricCurrencyAndAmount {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// AmendmentInformationDetails13 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AmendmentInformationDetails13 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMndtId", skip_serializing_if = "Option::is_none") )]
		pub orgnl_mndt_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCdtrSchmeId", skip_serializing_if = "Option::is_none") )]
		pub orgnl_cdtr_schme_id: Option<PartyIdentification135>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCdtrAgt", skip_serializing_if = "Option::is_none") )]
		pub orgnl_cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
		pub orgnl_cdtr_agt_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlDbtr", skip_serializing_if = "Option::is_none") )]
		pub orgnl_dbtr: Option<PartyIdentification135>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlDbtrAcct", skip_serializing_if = "Option::is_none") )]
		pub orgnl_dbtr_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlDbtrAgt", skip_serializing_if = "Option::is_none") )]
		pub orgnl_dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlDbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
		pub orgnl_dbtr_agt_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlFnlColltnDt", skip_serializing_if = "Option::is_none") )]
		pub orgnl_fnl_colltn_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlFrqcy", skip_serializing_if = "Option::is_none") )]
		pub orgnl_frqcy: Option<Frequency36Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlRsn", skip_serializing_if = "Option::is_none") )]
		pub orgnl_rsn: Option<MandateSetupReason1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTrckgDays", skip_serializing_if = "Option::is_none") )]
		pub orgnl_trckg_days: Option<String>,
	}
	
	impl AmendmentInformationDetails13 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.orgnl_mndt_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "orgnl_mndt_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "orgnl_mndt_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.orgnl_cdtr_schme_id { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.orgnl_cdtr_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.orgnl_cdtr_agt_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.orgnl_dbtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.orgnl_dbtr_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.orgnl_dbtr_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.orgnl_dbtr_agt_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.orgnl_frqcy { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.orgnl_rsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.orgnl_trckg_days {
				let pattern = Regex::new("[0-9]{2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "orgnl_trckg_days does not match the required pattern".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// AmountType4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AmountType4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none") )]
		pub instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EqvtAmt", skip_serializing_if = "Option::is_none") )]
		pub eqvt_amt: Option<EquivalentAmount2>,
	}
	
	impl AmountType4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.instd_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.eqvt_amt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Authorisation1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Authorisation1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<Authorisation1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl Authorisation1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 128 {
					return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 128".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// Authorisation1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Authorisation1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "AUTH") )]
		CodeAUTH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FDET") )]
		CodeFDET,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FSUM") )]
		CodeFSUM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ILEV") )]
		CodeILEV,
	}
	
	impl Authorisation1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BranchAndFinancialInstitutionIdentification6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BranchAndFinancialInstitutionIdentification6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstnId") )]
		pub fin_instn_id: FinancialInstitutionIdentification18,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BrnchId", skip_serializing_if = "Option::is_none") )]
		pub brnch_id: Option<BranchData3>,
	}
	
	impl BranchAndFinancialInstitutionIdentification6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.fin_instn_id.validate() { return Err(e); }
			if let Some(ref val) = self.brnch_id { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BranchData3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BranchData3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress24>,
	}
	
	impl BranchData3 {
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
	
	
	// CashAccount38 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CashAccount38 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: AccountIdentification4Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<CashAccountType2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
		pub ccy: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prxy", skip_serializing_if = "Option::is_none") )]
		pub prxy: Option<ProxyAccountIdentification1>,
	}
	
	impl CashAccount38 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ccy {
				let pattern = Regex::new("[A-Z]{3,3}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 70 {
					return Err(ValidationError::new(1002, "nm exceeds the maximum length of 70".to_string()));
				}
			}
			if let Some(ref val) = self.prxy { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CashAccountType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CashAccountType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl CashAccountType2Choice {
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
	
	
	// CategoryPurpose1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CategoryPurpose1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl CategoryPurpose1Choice {
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
	
	
	// ChargeBearerType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ChargeBearerType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DEBT") )]
		CodeDEBT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CRED") )]
		CodeCRED,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SHAR") )]
		CodeSHAR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SLEV") )]
		CodeSLEV,
	}
	
	impl ChargeBearerType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Charges7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Charges7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Agt") )]
		pub agt: BranchAndFinancialInstitutionIdentification6,
	}
	
	impl Charges7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Err(e) = self.agt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ClearingChannel2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ClearingChannel2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "RTGS") )]
		CodeRTGS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RTNS") )]
		CodeRTNS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MPNS") )]
		CodeMPNS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BOOK") )]
		CodeBOOK,
	}
	
	impl ClearingChannel2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// ClearingSystemIdentification3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ClearingSystemIdentification3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl ClearingSystemIdentification3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 3 {
					return Err(ValidationError::new(1002, "cd exceeds the maximum length of 3".to_string()));
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
	
	
	// Contact4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Contact4 {
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
		pub prefrd_mtd: Option<PreferredContactMethod1Code>,
	}
	
	impl Contact4 {
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
			if let Some(ref val) = self.email_adr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "email_adr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 2048 {
					return Err(ValidationError::new(1002, "email_adr exceeds the maximum length of 2048".to_string()));
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
	
	
	// CreditDebitCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CreditDebitCode {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CRDT") )]
		CodeCRDT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DBIT") )]
		CodeDBIT,
	}
	
	impl CreditDebitCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CreditTransferMandateData1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CreditTransferMandateData1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MndtId", skip_serializing_if = "Option::is_none") )]
		pub mndt_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<MandateTypeInformation2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtOfSgntr", skip_serializing_if = "Option::is_none") )]
		pub dt_of_sgntr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtOfVrfctn", skip_serializing_if = "Option::is_none") )]
		pub dt_of_vrfctn: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ElctrncSgntr", skip_serializing_if = "Option::is_none") )]
		pub elctrnc_sgntr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrstPmtDt", skip_serializing_if = "Option::is_none") )]
		pub frst_pmt_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FnlPmtDt", skip_serializing_if = "Option::is_none") )]
		pub fnl_pmt_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy", skip_serializing_if = "Option::is_none") )]
		pub frqcy: Option<Frequency36Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<MandateSetupReason1Choice>,
	}
	
	impl CreditTransferMandateData1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.mndt_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "mndt_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "mndt_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.elctrnc_sgntr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "elctrnc_sgntr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 10240 {
					return Err(ValidationError::new(1002, "elctrnc_sgntr exceeds the maximum length of 10240".to_string()));
				}
			}
			if let Some(ref val) = self.frqcy { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rsn { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CreditTransferTransaction45 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CreditTransferTransaction45 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
		pub ultmt_dbtr: Option<PartyIdentification135>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty", skip_serializing_if = "Option::is_none") )]
		pub initg_pty: Option<PartyIdentification135>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr") )]
		pub dbtr: PartyIdentification135,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
		pub dbtr_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt") )]
		pub dbtr_agt: BranchAndFinancialInstitutionIdentification6,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
		pub dbtr_agt_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt1", skip_serializing_if = "Option::is_none") )]
		pub prvs_instg_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt1Acct", skip_serializing_if = "Option::is_none") )]
		pub prvs_instg_agt1_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt2", skip_serializing_if = "Option::is_none") )]
		pub prvs_instg_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt2Acct", skip_serializing_if = "Option::is_none") )]
		pub prvs_instg_agt2_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt3", skip_serializing_if = "Option::is_none") )]
		pub prvs_instg_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt3Acct", skip_serializing_if = "Option::is_none") )]
		pub prvs_instg_agt3_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none") )]
		pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none") )]
		pub intrmy_agt1_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none") )]
		pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none") )]
		pub intrmy_agt2_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none") )]
		pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3Acct", skip_serializing_if = "Option::is_none") )]
		pub intrmy_agt3_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt") )]
		pub cdtr_agt: BranchAndFinancialInstitutionIdentification6,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
		pub cdtr_agt_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
		pub cdtr: PartyIdentification135,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
		pub cdtr_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
		pub ultmt_cdtr: Option<PartyIdentification135>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Option::is_none") )]
		pub instr_for_cdtr_agt: Option<Vec<InstructionForCreditorAgent3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForNxtAgt", skip_serializing_if = "Option::is_none") )]
		pub instr_for_nxt_agt: Option<Vec<InstructionForNextAgent1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
		pub tax: Option<TaxInformation8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
		pub rmt_inf: Option<RemittanceInformation16>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none") )]
		pub instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	}
	
	impl CreditTransferTransaction45 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.ultmt_dbtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.initg_pty { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.dbtr.validate() { return Err(e); }
			if let Some(ref val) = self.dbtr_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.dbtr_agt.validate() { return Err(e); }
			if let Some(ref val) = self.dbtr_agt_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prvs_instg_agt1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prvs_instg_agt1_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prvs_instg_agt2 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prvs_instg_agt2_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prvs_instg_agt3 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prvs_instg_agt3_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.intrmy_agt1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.intrmy_agt1_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.intrmy_agt2 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.intrmy_agt2_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.intrmy_agt3 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.intrmy_agt3_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.cdtr_agt.validate() { return Err(e); }
			if let Some(ref val) = self.cdtr_agt_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.cdtr.validate() { return Err(e); }
			if let Some(ref val) = self.cdtr_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ultmt_cdtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.instr_for_cdtr_agt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.instr_for_nxt_agt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.tax { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rmt_inf { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.instd_amt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CreditorReferenceInformation2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CreditorReferenceInformation2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<CreditorReferenceType2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ref", skip_serializing_if = "Option::is_none") )]
		pub ref_attr: Option<String>,
	}
	
	impl CreditorReferenceInformation2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ref_attr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "ref_attr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "ref_attr exceeds the maximum length of 35".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// CreditorReferenceType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CreditorReferenceType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<DocumentType3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl CreditorReferenceType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// CreditorReferenceType2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CreditorReferenceType2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
		pub cd_or_prtry: CreditorReferenceType1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<String>,
	}
	
	impl CreditorReferenceType2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd_or_prtry.validate() { return Err(e); }
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
	
	
	// DateAndDateTime2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DateAndDateTime2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
		pub dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtTm", skip_serializing_if = "Option::is_none") )]
		pub dt_tm: Option<String>,
	}
	
	impl DateAndDateTime2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// DiscountAmountAndType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DiscountAmountAndType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<DiscountAmountType1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
	}
	
	impl DiscountAmountAndType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// DiscountAmountType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DiscountAmountType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl DiscountAmountType1Choice {
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
	
	
	// DocumentAdjustment1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentAdjustment1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
		pub cdt_dbt_ind: Option<CreditDebitCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<String>,
	}
	
	impl DocumentAdjustment1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Some(ref val) = self.cdt_dbt_ind { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rsn {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "rsn is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 4 {
					return Err(ValidationError::new(1002, "rsn exceeds the maximum length of 4".to_string()));
				}
			}
			if let Some(ref val) = self.addtl_inf {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 140".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// DocumentLineIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentLineIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<DocumentLineType1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nb", skip_serializing_if = "Option::is_none") )]
		pub nb: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdDt", skip_serializing_if = "Option::is_none") )]
		pub rltd_dt: Option<String>,
	}
	
	impl DocumentLineIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.nb {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nb is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "nb exceeds the maximum length of 35".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// DocumentLineInformation1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentLineInformation1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Vec<DocumentLineIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<RemittanceAmount3>,
	}
	
	impl DocumentLineInformation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.id { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref val) = self.desc {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 2048 {
					return Err(ValidationError::new(1002, "desc exceeds the maximum length of 2048".to_string()));
				}
			}
			if let Some(ref val) = self.amt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DocumentLineType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentLineType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
		pub cd_or_prtry: DocumentLineType1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<String>,
	}
	
	impl DocumentLineType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd_or_prtry.validate() { return Err(e); }
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
	
	
	// DocumentLineType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentLineType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl DocumentLineType1Choice {
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
	
	
	// DocumentType3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum DocumentType3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "RADM") )]
		CodeRADM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RPIN") )]
		CodeRPIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FXDR") )]
		CodeFXDR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DISP") )]
		CodeDISP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PUOR") )]
		CodePUOR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SCOR") )]
		CodeSCOR,
	}
	
	impl DocumentType3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// DocumentType6Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum DocumentType6Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "MSIN") )]
		CodeMSIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CNFA") )]
		CodeCNFA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DNFA") )]
		CodeDNFA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CINV") )]
		CodeCINV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CREN") )]
		CodeCREN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DEBN") )]
		CodeDEBN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HIRI") )]
		CodeHIRI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SBIN") )]
		CodeSBIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CMCN") )]
		CodeCMCN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SOAC") )]
		CodeSOAC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DISP") )]
		CodeDISP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BOLD") )]
		CodeBOLD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VCHR") )]
		CodeVCHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AROI") )]
		CodeAROI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TSUT") )]
		CodeTSUT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PUOR") )]
		CodePUOR,
	}
	
	impl DocumentType6Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// EquivalentAmount2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EquivalentAmount2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CcyOfTrf") )]
		pub ccy_of_trf: String,
	}
	
	impl EquivalentAmount2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.ccy_of_trf) {
				return Err(ValidationError::new(1005, "ccy_of_trf does not match the required pattern".to_string()));
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
	
	
	// FinancialInstitutionIdentification18 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FinancialInstitutionIdentification18 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BICFI", skip_serializing_if = "Option::is_none") )]
		pub bicfi: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none") )]
		pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress24>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<GenericFinancialIdentification1>,
	}
	
	impl FinancialInstitutionIdentification18 {
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
	
	
	// Frequency36Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Frequency36Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<Frequency6Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
		pub prd: Option<FrequencyPeriod1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PtInTm", skip_serializing_if = "Option::is_none") )]
		pub pt_in_tm: Option<FrequencyAndMoment1>,
	}
	
	impl Frequency36Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.pt_in_tm { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Frequency6Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Frequency6Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
		CodeYEAR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
		CodeMNTH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QURT") )]
		CodeQURT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MIAN") )]
		CodeMIAN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
		CodeWEEK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
		CodeDAIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADHO") )]
		CodeADHO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INDA") )]
		CodeINDA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FRTN") )]
		CodeFRTN,
	}
	
	impl Frequency6Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// FrequencyAndMoment1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FrequencyAndMoment1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: Frequency6Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PtInTm") )]
		pub pt_in_tm: String,
	}
	
	impl FrequencyAndMoment1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			let pattern = Regex::new("[0-9]{2}").unwrap();
			if !pattern.is_match(&self.pt_in_tm) {
				return Err(ValidationError::new(1005, "pt_in_tm does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// FrequencyPeriod1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FrequencyPeriod1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: Frequency6Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CntPerPrd") )]
		pub cnt_per_prd: f64,
	}
	
	impl FrequencyPeriod1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// Garnishment3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Garnishment3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: GarnishmentType1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Grnshee", skip_serializing_if = "Option::is_none") )]
		pub grnshee: Option<PartyIdentification135>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GrnshmtAdmstr", skip_serializing_if = "Option::is_none") )]
		pub grnshmt_admstr: Option<PartyIdentification135>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefNb", skip_serializing_if = "Option::is_none") )]
		pub ref_nb: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
		pub dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none") )]
		pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FmlyMdclInsrncInd", skip_serializing_if = "Option::is_none") )]
		pub fmly_mdcl_insrnc_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MplyeeTermntnInd", skip_serializing_if = "Option::is_none") )]
		pub mplyee_termntn_ind: Option<bool>,
	}
	
	impl Garnishment3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Some(ref val) = self.grnshee { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.grnshmt_admstr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ref_nb {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "ref_nb is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "ref_nb exceeds the maximum length of 140".to_string()));
				}
			}
			if let Some(ref val) = self.rmtd_amt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GarnishmentType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GarnishmentType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
		pub cd_or_prtry: GarnishmentType1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<String>,
	}
	
	impl GarnishmentType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd_or_prtry.validate() { return Err(e); }
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
	
	
	// GarnishmentType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GarnishmentType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl GarnishmentType1Choice {
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
	
	
	// GenericOrganisationIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericOrganisationIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<String>,
	}
	
	impl GenericOrganisationIdentification1 {
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
	
	
	// GenericPersonIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericPersonIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<String>,
	}
	
	impl GenericPersonIdentification1 {
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
	
	
	// GroupHeader90 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GroupHeader90 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
		pub msg_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
		pub cre_dt_tm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Authstn", skip_serializing_if = "Option::is_none") )]
		pub authstn: Option<Vec<Authorisation1Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BtchBookg", skip_serializing_if = "Option::is_none") )]
		pub btch_bookg: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxs") )]
		pub nb_of_txs: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none") )]
		pub ctrl_sum: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GrpRtr", skip_serializing_if = "Option::is_none") )]
		pub grp_rtr: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlRtrdIntrBkSttlmAmt", skip_serializing_if = "Option::is_none") )]
		pub ttl_rtrd_intr_bk_sttlm_amt: Option<ActiveCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
		pub intr_bk_sttlm_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmInf") )]
		pub sttlm_inf: SettlementInstruction7,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none") )]
		pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none") )]
		pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	}
	
	impl GroupHeader90 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.msg_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
			}
			if self.msg_id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref vec) = self.authstn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(&self.nb_of_txs) {
				return Err(ValidationError::new(1005, "nb_of_txs does not match the required pattern".to_string()));
			}
			if let Some(ref val) = self.ttl_rtrd_intr_bk_sttlm_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.sttlm_inf.validate() { return Err(e); }
			if let Some(ref val) = self.instg_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.instd_agt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Instruction4Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Instruction4Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PHOA") )]
		CodePHOA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TELA") )]
		CodeTELA,
	}
	
	impl Instruction4Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// InstructionForCreditorAgent3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InstructionForCreditorAgent3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstrInf", skip_serializing_if = "Option::is_none") )]
		pub instr_inf: Option<String>,
	}
	
	impl InstructionForCreditorAgent3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 4 {
					return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
				}
			}
			if let Some(ref val) = self.instr_inf {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "instr_inf is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "instr_inf exceeds the maximum length of 140".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// InstructionForNextAgent1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InstructionForNextAgent1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<Instruction4Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstrInf", skip_serializing_if = "Option::is_none") )]
		pub instr_inf: Option<String>,
	}
	
	impl InstructionForNextAgent1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.instr_inf {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "instr_inf is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "instr_inf exceeds the maximum length of 140".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// LocalInstrument2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct LocalInstrument2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl LocalInstrument2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "cd exceeds the maximum length of 35".to_string()));
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
	
	
	// MandateClassification1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MandateClassification1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<MandateClassification1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl MandateClassification1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// MandateClassification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum MandateClassification1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "FIXE") )]
		CodeFIXE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "USGB") )]
		CodeUSGB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VARI") )]
		CodeVARI,
	}
	
	impl MandateClassification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// MandateRelatedData1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MandateRelatedData1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DrctDbtMndt", skip_serializing_if = "Option::is_none") )]
		pub drct_dbt_mndt: Option<MandateRelatedInformation14>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtTrfMndt", skip_serializing_if = "Option::is_none") )]
		pub cdt_trf_mndt: Option<CreditTransferMandateData1>,
	}
	
	impl MandateRelatedData1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.drct_dbt_mndt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.cdt_trf_mndt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// MandateRelatedInformation14 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MandateRelatedInformation14 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MndtId", skip_serializing_if = "Option::is_none") )]
		pub mndt_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtOfSgntr", skip_serializing_if = "Option::is_none") )]
		pub dt_of_sgntr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AmdmntInd", skip_serializing_if = "Option::is_none") )]
		pub amdmnt_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AmdmntInfDtls", skip_serializing_if = "Option::is_none") )]
		pub amdmnt_inf_dtls: Option<AmendmentInformationDetails13>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ElctrncSgntr", skip_serializing_if = "Option::is_none") )]
		pub elctrnc_sgntr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrstColltnDt", skip_serializing_if = "Option::is_none") )]
		pub frst_colltn_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FnlColltnDt", skip_serializing_if = "Option::is_none") )]
		pub fnl_colltn_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy", skip_serializing_if = "Option::is_none") )]
		pub frqcy: Option<Frequency36Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<MandateSetupReason1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TrckgDays", skip_serializing_if = "Option::is_none") )]
		pub trckg_days: Option<String>,
	}
	
	impl MandateRelatedInformation14 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.mndt_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "mndt_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "mndt_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.amdmnt_inf_dtls { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.elctrnc_sgntr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "elctrnc_sgntr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 1025 {
					return Err(ValidationError::new(1002, "elctrnc_sgntr exceeds the maximum length of 1025".to_string()));
				}
			}
			if let Some(ref val) = self.frqcy { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.trckg_days {
				let pattern = Regex::new("[0-9]{2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "trckg_days does not match the required pattern".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// MandateSetupReason1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MandateSetupReason1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl MandateSetupReason1Choice {
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
				if val.chars().count() > 70 {
					return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 70".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// MandateTypeInformation2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MandateTypeInformation2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none") )]
		pub svc_lvl: Option<ServiceLevel8Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none") )]
		pub lcl_instrm: Option<LocalInstrument2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none") )]
		pub ctgy_purp: Option<CategoryPurpose1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Clssfctn", skip_serializing_if = "Option::is_none") )]
		pub clssfctn: Option<MandateClassification1Choice>,
	}
	
	impl MandateTypeInformation2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.svc_lvl { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.lcl_instrm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ctgy_purp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.clssfctn { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// OrganisationIdentification29 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OrganisationIdentification29 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<Vec<GenericOrganisationIdentification1>>,
	}
	
	impl OrganisationIdentification29 {
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
	
	
	// OriginalGroupHeader18 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OriginalGroupHeader18 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgId") )]
		pub orgnl_msg_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgNmId") )]
		pub orgnl_msg_nm_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none") )]
		pub orgnl_cre_dt_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RtrRsnInf", skip_serializing_if = "Option::is_none") )]
		pub rtr_rsn_inf: Option<Vec<PaymentReturnReason6>>,
	}
	
	impl OriginalGroupHeader18 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.orgnl_msg_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_msg_id is shorter than the minimum length of 1".to_string()));
			}
			if self.orgnl_msg_id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_msg_id exceeds the maximum length of 35".to_string()));
			}
			if self.orgnl_msg_nm_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_msg_nm_id is shorter than the minimum length of 1".to_string()));
			}
			if self.orgnl_msg_nm_id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_msg_nm_id exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref vec) = self.rtr_rsn_inf { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// OriginalGroupInformation29 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OriginalGroupInformation29 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgId") )]
		pub orgnl_msg_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgNmId") )]
		pub orgnl_msg_nm_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none") )]
		pub orgnl_cre_dt_tm: Option<String>,
	}
	
	impl OriginalGroupInformation29 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.orgnl_msg_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_msg_id is shorter than the minimum length of 1".to_string()));
			}
			if self.orgnl_msg_id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_msg_id exceeds the maximum length of 35".to_string()));
			}
			if self.orgnl_msg_nm_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_msg_nm_id is shorter than the minimum length of 1".to_string()));
			}
			if self.orgnl_msg_nm_id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_msg_nm_id exceeds the maximum length of 35".to_string()));
			}
			Ok(())
		}
	}
	
	
	// OriginalTransactionReference32 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OriginalTransactionReference32 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmAmt", skip_serializing_if = "Option::is_none") )]
		pub intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<AmountType4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
		pub intr_bk_sttlm_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdColltnDt", skip_serializing_if = "Option::is_none") )]
		pub reqd_colltn_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none") )]
		pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none") )]
		pub cdtr_schme_id: Option<PartyIdentification135>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmInf", skip_serializing_if = "Option::is_none") )]
		pub sttlm_inf: Option<SettlementInstruction7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
		pub pmt_tp_inf: Option<PaymentTypeInformation27>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none") )]
		pub pmt_mtd: Option<PaymentMethod4Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MndtRltdInf", skip_serializing_if = "Option::is_none") )]
		pub mndt_rltd_inf: Option<MandateRelatedData1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
		pub rmt_inf: Option<RemittanceInformation16>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
		pub ultmt_dbtr: Option<Party40Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
		pub dbtr: Option<Party40Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
		pub dbtr_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
		pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
		pub dbtr_agt_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
		pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
		pub cdtr_agt_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
		pub cdtr: Option<Party40Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
		pub cdtr_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
		pub ultmt_cdtr: Option<Party40Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
		pub purp: Option<Purpose2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygCstmrCdtTrf", skip_serializing_if = "Option::is_none") )]
		pub undrlyg_cstmr_cdt_trf: Option<CreditTransferTransaction45>,
	}
	
	impl OriginalTransactionReference32 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.intr_bk_sttlm_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.reqd_exctn_dt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.cdtr_schme_id { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.sttlm_inf { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.pmt_tp_inf { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.pmt_mtd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.mndt_rltd_inf { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rmt_inf { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ultmt_dbtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dbtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dbtr_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dbtr_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dbtr_agt_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.cdtr_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.cdtr_agt_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.cdtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.cdtr_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ultmt_cdtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.purp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.undrlyg_cstmr_cdt_trf { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// Party38Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Party38Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgId", skip_serializing_if = "Option::is_none") )]
		pub org_id: Option<OrganisationIdentification29>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvtId", skip_serializing_if = "Option::is_none") )]
		pub prvt_id: Option<PersonIdentification13>,
	}
	
	impl Party38Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.org_id { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prvt_id { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Party40Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Party40Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pty", skip_serializing_if = "Option::is_none") )]
		pub pty: Option<PartyIdentification135>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Agt", skip_serializing_if = "Option::is_none") )]
		pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
	}
	
	impl Party40Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.pty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.agt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyIdentification135 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PartyIdentification135 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress24>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Party38Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none") )]
		pub ctry_of_res: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none") )]
		pub ctct_dtls: Option<Contact4>,
	}
	
	impl PartyIdentification135 {
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
	
	
	// PaymentMethod4Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PaymentMethod4Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CHK") )]
		CodeCHK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRF") )]
		CodeTRF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DD") )]
		CodeDD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRA") )]
		CodeTRA,
	}
	
	impl PaymentMethod4Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PaymentReturnReason6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentReturnReason6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
		pub orgtr: Option<PartyIdentification135>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<ReturnReason5Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Vec<String>>,
	}
	
	impl PaymentReturnReason6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.orgtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.addtl_inf {
				for item in vec {
					if item.chars().count() < 1 {
						return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
					}
					if item.chars().count() > 105 {
						return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 105".to_string()));
					}
				}
			}
			Ok(())
		}
	}
	
	
	// PaymentReturnV10 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentReturnV10 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "GrpHdr") )]
		pub grp_hdr: GroupHeader90,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none") )]
		pub orgnl_grp_inf: Option<OriginalGroupHeader18>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxInf", skip_serializing_if = "Option::is_none") )]
		pub tx_inf: Option<Vec<PaymentTransaction118>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl PaymentReturnV10 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.grp_hdr.validate() { return Err(e); }
			if let Some(ref val) = self.orgnl_grp_inf { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.tx_inf { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// PaymentTransaction118 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentTransaction118 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RtrId", skip_serializing_if = "Option::is_none") )]
		pub rtr_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none") )]
		pub orgnl_grp_inf: Option<OriginalGroupInformation29>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none") )]
		pub orgnl_instr_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none") )]
		pub orgnl_end_to_end_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxId", skip_serializing_if = "Option::is_none") )]
		pub orgnl_tx_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none") )]
		pub orgnl_uetr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlClrSysRef", skip_serializing_if = "Option::is_none") )]
		pub orgnl_clr_sys_ref: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlIntrBkSttlmAmt", skip_serializing_if = "Option::is_none") )]
		pub orgnl_intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlIntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
		pub orgnl_intr_bk_sttlm_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RtrdIntrBkSttlmAmt") )]
		pub rtrd_intr_bk_sttlm_amt: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
		pub intr_bk_sttlm_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmPrty", skip_serializing_if = "Option::is_none") )]
		pub sttlm_prty: Option<Priority3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmTmIndctn", skip_serializing_if = "Option::is_none") )]
		pub sttlm_tm_indctn: Option<SettlementDateTimeIndication1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RtrdInstdAmt", skip_serializing_if = "Option::is_none") )]
		pub rtrd_instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRate", skip_serializing_if = "Option::is_none") )]
		pub xchg_rate: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CompstnAmt", skip_serializing_if = "Option::is_none") )]
		pub compstn_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none") )]
		pub chrg_br: Option<ChargeBearerType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgsInf", skip_serializing_if = "Option::is_none") )]
		pub chrgs_inf: Option<Vec<Charges7>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none") )]
		pub clr_sys_ref: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none") )]
		pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none") )]
		pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RtrChain", skip_serializing_if = "Option::is_none") )]
		pub rtr_chain: Option<TransactionParties8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RtrRsnInf", skip_serializing_if = "Option::is_none") )]
		pub rtr_rsn_inf: Option<Vec<PaymentReturnReason6>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none") )]
		pub orgnl_tx_ref: Option<OriginalTransactionReference32>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl PaymentTransaction118 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.rtr_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "rtr_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "rtr_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.orgnl_grp_inf { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.orgnl_instr_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "orgnl_instr_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "orgnl_instr_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.orgnl_end_to_end_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "orgnl_end_to_end_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "orgnl_end_to_end_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.orgnl_tx_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "orgnl_tx_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "orgnl_tx_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.orgnl_uetr {
				let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "orgnl_uetr does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.orgnl_clr_sys_ref {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "orgnl_clr_sys_ref is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "orgnl_clr_sys_ref exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.orgnl_intr_bk_sttlm_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.rtrd_intr_bk_sttlm_amt.validate() { return Err(e); }
			if let Some(ref val) = self.sttlm_prty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.sttlm_tm_indctn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rtrd_instd_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.compstn_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.chrg_br { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.chrgs_inf { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.clr_sys_ref {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "clr_sys_ref is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "clr_sys_ref exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.instg_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.instd_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rtr_chain { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rtr_rsn_inf { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.orgnl_tx_ref { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// PaymentTypeInformation27 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentTypeInformation27 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none") )]
		pub instr_prty: Option<Priority2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrChanl", skip_serializing_if = "Option::is_none") )]
		pub clr_chanl: Option<ClearingChannel2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none") )]
		pub svc_lvl: Option<Vec<ServiceLevel8Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none") )]
		pub lcl_instrm: Option<LocalInstrument2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SeqTp", skip_serializing_if = "Option::is_none") )]
		pub seq_tp: Option<SequenceType3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none") )]
		pub ctgy_purp: Option<CategoryPurpose1Choice>,
	}
	
	impl PaymentTypeInformation27 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.instr_prty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.clr_chanl { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.svc_lvl { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.lcl_instrm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.seq_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ctgy_purp { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PersonIdentification13 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PersonIdentification13 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none") )]
		pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<Vec<GenericPersonIdentification1>>,
	}
	
	impl PersonIdentification13 {
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
	
	
	// PostalAddress24 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PostalAddress24 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
		pub adr_tp: Option<AddressType3Choice>,
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
	
	impl PostalAddress24 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.adr_tp { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.bldg_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "bldg_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "bldg_nm exceeds the maximum length of 35".to_string()));
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
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "twn_nm exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.twn_lctn_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "twn_lctn_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "twn_lctn_nm exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.dstrct_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "dstrct_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "dstrct_nm exceeds the maximum length of 35".to_string()));
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
	
	
	// PreferredContactMethod1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PreferredContactMethod1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "LETT") )]
		CodeLETT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MAIL") )]
		CodeMAIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PHON") )]
		CodePHON,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FAXX") )]
		CodeFAXX,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CELL") )]
		CodeCELL,
	}
	
	impl PreferredContactMethod1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Priority2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Priority2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "HIGH") )]
		CodeHIGH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NORM") )]
		CodeNORM,
	}
	
	impl Priority2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Priority3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Priority3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "URGT") )]
		CodeURGT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HIGH") )]
		CodeHIGH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NORM") )]
		CodeNORM,
	}
	
	impl Priority3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ProxyAccountIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ProxyAccountIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<ProxyAccountType1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
	}
	
	impl ProxyAccountIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 2048".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ProxyAccountType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ProxyAccountType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl ProxyAccountType1Choice {
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
	
	
	// Purpose2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Purpose2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl Purpose2Choice {
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
	
	
	// ReferredDocumentInformation7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReferredDocumentInformation7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<ReferredDocumentType4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nb", skip_serializing_if = "Option::is_none") )]
		pub nb: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdDt", skip_serializing_if = "Option::is_none") )]
		pub rltd_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LineDtls", skip_serializing_if = "Option::is_none") )]
		pub line_dtls: Option<Vec<DocumentLineInformation1>>,
	}
	
	impl ReferredDocumentInformation7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.nb {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nb is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "nb exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref vec) = self.line_dtls { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ReferredDocumentType3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReferredDocumentType3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<DocumentType6Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl ReferredDocumentType3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// ReferredDocumentType4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReferredDocumentType4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
		pub cd_or_prtry: ReferredDocumentType3Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<String>,
	}
	
	impl ReferredDocumentType4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd_or_prtry.validate() { return Err(e); }
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
	
	
	// RemittanceAmount2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RemittanceAmount2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DuePyblAmt", skip_serializing_if = "Option::is_none") )]
		pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DscntApldAmt", skip_serializing_if = "Option::is_none") )]
		pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtNoteAmt", skip_serializing_if = "Option::is_none") )]
		pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none") )]
		pub tax_amt: Option<Vec<TaxAmountAndType1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none") )]
		pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none") )]
		pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	}
	
	impl RemittanceAmount2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.due_pybl_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.dscnt_apld_amt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.cdt_note_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.tax_amt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.adjstmnt_amt_and_rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.rmtd_amt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RemittanceAmount3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RemittanceAmount3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DuePyblAmt", skip_serializing_if = "Option::is_none") )]
		pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DscntApldAmt", skip_serializing_if = "Option::is_none") )]
		pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtNoteAmt", skip_serializing_if = "Option::is_none") )]
		pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none") )]
		pub tax_amt: Option<Vec<TaxAmountAndType1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none") )]
		pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none") )]
		pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	}
	
	impl RemittanceAmount3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.due_pybl_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.dscnt_apld_amt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.cdt_note_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.tax_amt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.adjstmnt_amt_and_rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.rmtd_amt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RemittanceInformation16 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RemittanceInformation16 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ustrd", skip_serializing_if = "Option::is_none") )]
		pub ustrd: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Strd", skip_serializing_if = "Option::is_none") )]
		pub strd: Option<Vec<StructuredRemittanceInformation16>>,
	}
	
	impl RemittanceInformation16 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref vec) = self.ustrd {
				for item in vec {
					if item.chars().count() < 1 {
						return Err(ValidationError::new(1001, "ustrd is shorter than the minimum length of 1".to_string()));
					}
					if item.chars().count() > 140 {
						return Err(ValidationError::new(1002, "ustrd exceeds the maximum length of 140".to_string()));
					}
				}
			}
			if let Some(ref vec) = self.strd { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ReturnReason5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReturnReason5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl ReturnReason5Choice {
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
	
	
	// SequenceType3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum SequenceType3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "FRST") )]
		CodeFRST,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RCUR") )]
		CodeRCUR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FNAL") )]
		CodeFNAL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OOFF") )]
		CodeOOFF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RPRE") )]
		CodeRPRE,
	}
	
	impl SequenceType3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ServiceLevel8Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ServiceLevel8Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl ServiceLevel8Choice {
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
	
	
	// SettlementDateTimeIndication1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SettlementDateTimeIndication1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtDtTm", skip_serializing_if = "Option::is_none") )]
		pub dbt_dt_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDtTm", skip_serializing_if = "Option::is_none") )]
		pub cdt_dt_tm: Option<String>,
	}
	
	impl SettlementDateTimeIndication1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// SettlementInstruction7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SettlementInstruction7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmMtd") )]
		pub sttlm_mtd: SettlementMethod1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmAcct", skip_serializing_if = "Option::is_none") )]
		pub sttlm_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSys", skip_serializing_if = "Option::is_none") )]
		pub clr_sys: Option<ClearingSystemIdentification3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstgRmbrsmntAgt", skip_serializing_if = "Option::is_none") )]
		pub instg_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstgRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none") )]
		pub instg_rmbrsmnt_agt_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstdRmbrsmntAgt", skip_serializing_if = "Option::is_none") )]
		pub instd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstdRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none") )]
		pub instd_rmbrsmnt_agt_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ThrdRmbrsmntAgt", skip_serializing_if = "Option::is_none") )]
		pub thrd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ThrdRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none") )]
		pub thrd_rmbrsmnt_agt_acct: Option<CashAccount38>,
	}
	
	impl SettlementInstruction7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.sttlm_mtd.validate() { return Err(e); }
			if let Some(ref val) = self.sttlm_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.clr_sys { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.instg_rmbrsmnt_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.instg_rmbrsmnt_agt_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.instd_rmbrsmnt_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.instd_rmbrsmnt_agt_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.thrd_rmbrsmnt_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.thrd_rmbrsmnt_agt_acct { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SettlementMethod1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum SettlementMethod1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "INDA") )]
		CodeINDA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INGA") )]
		CodeINGA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COVE") )]
		CodeCOVE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLRG") )]
		CodeCLRG,
	}
	
	impl SettlementMethod1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// StructuredRemittanceInformation16 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct StructuredRemittanceInformation16 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RfrdDocInf", skip_serializing_if = "Option::is_none") )]
		pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation7>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RfrdDocAmt", skip_serializing_if = "Option::is_none") )]
		pub rfrd_doc_amt: Option<RemittanceAmount2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrRefInf", skip_serializing_if = "Option::is_none") )]
		pub cdtr_ref_inf: Option<CreditorReferenceInformation2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Invcr", skip_serializing_if = "Option::is_none") )]
		pub invcr: Option<PartyIdentification135>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Invcee", skip_serializing_if = "Option::is_none") )]
		pub invcee: Option<PartyIdentification135>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRmt", skip_serializing_if = "Option::is_none") )]
		pub tax_rmt: Option<TaxInformation7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GrnshmtRmt", skip_serializing_if = "Option::is_none") )]
		pub grnshmt_rmt: Option<Garnishment3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRmtInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_rmt_inf: Option<Vec<String>>,
	}
	
	impl StructuredRemittanceInformation16 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref vec) = self.rfrd_doc_inf { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.rfrd_doc_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.cdtr_ref_inf { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.invcr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.invcee { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.tax_rmt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.grnshmt_rmt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.addtl_rmt_inf {
				for item in vec {
					if item.chars().count() < 1 {
						return Err(ValidationError::new(1001, "addtl_rmt_inf is shorter than the minimum length of 1".to_string()));
					}
					if item.chars().count() > 140 {
						return Err(ValidationError::new(1002, "addtl_rmt_inf exceeds the maximum length of 140".to_string()));
					}
				}
			}
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
	
	
	// TaxAmount2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxAmount2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
		pub rate: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxblBaseAmt", skip_serializing_if = "Option::is_none") )]
		pub taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none") )]
		pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dtls", skip_serializing_if = "Option::is_none") )]
		pub dtls: Option<Vec<TaxRecordDetails2>>,
	}
	
	impl TaxAmount2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.taxbl_base_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ttl_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.dtls { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TaxAmountAndType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxAmountAndType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<TaxAmountType1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
	}
	
	impl TaxAmountAndType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// TaxAmountType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxAmountType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl TaxAmountType1Choice {
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
	
	
	// TaxAuthorisation1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxAuthorisation1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Titl", skip_serializing_if = "Option::is_none") )]
		pub titl: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<String>,
	}
	
	impl TaxAuthorisation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.titl {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "titl is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "titl exceeds the maximum length of 35".to_string()));
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
			Ok(())
		}
	}
	
	
	// TaxInformation7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxInformation7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
		pub cdtr: Option<TaxParty1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
		pub dbtr: Option<TaxParty2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
		pub ultmt_dbtr: Option<TaxParty2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdmstnZone", skip_serializing_if = "Option::is_none") )]
		pub admstn_zone: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefNb", skip_serializing_if = "Option::is_none") )]
		pub ref_nb: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mtd", skip_serializing_if = "Option::is_none") )]
		pub mtd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlTaxblBaseAmt", skip_serializing_if = "Option::is_none") )]
		pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlTaxAmt", skip_serializing_if = "Option::is_none") )]
		pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
		pub dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SeqNb", skip_serializing_if = "Option::is_none") )]
		pub seq_nb: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rcrd", skip_serializing_if = "Option::is_none") )]
		pub rcrd: Option<Vec<TaxRecord2>>,
	}
	
	impl TaxInformation7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cdtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dbtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ultmt_dbtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.admstn_zone {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "admstn_zone is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "admstn_zone exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.ref_nb {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "ref_nb is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "ref_nb exceeds the maximum length of 140".to_string()));
				}
			}
			if let Some(ref val) = self.mtd {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "mtd is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "mtd exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.ttl_taxbl_base_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ttl_tax_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rcrd { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TaxInformation8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxInformation8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
		pub cdtr: Option<TaxParty1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
		pub dbtr: Option<TaxParty2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdmstnZone", skip_serializing_if = "Option::is_none") )]
		pub admstn_zone: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefNb", skip_serializing_if = "Option::is_none") )]
		pub ref_nb: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mtd", skip_serializing_if = "Option::is_none") )]
		pub mtd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlTaxblBaseAmt", skip_serializing_if = "Option::is_none") )]
		pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlTaxAmt", skip_serializing_if = "Option::is_none") )]
		pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
		pub dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SeqNb", skip_serializing_if = "Option::is_none") )]
		pub seq_nb: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rcrd", skip_serializing_if = "Option::is_none") )]
		pub rcrd: Option<Vec<TaxRecord2>>,
	}
	
	impl TaxInformation8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cdtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dbtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.admstn_zone {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "admstn_zone is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "admstn_zone exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.ref_nb {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "ref_nb is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "ref_nb exceeds the maximum length of 140".to_string()));
				}
			}
			if let Some(ref val) = self.mtd {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "mtd is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "mtd exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.ttl_taxbl_base_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ttl_tax_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rcrd { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TaxParty1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxParty1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxId", skip_serializing_if = "Option::is_none") )]
		pub tax_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RegnId", skip_serializing_if = "Option::is_none") )]
		pub regn_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxTp", skip_serializing_if = "Option::is_none") )]
		pub tax_tp: Option<String>,
	}
	
	impl TaxParty1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tax_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tax_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "tax_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.regn_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "regn_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "regn_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.tax_tp {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tax_tp is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "tax_tp exceeds the maximum length of 35".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// TaxParty2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxParty2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxId", skip_serializing_if = "Option::is_none") )]
		pub tax_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RegnId", skip_serializing_if = "Option::is_none") )]
		pub regn_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxTp", skip_serializing_if = "Option::is_none") )]
		pub tax_tp: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Authstn", skip_serializing_if = "Option::is_none") )]
		pub authstn: Option<TaxAuthorisation1>,
	}
	
	impl TaxParty2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tax_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tax_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "tax_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.regn_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "regn_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "regn_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.tax_tp {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tax_tp is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "tax_tp exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.authstn { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TaxPeriod2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxPeriod2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Yr", skip_serializing_if = "Option::is_none") )]
		pub yr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<TaxRecordPeriod1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDt", skip_serializing_if = "Option::is_none") )]
		pub fr_to_dt: Option<DatePeriod2>,
	}
	
	impl TaxPeriod2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fr_to_dt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TaxRecord2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxRecord2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctgy", skip_serializing_if = "Option::is_none") )]
		pub ctgy: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtgyDtls", skip_serializing_if = "Option::is_none") )]
		pub ctgy_dtls: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrSts", skip_serializing_if = "Option::is_none") )]
		pub dbtr_sts: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CertId", skip_serializing_if = "Option::is_none") )]
		pub cert_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none") )]
		pub frms_cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
		pub prd: Option<TaxPeriod2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none") )]
		pub tax_amt: Option<TaxAmount2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<String>,
	}
	
	impl TaxRecord2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tp {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.ctgy {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "ctgy is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "ctgy exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.ctgy_dtls {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "ctgy_dtls is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "ctgy_dtls exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.dbtr_sts {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "dbtr_sts is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "dbtr_sts exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.cert_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "cert_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "cert_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.frms_cd {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "frms_cd is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "frms_cd exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.prd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.tax_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.addtl_inf {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 140".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// TaxRecordDetails2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxRecordDetails2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
		pub prd: Option<TaxPeriod2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
	}
	
	impl TaxRecordDetails2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.prd { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// TaxRecordPeriod1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum TaxRecordPeriod1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "MM01") )]
		CodeMM01,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MM02") )]
		CodeMM02,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MM03") )]
		CodeMM03,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MM04") )]
		CodeMM04,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MM05") )]
		CodeMM05,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MM06") )]
		CodeMM06,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MM07") )]
		CodeMM07,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MM08") )]
		CodeMM08,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MM09") )]
		CodeMM09,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MM10") )]
		CodeMM10,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MM11") )]
		CodeMM11,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MM12") )]
		CodeMM12,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QTR1") )]
		CodeQTR1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QTR2") )]
		CodeQTR2,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QTR3") )]
		CodeQTR3,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QTR4") )]
		CodeQTR4,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HLF1") )]
		CodeHLF1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HLF2") )]
		CodeHLF2,
	}
	
	impl TaxRecordPeriod1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TransactionParties8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TransactionParties8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
		pub ultmt_dbtr: Option<Party40Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr") )]
		pub dbtr: Party40Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
		pub dbtr_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty", skip_serializing_if = "Option::is_none") )]
		pub initg_pty: Option<Party40Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
		pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
		pub dbtr_agt_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt1", skip_serializing_if = "Option::is_none") )]
		pub prvs_instg_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt1Acct", skip_serializing_if = "Option::is_none") )]
		pub prvs_instg_agt1_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt2", skip_serializing_if = "Option::is_none") )]
		pub prvs_instg_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt2Acct", skip_serializing_if = "Option::is_none") )]
		pub prvs_instg_agt2_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt3", skip_serializing_if = "Option::is_none") )]
		pub prvs_instg_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt3Acct", skip_serializing_if = "Option::is_none") )]
		pub prvs_instg_agt3_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none") )]
		pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none") )]
		pub intrmy_agt1_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none") )]
		pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none") )]
		pub intrmy_agt2_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none") )]
		pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3Acct", skip_serializing_if = "Option::is_none") )]
		pub intrmy_agt3_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
		pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
		pub cdtr_agt_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
		pub cdtr: Party40Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
		pub cdtr_acct: Option<CashAccount38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
		pub ultmt_cdtr: Option<Party40Choice>,
	}
	
	impl TransactionParties8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.ultmt_dbtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.dbtr.validate() { return Err(e); }
			if let Some(ref val) = self.dbtr_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.initg_pty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dbtr_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dbtr_agt_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prvs_instg_agt1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prvs_instg_agt1_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prvs_instg_agt2 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prvs_instg_agt2_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prvs_instg_agt3 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prvs_instg_agt3_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.intrmy_agt1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.intrmy_agt1_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.intrmy_agt2 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.intrmy_agt2_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.intrmy_agt3 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.intrmy_agt3_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.cdtr_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.cdtr_agt_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.cdtr.validate() { return Err(e); }
			if let Some(ref val) = self.cdtr_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ultmt_cdtr { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
}