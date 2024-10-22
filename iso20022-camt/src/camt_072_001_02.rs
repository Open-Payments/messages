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
	
	
	// Amount2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Amount2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AmtWthtCcy", skip_serializing_if = "Option::is_none") )]
		pub amt_wtht_ccy: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AmtWthCcy", skip_serializing_if = "Option::is_none") )]
		pub amt_wth_ccy: Option<ActiveCurrencyAndAmount>,
	}
	
	impl Amount2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.amt_wtht_ccy {
				if *val < 0.000000 {
					return Err(ValidationError::new(1003, "amt_wtht_ccy is less than the minimum value of 0.000000".to_string()));
				}
			}
			if let Some(ref val) = self.amt_wth_ccy { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AmountAndDirection5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AmountAndDirection5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none") )]
		pub cdt_dbt: Option<CreditDebitCode>,
	}
	
	impl AmountAndDirection5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Some(ref val) = self.cdt_dbt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AmountAndQuantityBreakdown1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AmountAndQuantityBreakdown1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "LotNb", skip_serializing_if = "Option::is_none") )]
		pub lot_nb: Option<GenericIdentification37>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LotAmt", skip_serializing_if = "Option::is_none") )]
		pub lot_amt: Option<AmountAndDirection5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LotQty", skip_serializing_if = "Option::is_none") )]
		pub lot_qty: Option<FinancialInstrumentQuantity1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshSubBalTp", skip_serializing_if = "Option::is_none") )]
		pub csh_sub_bal_tp: Option<GenericIdentification30>,
	}
	
	impl AmountAndQuantityBreakdown1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.lot_nb { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.lot_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.lot_qty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.csh_sub_bal_tp { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// CashAccount40 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CashAccount40 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<AccountIdentification4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<CashAccountType2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
		pub ccy: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prxy", skip_serializing_if = "Option::is_none") )]
		pub prxy: Option<ProxyAccountIdentification1>,
	}
	
	impl CashAccount40 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.id { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// CashBalanceType3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CashBalanceType3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl CashBalanceType3Choice {
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
	
	
	// CashSubBalanceTypeAndQuantityBreakdown3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CashSubBalanceTypeAndQuantityBreakdown3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: CashBalanceType3Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QtyBrkdwn", skip_serializing_if = "Option::is_none") )]
		pub qty_brkdwn: Option<Vec<AmountAndQuantityBreakdown1>>,
	}
	
	impl CashSubBalanceTypeAndQuantityBreakdown3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Some(ref vec) = self.qty_brkdwn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	
	
	// CopyDuplicate1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CopyDuplicate1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CODU") )]
		CodeCODU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COPY") )]
		CodeCOPY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DUPL") )]
		CodeDUPL,
	}
	
	impl CopyDuplicate1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// DocumentIdentification51 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentIdentification51 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
		pub cre_dt_tm: Option<DateAndDateTime2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CpyDplct", skip_serializing_if = "Option::is_none") )]
		pub cpy_dplct: Option<CopyDuplicate1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgOrgtr", skip_serializing_if = "Option::is_none") )]
		pub msg_orgtr: Option<PartyIdentification136>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgRcpt", skip_serializing_if = "Option::is_none") )]
		pub msg_rcpt: Option<PartyIdentification136>,
	}
	
	impl DocumentIdentification51 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.cre_dt_tm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.cpy_dplct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.msg_orgtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.msg_rcpt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DocumentNumber5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentNumber5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNb", skip_serializing_if = "Option::is_none") )]
		pub shrt_nb: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LngNb", skip_serializing_if = "Option::is_none") )]
		pub lng_nb: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryNb", skip_serializing_if = "Option::is_none") )]
		pub prtry_nb: Option<GenericIdentification36>,
	}
	
	impl DocumentNumber5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.shrt_nb {
				let pattern = Regex::new("[0-9]{3}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "shrt_nb does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.lng_nb {
				let pattern = Regex::new("[a-z]{4}\\.[0-9]{3}\\.[0-9]{3}\\.[0-9]{2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "lng_nb does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.prtry_nb { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// FinancialInstrumentQuantity1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FinancialInstrumentQuantity1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
		pub unit: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none") )]
		pub face_amt: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none") )]
		pub amtsd_val: Option<f64>,
	}
	
	impl FinancialInstrumentQuantity1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.face_amt {
				if *val < 0.000000 {
					return Err(ValidationError::new(1003, "face_amt is less than the minimum value of 0.000000".to_string()));
				}
			}
			if let Some(ref val) = self.amtsd_val {
				if *val < 0.000000 {
					return Err(ValidationError::new(1003, "amtsd_val is less than the minimum value of 0.000000".to_string()));
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
	
	
	// GenericIdentification37 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericIdentification37 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<String>,
	}
	
	impl GenericIdentification37 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
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
	
	
	// IntraBalance5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IntraBalance5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmAmt") )]
		pub sttlm_amt: Amount2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmDt") )]
		pub sttlm_dt: DateAndDateTime2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BalFr") )]
		pub bal_fr: CashSubBalanceTypeAndQuantityBreakdown3,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BalTo") )]
		pub bal_to: CashSubBalanceTypeAndQuantityBreakdown3,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshSubBalId", skip_serializing_if = "Option::is_none") )]
		pub csh_sub_bal_id: Option<GenericIdentification37>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prty", skip_serializing_if = "Option::is_none") )]
		pub prty: Option<PriorityNumeric4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstrPrcgAddtlDtls", skip_serializing_if = "Option::is_none") )]
		pub instr_prcg_addtl_dtls: Option<String>,
	}
	
	impl IntraBalance5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.sttlm_amt.validate() { return Err(e); }
			if let Err(e) = self.sttlm_dt.validate() { return Err(e); }
			if let Err(e) = self.bal_fr.validate() { return Err(e); }
			if let Err(e) = self.bal_to.validate() { return Err(e); }
			if let Some(ref val) = self.csh_sub_bal_id { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.instr_prcg_addtl_dtls {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "instr_prcg_addtl_dtls is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "instr_prcg_addtl_dtls exceeds the maximum length of 350".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// IntraBalanceMovementModificationRequestV02 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IntraBalanceMovementModificationRequestV02 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<DocumentIdentification51>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcct") )]
		pub csh_acct: CashAccount40,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctOwnr", skip_serializing_if = "Option::is_none") )]
		pub csh_acct_ownr: Option<SystemPartyIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctSvcr", skip_serializing_if = "Option::is_none") )]
		pub csh_acct_svcr: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ReqDtls") )]
		pub req_dtls: Vec<RequestDetails22>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygIntraBal", skip_serializing_if = "Option::is_none") )]
		pub undrlyg_intra_bal: Option<IntraBalance5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl IntraBalanceMovementModificationRequestV02 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.id { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.csh_acct.validate() { return Err(e); }
			if let Some(ref val) = self.csh_acct_ownr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.csh_acct_svcr { if let Err(e) = val.validate() { return Err(e); } }
			for item in &self.req_dtls { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref val) = self.undrlyg_intra_bal { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// LinkageType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum LinkageType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "LINK") )]
		CodeLINK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UNLK") )]
		CodeUNLK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SOFT") )]
		CodeSOFT,
	}
	
	impl LinkageType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// LinkageType3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct LinkageType3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<LinkageType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl LinkageType3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Linkages57 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Linkages57 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgPos", skip_serializing_if = "Option::is_none") )]
		pub prcg_pos: Option<ProcessingPosition7Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNb", skip_serializing_if = "Option::is_none") )]
		pub msg_nb: Option<DocumentNumber5Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
		pub ref_attr: References34Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefOwnr", skip_serializing_if = "Option::is_none") )]
		pub ref_ownr: Option<PartyIdentification127Choice>,
	}
	
	impl Linkages57 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.prcg_pos { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.msg_nb { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.ref_attr.validate() { return Err(e); }
			if let Some(ref val) = self.ref_ownr { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// PartyIdentification127Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PartyIdentification127Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<GenericIdentification36>,
	}
	
	impl PartyIdentification127Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.any_bic {
				let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.prtry_id { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// PriorityNumeric4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PriorityNumeric4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nmrc", skip_serializing_if = "Option::is_none") )]
		pub nmrc: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl PriorityNumeric4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.nmrc {
				let pattern = Regex::new("[0-9]{4}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "nmrc does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ProcessingPosition3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ProcessingPosition3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "AFTE") )]
		CodeAFTE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WITH") )]
		CodeWITH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BEFO") )]
		CodeBEFO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INFO") )]
		CodeINFO,
	}
	
	impl ProcessingPosition3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ProcessingPosition7Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ProcessingPosition7Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ProcessingPosition3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl ProcessingPosition7Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// References14 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct References14 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOwnrTxId", skip_serializing_if = "Option::is_none") )]
		pub acct_ownr_tx_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none") )]
		pub acct_svcr_tx_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none") )]
		pub mkt_infrstrctr_tx_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrcrTxId", skip_serializing_if = "Option::is_none") )]
		pub prcr_tx_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PoolId", skip_serializing_if = "Option::is_none") )]
		pub pool_id: Option<String>,
	}
	
	impl References14 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.acct_ownr_tx_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "acct_ownr_tx_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "acct_ownr_tx_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.acct_svcr_tx_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "acct_svcr_tx_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "acct_svcr_tx_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.mkt_infrstrctr_tx_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "mkt_infrstrctr_tx_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "mkt_infrstrctr_tx_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.prcr_tx_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "prcr_tx_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "prcr_tx_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.pool_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "pool_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "pool_id exceeds the maximum length of 35".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// References34Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct References34Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesSttlmTxId", skip_serializing_if = "Option::is_none") )]
		pub scties_sttlm_tx_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntraPosMvmntId", skip_serializing_if = "Option::is_none") )]
		pub intra_pos_mvmnt_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntraBalMvmntId", skip_serializing_if = "Option::is_none") )]
		pub intra_bal_mvmnt_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none") )]
		pub acct_svcr_tx_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none") )]
		pub mkt_infrstrctr_tx_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PoolId", skip_serializing_if = "Option::is_none") )]
		pub pool_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrTxId", skip_serializing_if = "Option::is_none") )]
		pub othr_tx_id: Option<String>,
	}
	
	impl References34Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.scties_sttlm_tx_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "scties_sttlm_tx_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "scties_sttlm_tx_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.intra_pos_mvmnt_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "intra_pos_mvmnt_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "intra_pos_mvmnt_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.intra_bal_mvmnt_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "intra_bal_mvmnt_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "intra_bal_mvmnt_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.acct_svcr_tx_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "acct_svcr_tx_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "acct_svcr_tx_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.mkt_infrstrctr_tx_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "mkt_infrstrctr_tx_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "mkt_infrstrctr_tx_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.pool_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "pool_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "pool_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.othr_tx_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "othr_tx_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "othr_tx_id exceeds the maximum length of 35".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// RequestDetails22 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RequestDetails22 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
		pub ref_attr: References14,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Lkg", skip_serializing_if = "Option::is_none") )]
		pub lkg: Option<LinkageType3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prty", skip_serializing_if = "Option::is_none") )]
		pub prty: Option<PriorityNumeric4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPrcg", skip_serializing_if = "Option::is_none") )]
		pub othr_prcg: Option<Vec<GenericIdentification30>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtlSttlmInd", skip_serializing_if = "Option::is_none") )]
		pub prtl_sttlm_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrChanl", skip_serializing_if = "Option::is_none") )]
		pub clr_chanl: Option<ClearingChannel2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Lnkgs", skip_serializing_if = "Option::is_none") )]
		pub lnkgs: Option<Vec<Linkages57>>,
	}
	
	impl RequestDetails22 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ref_attr.validate() { return Err(e); }
			if let Some(ref val) = self.lkg { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.othr_prcg { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.clr_chanl { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.lnkgs { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	
}