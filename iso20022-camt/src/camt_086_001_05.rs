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
	
	
	// AccountLevel1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum AccountLevel1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "INTM") )]
		CodeINTM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SMRY") )]
		CodeSMRY,
	}
	
	impl AccountLevel1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AccountLevel2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum AccountLevel2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "INTM") )]
		CodeINTM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SMRY") )]
		CodeSMRY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DETL") )]
		CodeDETL,
	}
	
	impl AccountLevel2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// AccountTax1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountTax1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClctnMtd") )]
		pub clctn_mtd: BillingTaxCalculationMethod1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rgn", skip_serializing_if = "Option::is_none") )]
		pub rgn: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonResCtry", skip_serializing_if = "Option::is_none") )]
		pub non_res_ctry: Option<ResidenceLocation1Choice>,
	}
	
	impl AccountTax1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.clctn_mtd.validate() { return Err(e); }
			if let Some(ref val) = self.rgn {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "rgn is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 40 {
					return Err(ValidationError::new(1002, "rgn exceeds the maximum length of 40".to_string()));
				}
			}
			if let Some(ref val) = self.non_res_ctry { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// AmountAndDirection34 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AmountAndDirection34 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sgn") )]
		pub sgn: bool,
	}
	
	impl AmountAndDirection34 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BalanceAdjustment1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BalanceAdjustment1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: BalanceAdjustmentType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc") )]
		pub desc: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BalAmt") )]
		pub bal_amt: AmountAndDirection34,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AvrgAmt", skip_serializing_if = "Option::is_none") )]
		pub avrg_amt: Option<AmountAndDirection34>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ErrDt", skip_serializing_if = "Option::is_none") )]
		pub err_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstngDt") )]
		pub pstng_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Days", skip_serializing_if = "Option::is_none") )]
		pub days: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EarngsAdjstmntAmt", skip_serializing_if = "Option::is_none") )]
		pub earngs_adjstmnt_amt: Option<AmountAndDirection34>,
	}
	
	impl BalanceAdjustment1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if self.desc.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if self.desc.chars().count() > 105 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 105".to_string()));
			}
			if let Err(e) = self.bal_amt.validate() { return Err(e); }
			if let Some(ref val) = self.avrg_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.earngs_adjstmnt_amt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BalanceAdjustmentType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum BalanceAdjustmentType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "LDGR") )]
		CodeLDGR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FLOT") )]
		CodeFLOT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLLD") )]
		CodeCLLD,
	}
	
	impl BalanceAdjustmentType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BankServicesBillingStatementV05 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BankServicesBillingStatementV05 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptHdr") )]
		pub rpt_hdr: ReportHeader6,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BllgStmtGrp") )]
		pub bllg_stmt_grp: Vec<StatementGroup5>,
	}
	
	impl BankServicesBillingStatementV05 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rpt_hdr.validate() { return Err(e); }
			for item in &self.bllg_stmt_grp { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BankTransactionCodeStructure4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BankTransactionCodeStructure4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Domn", skip_serializing_if = "Option::is_none") )]
		pub domn: Option<BankTransactionCodeStructure5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<ProprietaryBankTransactionCodeStructure1>,
	}
	
	impl BankTransactionCodeStructure4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.domn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BankTransactionCodeStructure5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BankTransactionCodeStructure5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Fmly") )]
		pub fmly: BankTransactionCodeStructure6,
	}
	
	impl BankTransactionCodeStructure5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.cd.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if self.cd.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
			if let Err(e) = self.fmly.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BankTransactionCodeStructure6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BankTransactionCodeStructure6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubFmlyCd") )]
		pub sub_fmly_cd: String,
	}
	
	impl BankTransactionCodeStructure6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.cd.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if self.cd.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
			if self.sub_fmly_cd.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sub_fmly_cd is shorter than the minimum length of 1".to_string()));
			}
			if self.sub_fmly_cd.chars().count() > 4 {
				return Err(ValidationError::new(1002, "sub_fmly_cd exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// BillingBalance1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingBalance1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: BillingBalanceType1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
		pub val: AmountAndDirection34,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CcyTp", skip_serializing_if = "Option::is_none") )]
		pub ccy_tp: Option<BillingCurrencyType1Code>,
	}
	
	impl BillingBalance1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Err(e) = self.val.validate() { return Err(e); }
			if let Some(ref val) = self.ccy_tp { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingBalanceType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingBalanceType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl BillingBalanceType1Choice {
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
	
	
	// BillingChargeMethod1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum BillingChargeMethod1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "UPRC") )]
		CodeUPRC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "STAM") )]
		CodeSTAM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BCHG") )]
		CodeBCHG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DPRC") )]
		CodeDPRC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FCHG") )]
		CodeFCHG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LPRC") )]
		CodeLPRC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MCHG") )]
		CodeMCHG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MXRD") )]
		CodeMXRD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TIR1") )]
		CodeTIR1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TIR2") )]
		CodeTIR2,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TIR3") )]
		CodeTIR3,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TIR4") )]
		CodeTIR4,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TIR5") )]
		CodeTIR5,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TIR6") )]
		CodeTIR6,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TIR7") )]
		CodeTIR7,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TIR8") )]
		CodeTIR8,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TIR9") )]
		CodeTIR9,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TPRC") )]
		CodeTPRC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ZPRC") )]
		CodeZPRC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BBSE") )]
		CodeBBSE,
	}
	
	impl BillingChargeMethod1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BillingCompensation1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingCompensation1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: BillingCompensationType1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
		pub val: AmountAndDirection34,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CcyTp", skip_serializing_if = "Option::is_none") )]
		pub ccy_tp: Option<BillingCurrencyType2Code>,
	}
	
	impl BillingCompensation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Err(e) = self.val.validate() { return Err(e); }
			if let Some(ref val) = self.ccy_tp { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingCompensationType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingCompensationType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl BillingCompensationType1Choice {
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
	
	
	// BillingCurrencyType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum BillingCurrencyType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACCT") )]
		CodeACCT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "STLM") )]
		CodeSTLM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRCG") )]
		CodePRCG,
	}
	
	impl BillingCurrencyType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BillingCurrencyType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum BillingCurrencyType2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACCT") )]
		CodeACCT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "STLM") )]
		CodeSTLM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRCG") )]
		CodePRCG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HOST") )]
		CodeHOST,
	}
	
	impl BillingCurrencyType2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BillingMethod1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingMethod1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvcChrgHstAmt") )]
		pub svc_chrg_hst_amt: AmountAndDirection34,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvcTax") )]
		pub svc_tax: BillingServicesAmount1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlChrg") )]
		pub ttl_chrg: BillingServicesAmount2,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxId") )]
		pub tax_id: Vec<BillingServicesTax1>,
	}
	
	impl BillingMethod1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.svc_chrg_hst_amt.validate() { return Err(e); }
			if let Err(e) = self.svc_tax.validate() { return Err(e); }
			if let Err(e) = self.ttl_chrg.validate() { return Err(e); }
			for item in &self.tax_id { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingMethod1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingMethod1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MtdA", skip_serializing_if = "Option::is_none") )]
		pub mtd_a: Option<BillingMethod1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MtdB", skip_serializing_if = "Option::is_none") )]
		pub mtd_b: Option<BillingMethod2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MtdD", skip_serializing_if = "Option::is_none") )]
		pub mtd_d: Option<BillingMethod3>,
	}
	
	impl BillingMethod1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.mtd_a { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.mtd_b { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.mtd_d { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingMethod2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingMethod2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvcChrgHstAmt") )]
		pub svc_chrg_hst_amt: AmountAndDirection34,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvcTax") )]
		pub svc_tax: BillingServicesAmount1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxId") )]
		pub tax_id: Vec<BillingServicesTax1>,
	}
	
	impl BillingMethod2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.svc_chrg_hst_amt.validate() { return Err(e); }
			if let Err(e) = self.svc_tax.validate() { return Err(e); }
			for item in &self.tax_id { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingMethod3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingMethod3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvcTaxPricAmt") )]
		pub svc_tax_pric_amt: AmountAndDirection34,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxId") )]
		pub tax_id: Vec<BillingServicesTax2>,
	}
	
	impl BillingMethod3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.svc_tax_pric_amt.validate() { return Err(e); }
			for item in &self.tax_id { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingMethod4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingMethod4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvcDtl") )]
		pub svc_dtl: Vec<BillingServiceParameters2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxClctn") )]
		pub tax_clctn: TaxCalculation1,
	}
	
	impl BillingMethod4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.svc_dtl { if let Err(e) = item.validate() { return Err(e); } }
			if let Err(e) = self.tax_clctn.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BillingPrice1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingPrice1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
		pub ccy: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitPric", skip_serializing_if = "Option::is_none") )]
		pub unit_pric: Option<AmountAndDirection34>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mtd", skip_serializing_if = "Option::is_none") )]
		pub mtd: Option<BillingChargeMethod1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rule", skip_serializing_if = "Option::is_none") )]
		pub rule: Option<String>,
	}
	
	impl BillingPrice1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.ccy {
				let pattern = Regex::new("[A-Z]{3,3}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.unit_pric { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.mtd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rule {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "rule is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 20 {
					return Err(ValidationError::new(1002, "rule exceeds the maximum length of 20".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// BillingRate1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingRate1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: BillingRateIdentification1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
		pub val: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DaysInPrd", skip_serializing_if = "Option::is_none") )]
		pub days_in_prd: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DaysInYr", skip_serializing_if = "Option::is_none") )]
		pub days_in_yr: Option<f64>,
	}
	
	impl BillingRate1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BillingRateIdentification1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingRateIdentification1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl BillingRateIdentification1Choice {
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
	
	
	// BillingService2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingService2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvcDtl") )]
		pub svc_dtl: BillingServiceParameters3,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pric", skip_serializing_if = "Option::is_none") )]
		pub pric: Option<BillingPrice1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtd") )]
		pub pmt_mtd: ServicePaymentMethod1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlChrgPric") )]
		pub orgnl_chrg_pric: AmountAndDirection34,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlChrgSttlmAmt", skip_serializing_if = "Option::is_none") )]
		pub orgnl_chrg_sttlm_amt: Option<AmountAndDirection34>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BalReqrdAcctAmt", skip_serializing_if = "Option::is_none") )]
		pub bal_reqrd_acct_amt: Option<AmountAndDirection34>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxDsgnt") )]
		pub tax_dsgnt: ServiceTaxDesignation1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxClctn", skip_serializing_if = "Option::is_none") )]
		pub tax_clctn: Option<BillingMethod1Choice>,
	}
	
	impl BillingService2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.svc_dtl.validate() { return Err(e); }
			if let Some(ref val) = self.pric { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.pmt_mtd.validate() { return Err(e); }
			if let Err(e) = self.orgnl_chrg_pric.validate() { return Err(e); }
			if let Some(ref val) = self.orgnl_chrg_sttlm_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.bal_reqrd_acct_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.tax_dsgnt.validate() { return Err(e); }
			if let Some(ref val) = self.tax_clctn { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingServiceAdjustment1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingServiceAdjustment1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: ServiceAdjustmentType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc") )]
		pub desc: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: AmountAndDirection34,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BalReqrdAmt", skip_serializing_if = "Option::is_none") )]
		pub bal_reqrd_amt: Option<AmountAndDirection34>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ErrDt", skip_serializing_if = "Option::is_none") )]
		pub err_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdjstmntId", skip_serializing_if = "Option::is_none") )]
		pub adjstmnt_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubSvc", skip_serializing_if = "Option::is_none") )]
		pub sub_svc: Option<BillingSubServiceIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PricChng", skip_serializing_if = "Option::is_none") )]
		pub pric_chng: Option<AmountAndDirection34>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlPric", skip_serializing_if = "Option::is_none") )]
		pub orgnl_pric: Option<AmountAndDirection34>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NewPric", skip_serializing_if = "Option::is_none") )]
		pub new_pric: Option<AmountAndDirection34>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VolChng", skip_serializing_if = "Option::is_none") )]
		pub vol_chng: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlVol", skip_serializing_if = "Option::is_none") )]
		pub orgnl_vol: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NewVol", skip_serializing_if = "Option::is_none") )]
		pub new_vol: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlChrgAmt", skip_serializing_if = "Option::is_none") )]
		pub orgnl_chrg_amt: Option<AmountAndDirection34>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NewChrgAmt", skip_serializing_if = "Option::is_none") )]
		pub new_chrg_amt: Option<AmountAndDirection34>,
	}
	
	impl BillingServiceAdjustment1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if self.desc.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if self.desc.chars().count() > 140 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 140".to_string()));
			}
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Some(ref val) = self.bal_reqrd_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.adjstmnt_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "adjstmnt_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "adjstmnt_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.sub_svc { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.pric_chng { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.orgnl_pric { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.new_pric { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.orgnl_chrg_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.new_chrg_amt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingServiceCommonIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingServiceCommonIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
	}
	
	impl BillingServiceCommonIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.issr.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if self.issr.chars().count() > 6 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 6".to_string()));
			}
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 8 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 8".to_string()));
			}
			Ok(())
		}
	}
	
	
	// BillingServiceIdentification2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingServiceIdentification2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubSvc", skip_serializing_if = "Option::is_none") )]
		pub sub_svc: Option<BillingSubServiceIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc") )]
		pub desc: String,
	}
	
	impl BillingServiceIdentification2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.sub_svc { if let Err(e) = val.validate() { return Err(e); } }
			if self.desc.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if self.desc.chars().count() > 70 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 70".to_string()));
			}
			Ok(())
		}
	}
	
	
	// BillingServiceIdentification3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingServiceIdentification3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubSvc", skip_serializing_if = "Option::is_none") )]
		pub sub_svc: Option<BillingSubServiceIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc") )]
		pub desc: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CmonCd", skip_serializing_if = "Option::is_none") )]
		pub cmon_cd: Option<BillingServiceCommonIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BkTxCd", skip_serializing_if = "Option::is_none") )]
		pub bk_tx_cd: Option<BankTransactionCodeStructure4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvcTp", skip_serializing_if = "Option::is_none") )]
		pub svc_tp: Option<String>,
	}
	
	impl BillingServiceIdentification3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.sub_svc { if let Err(e) = val.validate() { return Err(e); } }
			if self.desc.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if self.desc.chars().count() > 70 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 70".to_string()));
			}
			if let Some(ref val) = self.cmon_cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.bk_tx_cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.svc_tp {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "svc_tp is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 12 {
					return Err(ValidationError::new(1002, "svc_tp exceeds the maximum length of 12".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// BillingServiceParameters2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingServiceParameters2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BkSvc") )]
		pub bk_svc: BillingServiceIdentification2,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Vol", skip_serializing_if = "Option::is_none") )]
		pub vol: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitPric", skip_serializing_if = "Option::is_none") )]
		pub unit_pric: Option<AmountAndDirection34>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvcChrgAmt") )]
		pub svc_chrg_amt: AmountAndDirection34,
	}
	
	impl BillingServiceParameters2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.bk_svc.validate() { return Err(e); }
			if let Some(ref val) = self.unit_pric { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.svc_chrg_amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BillingServiceParameters3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingServiceParameters3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BkSvc") )]
		pub bk_svc: BillingServiceIdentification3,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Vol", skip_serializing_if = "Option::is_none") )]
		pub vol: Option<f64>,
	}
	
	impl BillingServiceParameters3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.bk_svc.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BillingServicesAmount1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingServicesAmount1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "HstAmt") )]
		pub hst_amt: AmountAndDirection34,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PricgAmt", skip_serializing_if = "Option::is_none") )]
		pub pricg_amt: Option<AmountAndDirection34>,
	}
	
	impl BillingServicesAmount1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.hst_amt.validate() { return Err(e); }
			if let Some(ref val) = self.pricg_amt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingServicesAmount2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingServicesAmount2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "HstAmt") )]
		pub hst_amt: AmountAndDirection34,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmAmt", skip_serializing_if = "Option::is_none") )]
		pub sttlm_amt: Option<AmountAndDirection34>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PricgAmt", skip_serializing_if = "Option::is_none") )]
		pub pricg_amt: Option<AmountAndDirection34>,
	}
	
	impl BillingServicesAmount2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.hst_amt.validate() { return Err(e); }
			if let Some(ref val) = self.sttlm_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.pricg_amt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingServicesAmount3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingServicesAmount3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SrcAmt") )]
		pub src_amt: AmountAndDirection34,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HstAmt") )]
		pub hst_amt: AmountAndDirection34,
	}
	
	impl BillingServicesAmount3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.src_amt.validate() { return Err(e); }
			if let Err(e) = self.hst_amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BillingServicesTax1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingServicesTax1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nb") )]
		pub nb: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rate") )]
		pub rate: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HstAmt") )]
		pub hst_amt: AmountAndDirection34,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PricgAmt", skip_serializing_if = "Option::is_none") )]
		pub pricg_amt: Option<AmountAndDirection34>,
	}
	
	impl BillingServicesTax1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.nb.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nb is shorter than the minimum length of 1".to_string()));
			}
			if self.nb.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nb exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.desc {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 40 {
					return Err(ValidationError::new(1002, "desc exceeds the maximum length of 40".to_string()));
				}
			}
			if let Err(e) = self.hst_amt.validate() { return Err(e); }
			if let Some(ref val) = self.pricg_amt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingServicesTax2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingServicesTax2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nb") )]
		pub nb: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rate") )]
		pub rate: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PricgAmt") )]
		pub pricg_amt: AmountAndDirection34,
	}
	
	impl BillingServicesTax2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.nb.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nb is shorter than the minimum length of 1".to_string()));
			}
			if self.nb.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nb exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.desc {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 40 {
					return Err(ValidationError::new(1002, "desc exceeds the maximum length of 40".to_string()));
				}
			}
			if let Err(e) = self.pricg_amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BillingServicesTax3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingServicesTax3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nb") )]
		pub nb: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rate") )]
		pub rate: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlTaxAmt") )]
		pub ttl_tax_amt: AmountAndDirection34,
	}
	
	impl BillingServicesTax3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.nb.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nb is shorter than the minimum length of 1".to_string()));
			}
			if self.nb.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nb exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.desc {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 40 {
					return Err(ValidationError::new(1002, "desc exceeds the maximum length of 40".to_string()));
				}
			}
			if let Err(e) = self.ttl_tax_amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BillingStatement5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingStatement5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "StmtId") )]
		pub stmt_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDt") )]
		pub fr_to_dt: DatePeriod1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
		pub cre_dt_tm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
		pub sts: BillingStatementStatus1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctChrtcs") )]
		pub acct_chrtcs: CashAccountCharacteristics5,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RateData", skip_serializing_if = "Option::is_none") )]
		pub rate_data: Option<Vec<BillingRate1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CcyXchg", skip_serializing_if = "Option::is_none") )]
		pub ccy_xchg: Option<Vec<CurrencyExchange6>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Bal", skip_serializing_if = "Option::is_none") )]
		pub bal: Option<Vec<BillingBalance1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Compstn", skip_serializing_if = "Option::is_none") )]
		pub compstn: Option<Vec<BillingCompensation1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Svc", skip_serializing_if = "Option::is_none") )]
		pub svc: Option<Vec<BillingService2>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRgn", skip_serializing_if = "Option::is_none") )]
		pub tax_rgn: Option<Vec<BillingTaxRegion3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BalAdjstmnt", skip_serializing_if = "Option::is_none") )]
		pub bal_adjstmnt: Option<Vec<BalanceAdjustment1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvcAdjstmnt", skip_serializing_if = "Option::is_none") )]
		pub svc_adjstmnt: Option<Vec<BillingServiceAdjustment1>>,
	}
	
	impl BillingStatement5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.stmt_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "stmt_id is shorter than the minimum length of 1".to_string()));
			}
			if self.stmt_id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "stmt_id exceeds the maximum length of 35".to_string()));
			}
			if let Err(e) = self.fr_to_dt.validate() { return Err(e); }
			if let Err(e) = self.sts.validate() { return Err(e); }
			if let Err(e) = self.acct_chrtcs.validate() { return Err(e); }
			if let Some(ref vec) = self.rate_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.ccy_xchg { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.bal { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.compstn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.svc { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.tax_rgn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.bal_adjstmnt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.svc_adjstmnt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// BillingStatementStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum BillingStatementStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ORGN") )]
		CodeORGN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RPLC") )]
		CodeRPLC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TEST") )]
		CodeTEST,
	}
	
	impl BillingStatementStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BillingSubServiceIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingSubServiceIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: BillingSubServiceQualifier1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
	}
	
	impl BillingSubServiceIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.issr.validate() { return Err(e); }
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
			Ok(())
		}
	}
	
	
	// BillingSubServiceQualifier1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingSubServiceQualifier1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<BillingSubServiceQualifier1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl BillingSubServiceQualifier1Choice {
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
	
	
	// BillingSubServiceQualifier1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum BillingSubServiceQualifier1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "LBOX") )]
		CodeLBOX,
		#[cfg_attr( feature = "derive_serde", serde(rename = "STOR") )]
		CodeSTOR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BILA") )]
		CodeBILA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SEQN") )]
		CodeSEQN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MACT") )]
		CodeMACT,
	}
	
	impl BillingSubServiceQualifier1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BillingTaxCalculationMethod1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum BillingTaxCalculationMethod1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NTAX") )]
		CodeNTAX,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MTDA") )]
		CodeMTDA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MTDB") )]
		CodeMTDB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MTDC") )]
		CodeMTDC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MTDD") )]
		CodeMTDD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UDFD") )]
		CodeUDFD,
	}
	
	impl BillingTaxCalculationMethod1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BillingTaxIdentification3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingTaxIdentification3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "VATRegnNb", skip_serializing_if = "Option::is_none") )]
		pub vat_regn_nb: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRegnNb", skip_serializing_if = "Option::is_none") )]
		pub tax_regn_nb: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxCtct", skip_serializing_if = "Option::is_none") )]
		pub tax_ctct: Option<Contact13>,
	}
	
	impl BillingTaxIdentification3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.vat_regn_nb {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "vat_regn_nb is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "vat_regn_nb exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.tax_regn_nb {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tax_regn_nb is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "tax_regn_nb exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.tax_ctct { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingTaxRegion3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BillingTaxRegion3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RgnNb") )]
		pub rgn_nb: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RgnNm") )]
		pub rgn_nm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CstmrTaxId") )]
		pub cstmr_tax_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PtDt", skip_serializing_if = "Option::is_none") )]
		pub pt_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SndgFI", skip_serializing_if = "Option::is_none") )]
		pub sndg_fi: Option<BillingTaxIdentification3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InvcNb", skip_serializing_if = "Option::is_none") )]
		pub invc_nb: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MtdC", skip_serializing_if = "Option::is_none") )]
		pub mtd_c: Option<BillingMethod4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmAmt") )]
		pub sttlm_amt: AmountAndDirection34,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxDueToRgn") )]
		pub tax_due_to_rgn: AmountAndDirection34,
	}
	
	impl BillingTaxRegion3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.rgn_nb.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rgn_nb is shorter than the minimum length of 1".to_string()));
			}
			if self.rgn_nb.chars().count() > 40 {
				return Err(ValidationError::new(1002, "rgn_nb exceeds the maximum length of 40".to_string()));
			}
			if self.rgn_nm.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rgn_nm is shorter than the minimum length of 1".to_string()));
			}
			if self.rgn_nm.chars().count() > 40 {
				return Err(ValidationError::new(1002, "rgn_nm exceeds the maximum length of 40".to_string()));
			}
			if self.cstmr_tax_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cstmr_tax_id is shorter than the minimum length of 1".to_string()));
			}
			if self.cstmr_tax_id.chars().count() > 40 {
				return Err(ValidationError::new(1002, "cstmr_tax_id exceeds the maximum length of 40".to_string()));
			}
			if let Some(ref val) = self.sndg_fi { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.invc_nb {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "invc_nb is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 40 {
					return Err(ValidationError::new(1002, "invc_nb exceeds the maximum length of 40".to_string()));
				}
			}
			if let Some(ref val) = self.mtd_c { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.sttlm_amt.validate() { return Err(e); }
			if let Err(e) = self.tax_due_to_rgn.validate() { return Err(e); }
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
	
	
	// CashAccountCharacteristics5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CashAccountCharacteristics5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctLvl") )]
		pub acct_lvl: AccountLevel2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcct") )]
		pub csh_acct: CashAccount40,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none") )]
		pub acct_svcr: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrntAcct", skip_serializing_if = "Option::is_none") )]
		pub prnt_acct: Option<ParentCashAccount5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CompstnMtd") )]
		pub compstn_mtd: CompensationMethod1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtAcct", skip_serializing_if = "Option::is_none") )]
		pub dbt_acct: Option<AccountIdentification4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DelydDbtDt", skip_serializing_if = "Option::is_none") )]
		pub delyd_dbt_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmAdvc", skip_serializing_if = "Option::is_none") )]
		pub sttlm_advc: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctBalCcyCd") )]
		pub acct_bal_ccy_cd: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCcyCd", skip_serializing_if = "Option::is_none") )]
		pub sttlm_ccy_cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HstCcyCd", skip_serializing_if = "Option::is_none") )]
		pub hst_ccy_cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
		pub tax: Option<AccountTax1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrCtct") )]
		pub acct_svcr_ctct: Contact13,
	}
	
	impl CashAccountCharacteristics5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.acct_lvl.validate() { return Err(e); }
			if let Err(e) = self.csh_acct.validate() { return Err(e); }
			if let Some(ref val) = self.acct_svcr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prnt_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.compstn_mtd.validate() { return Err(e); }
			if let Some(ref val) = self.dbt_acct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.sttlm_advc {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "sttlm_advc is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 105 {
					return Err(ValidationError::new(1002, "sttlm_advc exceeds the maximum length of 105".to_string()));
				}
			}
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.acct_bal_ccy_cd) {
				return Err(ValidationError::new(1005, "acct_bal_ccy_cd does not match the required pattern".to_string()));
			}
			if let Some(ref val) = self.sttlm_ccy_cd {
				let pattern = Regex::new("[A-Z]{3,3}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "sttlm_ccy_cd does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.hst_ccy_cd {
				let pattern = Regex::new("[A-Z]{3,3}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "hst_ccy_cd does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.tax { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.acct_svcr_ctct.validate() { return Err(e); }
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
	
	
	// CompensationMethod1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CompensationMethod1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOCP") )]
		CodeNOCP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DBTD") )]
		CodeDBTD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVD") )]
		CodeINVD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DDBT") )]
		CodeDDBT,
	}
	
	impl CompensationMethod1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// CurrencyExchange6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CurrencyExchange6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SrcCcy") )]
		pub src_ccy: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TrgtCcy") )]
		pub trgt_ccy: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRate") )]
		pub xchg_rate: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none") )]
		pub unit_ccy: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cmnts", skip_serializing_if = "Option::is_none") )]
		pub cmnts: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QtnDt", skip_serializing_if = "Option::is_none") )]
		pub qtn_dt: Option<String>,
	}
	
	impl CurrencyExchange6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.src_ccy) {
				return Err(ValidationError::new(1005, "src_ccy does not match the required pattern".to_string()));
			}
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.trgt_ccy) {
				return Err(ValidationError::new(1005, "trgt_ccy does not match the required pattern".to_string()));
			}
			if let Some(ref val) = self.desc {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 40 {
					return Err(ValidationError::new(1002, "desc exceeds the maximum length of 40".to_string()));
				}
			}
			if let Some(ref val) = self.unit_ccy {
				let pattern = Regex::new("[A-Z]{3,3}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "unit_ccy does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.cmnts {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "cmnts is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 70 {
					return Err(ValidationError::new(1002, "cmnts exceeds the maximum length of 70".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// DatePeriod1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DatePeriod1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt", skip_serializing_if = "Option::is_none") )]
		pub fr_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt") )]
		pub to_dt: String,
	}
	
	impl DatePeriod1 {
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
	
	
	// FinancialInstitutionIdentification19 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FinancialInstitutionIdentification19 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BICFI", skip_serializing_if = "Option::is_none") )]
		pub bicfi: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none") )]
		pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<GenericFinancialIdentification1>,
	}
	
	impl FinancialInstitutionIdentification19 {
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
			if let Some(ref val) = self.othr { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// ParentCashAccount5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ParentCashAccount5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Lvl", skip_serializing_if = "Option::is_none") )]
		pub lvl: Option<AccountLevel1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: CashAccount40,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Svcr", skip_serializing_if = "Option::is_none") )]
		pub svcr: Option<BranchAndFinancialInstitutionIdentification8>,
	}
	
	impl ParentCashAccount5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.lvl { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref val) = self.svcr { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Party56Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Party56Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgId", skip_serializing_if = "Option::is_none") )]
		pub org_id: Option<OrganisationIdentification39>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FIId", skip_serializing_if = "Option::is_none") )]
		pub fi_id: Option<FinancialInstitutionIdentification19>,
	}
	
	impl Party56Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.org_id { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fi_id { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyIdentification273 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PartyIdentification273 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LglNm", skip_serializing_if = "Option::is_none") )]
		pub lgl_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress27>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Party56Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none") )]
		pub ctry_of_res: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none") )]
		pub ctct_dtls: Option<Contact13>,
	}
	
	impl PartyIdentification273 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.nm.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if self.nm.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
			if let Some(ref val) = self.lgl_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "lgl_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "lgl_nm exceeds the maximum length of 140".to_string()));
				}
			}
			if let Some(ref val) = self.pstl_adr { if let Err(e) = val.validate() { return Err(e); } }
			if let Err(e) = self.id.validate() { return Err(e); }
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
	
	
	// ProprietaryBankTransactionCodeStructure1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ProprietaryBankTransactionCodeStructure1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<String>,
	}
	
	impl ProprietaryBankTransactionCodeStructure1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.cd.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if self.cd.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 35".to_string()));
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
	
	
	// ReportHeader6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReportHeader6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptId") )]
		pub rpt_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none") )]
		pub msg_pgntn: Option<Pagination1>,
	}
	
	impl ReportHeader6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.rpt_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rpt_id is shorter than the minimum length of 1".to_string()));
			}
			if self.rpt_id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rpt_id exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.msg_pgntn { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ResidenceLocation1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ResidenceLocation1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
		pub ctry: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Area", skip_serializing_if = "Option::is_none") )]
		pub area: Option<String>,
	}
	
	impl ResidenceLocation1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.ctry {
				let pattern = Regex::new("[A-Z]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.area {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "area is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "area exceeds the maximum length of 35".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// ServiceAdjustmentType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ServiceAdjustmentType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "COMP") )]
		CodeCOMP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NCMP") )]
		CodeNCMP,
	}
	
	impl ServiceAdjustmentType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ServicePaymentMethod1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ServicePaymentMethod1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "BCMP") )]
		CodeBCMP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FLAT") )]
		CodeFLAT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PVCH") )]
		CodePVCH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVS") )]
		CodeINVS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WVED") )]
		CodeWVED,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FREE") )]
		CodeFREE,
	}
	
	impl ServicePaymentMethod1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ServiceTaxDesignation1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ServiceTaxDesignation1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: ServiceTaxDesignation1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rgn", skip_serializing_if = "Option::is_none") )]
		pub rgn: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRsn", skip_serializing_if = "Option::is_none") )]
		pub tax_rsn: Option<Vec<TaxReason1>>,
	}
	
	impl ServiceTaxDesignation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref val) = self.rgn {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "rgn is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "rgn exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref vec) = self.tax_rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ServiceTaxDesignation1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ServiceTaxDesignation1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "XMPT") )]
		CodeXMPT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ZERO") )]
		CodeZERO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TAXE") )]
		CodeTAXE,
	}
	
	impl ServiceTaxDesignation1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// StatementGroup5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct StatementGroup5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "GrpId") )]
		pub grp_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sndr") )]
		pub sndr: PartyIdentification273,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SndrIndvCtct", skip_serializing_if = "Option::is_none") )]
		pub sndr_indv_ctct: Option<Vec<Contact13>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rcvr") )]
		pub rcvr: PartyIdentification273,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RcvrIndvCtct", skip_serializing_if = "Option::is_none") )]
		pub rcvr_indv_ctct: Option<Vec<Contact13>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BllgStmt") )]
		pub bllg_stmt: Vec<BillingStatement5>,
	}
	
	impl StatementGroup5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.grp_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "grp_id is shorter than the minimum length of 1".to_string()));
			}
			if self.grp_id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "grp_id exceeds the maximum length of 35".to_string()));
			}
			if let Err(e) = self.sndr.validate() { return Err(e); }
			if let Some(ref vec) = self.sndr_indv_ctct { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Err(e) = self.rcvr.validate() { return Err(e); }
			if let Some(ref vec) = self.rcvr_indv_ctct { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			for item in &self.bllg_stmt { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TaxCalculation1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxCalculation1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "HstCcy") )]
		pub hst_ccy: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxblSvcChrgConvs") )]
		pub taxbl_svc_chrg_convs: Vec<BillingServicesAmount3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlTaxblSvcChrgHstAmt") )]
		pub ttl_taxbl_svc_chrg_hst_amt: AmountAndDirection34,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxId") )]
		pub tax_id: Vec<BillingServicesTax3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlTax") )]
		pub ttl_tax: AmountAndDirection34,
	}
	
	impl TaxCalculation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.hst_ccy) {
				return Err(ValidationError::new(1005, "hst_ccy does not match the required pattern".to_string()));
			}
			for item in &self.taxbl_svc_chrg_convs { if let Err(e) = item.validate() { return Err(e); } }
			if let Err(e) = self.ttl_taxbl_svc_chrg_hst_amt.validate() { return Err(e); }
			for item in &self.tax_id { if let Err(e) = item.validate() { return Err(e); } }
			if let Err(e) = self.ttl_tax.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// TaxReason1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxReason1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Expltn") )]
		pub expltn: String,
	}
	
	impl TaxReason1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.cd.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if self.cd.chars().count() > 10 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 10".to_string()));
			}
			if self.expltn.chars().count() < 1 {
				return Err(ValidationError::new(1001, "expltn is shorter than the minimum length of 1".to_string()));
			}
			if self.expltn.chars().count() > 105 {
				return Err(ValidationError::new(1002, "expltn exceeds the maximum length of 105".to_string()));
			}
			Ok(())
		}
	}
	
}