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
	
	
	// AccountIdentification4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AccountIdentification4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "IBAN", skip_serializing_if = "Option::is_none") )]
		pub iban: Option<IBAN2007Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<GenericAccountIdentification1>,
	}
	
	impl AccountIdentification4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref iban_value) = self.iban { if let Err(e) = iban_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AccountLevel1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct AccountSchemeName1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalAccountIdentification1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl AccountSchemeName1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AccountTax1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AccountTax1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClctnMtd") )]
		pub clctn_mtd: BillingTaxCalculationMethod1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rgn", skip_serializing_if = "Option::is_none") )]
		pub rgn: Option<Max40Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonResCtry", skip_serializing_if = "Option::is_none") )]
		pub non_res_ctry: Option<ResidenceLocation1Choice>,
	}
	
	impl AccountTax1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.clctn_mtd.validate() { return Err(e); }
			if let Some(ref rgn_value) = self.rgn { if let Err(e) = rgn_value.validate() { return Err(e); } }
			if let Some(ref non_res_ctry_value) = self.non_res_ctry { if let Err(e) = non_res_ctry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ActiveOrHistoricCurrencyAndAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_or_historic_currency_and_amount_simple_type: f64,
	}
	
	impl ActiveOrHistoricCurrencyAndAmountSimpleType {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.active_or_historic_currency_and_amount_simple_type < 0.000000 {
				return Err(ValidationError::new(1003, "active_or_historic_currency_and_amount_simple_type is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ActiveOrHistoricCurrencyAndAmount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// ActiveOrHistoricCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveOrHistoricCurrencyCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_or_historic_currency_code: String,
	}
	
	impl ActiveOrHistoricCurrencyCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.active_or_historic_currency_code) {
				return Err(ValidationError::new(1005, "active_or_historic_currency_code does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// AddressType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct AddressType3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<AddressType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl AddressType3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AmountAndDirection34 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// AnyBICDec2014Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct AnyBICDec2014Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub any_bic_dec2014_identifier: String,
	}
	
	impl AnyBICDec2014Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(&self.any_bic_dec2014_identifier) {
				return Err(ValidationError::new(1005, "any_bic_dec2014_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// BICFIDec2014Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct BICFIDec2014Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub bicfi_dec2014_identifier: String,
	}
	
	impl BICFIDec2014Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(&self.bicfi_dec2014_identifier) {
				return Err(ValidationError::new(1005, "bicfi_dec2014_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// BalanceAdjustment1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BalanceAdjustment1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: BalanceAdjustmentType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc") )]
		pub desc: Max105Text,
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
			if let Err(e) = self.desc.validate() { return Err(e); }
			if let Err(e) = self.bal_amt.validate() { return Err(e); }
			if let Some(ref avrg_amt_value) = self.avrg_amt { if let Err(e) = avrg_amt_value.validate() { return Err(e); } }
			if let Some(ref earngs_adjstmnt_amt_value) = self.earngs_adjstmnt_amt { if let Err(e) = earngs_adjstmnt_amt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BalanceAdjustmentType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct BankTransactionCodeStructure4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Domn", skip_serializing_if = "Option::is_none") )]
		pub domn: Option<BankTransactionCodeStructure5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<ProprietaryBankTransactionCodeStructure1>,
	}
	
	impl BankTransactionCodeStructure4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref domn_value) = self.domn { if let Err(e) = domn_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BankTransactionCodeStructure5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BankTransactionCodeStructure5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: ExternalBankTransactionDomain1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Fmly") )]
		pub fmly: BankTransactionCodeStructure6,
	}
	
	impl BankTransactionCodeStructure5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Err(e) = self.fmly.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BankTransactionCodeStructure6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BankTransactionCodeStructure6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: ExternalBankTransactionFamily1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubFmlyCd") )]
		pub sub_fmly_cd: ExternalBankTransactionSubFamily1Code,
	}
	
	impl BankTransactionCodeStructure6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Err(e) = self.sub_fmly_cd.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BaseOneRate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct BaseOneRate {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub base_one_rate: f64,
	}
	
	impl BaseOneRate {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BillingBalance1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref ccy_tp_value) = self.ccy_tp { if let Err(e) = ccy_tp_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingBalanceType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BillingBalanceType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalBillingBalanceType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl BillingBalanceType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingChargeMethod1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref ccy_tp_value) = self.ccy_tp { if let Err(e) = ccy_tp_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingCompensationType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BillingCompensationType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalBillingCompensationType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl BillingCompensationType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingCurrencyType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref mtd_a_value) = self.mtd_a { if let Err(e) = mtd_a_value.validate() { return Err(e); } }
			if let Some(ref mtd_b_value) = self.mtd_b { if let Err(e) = mtd_b_value.validate() { return Err(e); } }
			if let Some(ref mtd_d_value) = self.mtd_d { if let Err(e) = mtd_d_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingMethod2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct BillingPrice1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
		pub ccy: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitPric", skip_serializing_if = "Option::is_none") )]
		pub unit_pric: Option<AmountAndDirection34>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mtd", skip_serializing_if = "Option::is_none") )]
		pub mtd: Option<BillingChargeMethod1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rule", skip_serializing_if = "Option::is_none") )]
		pub rule: Option<Max20Text>,
	}
	
	impl BillingPrice1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ccy_value) = self.ccy { if let Err(e) = ccy_value.validate() { return Err(e); } }
			if let Some(ref unit_pric_value) = self.unit_pric { if let Err(e) = unit_pric_value.validate() { return Err(e); } }
			if let Some(ref mtd_value) = self.mtd { if let Err(e) = mtd_value.validate() { return Err(e); } }
			if let Some(ref rule_value) = self.rule { if let Err(e) = rule_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingRate1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct BillingRateIdentification1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalBillingRateIdentification1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl BillingRateIdentification1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingService2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref pric_value) = self.pric { if let Err(e) = pric_value.validate() { return Err(e); } }
			if let Err(e) = self.pmt_mtd.validate() { return Err(e); }
			if let Err(e) = self.orgnl_chrg_pric.validate() { return Err(e); }
			if let Some(ref orgnl_chrg_sttlm_amt_value) = self.orgnl_chrg_sttlm_amt { if let Err(e) = orgnl_chrg_sttlm_amt_value.validate() { return Err(e); } }
			if let Some(ref bal_reqrd_acct_amt_value) = self.bal_reqrd_acct_amt { if let Err(e) = bal_reqrd_acct_amt_value.validate() { return Err(e); } }
			if let Err(e) = self.tax_dsgnt.validate() { return Err(e); }
			if let Some(ref tax_clctn_value) = self.tax_clctn { if let Err(e) = tax_clctn_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingServiceAdjustment1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BillingServiceAdjustment1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: ServiceAdjustmentType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc") )]
		pub desc: Max140Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: AmountAndDirection34,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BalReqrdAmt", skip_serializing_if = "Option::is_none") )]
		pub bal_reqrd_amt: Option<AmountAndDirection34>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ErrDt", skip_serializing_if = "Option::is_none") )]
		pub err_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdjstmntId", skip_serializing_if = "Option::is_none") )]
		pub adjstmnt_id: Option<Max35Text>,
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
			if let Err(e) = self.desc.validate() { return Err(e); }
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Some(ref bal_reqrd_amt_value) = self.bal_reqrd_amt { if let Err(e) = bal_reqrd_amt_value.validate() { return Err(e); } }
			if let Some(ref adjstmnt_id_value) = self.adjstmnt_id { if let Err(e) = adjstmnt_id_value.validate() { return Err(e); } }
			if let Some(ref sub_svc_value) = self.sub_svc { if let Err(e) = sub_svc_value.validate() { return Err(e); } }
			if let Some(ref pric_chng_value) = self.pric_chng { if let Err(e) = pric_chng_value.validate() { return Err(e); } }
			if let Some(ref orgnl_pric_value) = self.orgnl_pric { if let Err(e) = orgnl_pric_value.validate() { return Err(e); } }
			if let Some(ref new_pric_value) = self.new_pric { if let Err(e) = new_pric_value.validate() { return Err(e); } }
			if let Some(ref orgnl_chrg_amt_value) = self.orgnl_chrg_amt { if let Err(e) = orgnl_chrg_amt_value.validate() { return Err(e); } }
			if let Some(ref new_chrg_amt_value) = self.new_chrg_amt { if let Err(e) = new_chrg_amt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingServiceCommonIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BillingServiceCommonIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: Max6Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max8Text,
	}
	
	impl BillingServiceCommonIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.issr.validate() { return Err(e); }
			if let Err(e) = self.id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BillingServiceIdentification2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BillingServiceIdentification2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubSvc", skip_serializing_if = "Option::is_none") )]
		pub sub_svc: Option<BillingSubServiceIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc") )]
		pub desc: Max70Text,
	}
	
	impl BillingServiceIdentification2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref sub_svc_value) = self.sub_svc { if let Err(e) = sub_svc_value.validate() { return Err(e); } }
			if let Err(e) = self.desc.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BillingServiceIdentification3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BillingServiceIdentification3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubSvc", skip_serializing_if = "Option::is_none") )]
		pub sub_svc: Option<BillingSubServiceIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc") )]
		pub desc: Max70Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CmonCd", skip_serializing_if = "Option::is_none") )]
		pub cmon_cd: Option<BillingServiceCommonIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BkTxCd", skip_serializing_if = "Option::is_none") )]
		pub bk_tx_cd: Option<BankTransactionCodeStructure4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvcTp", skip_serializing_if = "Option::is_none") )]
		pub svc_tp: Option<Max12Text>,
	}
	
	impl BillingServiceIdentification3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref sub_svc_value) = self.sub_svc { if let Err(e) = sub_svc_value.validate() { return Err(e); } }
			if let Err(e) = self.desc.validate() { return Err(e); }
			if let Some(ref cmon_cd_value) = self.cmon_cd { if let Err(e) = cmon_cd_value.validate() { return Err(e); } }
			if let Some(ref bk_tx_cd_value) = self.bk_tx_cd { if let Err(e) = bk_tx_cd_value.validate() { return Err(e); } }
			if let Some(ref svc_tp_value) = self.svc_tp { if let Err(e) = svc_tp_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingServiceParameters2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref unit_pric_value) = self.unit_pric { if let Err(e) = unit_pric_value.validate() { return Err(e); } }
			if let Err(e) = self.svc_chrg_amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BillingServiceParameters3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct BillingServicesAmount1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "HstAmt") )]
		pub hst_amt: AmountAndDirection34,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PricgAmt", skip_serializing_if = "Option::is_none") )]
		pub pricg_amt: Option<AmountAndDirection34>,
	}
	
	impl BillingServicesAmount1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.hst_amt.validate() { return Err(e); }
			if let Some(ref pricg_amt_value) = self.pricg_amt { if let Err(e) = pricg_amt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingServicesAmount2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref sttlm_amt_value) = self.sttlm_amt { if let Err(e) = sttlm_amt_value.validate() { return Err(e); } }
			if let Some(ref pricg_amt_value) = self.pricg_amt { if let Err(e) = pricg_amt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingServicesAmount3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct BillingServicesTax1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nb") )]
		pub nb: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max40Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rate") )]
		pub rate: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HstAmt") )]
		pub hst_amt: AmountAndDirection34,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PricgAmt", skip_serializing_if = "Option::is_none") )]
		pub pricg_amt: Option<AmountAndDirection34>,
	}
	
	impl BillingServicesTax1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.nb.validate() { return Err(e); }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			if let Err(e) = self.hst_amt.validate() { return Err(e); }
			if let Some(ref pricg_amt_value) = self.pricg_amt { if let Err(e) = pricg_amt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingServicesTax2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BillingServicesTax2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nb") )]
		pub nb: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max40Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rate") )]
		pub rate: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PricgAmt") )]
		pub pricg_amt: AmountAndDirection34,
	}
	
	impl BillingServicesTax2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.nb.validate() { return Err(e); }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			if let Err(e) = self.pricg_amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BillingServicesTax3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BillingServicesTax3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nb") )]
		pub nb: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max40Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rate") )]
		pub rate: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlTaxAmt") )]
		pub ttl_tax_amt: AmountAndDirection34,
	}
	
	impl BillingServicesTax3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.nb.validate() { return Err(e); }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			if let Err(e) = self.ttl_tax_amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BillingStatement5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BillingStatement5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "StmtId") )]
		pub stmt_id: Max35Text,
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
			if let Err(e) = self.stmt_id.validate() { return Err(e); }
			if let Err(e) = self.fr_to_dt.validate() { return Err(e); }
			if let Err(e) = self.sts.validate() { return Err(e); }
			if let Err(e) = self.acct_chrtcs.validate() { return Err(e); }
			if let Some(ref rate_data_vec) = self.rate_data { for item in rate_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ccy_xchg_vec) = self.ccy_xchg { for item in ccy_xchg_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref bal_vec) = self.bal { for item in bal_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref compstn_vec) = self.compstn { for item in compstn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref svc_vec) = self.svc { for item in svc_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref tax_rgn_vec) = self.tax_rgn { for item in tax_rgn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref bal_adjstmnt_vec) = self.bal_adjstmnt { for item in bal_adjstmnt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref svc_adjstmnt_vec) = self.svc_adjstmnt { for item in svc_adjstmnt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// BillingStatementStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct BillingSubServiceIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: BillingSubServiceQualifier1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
	}
	
	impl BillingSubServiceIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.issr.validate() { return Err(e); }
			if let Err(e) = self.id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BillingSubServiceQualifier1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BillingSubServiceQualifier1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<BillingSubServiceQualifier1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl BillingSubServiceQualifier1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingSubServiceQualifier1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct BillingTaxIdentification3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "VATRegnNb", skip_serializing_if = "Option::is_none") )]
		pub vat_regn_nb: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRegnNb", skip_serializing_if = "Option::is_none") )]
		pub tax_regn_nb: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxCtct", skip_serializing_if = "Option::is_none") )]
		pub tax_ctct: Option<Contact13>,
	}
	
	impl BillingTaxIdentification3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref vat_regn_nb_value) = self.vat_regn_nb { if let Err(e) = vat_regn_nb_value.validate() { return Err(e); } }
			if let Some(ref tax_regn_nb_value) = self.tax_regn_nb { if let Err(e) = tax_regn_nb_value.validate() { return Err(e); } }
			if let Some(ref tax_ctct_value) = self.tax_ctct { if let Err(e) = tax_ctct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BillingTaxRegion3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BillingTaxRegion3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RgnNb") )]
		pub rgn_nb: Max40Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RgnNm") )]
		pub rgn_nm: Max40Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CstmrTaxId") )]
		pub cstmr_tax_id: Max40Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PtDt", skip_serializing_if = "Option::is_none") )]
		pub pt_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SndgFI", skip_serializing_if = "Option::is_none") )]
		pub sndg_fi: Option<BillingTaxIdentification3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InvcNb", skip_serializing_if = "Option::is_none") )]
		pub invc_nb: Option<Max40Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MtdC", skip_serializing_if = "Option::is_none") )]
		pub mtd_c: Option<BillingMethod4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmAmt") )]
		pub sttlm_amt: AmountAndDirection34,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxDueToRgn") )]
		pub tax_due_to_rgn: AmountAndDirection34,
	}
	
	impl BillingTaxRegion3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rgn_nb.validate() { return Err(e); }
			if let Err(e) = self.rgn_nm.validate() { return Err(e); }
			if let Err(e) = self.cstmr_tax_id.validate() { return Err(e); }
			if let Some(ref sndg_fi_value) = self.sndg_fi { if let Err(e) = sndg_fi_value.validate() { return Err(e); } }
			if let Some(ref invc_nb_value) = self.invc_nb { if let Err(e) = invc_nb_value.validate() { return Err(e); } }
			if let Some(ref mtd_c_value) = self.mtd_c { if let Err(e) = mtd_c_value.validate() { return Err(e); } }
			if let Err(e) = self.sttlm_amt.validate() { return Err(e); }
			if let Err(e) = self.tax_due_to_rgn.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BranchAndFinancialInstitutionIdentification8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BranchAndFinancialInstitutionIdentification8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstnId") )]
		pub fin_instn_id: FinancialInstitutionIdentification23,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BrnchId", skip_serializing_if = "Option::is_none") )]
		pub brnch_id: Option<BranchData5>,
	}
	
	impl BranchAndFinancialInstitutionIdentification8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.fin_instn_id.validate() { return Err(e); }
			if let Some(ref brnch_id_value) = self.brnch_id { if let Err(e) = brnch_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BranchData5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BranchData5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress27>,
	}
	
	impl BranchData5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CashAccount40 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CashAccount40 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<AccountIdentification4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<CashAccountType2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
		pub ccy: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prxy", skip_serializing_if = "Option::is_none") )]
		pub prxy: Option<ProxyAccountIdentification1>,
	}
	
	impl CashAccount40 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref ccy_value) = self.ccy { if let Err(e) = ccy_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref prxy_value) = self.prxy { if let Err(e) = prxy_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CashAccountCharacteristics5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
		pub sttlm_advc: Option<Max105Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctBalCcyCd") )]
		pub acct_bal_ccy_cd: ActiveOrHistoricCurrencyCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCcyCd", skip_serializing_if = "Option::is_none") )]
		pub sttlm_ccy_cd: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HstCcyCd", skip_serializing_if = "Option::is_none") )]
		pub hst_ccy_cd: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
		pub tax: Option<AccountTax1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrCtct") )]
		pub acct_svcr_ctct: Contact13,
	}
	
	impl CashAccountCharacteristics5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.acct_lvl.validate() { return Err(e); }
			if let Err(e) = self.csh_acct.validate() { return Err(e); }
			if let Some(ref acct_svcr_value) = self.acct_svcr { if let Err(e) = acct_svcr_value.validate() { return Err(e); } }
			if let Some(ref prnt_acct_value) = self.prnt_acct { if let Err(e) = prnt_acct_value.validate() { return Err(e); } }
			if let Err(e) = self.compstn_mtd.validate() { return Err(e); }
			if let Some(ref dbt_acct_value) = self.dbt_acct { if let Err(e) = dbt_acct_value.validate() { return Err(e); } }
			if let Some(ref sttlm_advc_value) = self.sttlm_advc { if let Err(e) = sttlm_advc_value.validate() { return Err(e); } }
			if let Err(e) = self.acct_bal_ccy_cd.validate() { return Err(e); }
			if let Some(ref sttlm_ccy_cd_value) = self.sttlm_ccy_cd { if let Err(e) = sttlm_ccy_cd_value.validate() { return Err(e); } }
			if let Some(ref hst_ccy_cd_value) = self.hst_ccy_cd { if let Err(e) = hst_ccy_cd_value.validate() { return Err(e); } }
			if let Some(ref tax_value) = self.tax { if let Err(e) = tax_value.validate() { return Err(e); } }
			if let Err(e) = self.acct_svcr_ctct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// CashAccountType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CashAccountType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalCashAccountType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl CashAccountType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ClearingSystemIdentification2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ClearingSystemIdentification2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalClearingSystemIdentification1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl ClearingSystemIdentification2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ClearingSystemMemberIdentification2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ClearingSystemMemberIdentification2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none") )]
		pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MmbId") )]
		pub mmb_id: Max35Text,
	}
	
	impl ClearingSystemMemberIdentification2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref clr_sys_id_value) = self.clr_sys_id { if let Err(e) = clr_sys_id_value.validate() { return Err(e); } }
			if let Err(e) = self.mmb_id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// CompensationMethod1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct Contact13 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none") )]
		pub nm_prfx: Option<NamePrefix2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PhneNb", skip_serializing_if = "Option::is_none") )]
		pub phne_nb: Option<PhoneNumber>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MobNb", skip_serializing_if = "Option::is_none") )]
		pub mob_nb: Option<PhoneNumber>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FaxNb", skip_serializing_if = "Option::is_none") )]
		pub fax_nb: Option<PhoneNumber>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "URLAdr", skip_serializing_if = "Option::is_none") )]
		pub url_adr: Option<Max2048Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none") )]
		pub email_adr: Option<Max256Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EmailPurp", skip_serializing_if = "Option::is_none") )]
		pub email_purp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "JobTitl", skip_serializing_if = "Option::is_none") )]
		pub job_titl: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rspnsblty", skip_serializing_if = "Option::is_none") )]
		pub rspnsblty: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dept", skip_serializing_if = "Option::is_none") )]
		pub dept: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<Vec<OtherContact1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrefrdMtd", skip_serializing_if = "Option::is_none") )]
		pub prefrd_mtd: Option<PreferredContactMethod2Code>,
	}
	
	impl Contact13 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nm_prfx_value) = self.nm_prfx { if let Err(e) = nm_prfx_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref phne_nb_value) = self.phne_nb { if let Err(e) = phne_nb_value.validate() { return Err(e); } }
			if let Some(ref mob_nb_value) = self.mob_nb { if let Err(e) = mob_nb_value.validate() { return Err(e); } }
			if let Some(ref fax_nb_value) = self.fax_nb { if let Err(e) = fax_nb_value.validate() { return Err(e); } }
			if let Some(ref url_adr_value) = self.url_adr { if let Err(e) = url_adr_value.validate() { return Err(e); } }
			if let Some(ref email_adr_value) = self.email_adr { if let Err(e) = email_adr_value.validate() { return Err(e); } }
			if let Some(ref email_purp_value) = self.email_purp { if let Err(e) = email_purp_value.validate() { return Err(e); } }
			if let Some(ref job_titl_value) = self.job_titl { if let Err(e) = job_titl_value.validate() { return Err(e); } }
			if let Some(ref rspnsblty_value) = self.rspnsblty { if let Err(e) = rspnsblty_value.validate() { return Err(e); } }
			if let Some(ref dept_value) = self.dept { if let Err(e) = dept_value.validate() { return Err(e); } }
			if let Some(ref othr_vec) = self.othr { for item in othr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref prefrd_mtd_value) = self.prefrd_mtd { if let Err(e) = prefrd_mtd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CountryCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct CountryCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub country_code: String,
	}
	
	impl CountryCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&self.country_code) {
				return Err(ValidationError::new(1005, "country_code does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// CurrencyExchange6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CurrencyExchange6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SrcCcy") )]
		pub src_ccy: ActiveOrHistoricCurrencyCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TrgtCcy") )]
		pub trgt_ccy: ActiveOrHistoricCurrencyCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRate") )]
		pub xchg_rate: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max40Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none") )]
		pub unit_ccy: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cmnts", skip_serializing_if = "Option::is_none") )]
		pub cmnts: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QtnDt", skip_serializing_if = "Option::is_none") )]
		pub qtn_dt: Option<String>,
	}
	
	impl CurrencyExchange6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.src_ccy.validate() { return Err(e); }
			if let Err(e) = self.trgt_ccy.validate() { return Err(e); }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			if let Some(ref unit_ccy_value) = self.unit_ccy { if let Err(e) = unit_ccy_value.validate() { return Err(e); } }
			if let Some(ref cmnts_value) = self.cmnts { if let Err(e) = cmnts_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DatePeriod1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// DecimalNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct DecimalNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub decimal_number: f64,
	}
	
	impl DecimalNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Exact4AlphaNumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Exact4AlphaNumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub exact4_alpha_numeric_text: String,
	}
	
	impl Exact4AlphaNumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
			if !pattern.is_match(&self.exact4_alpha_numeric_text) {
				return Err(ValidationError::new(1005, "exact4_alpha_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalAccountIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalAccountIdentification1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_account_identification1_code: String,
	}
	
	impl ExternalAccountIdentification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_account_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_account_identification1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_account_identification1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_account_identification1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalBankTransactionDomain1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalBankTransactionDomain1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_bank_transaction_domain1_code: String,
	}
	
	impl ExternalBankTransactionDomain1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_bank_transaction_domain1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_bank_transaction_domain1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_bank_transaction_domain1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_bank_transaction_domain1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalBankTransactionFamily1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalBankTransactionFamily1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_bank_transaction_family1_code: String,
	}
	
	impl ExternalBankTransactionFamily1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_bank_transaction_family1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_bank_transaction_family1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_bank_transaction_family1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_bank_transaction_family1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalBankTransactionSubFamily1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalBankTransactionSubFamily1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_bank_transaction_sub_family1_code: String,
	}
	
	impl ExternalBankTransactionSubFamily1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_bank_transaction_sub_family1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_bank_transaction_sub_family1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_bank_transaction_sub_family1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_bank_transaction_sub_family1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalBillingBalanceType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalBillingBalanceType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_billing_balance_type1_code: String,
	}
	
	impl ExternalBillingBalanceType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_billing_balance_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_billing_balance_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_billing_balance_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_billing_balance_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalBillingCompensationType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalBillingCompensationType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_billing_compensation_type1_code: String,
	}
	
	impl ExternalBillingCompensationType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_billing_compensation_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_billing_compensation_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_billing_compensation_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_billing_compensation_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalBillingRateIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalBillingRateIdentification1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_billing_rate_identification1_code: String,
	}
	
	impl ExternalBillingRateIdentification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_billing_rate_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_billing_rate_identification1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_billing_rate_identification1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_billing_rate_identification1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalCashAccountType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalCashAccountType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_cash_account_type1_code: String,
	}
	
	impl ExternalCashAccountType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_cash_account_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_cash_account_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_cash_account_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_cash_account_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalClearingSystemIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalClearingSystemIdentification1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_clearing_system_identification1_code: String,
	}
	
	impl ExternalClearingSystemIdentification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_clearing_system_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_clearing_system_identification1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_clearing_system_identification1_code.chars().count() > 5 {
				return Err(ValidationError::new(1002, "external_clearing_system_identification1_code exceeds the maximum length of 5".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalFinancialInstitutionIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalFinancialInstitutionIdentification1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_financial_institution_identification1_code: String,
	}
	
	impl ExternalFinancialInstitutionIdentification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_financial_institution_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_financial_institution_identification1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_financial_institution_identification1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_financial_institution_identification1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalOrganisationIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalOrganisationIdentification1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_organisation_identification1_code: String,
	}
	
	impl ExternalOrganisationIdentification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_organisation_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_organisation_identification1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_organisation_identification1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_organisation_identification1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalProxyAccountType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalProxyAccountType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_proxy_account_type1_code: String,
	}
	
	impl ExternalProxyAccountType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_proxy_account_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_proxy_account_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_proxy_account_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_proxy_account_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// FinancialIdentificationSchemeName1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FinancialIdentificationSchemeName1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalFinancialInstitutionIdentification1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl FinancialIdentificationSchemeName1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FinancialInstitutionIdentification19 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FinancialInstitutionIdentification19 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BICFI", skip_serializing_if = "Option::is_none") )]
		pub bicfi: Option<BICFIDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none") )]
		pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<GenericFinancialIdentification1>,
	}
	
	impl FinancialInstitutionIdentification19 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref bicfi_value) = self.bicfi { if let Err(e) = bicfi_value.validate() { return Err(e); } }
			if let Some(ref clr_sys_mmb_id_value) = self.clr_sys_mmb_id { if let Err(e) = clr_sys_mmb_id_value.validate() { return Err(e); } }
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FinancialInstitutionIdentification23 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FinancialInstitutionIdentification23 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BICFI", skip_serializing_if = "Option::is_none") )]
		pub bicfi: Option<BICFIDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none") )]
		pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress27>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<GenericFinancialIdentification1>,
	}
	
	impl FinancialInstitutionIdentification23 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref bicfi_value) = self.bicfi { if let Err(e) = bicfi_value.validate() { return Err(e); } }
			if let Some(ref clr_sys_mmb_id_value) = self.clr_sys_mmb_id { if let Err(e) = clr_sys_mmb_id_value.validate() { return Err(e); } }
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericAccountIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericAccountIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max34Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<AccountSchemeName1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericAccountIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericFinancialIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericFinancialIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericFinancialIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericIdentification30 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification30 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Exact4AlphaNumericText,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
	}
	
	impl GenericIdentification30 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.issr.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericOrganisationIdentification3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericOrganisationIdentification3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max256Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericOrganisationIdentification3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// IBAN2007Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct IBAN2007Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iban2007_identifier: String,
	}
	
	impl IBAN2007Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}").unwrap();
			if !pattern.is_match(&self.iban2007_identifier) {
				return Err(ValidationError::new(1005, "iban2007_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ISODate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISODate {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_date: String,
	}
	
	impl ISODate {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// LEIIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct LEIIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub lei_identifier: String,
	}
	
	impl LEIIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&self.lei_identifier) {
				return Err(ValidationError::new(1005, "lei_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max105Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max105Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max105_text: String,
	}
	
	impl Max105Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max105_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max105_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max105_text.chars().count() > 105 {
				return Err(ValidationError::new(1002, "max105_text exceeds the maximum length of 105".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max10Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max10Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max10_text: String,
	}
	
	impl Max10Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max10_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max10_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max10_text.chars().count() > 10 {
				return Err(ValidationError::new(1002, "max10_text exceeds the maximum length of 10".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max128Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max128Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max128_text: String,
	}
	
	impl Max128Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max128_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max128_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max128_text.chars().count() > 128 {
				return Err(ValidationError::new(1002, "max128_text exceeds the maximum length of 128".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max12Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max12Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max12_text: String,
	}
	
	impl Max12Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max12_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max12_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max12_text.chars().count() > 12 {
				return Err(ValidationError::new(1002, "max12_text exceeds the maximum length of 12".to_string()));
			}
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
	
	
	// Max16Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max16Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max16_text: String,
	}
	
	impl Max16Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max16_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max16_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max16_text.chars().count() > 16 {
				return Err(ValidationError::new(1002, "max16_text exceeds the maximum length of 16".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max2048Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max2048Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max2048_text: String,
	}
	
	impl Max2048Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max2048_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max2048_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max2048_text.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "max2048_text exceeds the maximum length of 2048".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max20Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max20Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max20_text: String,
	}
	
	impl Max20Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max20_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max20_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max20_text.chars().count() > 20 {
				return Err(ValidationError::new(1002, "max20_text exceeds the maximum length of 20".to_string()));
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
	
	
	// Max34Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max34Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max34_text: String,
	}
	
	impl Max34Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max34_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max34_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max34_text.chars().count() > 34 {
				return Err(ValidationError::new(1002, "max34_text exceeds the maximum length of 34".to_string()));
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
	
	
	// Max40Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max40Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max40_text: String,
	}
	
	impl Max40Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max40_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max40_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max40_text.chars().count() > 40 {
				return Err(ValidationError::new(1002, "max40_text exceeds the maximum length of 40".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max4Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max4Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max4_text: String,
	}
	
	impl Max4Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max4_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max4_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max4_text.chars().count() > 4 {
				return Err(ValidationError::new(1002, "max4_text exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max5NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max5NumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max5_numeric_text: String,
	}
	
	impl Max5NumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{1,5}").unwrap();
			if !pattern.is_match(&self.max5_numeric_text) {
				return Err(ValidationError::new(1005, "max5_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max6Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max6Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max6_text: String,
	}
	
	impl Max6Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max6_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max6_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max6_text.chars().count() > 6 {
				return Err(ValidationError::new(1002, "max6_text exceeds the maximum length of 6".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max70Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max70Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max70_text: String,
	}
	
	impl Max70Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max70_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max70_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max70_text.chars().count() > 70 {
				return Err(ValidationError::new(1002, "max70_text exceeds the maximum length of 70".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max8Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max8Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max8_text: String,
	}
	
	impl Max8Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max8_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max8_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max8_text.chars().count() > 8 {
				return Err(ValidationError::new(1002, "max8_text exceeds the maximum length of 8".to_string()));
			}
			Ok(())
		}
	}
	
	
	// NamePrefix2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// OrganisationIdentification39 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct OrganisationIdentification39 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<AnyBICDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<Vec<GenericOrganisationIdentification3>>,
	}
	
	impl OrganisationIdentification39 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			if let Some(ref othr_vec) = self.othr { for item in othr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// OrganisationIdentificationSchemeName1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct OrganisationIdentificationSchemeName1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalOrganisationIdentification1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl OrganisationIdentificationSchemeName1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OtherContact1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct OtherContact1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ChanlTp") )]
		pub chanl_tp: Max4Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Max128Text>,
	}
	
	impl OtherContact1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.chanl_tp.validate() { return Err(e); }
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Pagination1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Pagination1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PgNb") )]
		pub pg_nb: Max5NumericText,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LastPgInd") )]
		pub last_pg_ind: bool,
	}
	
	impl Pagination1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pg_nb.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ParentCashAccount5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref lvl_value) = self.lvl { if let Err(e) = lvl_value.validate() { return Err(e); } }
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref svcr_value) = self.svcr { if let Err(e) = svcr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Party56Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Party56Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgId", skip_serializing_if = "Option::is_none") )]
		pub org_id: Option<OrganisationIdentification39>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FIId", skip_serializing_if = "Option::is_none") )]
		pub fi_id: Option<FinancialInstitutionIdentification19>,
	}
	
	impl Party56Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref org_id_value) = self.org_id { if let Err(e) = org_id_value.validate() { return Err(e); } }
			if let Some(ref fi_id_value) = self.fi_id { if let Err(e) = fi_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyIdentification273 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PartyIdentification273 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: Max140Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LglNm", skip_serializing_if = "Option::is_none") )]
		pub lgl_nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress27>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Party56Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none") )]
		pub ctry_of_res: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none") )]
		pub ctct_dtls: Option<Contact13>,
	}
	
	impl PartyIdentification273 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.nm.validate() { return Err(e); }
			if let Some(ref lgl_nm_value) = self.lgl_nm { if let Err(e) = lgl_nm_value.validate() { return Err(e); } }
			if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref ctry_of_res_value) = self.ctry_of_res { if let Err(e) = ctry_of_res_value.validate() { return Err(e); } }
			if let Some(ref ctct_dtls_value) = self.ctct_dtls { if let Err(e) = ctct_dtls_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PercentageRate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct PercentageRate {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub percentage_rate: f64,
	}
	
	impl PercentageRate {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PhoneNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct PhoneNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub phone_number: String,
	}
	
	impl PhoneNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(&self.phone_number) {
				return Err(ValidationError::new(1005, "phone_number does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// PlusOrMinusIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct PlusOrMinusIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub plus_or_minus_indicator: bool,
	}
	
	impl PlusOrMinusIndicator {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PostalAddress27 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PostalAddress27 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
		pub adr_tp: Option<AddressType3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CareOf", skip_serializing_if = "Option::is_none") )]
		pub care_of: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dept", skip_serializing_if = "Option::is_none") )]
		pub dept: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubDept", skip_serializing_if = "Option::is_none") )]
		pub sub_dept: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
		pub strt_nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
		pub bldg_nb: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNm", skip_serializing_if = "Option::is_none") )]
		pub bldg_nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Flr", skip_serializing_if = "Option::is_none") )]
		pub flr: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitNb", skip_serializing_if = "Option::is_none") )]
		pub unit_nb: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstBx", skip_serializing_if = "Option::is_none") )]
		pub pst_bx: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Room", skip_serializing_if = "Option::is_none") )]
		pub room: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
		pub pst_cd: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
		pub twn_nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none") )]
		pub twn_lctn_nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none") )]
		pub dstrct_nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
		pub ctry_sub_dvsn: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
		pub ctry: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
		pub adr_line: Option<Vec<Max70Text>>,
	}
	
	impl PostalAddress27 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref adr_tp_value) = self.adr_tp { if let Err(e) = adr_tp_value.validate() { return Err(e); } }
			if let Some(ref care_of_value) = self.care_of { if let Err(e) = care_of_value.validate() { return Err(e); } }
			if let Some(ref dept_value) = self.dept { if let Err(e) = dept_value.validate() { return Err(e); } }
			if let Some(ref sub_dept_value) = self.sub_dept { if let Err(e) = sub_dept_value.validate() { return Err(e); } }
			if let Some(ref strt_nm_value) = self.strt_nm { if let Err(e) = strt_nm_value.validate() { return Err(e); } }
			if let Some(ref bldg_nb_value) = self.bldg_nb { if let Err(e) = bldg_nb_value.validate() { return Err(e); } }
			if let Some(ref bldg_nm_value) = self.bldg_nm { if let Err(e) = bldg_nm_value.validate() { return Err(e); } }
			if let Some(ref flr_value) = self.flr { if let Err(e) = flr_value.validate() { return Err(e); } }
			if let Some(ref unit_nb_value) = self.unit_nb { if let Err(e) = unit_nb_value.validate() { return Err(e); } }
			if let Some(ref pst_bx_value) = self.pst_bx { if let Err(e) = pst_bx_value.validate() { return Err(e); } }
			if let Some(ref room_value) = self.room { if let Err(e) = room_value.validate() { return Err(e); } }
			if let Some(ref pst_cd_value) = self.pst_cd { if let Err(e) = pst_cd_value.validate() { return Err(e); } }
			if let Some(ref twn_nm_value) = self.twn_nm { if let Err(e) = twn_nm_value.validate() { return Err(e); } }
			if let Some(ref twn_lctn_nm_value) = self.twn_lctn_nm { if let Err(e) = twn_lctn_nm_value.validate() { return Err(e); } }
			if let Some(ref dstrct_nm_value) = self.dstrct_nm { if let Err(e) = dstrct_nm_value.validate() { return Err(e); } }
			if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if let Err(e) = ctry_sub_dvsn_value.validate() { return Err(e); } }
			if let Some(ref ctry_value) = self.ctry { if let Err(e) = ctry_value.validate() { return Err(e); } }
			if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// PreferredContactMethod2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct ProprietaryBankTransactionCodeStructure1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl ProprietaryBankTransactionCodeStructure1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ProxyAccountIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ProxyAccountIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<ProxyAccountType1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max2048Text,
	}
	
	impl ProxyAccountIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Err(e) = self.id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ProxyAccountType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ProxyAccountType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalProxyAccountType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl ProxyAccountType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ReportHeader6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ReportHeader6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptId") )]
		pub rpt_id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none") )]
		pub msg_pgntn: Option<Pagination1>,
	}
	
	impl ReportHeader6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rpt_id.validate() { return Err(e); }
			if let Some(ref msg_pgntn_value) = self.msg_pgntn { if let Err(e) = msg_pgntn_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ResidenceLocation1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ResidenceLocation1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
		pub ctry: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Area", skip_serializing_if = "Option::is_none") )]
		pub area: Option<Max35Text>,
	}
	
	impl ResidenceLocation1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ctry_value) = self.ctry { if let Err(e) = ctry_value.validate() { return Err(e); } }
			if let Some(ref area_value) = self.area { if let Err(e) = area_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ServiceAdjustmentType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct ServiceTaxDesignation1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: ServiceTaxDesignation1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rgn", skip_serializing_if = "Option::is_none") )]
		pub rgn: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRsn", skip_serializing_if = "Option::is_none") )]
		pub tax_rsn: Option<Vec<TaxReason1>>,
	}
	
	impl ServiceTaxDesignation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref rgn_value) = self.rgn { if let Err(e) = rgn_value.validate() { return Err(e); } }
			if let Some(ref tax_rsn_vec) = self.tax_rsn { for item in tax_rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ServiceTaxDesignation1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct StatementGroup5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "GrpId") )]
		pub grp_id: Max35Text,
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
			if let Err(e) = self.grp_id.validate() { return Err(e); }
			if let Err(e) = self.sndr.validate() { return Err(e); }
			if let Some(ref sndr_indv_ctct_vec) = self.sndr_indv_ctct { for item in sndr_indv_ctct_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Err(e) = self.rcvr.validate() { return Err(e); }
			if let Some(ref rcvr_indv_ctct_vec) = self.rcvr_indv_ctct { for item in rcvr_indv_ctct_vec { if let Err(e) = item.validate() { return Err(e); } } }
			for item in &self.bllg_stmt { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TaxCalculation1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TaxCalculation1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "HstCcy") )]
		pub hst_ccy: ActiveOrHistoricCurrencyCode,
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
			if let Err(e) = self.hst_ccy.validate() { return Err(e); }
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
	pub struct TaxReason1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: Max10Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Expltn") )]
		pub expltn: Max105Text,
	}
	
	impl TaxReason1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Err(e) = self.expltn.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// YesNoIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct YesNoIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub yes_no_indicator: bool,
	}
	
	impl YesNoIndicator {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
}