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
	
	
	// AccountCashEntryReturnCriteria3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountCashEntryReturnCriteria3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtryRefInd", skip_serializing_if = "Option::is_none") )]
		pub ntry_ref_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctTpInd", skip_serializing_if = "Option::is_none") )]
		pub acct_tp_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtryAmtInd", skip_serializing_if = "Option::is_none") )]
		pub ntry_amt_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctCcyInd", skip_serializing_if = "Option::is_none") )]
		pub acct_ccy_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtryStsInd", skip_serializing_if = "Option::is_none") )]
		pub ntry_sts_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtryDtInd", skip_serializing_if = "Option::is_none") )]
		pub ntry_dt_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrInd", skip_serializing_if = "Option::is_none") )]
		pub acct_svcr_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOwnrInd", skip_serializing_if = "Option::is_none") )]
		pub acct_ownr_ind: Option<bool>,
	}
	
	impl AccountCashEntryReturnCriteria3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// AccountIdentificationSearchCriteria2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountIdentificationSearchCriteria2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "EQ", skip_serializing_if = "Option::is_none") )]
		pub eq: Option<AccountIdentification4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CTTxt", skip_serializing_if = "Option::is_none") )]
		pub ct_txt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NCTTxt", skip_serializing_if = "Option::is_none") )]
		pub nct_txt: Option<String>,
	}
	
	impl AccountIdentificationSearchCriteria2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.eq { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ct_txt {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "ct_txt is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "ct_txt exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.nct_txt {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nct_txt is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "nct_txt exceeds the maximum length of 35".to_string()));
				}
			}
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
	
	
	// ActiveAmountRange3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ActiveAmountRange3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ImpldCcyAndAmtRg", skip_serializing_if = "Option::is_none") )]
		pub impld_ccy_and_amt_rg: Option<ImpliedCurrencyAndAmountRange1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CcyAndAmtRg", skip_serializing_if = "Option::is_none") )]
		pub ccy_and_amt_rg: Option<ActiveCurrencyAndAmountRange3>,
	}
	
	impl ActiveAmountRange3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.impld_ccy_and_amt_rg { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ccy_and_amt_rg { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ActiveCurrencyAndAmountRange3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ActiveCurrencyAndAmountRange3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ImpliedCurrencyAmountRange1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
		pub cdt_dbt_ind: Option<CreditDebitCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
		pub ccy: String,
	}
	
	impl ActiveCurrencyAndAmountRange3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Some(ref val) = self.cdt_dbt_ind { if let Err(e) = val.validate() { return Err(e); } }
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.ccy) {
				return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ActiveOrHistoricAmountRange2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ActiveOrHistoricAmountRange2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ImpldCcyAndAmtRg", skip_serializing_if = "Option::is_none") )]
		pub impld_ccy_and_amt_rg: Option<ImpliedCurrencyAndAmountRange1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CcyAndAmtRg", skip_serializing_if = "Option::is_none") )]
		pub ccy_and_amt_rg: Option<ActiveOrHistoricCurrencyAndAmountRange2>,
	}
	
	impl ActiveOrHistoricAmountRange2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.impld_ccy_and_amt_rg { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ccy_and_amt_rg { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ActiveOrHistoricCurrencyAndAmountRange2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ActiveOrHistoricCurrencyAndAmountRange2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ImpliedCurrencyAmountRange1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
		pub cdt_dbt_ind: Option<CreditDebitCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
		pub ccy: String,
	}
	
	impl ActiveOrHistoricCurrencyAndAmountRange2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Some(ref val) = self.cdt_dbt_ind { if let Err(e) = val.validate() { return Err(e); } }
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.ccy) {
				return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
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
	
	
	// AmountRangeBoundary1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AmountRangeBoundary1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BdryAmt") )]
		pub bdry_amt: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Incl") )]
		pub incl: bool,
	}
	
	impl AmountRangeBoundary1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.bdry_amt < 0.000000 {
				return Err(ValidationError::new(1003, "bdry_amt is less than the minimum value of 0.000000".to_string()));
			}
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
	
	
	// CashAccountEntrySearch8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CashAccountEntrySearch8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctId", skip_serializing_if = "Option::is_none") )]
		pub acct_id: Option<Vec<AccountIdentificationSearchCriteria2Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtryAmt", skip_serializing_if = "Option::is_none") )]
		pub ntry_amt: Option<Vec<ActiveOrHistoricAmountRange2Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtryAmtCcy", skip_serializing_if = "Option::is_none") )]
		pub ntry_amt_ccy: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
		pub cdt_dbt_ind: Option<CreditDebitCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtrySts", skip_serializing_if = "Option::is_none") )]
		pub ntry_sts: Option<Vec<EntryStatus1Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtryDt", skip_serializing_if = "Option::is_none") )]
		pub ntry_dt: Option<Vec<DateAndDateTimeSearch3Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none") )]
		pub acct_ownr: Option<PartyIdentification272>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none") )]
		pub acct_svcr: Option<BranchAndFinancialInstitutionIdentification8>,
	}
	
	impl CashAccountEntrySearch8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref vec) = self.acct_id { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.ntry_amt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.ntry_amt_ccy {
				for item in vec {
					let pattern = Regex::new("[A-Z]{3,3}").unwrap();
					if !pattern.is_match(&item) {
						return Err(ValidationError::new(1005, "ntry_amt_ccy does not match the required pattern".to_string()));
					}
				}
			}
			if let Some(ref val) = self.cdt_dbt_ind { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.ntry_sts { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.ntry_dt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.acct_ownr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.acct_svcr { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CashPaymentStatus2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CashPaymentStatus2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PDNG") )]
		CodePDNG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FINL") )]
		CodeFINL,
	}
	
	impl CashPaymentStatus2Code {
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
	
	
	// DateAndDateTimeSearch3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DateAndDateTimeSearch3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtTmSch", skip_serializing_if = "Option::is_none") )]
		pub dt_tm_sch: Option<DateTimePeriod1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtSch", skip_serializing_if = "Option::is_none") )]
		pub dt_sch: Option<DatePeriodSearch1Choice>,
	}
	
	impl DateAndDateTimeSearch3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.dt_tm_sch { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dt_sch { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// DatePeriodSearch1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DatePeriodSearch1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt", skip_serializing_if = "Option::is_none") )]
		pub fr_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt", skip_serializing_if = "Option::is_none") )]
		pub to_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDt", skip_serializing_if = "Option::is_none") )]
		pub fr_to_dt: Option<DatePeriod2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EQDt", skip_serializing_if = "Option::is_none") )]
		pub eq_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NEQDt", skip_serializing_if = "Option::is_none") )]
		pub neq_dt: Option<String>,
	}
	
	impl DatePeriodSearch1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.fr_to_dt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DateTimePeriod1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DateTimePeriod1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtTm") )]
		pub fr_dt_tm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToDtTm") )]
		pub to_dt_tm: String,
	}
	
	impl DateTimePeriod1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// DateTimePeriod1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DateTimePeriod1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtTm", skip_serializing_if = "Option::is_none") )]
		pub fr_dt_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none") )]
		pub to_dt_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtTmRg", skip_serializing_if = "Option::is_none") )]
		pub dt_tm_rg: Option<DateTimePeriod1>,
	}
	
	impl DateTimePeriod1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.dt_tm_rg { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EntryStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum EntryStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "BOOK") )]
		CodeBOOK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PDNG") )]
		CodePDNG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FUTR") )]
		CodeFUTR,
	}
	
	impl EntryStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// FinalStatusCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum FinalStatusCode {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "STLD") )]
		CodeSTLD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RJTD") )]
		CodeRJTD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CAND") )]
		CodeCAND,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FNLD") )]
		CodeFNLD,
	}
	
	impl FinalStatusCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// FromToAmountRange1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FromToAmountRange1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrAmt") )]
		pub fr_amt: AmountRangeBoundary1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToAmt") )]
		pub to_amt: AmountRangeBoundary1,
	}
	
	impl FromToAmountRange1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.fr_amt.validate() { return Err(e); }
			if let Err(e) = self.to_amt.validate() { return Err(e); }
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
	
	
	// GenericIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<String>,
	}
	
	impl GenericIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.schme_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
				}
			}
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
	
	
	// GetTransactionV11 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GetTransactionV11 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgHdr") )]
		pub msg_hdr: MessageHeader9,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxQryDef", skip_serializing_if = "Option::is_none") )]
		pub tx_qry_def: Option<TransactionQuery8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl GetTransactionV11 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.msg_hdr.validate() { return Err(e); }
			if let Some(ref val) = self.tx_qry_def { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ImpliedCurrencyAmountRange1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ImpliedCurrencyAmountRange1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrAmt", skip_serializing_if = "Option::is_none") )]
		pub fr_amt: Option<AmountRangeBoundary1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToAmt", skip_serializing_if = "Option::is_none") )]
		pub to_amt: Option<AmountRangeBoundary1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrToAmt", skip_serializing_if = "Option::is_none") )]
		pub fr_to_amt: Option<FromToAmountRange1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EQAmt", skip_serializing_if = "Option::is_none") )]
		pub eq_amt: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NEQAmt", skip_serializing_if = "Option::is_none") )]
		pub neq_amt: Option<f64>,
	}
	
	impl ImpliedCurrencyAmountRange1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.fr_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.to_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fr_to_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.eq_amt {
				if *val < 0.000000 {
					return Err(ValidationError::new(1003, "eq_amt is less than the minimum value of 0.000000".to_string()));
				}
			}
			if let Some(ref val) = self.neq_amt {
				if *val < 0.000000 {
					return Err(ValidationError::new(1003, "neq_amt is less than the minimum value of 0.000000".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// ImpliedCurrencyAndAmountRange1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ImpliedCurrencyAndAmountRange1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ImpliedCurrencyAmountRange1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
		pub cdt_dbt_ind: Option<CreditDebitCode>,
	}
	
	impl ImpliedCurrencyAndAmountRange1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Some(ref val) = self.cdt_dbt_ind { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Instruction1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Instruction1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PBEN") )]
		CodePBEN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TTIL") )]
		CodeTTIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TFRO") )]
		CodeTFRO,
	}
	
	impl Instruction1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// InstructionStatusReturnCriteria1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InstructionStatusReturnCriteria1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInstrStsInd") )]
		pub pmt_instr_sts_ind: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInstrStsDtTmInd", skip_serializing_if = "Option::is_none") )]
		pub pmt_instr_sts_dt_tm_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInstrStsRsnInd", skip_serializing_if = "Option::is_none") )]
		pub pmt_instr_sts_rsn_ind: Option<bool>,
	}
	
	impl InstructionStatusReturnCriteria1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// InstructionStatusSearch5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InstructionStatusSearch5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInstrSts", skip_serializing_if = "Option::is_none") )]
		pub pmt_instr_sts: Option<PaymentStatusCodeSearch2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInstrStsDtTm", skip_serializing_if = "Option::is_none") )]
		pub pmt_instr_sts_dt_tm: Option<DateTimePeriod1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryStsRsn", skip_serializing_if = "Option::is_none") )]
		pub prtry_sts_rsn: Option<String>,
	}
	
	impl InstructionStatusSearch5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.pmt_instr_sts { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.pmt_instr_sts_dt_tm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry_sts_rsn {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "prtry_sts_rsn is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 4 {
					return Err(ValidationError::new(1002, "prtry_sts_rsn exceeds the maximum length of 4".to_string()));
				}
				let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "prtry_sts_rsn does not match the required pattern".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// LongPaymentIdentification4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct LongPaymentIdentification4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
		pub tx_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
		pub uetr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmAmt") )]
		pub intr_bk_sttlm_amt: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt") )]
		pub intr_bk_sttlm_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none") )]
		pub pmt_mtd: Option<PaymentOrigin1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt") )]
		pub instg_agt: BranchAndFinancialInstitutionIdentification8,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAgt") )]
		pub instd_agt: BranchAndFinancialInstitutionIdentification8,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtryTp", skip_serializing_if = "Option::is_none") )]
		pub ntry_tp: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none") )]
		pub end_to_end_id: Option<String>,
	}
	
	impl LongPaymentIdentification4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tx_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tx_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "tx_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.uetr {
				let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.pmt_mtd { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.instg_agt.validate() { return Err(e); }
			if let Err(e) = self.instd_agt.validate() { return Err(e); }
			if let Some(ref val) = self.ntry_tp {
				let pattern = Regex::new("[BEOVW]{1,1}[0-9]{2,2}|DUM").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "ntry_tp does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.end_to_end_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "end_to_end_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "end_to_end_id exceeds the maximum length of 35".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// MessageHeader9 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MessageHeader9 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
		pub msg_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
		pub cre_dt_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ReqTp", skip_serializing_if = "Option::is_none") )]
		pub req_tp: Option<RequestType4Choice>,
	}
	
	impl MessageHeader9 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.msg_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
			}
			if self.msg_id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.req_tp { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// Party50Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Party50Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pty", skip_serializing_if = "Option::is_none") )]
		pub pty: Option<PartyIdentification272>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Agt", skip_serializing_if = "Option::is_none") )]
		pub agt: Option<BranchAndFinancialInstitutionIdentification8>,
	}
	
	impl Party50Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.pty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.agt { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// PaymentIdentification8Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentIdentification8Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
		pub tx_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
		pub uetr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QId", skip_serializing_if = "Option::is_none") )]
		pub q_id: Option<QueueTransactionIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LngBizId", skip_serializing_if = "Option::is_none") )]
		pub lng_biz_id: Option<LongPaymentIdentification4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtBizId", skip_serializing_if = "Option::is_none") )]
		pub shrt_biz_id: Option<ShortPaymentIdentification4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<String>,
	}
	
	impl PaymentIdentification8Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tx_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tx_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "tx_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.uetr {
				let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.q_id { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.lng_biz_id { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.shrt_biz_id { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "prtry_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 70 {
					return Err(ValidationError::new(1002, "prtry_id exceeds the maximum length of 70".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// PaymentInstrument1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PaymentInstrument1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "BDT") )]
		CodeBDT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BCT") )]
		CodeBCT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CDT") )]
		CodeCDT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CCT") )]
		CodeCCT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CHK") )]
		CodeCHK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BKT") )]
		CodeBKT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DCP") )]
		CodeDCP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CCP") )]
		CodeCCP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RTI") )]
		CodeRTI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CAN") )]
		CodeCAN,
	}
	
	impl PaymentInstrument1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PaymentOrigin1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentOrigin1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FINMT", skip_serializing_if = "Option::is_none") )]
		pub finmt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XMLMsgNm", skip_serializing_if = "Option::is_none") )]
		pub xml_msg_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Instrm", skip_serializing_if = "Option::is_none") )]
		pub instrm: Option<PaymentInstrument1Code>,
	}
	
	impl PaymentOrigin1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.finmt {
				let pattern = Regex::new("[0-9]{1,3}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "finmt does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.xml_msg_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "xml_msg_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "xml_msg_nm exceeds the maximum length of 35".to_string()));
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
			if let Some(ref val) = self.instrm { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaymentReturnCriteria4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentReturnCriteria4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgIdInd", skip_serializing_if = "Option::is_none") )]
		pub msg_id_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdExctnDtInd", skip_serializing_if = "Option::is_none") )]
		pub reqd_exctn_dt_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstrInd", skip_serializing_if = "Option::is_none") )]
		pub instr_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstrStsRtrCrit", skip_serializing_if = "Option::is_none") )]
		pub instr_sts_rtr_crit: Option<InstructionStatusReturnCriteria1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAmtInd", skip_serializing_if = "Option::is_none") )]
		pub instd_amt_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
		pub cdt_dbt_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmAmtInd", skip_serializing_if = "Option::is_none") )]
		pub intr_bk_sttlm_amt_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtyInd", skip_serializing_if = "Option::is_none") )]
		pub prty_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgVldtyTmInd", skip_serializing_if = "Option::is_none") )]
		pub prcg_vldty_tm_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PurpInd", skip_serializing_if = "Option::is_none") )]
		pub purp_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstrCpyInd", skip_serializing_if = "Option::is_none") )]
		pub instr_cpy_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMTInd", skip_serializing_if = "Option::is_none") )]
		pub pmt_mt_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInd", skip_serializing_if = "Option::is_none") )]
		pub pmt_tp_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxIdInd", skip_serializing_if = "Option::is_none") )]
		pub tx_id_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDtInd", skip_serializing_if = "Option::is_none") )]
		pub intr_bk_sttlm_dt_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EndToEndIdInd", skip_serializing_if = "Option::is_none") )]
		pub end_to_end_id_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtdInd", skip_serializing_if = "Option::is_none") )]
		pub pmt_mtd_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrInd", skip_serializing_if = "Option::is_none") )]
		pub dbtr_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtInd", skip_serializing_if = "Option::is_none") )]
		pub dbtr_agt_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstgRmbrsmntAgtInd", skip_serializing_if = "Option::is_none") )]
		pub instg_rmbrsmnt_agt_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstdRmbrsmntAgtInd", skip_serializing_if = "Option::is_none") )]
		pub instd_rmbrsmnt_agt_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyInd", skip_serializing_if = "Option::is_none") )]
		pub intrmy_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtInd", skip_serializing_if = "Option::is_none") )]
		pub cdtr_agt_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrInd", skip_serializing_if = "Option::is_none") )]
		pub cdtr_ind: Option<bool>,
	}
	
	impl PaymentReturnCriteria4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.instr_sts_rtr_crit { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaymentSearch10 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentSearch10 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId", skip_serializing_if = "Option::is_none") )]
		pub msg_id: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none") )]
		pub reqd_exctn_dt: Option<Vec<DateAndDateTimeSearch3Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtId", skip_serializing_if = "Option::is_none") )]
		pub pmt_id: Option<Vec<PaymentIdentification8Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
		pub sts: Option<Vec<InstructionStatusSearch5>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none") )]
		pub instd_amt: Option<Vec<ActiveOrHistoricAmountRange2Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAmtCcy", skip_serializing_if = "Option::is_none") )]
		pub instd_amt_ccy: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
		pub cdt_dbt_ind: Option<CreditDebitCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmAmt", skip_serializing_if = "Option::is_none") )]
		pub intr_bk_sttlm_amt: Option<Vec<ActiveAmountRange3Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmAmtCcy", skip_serializing_if = "Option::is_none") )]
		pub intr_bk_sttlm_amt_ccy: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none") )]
		pub pmt_mtd: Option<Vec<PaymentOrigin1Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTp", skip_serializing_if = "Option::is_none") )]
		pub pmt_tp: Option<Vec<PaymentType4Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prty", skip_serializing_if = "Option::is_none") )]
		pub prty: Option<Vec<Priority1Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgVldtyTm", skip_serializing_if = "Option::is_none") )]
		pub prcg_vldty_tm: Option<Vec<DateTimePeriod1Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Instr", skip_serializing_if = "Option::is_none") )]
		pub instr: Option<Vec<Instruction1Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
		pub tx_id: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
		pub uetr: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
		pub intr_bk_sttlm_dt: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none") )]
		pub end_to_end_id: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pties", skip_serializing_if = "Option::is_none") )]
		pub pties: Option<PaymentTransactionParty4>,
	}
	
	impl PaymentSearch10 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref vec) = self.msg_id {
				for item in vec {
					if item.chars().count() < 1 {
						return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
					}
					if item.chars().count() > 35 {
						return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
					}
				}
			}
			if let Some(ref vec) = self.reqd_exctn_dt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.pmt_id { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.sts { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.instd_amt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.instd_amt_ccy {
				for item in vec {
					let pattern = Regex::new("[A-Z]{3,3}").unwrap();
					if !pattern.is_match(&item) {
						return Err(ValidationError::new(1005, "instd_amt_ccy does not match the required pattern".to_string()));
					}
				}
			}
			if let Some(ref val) = self.cdt_dbt_ind { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.intr_bk_sttlm_amt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.intr_bk_sttlm_amt_ccy {
				for item in vec {
					let pattern = Regex::new("[A-Z]{3,3}").unwrap();
					if !pattern.is_match(&item) {
						return Err(ValidationError::new(1005, "intr_bk_sttlm_amt_ccy does not match the required pattern".to_string()));
					}
				}
			}
			if let Some(ref vec) = self.pmt_mtd { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.pmt_tp { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.prty { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.prcg_vldty_tm { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.instr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.tx_id {
				for item in vec {
					if item.chars().count() < 1 {
						return Err(ValidationError::new(1001, "tx_id is shorter than the minimum length of 1".to_string()));
					}
					if item.chars().count() > 35 {
						return Err(ValidationError::new(1002, "tx_id exceeds the maximum length of 35".to_string()));
					}
				}
			}
			if let Some(ref vec) = self.uetr {
				for item in vec {
					let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
					if !pattern.is_match(&item) {
						return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
					}
				}
			}
			if let Some(ref vec) = self.end_to_end_id {
				for item in vec {
					if item.chars().count() < 1 {
						return Err(ValidationError::new(1001, "end_to_end_id is shorter than the minimum length of 1".to_string()));
					}
					if item.chars().count() > 35 {
						return Err(ValidationError::new(1002, "end_to_end_id exceeds the maximum length of 35".to_string()));
					}
				}
			}
			if let Some(ref val) = self.pties { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaymentStatusCodeSearch2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentStatusCodeSearch2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdgSts", skip_serializing_if = "Option::is_none") )]
		pub pdg_sts: Option<PendingStatus4Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FnlSts", skip_serializing_if = "Option::is_none") )]
		pub fnl_sts: Option<FinalStatusCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdgAndFnlSts", skip_serializing_if = "Option::is_none") )]
		pub pdg_and_fnl_sts: Option<CashPaymentStatus2Code>,
	}
	
	impl PaymentStatusCodeSearch2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.pdg_sts { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fnl_sts { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.pdg_and_fnl_sts { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaymentTransactionParty4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentTransactionParty4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none") )]
		pub instg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none") )]
		pub instd_agt: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
		pub ultmt_dbtr: Option<Party50Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
		pub dbtr: Option<Party50Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
		pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstgRmbrsmntAgt", skip_serializing_if = "Option::is_none") )]
		pub instg_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstdRmbrsmntAgt", skip_serializing_if = "Option::is_none") )]
		pub instd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none") )]
		pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none") )]
		pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none") )]
		pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
		pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
		pub cdtr: Option<Party50Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
		pub ultmt_cdtr: Option<Party50Choice>,
	}
	
	impl PaymentTransactionParty4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.instg_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.instd_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ultmt_dbtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dbtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dbtr_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.instg_rmbrsmnt_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.instd_rmbrsmnt_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.intrmy_agt1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.intrmy_agt2 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.intrmy_agt3 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.cdtr_agt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.cdtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ultmt_cdtr { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaymentType3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PaymentType3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CBS") )]
		CodeCBS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BCK") )]
		CodeBCK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BAL") )]
		CodeBAL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLS") )]
		CodeCLS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CTR") )]
		CodeCTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CBH") )]
		CodeCBH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CBP") )]
		CodeCBP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DPG") )]
		CodeDPG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DPN") )]
		CodeDPN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EXP") )]
		CodeEXP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TCH") )]
		CodeTCH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LMT") )]
		CodeLMT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LIQ") )]
		CodeLIQ,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DPP") )]
		CodeDPP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DPH") )]
		CodeDPH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DPS") )]
		CodeDPS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "STF") )]
		CodeSTF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRP") )]
		CodeTRP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TCS") )]
		CodeTCS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LOA") )]
		CodeLOA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LOR") )]
		CodeLOR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TCP") )]
		CodeTCP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OND") )]
		CodeOND,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MGL") )]
		CodeMGL,
	}
	
	impl PaymentType3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PaymentType4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentType4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<PaymentType3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl PaymentType4Choice {
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
	
	
	// PendingStatus4Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PendingStatus4Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACPD") )]
		CodeACPD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VALD") )]
		CodeVALD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MATD") )]
		CodeMATD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AUTD") )]
		CodeAUTD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVD") )]
		CodeINVD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UMAC") )]
		CodeUMAC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "STLE") )]
		CodeSTLE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "STLM") )]
		CodeSTLM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SSPD") )]
		CodeSSPD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PCAN") )]
		CodePCAN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PSTL") )]
		CodePSTL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PFST") )]
		CodePFST,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SMLR") )]
		CodeSMLR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RMLR") )]
		CodeRMLR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SRBL") )]
		CodeSRBL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AVLB") )]
		CodeAVLB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SRML") )]
		CodeSRML,
	}
	
	impl PendingStatus4Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// Priority1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Priority1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<Priority5Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl Priority1Choice {
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
	
	
	// Priority5Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Priority5Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "HIGH") )]
		CodeHIGH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LOWW") )]
		CodeLOWW,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NORM") )]
		CodeNORM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "URGT") )]
		CodeURGT,
	}
	
	impl Priority5Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// QueryType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum QueryType2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ALLL") )]
		CodeALLL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CHNG") )]
		CodeCHNG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MODF") )]
		CodeMODF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DELD") )]
		CodeDELD,
	}
	
	impl QueryType2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// QueueTransactionIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct QueueTransactionIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "QId") )]
		pub q_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PosInQ") )]
		pub pos_in_q: String,
	}
	
	impl QueueTransactionIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.q_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "q_id is shorter than the minimum length of 1".to_string()));
			}
			if self.q_id.chars().count() > 16 {
				return Err(ValidationError::new(1002, "q_id exceeds the maximum length of 16".to_string()));
			}
			if self.pos_in_q.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pos_in_q is shorter than the minimum length of 1".to_string()));
			}
			if self.pos_in_q.chars().count() > 16 {
				return Err(ValidationError::new(1002, "pos_in_q exceeds the maximum length of 16".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ReportIndicator1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ReportIndicator1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "STND") )]
		CodeSTND,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRPR") )]
		CodePRPR,
	}
	
	impl ReportIndicator1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// RequestType4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RequestType4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCtrl", skip_serializing_if = "Option::is_none") )]
		pub pmt_ctrl: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Enqry", skip_serializing_if = "Option::is_none") )]
		pub enqry: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification1>,
	}
	
	impl RequestType4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.pmt_ctrl {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "pmt_ctrl is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 4 {
					return Err(ValidationError::new(1002, "pmt_ctrl exceeds the maximum length of 4".to_string()));
				}
			}
			if let Some(ref val) = self.enqry {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "enqry is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 4 {
					return Err(ValidationError::new(1002, "enqry exceeds the maximum length of 4".to_string()));
				}
			}
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ShortPaymentIdentification4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ShortPaymentIdentification4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
		pub tx_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
		pub uetr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt") )]
		pub intr_bk_sttlm_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt") )]
		pub instg_agt: BranchAndFinancialInstitutionIdentification8,
	}
	
	impl ShortPaymentIdentification4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tx_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tx_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "tx_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.uetr {
				let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
				}
			}
			if let Err(e) = self.instg_agt.validate() { return Err(e); }
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
	
	
	// SystemReturnCriteria2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SystemReturnCriteria2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SysIdInd", skip_serializing_if = "Option::is_none") )]
		pub sys_id_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MmbIdInd", skip_serializing_if = "Option::is_none") )]
		pub mmb_id_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryIdInd", skip_serializing_if = "Option::is_none") )]
		pub ctry_id_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctIdInd", skip_serializing_if = "Option::is_none") )]
		pub acct_id_ind: Option<bool>,
	}
	
	impl SystemReturnCriteria2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// SystemSearch5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SystemSearch5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SysId", skip_serializing_if = "Option::is_none") )]
		pub sys_id: Option<Vec<ClearingSystemIdentification3Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MmbId", skip_serializing_if = "Option::is_none") )]
		pub mmb_id: Option<Vec<BranchAndFinancialInstitutionIdentification8>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
		pub ctry: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctId", skip_serializing_if = "Option::is_none") )]
		pub acct_id: Option<AccountIdentification4Choice>,
	}
	
	impl SystemSearch5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref vec) = self.sys_id { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.mmb_id { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.ctry {
				let pattern = Regex::new("[A-Z]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.acct_id { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TransactionCriteria11 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TransactionCriteria11 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NewQryNm", skip_serializing_if = "Option::is_none") )]
		pub new_qry_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchCrit", skip_serializing_if = "Option::is_none") )]
		pub sch_crit: Option<Vec<TransactionSearchCriteria11>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StmtRpt", skip_serializing_if = "Option::is_none") )]
		pub stmt_rpt: Option<ReportIndicator1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RtrCrit", skip_serializing_if = "Option::is_none") )]
		pub rtr_crit: Option<TransactionReturnCriteria5>,
	}
	
	impl TransactionCriteria11 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.new_qry_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "new_qry_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "new_qry_nm exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref vec) = self.sch_crit { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.stmt_rpt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rtr_crit { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TransactionCriteria8Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TransactionCriteria8Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "QryNm", skip_serializing_if = "Option::is_none") )]
		pub qry_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NewCrit", skip_serializing_if = "Option::is_none") )]
		pub new_crit: Option<TransactionCriteria11>,
	}
	
	impl TransactionCriteria8Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.qry_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "qry_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "qry_nm exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.new_crit { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TransactionQuery8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TransactionQuery8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "QryTp", skip_serializing_if = "Option::is_none") )]
		pub qry_tp: Option<QueryType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxCrit", skip_serializing_if = "Option::is_none") )]
		pub tx_crit: Option<TransactionCriteria8Choice>,
	}
	
	impl TransactionQuery8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.qry_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.tx_crit { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TransactionReturnCriteria5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TransactionReturnCriteria5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtToRtrCrit", skip_serializing_if = "Option::is_none") )]
		pub pmt_to_rtr_crit: Option<SystemReturnCriteria2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtFrRtrCrit", skip_serializing_if = "Option::is_none") )]
		pub pmt_fr_rtr_crit: Option<SystemReturnCriteria2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctCshNtryRtrCrit", skip_serializing_if = "Option::is_none") )]
		pub acct_csh_ntry_rtr_crit: Option<AccountCashEntryReturnCriteria3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtRtrCrit", skip_serializing_if = "Option::is_none") )]
		pub pmt_rtr_crit: Option<PaymentReturnCriteria4>,
	}
	
	impl TransactionReturnCriteria5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.pmt_to_rtr_crit { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.pmt_fr_rtr_crit { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.acct_csh_ntry_rtr_crit { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.pmt_rtr_crit { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TransactionSearchCriteria11 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TransactionSearchCriteria11 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTo", skip_serializing_if = "Option::is_none") )]
		pub pmt_to: Option<Vec<SystemSearch5>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtFr", skip_serializing_if = "Option::is_none") )]
		pub pmt_fr: Option<Vec<SystemSearch5>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtSch", skip_serializing_if = "Option::is_none") )]
		pub pmt_sch: Option<PaymentSearch10>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctNtrySch", skip_serializing_if = "Option::is_none") )]
		pub acct_ntry_sch: Option<CashAccountEntrySearch8>,
	}
	
	impl TransactionSearchCriteria11 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref vec) = self.pmt_to { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.pmt_fr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.pmt_sch { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.acct_ntry_sch { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
}