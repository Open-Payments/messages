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
	
	
	// AcknowledgedAcceptedStatus21Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AcknowledgedAcceptedStatus21Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
		pub no_spcfd_rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Vec<AcknowledgementReason9>>,
	}
	
	impl AcknowledgedAcceptedStatus21Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.no_spcfd_rsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// AcknowledgementReason12Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AcknowledgementReason12Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<AcknowledgementReason5Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl AcknowledgementReason12Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AcknowledgementReason5Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum AcknowledgementReason5Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADEA") )]
		CodeADEA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SMPG") )]
		CodeSMPG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CDCY") )]
		CodeCDCY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CDRG") )]
		CodeCDRG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CDRE") )]
		CodeCDRE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NSTP") )]
		CodeNSTP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RQWV") )]
		CodeRQWV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LATE") )]
		CodeLATE,
	}
	
	impl AcknowledgementReason5Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AcknowledgementReason9 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AcknowledgementReason9 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: AcknowledgementReason12Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_rsn_inf: Option<String>,
	}
	
	impl AcknowledgementReason9 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref val) = self.addtl_rsn_inf {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_rsn_inf is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 210 {
					return Err(ValidationError::new(1002, "addtl_rsn_inf exceeds the maximum length of 210".to_string()));
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
	
	
	// CancellationReason19Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CancellationReason19Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<CancelledStatusReason13Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl CancellationReason19Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CancellationReason9 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CancellationReason9 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: CancellationReason19Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_rsn_inf: Option<String>,
	}
	
	impl CancellationReason9 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref val) = self.addtl_rsn_inf {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_rsn_inf is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 210 {
					return Err(ValidationError::new(1002, "addtl_rsn_inf exceeds the maximum length of 210".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// CancellationStatus14Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CancellationStatus14Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
		pub no_spcfd_rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Vec<CancellationReason9>>,
	}
	
	impl CancellationStatus14Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.no_spcfd_rsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// CancelledStatusReason13Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CancelledStatusReason13Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CANI") )]
		CodeCANI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CANS") )]
		CodeCANS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CSUB") )]
		CodeCSUB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CXLR") )]
		CodeCXLR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CANT") )]
		CodeCANT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CANZ") )]
		CodeCANZ,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CORP") )]
		CodeCORP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SCEX") )]
		CodeSCEX,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CTHP") )]
		CodeCTHP,
	}
	
	impl CancelledStatusReason13Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// EventFrequency7Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum EventFrequency7Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
		CodeYEAR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADHO") )]
		CodeADHO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
		CodeMNTH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
		CodeDAIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INDA") )]
		CodeINDA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
		CodeWEEK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SEMI") )]
		CodeSEMI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QUTR") )]
		CodeQUTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TOMN") )]
		CodeTOMN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TOWK") )]
		CodeTOWK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TWMN") )]
		CodeTWMN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OVNG") )]
		CodeOVNG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ONDE") )]
		CodeONDE,
	}
	
	impl EventFrequency7Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// FailingReason3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum FailingReason3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "AWMO") )]
		CodeAWMO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BYIY") )]
		CodeBYIY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLAT") )]
		CodeCLAT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADEA") )]
		CodeADEA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CANR") )]
		CodeCANR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CAIS") )]
		CodeCAIS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OBJT") )]
		CodeOBJT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AWSH") )]
		CodeAWSH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PHSE") )]
		CodePHSE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "STCD") )]
		CodeSTCD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DOCY") )]
		CodeDOCY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MLAT") )]
		CodeMLAT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DOCC") )]
		CodeDOCC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BLOC") )]
		CodeBLOC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CHAS") )]
		CodeCHAS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NEWI") )]
		CodeNEWI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLAC") )]
		CodeCLAC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MUNO") )]
		CodeMUNO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GLOB") )]
		CodeGLOB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PREA") )]
		CodePREA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PART") )]
		CodePART,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOFX") )]
		CodeNOFX,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CMON") )]
		CodeCMON,
		#[cfg_attr( feature = "derive_serde", serde(rename = "YCOL") )]
		CodeYCOL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COLL") )]
		CodeCOLL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DEPO") )]
		CodeDEPO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FLIM") )]
		CodeFLIM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INCA") )]
		CodeINCA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LINK") )]
		CodeLINK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LACK") )]
		CodeLACK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LALO") )]
		CodeLALO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MONY") )]
		CodeMONY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NCON") )]
		CodeNCON,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REFS") )]
		CodeREFS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SDUT") )]
		CodeSDUT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BATC") )]
		CodeBATC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CYCL") )]
		CodeCYCL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SBLO") )]
		CodeSBLO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CPEC") )]
		CodeCPEC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MINO") )]
		CodeMINO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IAAD") )]
		CodeIAAD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PHCK") )]
		CodePHCK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BENO") )]
		CodeBENO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BOTH") )]
		CodeBOTH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLHT") )]
		CodeCLHT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DENO") )]
		CodeDENO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DISA") )]
		CodeDISA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DKNY") )]
		CodeDKNY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FROZ") )]
		CodeFROZ,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LAAW") )]
		CodeLAAW,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LATE") )]
		CodeLATE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LIQU") )]
		CodeLIQU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRCY") )]
		CodePRCY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REGT") )]
		CodeREGT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SETS") )]
		CodeSETS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CERT") )]
		CodeCERT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRSY") )]
		CodePRSY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INBC") )]
		CodeINBC,
	}
	
	impl FailingReason3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// FailingReason7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FailingReason7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: FailingReason7Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_rsn_inf: Option<String>,
	}
	
	impl FailingReason7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref val) = self.addtl_rsn_inf {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_rsn_inf is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 210 {
					return Err(ValidationError::new(1002, "addtl_rsn_inf exceeds the maximum length of 210".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// FailingReason7Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FailingReason7Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<FailingReason3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl FailingReason7Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FailingStatus9Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FailingStatus9Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
		pub no_spcfd_rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Vec<FailingReason7>>,
	}
	
	impl FailingStatus9Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.no_spcfd_rsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	
	
	// Frequency22Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Frequency22Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<EventFrequency7Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl Frequency22Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// IntraBalanceMovementPendingReportV02 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IntraBalanceMovementPendingReportV02 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<DocumentIdentification51>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pgntn") )]
		pub pgntn: Pagination1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptGnlDtls") )]
		pub rpt_gnl_dtls: IntraBalanceReport6,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcct") )]
		pub csh_acct: CashAccount40,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctOwnr", skip_serializing_if = "Option::is_none") )]
		pub csh_acct_ownr: Option<SystemPartyIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctSvcr", skip_serializing_if = "Option::is_none") )]
		pub csh_acct_svcr: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mvmnts", skip_serializing_if = "Option::is_none") )]
		pub mvmnts: Option<Vec<IntraBalancePending5>>,
	}
	
	impl IntraBalanceMovementPendingReportV02 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.id { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.pgntn.validate() { return Err(e); }
			if let Err(e) = self.rpt_gnl_dtls.validate() { return Err(e); }
			if let Err(e) = self.csh_acct.validate() { return Err(e); }
			if let Some(ref val) = self.csh_acct_ownr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.csh_acct_svcr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.mvmnts { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// IntraBalancePending5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IntraBalancePending5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "StsAndRsn", skip_serializing_if = "Option::is_none") )]
		pub sts_and_rsn: Option<PendingStatusAndReason2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mvmnt") )]
		pub mvmnt: Vec<IntraBalancePending6>,
	}
	
	impl IntraBalancePending5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.sts_and_rsn { if let Err(e) = val.validate() { return Err(e); } }
			for item in &self.mvmnt { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// IntraBalancePending6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IntraBalancePending6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "StsAndRsn", skip_serializing_if = "Option::is_none") )]
		pub sts_and_rsn: Option<PendingStatusAndReason2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOwnrTxId") )]
		pub acct_ownr_tx_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none") )]
		pub acct_svcr_tx_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none") )]
		pub mkt_infrstrctr_tx_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrcrTxId", skip_serializing_if = "Option::is_none") )]
		pub prcr_tx_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PoolId", skip_serializing_if = "Option::is_none") )]
		pub pool_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CorpActnEvtId", skip_serializing_if = "Option::is_none") )]
		pub corp_actn_evt_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BalFr") )]
		pub bal_fr: CashSubBalanceTypeAndQuantityBreakdown3,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BalTo") )]
		pub bal_to: CashSubBalanceTypeAndQuantityBreakdown3,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmAmt") )]
		pub sttlm_amt: Amount2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntnddSttlmDt") )]
		pub intndd_sttlm_dt: DateAndDateTime2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StsDt", skip_serializing_if = "Option::is_none") )]
		pub sts_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshSubBalId", skip_serializing_if = "Option::is_none") )]
		pub csh_sub_bal_id: Option<GenericIdentification37>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Lnkgs", skip_serializing_if = "Option::is_none") )]
		pub lnkgs: Option<Vec<Linkages57>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prty", skip_serializing_if = "Option::is_none") )]
		pub prty: Option<PriorityNumeric4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgOrgtr", skip_serializing_if = "Option::is_none") )]
		pub msg_orgtr: Option<SystemPartyIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
		pub cre_dt_tm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstrPrcgAddtlDtls", skip_serializing_if = "Option::is_none") )]
		pub instr_prcg_addtl_dtls: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl IntraBalancePending6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.sts_and_rsn { if let Err(e) = val.validate() { return Err(e); } }
			if self.acct_ownr_tx_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_ownr_tx_id is shorter than the minimum length of 1".to_string()));
			}
			if self.acct_ownr_tx_id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_ownr_tx_id exceeds the maximum length of 35".to_string()));
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
			if let Some(ref val) = self.corp_actn_evt_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "corp_actn_evt_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "corp_actn_evt_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Err(e) = self.bal_fr.validate() { return Err(e); }
			if let Err(e) = self.bal_to.validate() { return Err(e); }
			if let Err(e) = self.sttlm_amt.validate() { return Err(e); }
			if let Err(e) = self.intndd_sttlm_dt.validate() { return Err(e); }
			if let Some(ref val) = self.csh_sub_bal_id { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.lnkgs { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.prty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.msg_orgtr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.instr_prcg_addtl_dtls {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "instr_prcg_addtl_dtls is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "instr_prcg_addtl_dtls exceeds the maximum length of 350".to_string()));
				}
			}
			if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// IntraBalanceReport6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IntraBalanceReport6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptNb", skip_serializing_if = "Option::is_none") )]
		pub rpt_nb: Option<Number3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QryRef", skip_serializing_if = "Option::is_none") )]
		pub qry_ref: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptId", skip_serializing_if = "Option::is_none") )]
		pub rpt_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptDtTm", skip_serializing_if = "Option::is_none") )]
		pub rpt_dt_tm: Option<DateAndDateTime2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptPrd", skip_serializing_if = "Option::is_none") )]
		pub rpt_prd: Option<Period7Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy", skip_serializing_if = "Option::is_none") )]
		pub frqcy: Option<Frequency22Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UpdTp") )]
		pub upd_tp: UpdateType15Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ActvtyInd") )]
		pub actvty_ind: bool,
	}
	
	impl IntraBalanceReport6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.rpt_nb { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.qry_ref {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "qry_ref is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "qry_ref exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.rpt_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "rpt_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "rpt_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.rpt_dt_tm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rpt_prd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.frqcy { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.upd_tp.validate() { return Err(e); }
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
	
	
	// NoReasonCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum NoReasonCode {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NORE") )]
		CodeNORE,
	}
	
	impl NoReasonCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Number3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Number3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Shrt", skip_serializing_if = "Option::is_none") )]
		pub shrt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Lng", skip_serializing_if = "Option::is_none") )]
		pub lng: Option<String>,
	}
	
	impl Number3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.shrt {
				let pattern = Regex::new("[0-9]{3}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "shrt does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.lng {
				let pattern = Regex::new("[0-9]{5}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "lng does not match the required pattern".to_string()));
				}
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
	
	
	// PendingReason10Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PendingReason10Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "AWMO") )]
		CodeAWMO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADEA") )]
		CodeADEA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CAIS") )]
		CodeCAIS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REFU") )]
		CodeREFU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AWSH") )]
		CodeAWSH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PHSE") )]
		CodePHSE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TAMM") )]
		CodeTAMM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DOCY") )]
		CodeDOCY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DOCC") )]
		CodeDOCC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BLOC") )]
		CodeBLOC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CHAS") )]
		CodeCHAS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NEWI") )]
		CodeNEWI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLAC") )]
		CodeCLAC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MUNO") )]
		CodeMUNO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GLOB") )]
		CodeGLOB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PREA") )]
		CodePREA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PART") )]
		CodePART,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NMAS") )]
		CodeNMAS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOFX") )]
		CodeNOFX,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CMON") )]
		CodeCMON,
		#[cfg_attr( feature = "derive_serde", serde(rename = "YCOL") )]
		CodeYCOL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COLL") )]
		CodeCOLL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DEPO") )]
		CodeDEPO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FLIM") )]
		CodeFLIM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INCA") )]
		CodeINCA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LINK") )]
		CodeLINK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FUTU") )]
		CodeFUTU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LACK") )]
		CodeLACK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LALO") )]
		CodeLALO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MONY") )]
		CodeMONY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NCON") )]
		CodeNCON,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REFS") )]
		CodeREFS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SDUT") )]
		CodeSDUT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BATC") )]
		CodeBATC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CYCL") )]
		CodeCYCL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SBLO") )]
		CodeSBLO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CPEC") )]
		CodeCPEC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MINO") )]
		CodeMINO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IAAD") )]
		CodeIAAD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PHCK") )]
		CodePHCK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BENO") )]
		CodeBENO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BOTH") )]
		CodeBOTH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLHT") )]
		CodeCLHT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DENO") )]
		CodeDENO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DISA") )]
		CodeDISA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DKNY") )]
		CodeDKNY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FROZ") )]
		CodeFROZ,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LAAW") )]
		CodeLAAW,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LATE") )]
		CodeLATE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LIQU") )]
		CodeLIQU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRCY") )]
		CodePRCY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REGT") )]
		CodeREGT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SETS") )]
		CodeSETS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CERT") )]
		CodeCERT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRSY") )]
		CodePRSY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INBC") )]
		CodeINBC,
	}
	
	impl PendingReason10Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PendingReason14 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PendingReason14 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: PendingReason26Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_rsn_inf: Option<String>,
	}
	
	impl PendingReason14 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref val) = self.addtl_rsn_inf {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_rsn_inf is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 210 {
					return Err(ValidationError::new(1002, "addtl_rsn_inf exceeds the maximum length of 210".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// PendingReason26Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PendingReason26Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<PendingReason10Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl PendingReason26Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PendingStatus36Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PendingStatus36Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
		pub no_spcfd_rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Vec<PendingReason14>>,
	}
	
	impl PendingStatus36Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.no_spcfd_rsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// PendingStatusAndReason2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PendingStatusAndReason2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgSts", skip_serializing_if = "Option::is_none") )]
		pub prcg_sts: Option<Vec<ProcessingStatus66Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmSts", skip_serializing_if = "Option::is_none") )]
		pub sttlm_sts: Option<Vec<SettlementStatus16Choice>>,
	}
	
	impl PendingStatusAndReason2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref vec) = self.prcg_sts { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.sttlm_sts { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// Period2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Period2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt") )]
		pub fr_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt") )]
		pub to_dt: String,
	}
	
	impl Period2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Period7Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Period7Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtTmToDtTm", skip_serializing_if = "Option::is_none") )]
		pub fr_dt_tm_to_dt_tm: Option<DateTimePeriod1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none") )]
		pub fr_dt_to_dt: Option<Period2>,
	}
	
	impl Period7Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.fr_dt_tm_to_dt_tm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fr_dt_to_dt { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// ProcessingStatus66Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ProcessingStatus66Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AckdAccptd", skip_serializing_if = "Option::is_none") )]
		pub ackd_accptd: Option<AcknowledgedAcceptedStatus21Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rpr", skip_serializing_if = "Option::is_none") )]
		pub rpr: Option<RejectionOrRepairStatus38Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Canc", skip_serializing_if = "Option::is_none") )]
		pub canc: Option<CancellationStatus14Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<ProprietaryStatusAndReason6>,
	}
	
	impl ProcessingStatus66Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.ackd_accptd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rpr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.canc { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ProprietaryReason4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ProprietaryReason4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<GenericIdentification30>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_rsn_inf: Option<String>,
	}
	
	impl ProprietaryReason4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.rsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.addtl_rsn_inf {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_rsn_inf is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 210 {
					return Err(ValidationError::new(1002, "addtl_rsn_inf exceeds the maximum length of 210".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// ProprietaryStatusAndReason6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ProprietaryStatusAndReason6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtrySts") )]
		pub prtry_sts: GenericIdentification30,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryRsn", skip_serializing_if = "Option::is_none") )]
		pub prtry_rsn: Option<Vec<ProprietaryReason4>>,
	}
	
	impl ProprietaryStatusAndReason6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.prtry_sts.validate() { return Err(e); }
			if let Some(ref vec) = self.prtry_rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	
	
	// RejectionAndRepairReason32Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RejectionAndRepairReason32Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<RejectionReason33Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl RejectionAndRepairReason32Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RejectionOrRepairReason32 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RejectionOrRepairReason32 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<Vec<RejectionAndRepairReason32Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_rsn_inf: Option<String>,
	}
	
	impl RejectionOrRepairReason32 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref vec) = self.cd { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.addtl_rsn_inf {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_rsn_inf is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 210 {
					return Err(ValidationError::new(1002, "addtl_rsn_inf exceeds the maximum length of 210".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// RejectionOrRepairStatus38Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RejectionOrRepairStatus38Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
		pub no_spcfd_rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Vec<RejectionOrRepairReason32>>,
	}
	
	impl RejectionOrRepairStatus38Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.no_spcfd_rsn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// RejectionReason33Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum RejectionReason33Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CASH") )]
		CodeCASH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADEA") )]
		CodeADEA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DMON") )]
		CodeDMON,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NCRR") )]
		CodeNCRR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LATE") )]
		CodeLATE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVL") )]
		CodeINVL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVB") )]
		CodeINVB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVN") )]
		CodeINVN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VALR") )]
		CodeVALR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MONY") )]
		CodeMONY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CAEV") )]
		CodeCAEV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DDAT") )]
		CodeDDAT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REFE") )]
		CodeREFE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DQUA") )]
		CodeDQUA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DSEC") )]
		CodeDSEC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MINO") )]
		CodeMINO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MUNO") )]
		CodeMUNO,
	}
	
	impl RejectionReason33Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// SettlementStatus16Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SettlementStatus16Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pdg", skip_serializing_if = "Option::is_none") )]
		pub pdg: Option<PendingStatus36Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Flng", skip_serializing_if = "Option::is_none") )]
		pub flng: Option<FailingStatus9Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<ProprietaryStatusAndReason6>,
	}
	
	impl SettlementStatus16Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.pdg { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.flng { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// StatementUpdateType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum StatementUpdateType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "COMP") )]
		CodeCOMP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DELT") )]
		CodeDELT,
	}
	
	impl StatementUpdateType1Code {
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
	
	
	// UpdateType15Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct UpdateType15Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<StatementUpdateType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl UpdateType15Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
}