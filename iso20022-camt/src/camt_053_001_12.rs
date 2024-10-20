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
	
	
	// AccountInterest4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountInterest4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<InterestType1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
		pub rate: Option<Vec<Rate4>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDt", skip_serializing_if = "Option::is_none") )]
		pub fr_to_dt: Option<DateTimePeriod1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
		pub tax: Option<TaxCharges2>,
	}
	
	impl AccountInterest4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref rate_vec) = self.rate { for item in rate_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref fr_to_dt_value) = self.fr_to_dt { if let Err(e) = fr_to_dt_value.validate() { return Err(e); } }
			if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
			if let Some(ref tax_value) = self.tax { if let Err(e) = tax_value.validate() { return Err(e); } }
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
	
	
	// AccountStatement13 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountStatement13 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StmtPgntn", skip_serializing_if = "Option::is_none") )]
		pub stmt_pgntn: Option<Pagination1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ElctrncSeqNb", skip_serializing_if = "Option::is_none") )]
		pub elctrnc_seq_nb: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgSeq", skip_serializing_if = "Option::is_none") )]
		pub rptg_seq: Option<SequenceRange1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LglSeqNb", skip_serializing_if = "Option::is_none") )]
		pub lgl_seq_nb: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
		pub cre_dt_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDt", skip_serializing_if = "Option::is_none") )]
		pub fr_to_dt: Option<DateTimePeriod1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CpyDplctInd", skip_serializing_if = "Option::is_none") )]
		pub cpy_dplct_ind: Option<CopyDuplicate1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgSrc", skip_serializing_if = "Option::is_none") )]
		pub rptg_src: Option<ReportingSource1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Acct") )]
		pub acct: CashAccount43,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdAcct", skip_serializing_if = "Option::is_none") )]
		pub rltd_acct: Option<CashAccount40>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Intrst", skip_serializing_if = "Option::is_none") )]
		pub intrst: Option<Vec<AccountInterest4>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Bal") )]
		pub bal: Vec<CashBalance8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxsSummry", skip_serializing_if = "Option::is_none") )]
		pub txs_summry: Option<TotalTransactions6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ntry", skip_serializing_if = "Option::is_none") )]
		pub ntry: Option<Vec<ReportEntry14>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlStmtInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_stmt_inf: Option<Max500Text>,
	}
	
	impl AccountStatement13 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref stmt_pgntn_value) = self.stmt_pgntn { if let Err(e) = stmt_pgntn_value.validate() { return Err(e); } }
			if let Some(ref rptg_seq_value) = self.rptg_seq { if let Err(e) = rptg_seq_value.validate() { return Err(e); } }
			if let Some(ref fr_to_dt_value) = self.fr_to_dt { if let Err(e) = fr_to_dt_value.validate() { return Err(e); } }
			if let Some(ref cpy_dplct_ind_value) = self.cpy_dplct_ind { if let Err(e) = cpy_dplct_ind_value.validate() { return Err(e); } }
			if let Some(ref rptg_src_value) = self.rptg_src { if let Err(e) = rptg_src_value.validate() { return Err(e); } }
			if let Err(e) = self.acct.validate() { return Err(e); }
			if let Some(ref rltd_acct_value) = self.rltd_acct { if let Err(e) = rltd_acct_value.validate() { return Err(e); } }
			if let Some(ref intrst_vec) = self.intrst { for item in intrst_vec { if let Err(e) = item.validate() { return Err(e); } } }
			for item in &self.bal { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref txs_summry_value) = self.txs_summry { if let Err(e) = txs_summry_value.validate() { return Err(e); } }
			if let Some(ref ntry_vec) = self.ntry { for item in ntry_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref addtl_stmt_inf_value) = self.addtl_stmt_inf { if let Err(e) = addtl_stmt_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ActiveCurrencyAndAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveCurrencyAndAmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_currency_and_amount_simple_type: f64,
	}
	
	impl ActiveCurrencyAndAmountSimpleType {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.active_currency_and_amount_simple_type < 0.000000 {
				return Err(ValidationError::new(1003, "active_currency_and_amount_simple_type is less than the minimum value of 0.000000".to_string()));
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
	
	
	// ActiveCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveCurrencyCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_currency_code: String,
	}
	
	impl ActiveCurrencyCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.active_currency_code) {
				return Err(ValidationError::new(1005, "active_currency_code does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_or_historic_currency_and13_decimal_amount_simple_type: f64,
	}
	
	impl ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.active_or_historic_currency_and13_decimal_amount_simple_type < 0.000000 {
				return Err(ValidationError::new(1003, "active_or_historic_currency_and13_decimal_amount_simple_type is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ActiveOrHistoricCurrencyAnd13DecimalAmount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
		pub ccy: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub value: f64,
	}
	
	impl ActiveOrHistoricCurrencyAnd13DecimalAmount {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ActiveOrHistoricCurrencyAndAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
		pub ccy: ActiveOrHistoricCurrencyCode,
	}
	
	impl ActiveOrHistoricCurrencyAndAmountRange2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Some(ref cdt_dbt_ind_value) = self.cdt_dbt_ind { if let Err(e) = cdt_dbt_ind_value.validate() { return Err(e); } }
			if let Err(e) = self.ccy.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ActiveOrHistoricCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AmountAndCurrencyExchange4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AmountAndCurrencyExchange4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none") )]
		pub instd_amt: Option<AmountAndCurrencyExchangeDetails5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxAmt", skip_serializing_if = "Option::is_none") )]
		pub tx_amt: Option<AmountAndCurrencyExchangeDetails5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CntrValAmt", skip_serializing_if = "Option::is_none") )]
		pub cntr_val_amt: Option<AmountAndCurrencyExchangeDetails5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnncdPstngAmt", skip_serializing_if = "Option::is_none") )]
		pub anncd_pstng_amt: Option<AmountAndCurrencyExchangeDetails5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryAmt", skip_serializing_if = "Option::is_none") )]
		pub prtry_amt: Option<Vec<AmountAndCurrencyExchangeDetails6>>,
	}
	
	impl AmountAndCurrencyExchange4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref instd_amt_value) = self.instd_amt { if let Err(e) = instd_amt_value.validate() { return Err(e); } }
			if let Some(ref tx_amt_value) = self.tx_amt { if let Err(e) = tx_amt_value.validate() { return Err(e); } }
			if let Some(ref cntr_val_amt_value) = self.cntr_val_amt { if let Err(e) = cntr_val_amt_value.validate() { return Err(e); } }
			if let Some(ref anncd_pstng_amt_value) = self.anncd_pstng_amt { if let Err(e) = anncd_pstng_amt_value.validate() { return Err(e); } }
			if let Some(ref prtry_amt_vec) = self.prtry_amt { for item in prtry_amt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// AmountAndCurrencyExchangeDetails5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AmountAndCurrencyExchangeDetails5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CcyXchg", skip_serializing_if = "Option::is_none") )]
		pub ccy_xchg: Option<CurrencyExchange24>,
	}
	
	impl AmountAndCurrencyExchangeDetails5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Some(ref ccy_xchg_value) = self.ccy_xchg { if let Err(e) = ccy_xchg_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AmountAndCurrencyExchangeDetails6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AmountAndCurrencyExchangeDetails6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CcyXchg", skip_serializing_if = "Option::is_none") )]
		pub ccy_xchg: Option<CurrencyExchange24>,
	}
	
	impl AmountAndCurrencyExchangeDetails6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Some(ref ccy_xchg_value) = self.ccy_xchg { if let Err(e) = ccy_xchg_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AmountAndDirection35 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AmountAndDirection35 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd") )]
		pub cdt_dbt_ind: CreditDebitCode,
	}
	
	impl AmountAndDirection35 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cdt_dbt_ind.validate() { return Err(e); }
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
			Ok(())
		}
	}
	
	
	// AnyBICDec2014Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// AttendanceContext1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum AttendanceContext1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ATTD") )]
		CodeATTD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SATT") )]
		CodeSATT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UATT") )]
		CodeUATT,
	}
	
	impl AttendanceContext1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AuthenticationEntity1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum AuthenticationEntity1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ICCD") )]
		CodeICCD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AGNT") )]
		CodeAGNT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MERC") )]
		CodeMERC,
	}
	
	impl AuthenticationEntity1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AuthenticationMethod1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum AuthenticationMethod1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "UKNW") )]
		CodeUKNW,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BYPS") )]
		CodeBYPS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NPIN") )]
		CodeNPIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FPIN") )]
		CodeFPIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CPSG") )]
		CodeCPSG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PPSG") )]
		CodePPSG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MANU") )]
		CodeMANU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MERC") )]
		CodeMERC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SCRT") )]
		CodeSCRT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SNCT") )]
		CodeSNCT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SCNL") )]
		CodeSCNL,
	}
	
	impl AuthenticationMethod1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BICFIDec2014Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// BalanceSubType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BalanceSubType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalBalanceSubType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl BalanceSubType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BalanceType10Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BalanceType10Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalBalanceType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl BalanceType10Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BalanceType13 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BalanceType13 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
		pub cd_or_prtry: BalanceType10Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubTp", skip_serializing_if = "Option::is_none") )]
		pub sub_tp: Option<BalanceSubType1Choice>,
	}
	
	impl BalanceType13 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd_or_prtry.validate() { return Err(e); }
			if let Some(ref sub_tp_value) = self.sub_tp { if let Err(e) = sub_tp_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BankToCustomerStatementV12 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BankToCustomerStatementV12 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "GrpHdr") )]
		pub grp_hdr: GroupHeader116,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Stmt") )]
		pub stmt: Vec<AccountStatement13>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl BankToCustomerStatementV12 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.grp_hdr.validate() { return Err(e); }
			for item in &self.stmt { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
			if let Some(ref domn_value) = self.domn { if let Err(e) = domn_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// BatchInformation2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BatchInformation2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId", skip_serializing_if = "Option::is_none") )]
		pub msg_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInfId", skip_serializing_if = "Option::is_none") )]
		pub pmt_inf_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxs", skip_serializing_if = "Option::is_none") )]
		pub nb_of_txs: Option<Max15NumericText>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none") )]
		pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
		pub cdt_dbt_ind: Option<CreditDebitCode>,
	}
	
	impl BatchInformation2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref msg_id_value) = self.msg_id { if let Err(e) = msg_id_value.validate() { return Err(e); } }
			if let Some(ref pmt_inf_id_value) = self.pmt_inf_id { if let Err(e) = pmt_inf_id_value.validate() { return Err(e); } }
			if let Some(ref nb_of_txs_value) = self.nb_of_txs { if let Err(e) = nb_of_txs_value.validate() { return Err(e); } }
			if let Some(ref ttl_amt_value) = self.ttl_amt { if let Err(e) = ttl_amt_value.validate() { return Err(e); } }
			if let Some(ref cdt_dbt_ind_value) = self.cdt_dbt_ind { if let Err(e) = cdt_dbt_ind_value.validate() { return Err(e); } }
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
			if let Some(ref brnch_id_value) = self.brnch_id { if let Err(e) = brnch_id_value.validate() { return Err(e); } }
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
	
	
	// CSCManagement1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CSCManagement1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRST") )]
		CodePRST,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BYPS") )]
		CodeBYPS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UNRD") )]
		CodeUNRD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NCSC") )]
		CodeNCSC,
	}
	
	impl CSCManagement1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CardAggregated2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CardAggregated2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSvc", skip_serializing_if = "Option::is_none") )]
		pub addtl_svc: Option<CardPaymentServiceType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxCtgy", skip_serializing_if = "Option::is_none") )]
		pub tx_ctgy: Option<ExternalCardTransactionCategory1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SaleRcncltnId", skip_serializing_if = "Option::is_none") )]
		pub sale_rcncltn_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SeqNbRg", skip_serializing_if = "Option::is_none") )]
		pub seq_nb_rg: Option<CardSequenceNumberRange1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxDtRg", skip_serializing_if = "Option::is_none") )]
		pub tx_dt_rg: Option<DateOrDateTimePeriod1Choice>,
	}
	
	impl CardAggregated2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref addtl_svc_value) = self.addtl_svc { if let Err(e) = addtl_svc_value.validate() { return Err(e); } }
			if let Some(ref tx_ctgy_value) = self.tx_ctgy { if let Err(e) = tx_ctgy_value.validate() { return Err(e); } }
			if let Some(ref sale_rcncltn_id_value) = self.sale_rcncltn_id { if let Err(e) = sale_rcncltn_id_value.validate() { return Err(e); } }
			if let Some(ref seq_nb_rg_value) = self.seq_nb_rg { if let Err(e) = seq_nb_rg_value.validate() { return Err(e); } }
			if let Some(ref tx_dt_rg_value) = self.tx_dt_rg { if let Err(e) = tx_dt_rg_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CardDataReading1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CardDataReading1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "TAGC") )]
		CodeTAGC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PHYS") )]
		CodePHYS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BRCD") )]
		CodeBRCD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MGST") )]
		CodeMGST,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CICC") )]
		CodeCICC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DFLE") )]
		CodeDFLE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CTLS") )]
		CodeCTLS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ECTL") )]
		CodeECTL,
	}
	
	impl CardDataReading1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CardEntry5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CardEntry5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Card", skip_serializing_if = "Option::is_none") )]
		pub card: Option<PaymentCard4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "POI", skip_serializing_if = "Option::is_none") )]
		pub poi: Option<PointOfInteraction1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AggtdNtry", skip_serializing_if = "Option::is_none") )]
		pub aggtd_ntry: Option<CardAggregated2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrePdAcct", skip_serializing_if = "Option::is_none") )]
		pub pre_pd_acct: Option<CashAccount40>,
	}
	
	impl CardEntry5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref card_value) = self.card { if let Err(e) = card_value.validate() { return Err(e); } }
			if let Some(ref poi_value) = self.poi { if let Err(e) = poi_value.validate() { return Err(e); } }
			if let Some(ref aggtd_ntry_value) = self.aggtd_ntry { if let Err(e) = aggtd_ntry_value.validate() { return Err(e); } }
			if let Some(ref pre_pd_acct_value) = self.pre_pd_acct { if let Err(e) = pre_pd_acct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CardIndividualTransaction2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CardIndividualTransaction2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ICCRltdData", skip_serializing_if = "Option::is_none") )]
		pub icc_rltd_data: Option<Max1025Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCntxt", skip_serializing_if = "Option::is_none") )]
		pub pmt_cntxt: Option<PaymentContext3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSvc", skip_serializing_if = "Option::is_none") )]
		pub addtl_svc: Option<CardPaymentServiceType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxCtgy", skip_serializing_if = "Option::is_none") )]
		pub tx_ctgy: Option<ExternalCardTransactionCategory1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SaleRcncltnId", skip_serializing_if = "Option::is_none") )]
		pub sale_rcncltn_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SaleRefNb", skip_serializing_if = "Option::is_none") )]
		pub sale_ref_nb: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RePresntmntRsn", skip_serializing_if = "Option::is_none") )]
		pub re_presntmnt_rsn: Option<ExternalRePresentmentReason1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SeqNb", skip_serializing_if = "Option::is_none") )]
		pub seq_nb: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
		pub tx_id: Option<TransactionIdentifier1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pdct", skip_serializing_if = "Option::is_none") )]
		pub pdct: Option<Product2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VldtnDt", skip_serializing_if = "Option::is_none") )]
		pub vldtn_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VldtnSeqNb", skip_serializing_if = "Option::is_none") )]
		pub vldtn_seq_nb: Option<Max35Text>,
	}
	
	impl CardIndividualTransaction2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref icc_rltd_data_value) = self.icc_rltd_data { if let Err(e) = icc_rltd_data_value.validate() { return Err(e); } }
			if let Some(ref pmt_cntxt_value) = self.pmt_cntxt { if let Err(e) = pmt_cntxt_value.validate() { return Err(e); } }
			if let Some(ref addtl_svc_value) = self.addtl_svc { if let Err(e) = addtl_svc_value.validate() { return Err(e); } }
			if let Some(ref tx_ctgy_value) = self.tx_ctgy { if let Err(e) = tx_ctgy_value.validate() { return Err(e); } }
			if let Some(ref sale_rcncltn_id_value) = self.sale_rcncltn_id { if let Err(e) = sale_rcncltn_id_value.validate() { return Err(e); } }
			if let Some(ref sale_ref_nb_value) = self.sale_ref_nb { if let Err(e) = sale_ref_nb_value.validate() { return Err(e); } }
			if let Some(ref re_presntmnt_rsn_value) = self.re_presntmnt_rsn { if let Err(e) = re_presntmnt_rsn_value.validate() { return Err(e); } }
			if let Some(ref seq_nb_value) = self.seq_nb { if let Err(e) = seq_nb_value.validate() { return Err(e); } }
			if let Some(ref tx_id_value) = self.tx_id { if let Err(e) = tx_id_value.validate() { return Err(e); } }
			if let Some(ref pdct_value) = self.pdct { if let Err(e) = pdct_value.validate() { return Err(e); } }
			if let Some(ref vldtn_seq_nb_value) = self.vldtn_seq_nb { if let Err(e) = vldtn_seq_nb_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CardPaymentServiceType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CardPaymentServiceType2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "AGGR") )]
		CodeAGGR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DCCV") )]
		CodeDCCV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GRTT") )]
		CodeGRTT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INSP") )]
		CodeINSP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LOYT") )]
		CodeLOYT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NRES") )]
		CodeNRES,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PUCO") )]
		CodePUCO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RECP") )]
		CodeRECP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SOAF") )]
		CodeSOAF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UNAF") )]
		CodeUNAF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VCAU") )]
		CodeVCAU,
	}
	
	impl CardPaymentServiceType2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CardSecurityInformation1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CardSecurityInformation1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CSCMgmt") )]
		pub csc_mgmt: CSCManagement1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CSCVal", skip_serializing_if = "Option::is_none") )]
		pub csc_val: Option<Min3Max4NumericText>,
	}
	
	impl CardSecurityInformation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.csc_mgmt.validate() { return Err(e); }
			if let Some(ref csc_val_value) = self.csc_val { if let Err(e) = csc_val_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CardSequenceNumberRange1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CardSequenceNumberRange1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrstTx", skip_serializing_if = "Option::is_none") )]
		pub frst_tx: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LastTx", skip_serializing_if = "Option::is_none") )]
		pub last_tx: Option<Max35Text>,
	}
	
	impl CardSequenceNumberRange1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref frst_tx_value) = self.frst_tx { if let Err(e) = frst_tx_value.validate() { return Err(e); } }
			if let Some(ref last_tx_value) = self.last_tx { if let Err(e) = last_tx_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CardTransaction18 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CardTransaction18 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Card", skip_serializing_if = "Option::is_none") )]
		pub card: Option<PaymentCard4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "POI", skip_serializing_if = "Option::is_none") )]
		pub poi: Option<PointOfInteraction1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tx", skip_serializing_if = "Option::is_none") )]
		pub tx: Option<CardTransaction3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrePdAcct", skip_serializing_if = "Option::is_none") )]
		pub pre_pd_acct: Option<CashAccount40>,
	}
	
	impl CardTransaction18 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref card_value) = self.card { if let Err(e) = card_value.validate() { return Err(e); } }
			if let Some(ref poi_value) = self.poi { if let Err(e) = poi_value.validate() { return Err(e); } }
			if let Some(ref tx_value) = self.tx { if let Err(e) = tx_value.validate() { return Err(e); } }
			if let Some(ref pre_pd_acct_value) = self.pre_pd_acct { if let Err(e) = pre_pd_acct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CardTransaction3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CardTransaction3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Aggtd", skip_serializing_if = "Option::is_none") )]
		pub aggtd: Option<CardAggregated2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Indv", skip_serializing_if = "Option::is_none") )]
		pub indv: Option<CardIndividualTransaction2>,
	}
	
	impl CardTransaction3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref aggtd_value) = self.aggtd { if let Err(e) = aggtd_value.validate() { return Err(e); } }
			if let Some(ref indv_value) = self.indv { if let Err(e) = indv_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CardholderAuthentication2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CardholderAuthentication2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AuthntcnMtd") )]
		pub authntcn_mtd: AuthenticationMethod1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AuthntcnNtty") )]
		pub authntcn_ntty: AuthenticationEntity1Code,
	}
	
	impl CardholderAuthentication2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.authntcn_mtd.validate() { return Err(e); }
			if let Err(e) = self.authntcn_ntty.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// CardholderVerificationCapability1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CardholderVerificationCapability1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "MNSG") )]
		CodeMNSG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NPIN") )]
		CodeNPIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FCPN") )]
		CodeFCPN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FEPN") )]
		CodeFEPN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FDSG") )]
		CodeFDSG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FBIO") )]
		CodeFBIO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MNVR") )]
		CodeMNVR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FBIG") )]
		CodeFBIG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "APKI") )]
		CodeAPKI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PKIS") )]
		CodePKIS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CHDT") )]
		CodeCHDT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SCEC") )]
		CodeSCEC,
	}
	
	impl CardholderVerificationCapability1Code {
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
	
	
	// CashAccount43 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CashAccount43 {
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
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ownr", skip_serializing_if = "Option::is_none") )]
		pub ownr: Option<PartyIdentification272>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Svcr", skip_serializing_if = "Option::is_none") )]
		pub svcr: Option<BranchAndFinancialInstitutionIdentification8>,
	}
	
	impl CashAccount43 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref ccy_value) = self.ccy { if let Err(e) = ccy_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref prxy_value) = self.prxy { if let Err(e) = prxy_value.validate() { return Err(e); } }
			if let Some(ref ownr_value) = self.ownr { if let Err(e) = ownr_value.validate() { return Err(e); } }
			if let Some(ref svcr_value) = self.svcr { if let Err(e) = svcr_value.validate() { return Err(e); } }
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
	
	
	// CashAvailability1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CashAvailability1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt") )]
		pub dt: CashAvailabilityDate1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd") )]
		pub cdt_dbt_ind: CreditDebitCode,
	}
	
	impl CashAvailability1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.dt.validate() { return Err(e); }
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Err(e) = self.cdt_dbt_ind.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// CashAvailabilityDate1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CashAvailabilityDate1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfDays", skip_serializing_if = "Option::is_none") )]
		pub nb_of_days: Option<Max15PlusSignedNumericText>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ActlDt", skip_serializing_if = "Option::is_none") )]
		pub actl_dt: Option<String>,
	}
	
	impl CashAvailabilityDate1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nb_of_days_value) = self.nb_of_days { if let Err(e) = nb_of_days_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CashBalance8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CashBalance8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: BalanceType13,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtLine", skip_serializing_if = "Option::is_none") )]
		pub cdt_line: Option<Vec<CreditLine3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd") )]
		pub cdt_dbt_ind: CreditDebitCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt") )]
		pub dt: DateAndDateTime2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Avlbty", skip_serializing_if = "Option::is_none") )]
		pub avlbty: Option<Vec<CashAvailability1>>,
	}
	
	impl CashBalance8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Some(ref cdt_line_vec) = self.cdt_line { for item in cdt_line_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Err(e) = self.cdt_dbt_ind.validate() { return Err(e); }
			if let Err(e) = self.dt.validate() { return Err(e); }
			if let Some(ref avlbty_vec) = self.avlbty { for item in avlbty_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// CashDeposit1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CashDeposit1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoteDnmtn") )]
		pub note_dnmtn: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfNotes") )]
		pub nb_of_notes: Max15NumericText,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveCurrencyAndAmount,
	}
	
	impl CashDeposit1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.note_dnmtn.validate() { return Err(e); }
			if let Err(e) = self.nb_of_notes.validate() { return Err(e); }
			if let Err(e) = self.amt.validate() { return Err(e); }
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
		pub cd: Option<ExternalCategoryPurpose1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl CategoryPurpose1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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
	
	
	// ChargeIncludedIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ChargeIncludedIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub charge_included_indicator: bool,
	}
	
	impl ChargeIncludedIndicator {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ChargeType3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ChargeType3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalChargeType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification3>,
	}
	
	impl ChargeType3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Charges15 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Charges15 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlChrgsAndTaxAmt", skip_serializing_if = "Option::is_none") )]
		pub ttl_chrgs_and_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rcrd", skip_serializing_if = "Option::is_none") )]
		pub rcrd: Option<Vec<ChargesRecord8>>,
	}
	
	impl Charges15 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ttl_chrgs_and_tax_amt_value) = self.ttl_chrgs_and_tax_amt { if let Err(e) = ttl_chrgs_and_tax_amt_value.validate() { return Err(e); } }
			if let Some(ref rcrd_vec) = self.rcrd { for item in rcrd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ChargesRecord8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ChargesRecord8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
		pub cdt_dbt_ind: Option<CreditDebitCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgInclInd", skip_serializing_if = "Option::is_none") )]
		pub chrg_incl_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<ChargeType3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
		pub rate: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Br", skip_serializing_if = "Option::is_none") )]
		pub br: Option<ChargeBearerType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Agt", skip_serializing_if = "Option::is_none") )]
		pub agt: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
		pub tax: Option<TaxCharges2>,
	}
	
	impl ChargesRecord8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Some(ref cdt_dbt_ind_value) = self.cdt_dbt_ind { if let Err(e) = cdt_dbt_ind_value.validate() { return Err(e); } }
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref br_value) = self.br { if let Err(e) = br_value.validate() { return Err(e); } }
			if let Some(ref agt_value) = self.agt { if let Err(e) = agt_value.validate() { return Err(e); } }
			if let Some(ref tax_value) = self.tax { if let Err(e) = tax_value.validate() { return Err(e); } }
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CorporateAction9 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CorporateAction9 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "EvtTp") )]
		pub evt_tp: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EvtId") )]
		pub evt_id: Max35Text,
	}
	
	impl CorporateAction9 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.evt_tp.validate() { return Err(e); }
			if let Err(e) = self.evt_id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// CountryCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CreditLine3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CreditLine3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Incl") )]
		pub incl: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<CreditLineType1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
		pub dt: Option<DateAndDateTime2Choice>,
	}
	
	impl CreditLine3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
			if let Some(ref dt_value) = self.dt { if let Err(e) = dt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CreditLineType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CreditLineType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalCreditLineType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl CreditLineType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CreditorReferenceInformation3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CreditorReferenceInformation3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<CreditorReferenceType3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ref", skip_serializing_if = "Option::is_none") )]
		pub ref_attr: Option<Max35Text>,
	}
	
	impl CreditorReferenceInformation3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref ref_attr_value) = self.ref_attr { if let Err(e) = ref_attr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CreditorReferenceType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CreditorReferenceType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalCreditorReferenceType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl CreditorReferenceType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CreditorReferenceType3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CreditorReferenceType3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
		pub cd_or_prtry: CreditorReferenceType2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl CreditorReferenceType3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd_or_prtry.validate() { return Err(e); }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CurrencyExchange24 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CurrencyExchange24 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SrcCcy") )]
		pub src_ccy: ActiveOrHistoricCurrencyCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TrgtCcy", skip_serializing_if = "Option::is_none") )]
		pub trgt_ccy: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none") )]
		pub unit_ccy: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRate") )]
		pub xchg_rate: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctId", skip_serializing_if = "Option::is_none") )]
		pub ctrct_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QtnDt", skip_serializing_if = "Option::is_none") )]
		pub qtn_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRateBase", skip_serializing_if = "Option::is_none") )]
		pub xchg_rate_base: Option<f64>,
	}
	
	impl CurrencyExchange24 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.src_ccy.validate() { return Err(e); }
			if let Some(ref trgt_ccy_value) = self.trgt_ccy { if let Err(e) = trgt_ccy_value.validate() { return Err(e); } }
			if let Some(ref unit_ccy_value) = self.unit_ccy { if let Err(e) = unit_ccy_value.validate() { return Err(e); } }
			if let Some(ref ctrct_id_value) = self.ctrct_id { if let Err(e) = ctrct_id_value.validate() { return Err(e); } }
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
		pub prvc_of_birth: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CityOfBirth") )]
		pub city_of_birth: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfBirth") )]
		pub ctry_of_birth: CountryCode,
	}
	
	impl DateAndPlaceOfBirth1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref prvc_of_birth_value) = self.prvc_of_birth { if let Err(e) = prvc_of_birth_value.validate() { return Err(e); } }
			if let Err(e) = self.city_of_birth.validate() { return Err(e); }
			if let Err(e) = self.ctry_of_birth.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// DateAndType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DateAndType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: DateType2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt") )]
		pub dt: String,
	}
	
	impl DateAndType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// DateOrDateTimePeriod1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DateOrDateTimePeriod1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
		pub dt: Option<DatePeriod2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtTm", skip_serializing_if = "Option::is_none") )]
		pub dt_tm: Option<DateTimePeriod1>,
	}
	
	impl DateOrDateTimePeriod1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref dt_value) = self.dt { if let Err(e) = dt_value.validate() { return Err(e); } }
			if let Some(ref dt_tm_value) = self.dt_tm { if let Err(e) = dt_tm_value.validate() { return Err(e); } }
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
	
	
	// DateType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DateType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalDateType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl DateType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DecimalNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// DisplayCapabilities1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DisplayCapabilities1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DispTp") )]
		pub disp_tp: UserInterface2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfLines") )]
		pub nb_of_lines: Max3NumericText,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LineWidth") )]
		pub line_width: Max3NumericText,
	}
	
	impl DisplayCapabilities1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.disp_tp.validate() { return Err(e); }
			if let Err(e) = self.nb_of_lines.validate() { return Err(e); }
			if let Err(e) = self.line_width.validate() { return Err(e); }
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
		pub rsn: Option<Max4Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Max140Text>,
	}
	
	impl DocumentAdjustment1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Some(ref cdt_dbt_ind_value) = self.cdt_dbt_ind { if let Err(e) = cdt_dbt_ind_value.validate() { return Err(e); } }
			if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DocumentAmount1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentAmount1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: DocumentAmountType1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
	}
	
	impl DocumentAmount1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Err(e) = self.amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// DocumentAmountType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentAmountType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalDocumentAmountType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl DocumentAmountType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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
		pub nb: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdDt", skip_serializing_if = "Option::is_none") )]
		pub rltd_dt: Option<String>,
	}
	
	impl DocumentLineIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref nb_value) = self.nb { if let Err(e) = nb_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DocumentLineInformation2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentLineInformation2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Vec<DocumentLineIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max2048Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<RemittanceAmount4>,
	}
	
	impl DocumentLineInformation2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.id { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
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
		pub issr: Option<Max35Text>,
	}
	
	impl DocumentLineType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd_or_prtry.validate() { return Err(e); }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
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
		pub cd: Option<ExternalDocumentLineType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl DocumentLineType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DocumentType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
		pub cd_or_prtry: DocumentType2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl DocumentType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd_or_prtry.validate() { return Err(e); }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DocumentType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalDocumentType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl DocumentType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EntryDetails13 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EntryDetails13 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Btch", skip_serializing_if = "Option::is_none") )]
		pub btch: Option<BatchInformation2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxDtls", skip_serializing_if = "Option::is_none") )]
		pub tx_dtls: Option<Vec<EntryTransaction14>>,
	}
	
	impl EntryDetails13 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref btch_value) = self.btch { if let Err(e) = btch_value.validate() { return Err(e); } }
			if let Some(ref tx_dtls_vec) = self.tx_dtls { for item in tx_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// EntryStatus1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EntryStatus1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalEntryStatus1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl EntryStatus1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EntryTransaction14 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EntryTransaction14 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Refs", skip_serializing_if = "Option::is_none") )]
		pub refs: Option<TransactionReferences6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
		pub cdt_dbt_ind: Option<CreditDebitCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AmtDtls", skip_serializing_if = "Option::is_none") )]
		pub amt_dtls: Option<AmountAndCurrencyExchange4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Avlbty", skip_serializing_if = "Option::is_none") )]
		pub avlbty: Option<Vec<CashAvailability1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BkTxCd", skip_serializing_if = "Option::is_none") )]
		pub bk_tx_cd: Option<BankTransactionCodeStructure4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Chrgs", skip_serializing_if = "Option::is_none") )]
		pub chrgs: Option<Charges15>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Intrst", skip_serializing_if = "Option::is_none") )]
		pub intrst: Option<TransactionInterest4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdPties", skip_serializing_if = "Option::is_none") )]
		pub rltd_pties: Option<TransactionParties12>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdAgts", skip_serializing_if = "Option::is_none") )]
		pub rltd_agts: Option<TransactionAgents6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none") )]
		pub lcl_instrm: Option<LocalInstrument2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
		pub pmt_tp_inf: Option<PaymentTypeInformation27>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
		pub purp: Option<Purpose2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdRmtInf", skip_serializing_if = "Option::is_none") )]
		pub rltd_rmt_inf: Option<Vec<RemittanceLocation8>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
		pub rmt_inf: Option<RemittanceInformation22>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdDts", skip_serializing_if = "Option::is_none") )]
		pub rltd_dts: Option<TransactionDates3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdPric", skip_serializing_if = "Option::is_none") )]
		pub rltd_pric: Option<TransactionPrice4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdQties", skip_serializing_if = "Option::is_none") )]
		pub rltd_qties: Option<Vec<TransactionQuantities3Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none") )]
		pub fin_instrm_id: Option<SecurityIdentification19>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
		pub tax: Option<TaxData1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RtrInf", skip_serializing_if = "Option::is_none") )]
		pub rtr_inf: Option<PaymentReturnReason8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CorpActn", skip_serializing_if = "Option::is_none") )]
		pub corp_actn: Option<CorporateAction9>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none") )]
		pub sfkpg_acct: Option<SecuritiesAccount19>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshDpst", skip_serializing_if = "Option::is_none") )]
		pub csh_dpst: Option<Vec<CashDeposit1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CardTx", skip_serializing_if = "Option::is_none") )]
		pub card_tx: Option<CardTransaction18>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlTxInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_tx_inf: Option<Max500Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl EntryTransaction14 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref refs_value) = self.refs { if let Err(e) = refs_value.validate() { return Err(e); } }
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
			if let Some(ref cdt_dbt_ind_value) = self.cdt_dbt_ind { if let Err(e) = cdt_dbt_ind_value.validate() { return Err(e); } }
			if let Some(ref amt_dtls_value) = self.amt_dtls { if let Err(e) = amt_dtls_value.validate() { return Err(e); } }
			if let Some(ref avlbty_vec) = self.avlbty { for item in avlbty_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref bk_tx_cd_value) = self.bk_tx_cd { if let Err(e) = bk_tx_cd_value.validate() { return Err(e); } }
			if let Some(ref chrgs_value) = self.chrgs { if let Err(e) = chrgs_value.validate() { return Err(e); } }
			if let Some(ref intrst_value) = self.intrst { if let Err(e) = intrst_value.validate() { return Err(e); } }
			if let Some(ref rltd_pties_value) = self.rltd_pties { if let Err(e) = rltd_pties_value.validate() { return Err(e); } }
			if let Some(ref rltd_agts_value) = self.rltd_agts { if let Err(e) = rltd_agts_value.validate() { return Err(e); } }
			if let Some(ref lcl_instrm_value) = self.lcl_instrm { if let Err(e) = lcl_instrm_value.validate() { return Err(e); } }
			if let Some(ref pmt_tp_inf_value) = self.pmt_tp_inf { if let Err(e) = pmt_tp_inf_value.validate() { return Err(e); } }
			if let Some(ref purp_value) = self.purp { if let Err(e) = purp_value.validate() { return Err(e); } }
			if let Some(ref rltd_rmt_inf_vec) = self.rltd_rmt_inf { for item in rltd_rmt_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref rmt_inf_value) = self.rmt_inf { if let Err(e) = rmt_inf_value.validate() { return Err(e); } }
			if let Some(ref rltd_dts_value) = self.rltd_dts { if let Err(e) = rltd_dts_value.validate() { return Err(e); } }
			if let Some(ref rltd_pric_value) = self.rltd_pric { if let Err(e) = rltd_pric_value.validate() { return Err(e); } }
			if let Some(ref rltd_qties_vec) = self.rltd_qties { for item in rltd_qties_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref fin_instrm_id_value) = self.fin_instrm_id { if let Err(e) = fin_instrm_id_value.validate() { return Err(e); } }
			if let Some(ref tax_value) = self.tax { if let Err(e) = tax_value.validate() { return Err(e); } }
			if let Some(ref rtr_inf_value) = self.rtr_inf { if let Err(e) = rtr_inf_value.validate() { return Err(e); } }
			if let Some(ref corp_actn_value) = self.corp_actn { if let Err(e) = corp_actn_value.validate() { return Err(e); } }
			if let Some(ref sfkpg_acct_value) = self.sfkpg_acct { if let Err(e) = sfkpg_acct_value.validate() { return Err(e); } }
			if let Some(ref csh_dpst_vec) = self.csh_dpst { for item in csh_dpst_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref card_tx_value) = self.card_tx { if let Err(e) = card_tx_value.validate() { return Err(e); } }
			if let Some(ref addtl_tx_inf_value) = self.addtl_tx_inf { if let Err(e) = addtl_tx_inf_value.validate() { return Err(e); } }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// Exact1NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Exact1NumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub exact1_numeric_text: String,
	}
	
	impl Exact1NumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]").unwrap();
			if !pattern.is_match(&self.exact1_numeric_text) {
				return Err(ValidationError::new(1005, "exact1_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Exact3NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Exact3NumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub exact3_numeric_text: String,
	}
	
	impl Exact3NumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{3}").unwrap();
			if !pattern.is_match(&self.exact3_numeric_text) {
				return Err(ValidationError::new(1005, "exact3_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Exact4AlphaNumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ExternalBalanceSubType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalBalanceSubType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_balance_sub_type1_code: String,
	}
	
	impl ExternalBalanceSubType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_balance_sub_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_balance_sub_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_balance_sub_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_balance_sub_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalBalanceType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalBalanceType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_balance_type1_code: String,
	}
	
	impl ExternalBalanceType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_balance_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_balance_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_balance_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_balance_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalBankTransactionDomain1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ExternalCardTransactionCategory1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalCardTransactionCategory1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_card_transaction_category1_code: String,
	}
	
	impl ExternalCardTransactionCategory1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_card_transaction_category1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_card_transaction_category1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_card_transaction_category1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_card_transaction_category1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalCashAccountType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ExternalCategoryPurpose1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalCategoryPurpose1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_category_purpose1_code: String,
	}
	
	impl ExternalCategoryPurpose1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_category_purpose1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_category_purpose1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_category_purpose1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_category_purpose1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalChargeType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalChargeType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_charge_type1_code: String,
	}
	
	impl ExternalChargeType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_charge_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_charge_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_charge_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_charge_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalClearingSystemIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ExternalCreditLineType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalCreditLineType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_credit_line_type1_code: String,
	}
	
	impl ExternalCreditLineType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_credit_line_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_credit_line_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_credit_line_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_credit_line_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalCreditorReferenceType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalCreditorReferenceType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_creditor_reference_type1_code: String,
	}
	
	impl ExternalCreditorReferenceType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_creditor_reference_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_creditor_reference_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_creditor_reference_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_creditor_reference_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalDateType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalDateType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_date_type1_code: String,
	}
	
	impl ExternalDateType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_date_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_date_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_date_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_date_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalDocumentAmountType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalDocumentAmountType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_document_amount_type1_code: String,
	}
	
	impl ExternalDocumentAmountType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_document_amount_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_document_amount_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_document_amount_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_document_amount_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalDocumentLineType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalDocumentLineType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_document_line_type1_code: String,
	}
	
	impl ExternalDocumentLineType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_document_line_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_document_line_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_document_line_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_document_line_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalDocumentType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalDocumentType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_document_type1_code: String,
	}
	
	impl ExternalDocumentType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_document_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_document_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_document_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_document_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalEntryStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalEntryStatus1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_entry_status1_code: String,
	}
	
	impl ExternalEntryStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_entry_status1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_entry_status1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_entry_status1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_entry_status1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalFinancialInstitutionIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ExternalFinancialInstrumentIdentificationType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalFinancialInstrumentIdentificationType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_financial_instrument_identification_type1_code: String,
	}
	
	impl ExternalFinancialInstrumentIdentificationType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_financial_instrument_identification_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_financial_instrument_identification_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_financial_instrument_identification_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_financial_instrument_identification_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalGarnishmentType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalGarnishmentType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_garnishment_type1_code: String,
	}
	
	impl ExternalGarnishmentType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_garnishment_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_garnishment_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_garnishment_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_garnishment_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalLocalInstrument1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalLocalInstrument1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_local_instrument1_code: String,
	}
	
	impl ExternalLocalInstrument1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_local_instrument1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_local_instrument1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_local_instrument1_code.chars().count() > 35 {
				return Err(ValidationError::new(1002, "external_local_instrument1_code exceeds the maximum length of 35".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalOrganisationIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ExternalPersonIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalPersonIdentification1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_person_identification1_code: String,
	}
	
	impl ExternalPersonIdentification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_person_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_person_identification1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_person_identification1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_person_identification1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalProxyAccountType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ExternalPurpose1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalPurpose1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_purpose1_code: String,
	}
	
	impl ExternalPurpose1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_purpose1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_purpose1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_purpose1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_purpose1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalRePresentmentReason1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalRePresentmentReason1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_re_presentment_reason1_code: String,
	}
	
	impl ExternalRePresentmentReason1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_re_presentment_reason1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_re_presentment_reason1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_re_presentment_reason1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_re_presentment_reason1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalReportingSource1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalReportingSource1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_reporting_source1_code: String,
	}
	
	impl ExternalReportingSource1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_reporting_source1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_reporting_source1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_reporting_source1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_reporting_source1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalReturnReason1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalReturnReason1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_return_reason1_code: String,
	}
	
	impl ExternalReturnReason1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_return_reason1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_return_reason1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_return_reason1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_return_reason1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalServiceLevel1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalServiceLevel1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_service_level1_code: String,
	}
	
	impl ExternalServiceLevel1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_service_level1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_service_level1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_service_level1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_service_level1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalTechnicalInputChannel1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalTechnicalInputChannel1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_technical_input_channel1_code: String,
	}
	
	impl ExternalTechnicalInputChannel1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_technical_input_channel1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_technical_input_channel1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_technical_input_channel1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_technical_input_channel1_code exceeds the maximum length of 4".to_string()));
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
	
	
	// FinancialInstitutionIdentification23 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Garnishment4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Garnishment4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: GarnishmentType1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Grnshee", skip_serializing_if = "Option::is_none") )]
		pub grnshee: Option<PartyIdentification272>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GrnshmtAdmstr", skip_serializing_if = "Option::is_none") )]
		pub grnshmt_admstr: Option<PartyIdentification272>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefNb", skip_serializing_if = "Option::is_none") )]
		pub ref_nb: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
		pub dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none") )]
		pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FmlyMdclInsrncInd", skip_serializing_if = "Option::is_none") )]
		pub fmly_mdcl_insrnc_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MplyeeTermntnInd", skip_serializing_if = "Option::is_none") )]
		pub mplyee_termntn_ind: Option<bool>,
	}
	
	impl Garnishment4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Some(ref grnshee_value) = self.grnshee { if let Err(e) = grnshee_value.validate() { return Err(e); } }
			if let Some(ref grnshmt_admstr_value) = self.grnshmt_admstr { if let Err(e) = grnshmt_admstr_value.validate() { return Err(e); } }
			if let Some(ref ref_nb_value) = self.ref_nb { if let Err(e) = ref_nb_value.validate() { return Err(e); } }
			if let Some(ref rmtd_amt_value) = self.rmtd_amt { if let Err(e) = rmtd_amt_value.validate() { return Err(e); } }
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
		pub issr: Option<Max35Text>,
	}
	
	impl GarnishmentType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd_or_prtry.validate() { return Err(e); }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
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
		pub cd: Option<ExternalGarnishmentType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl GarnishmentType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// GenericIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericIdentification3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericIdentification3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericIdentification3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
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
	
	
	// GenericIdentification32 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericIdentification32 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<PartyType3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<PartyType4Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
		pub shrt_nm: Option<Max35Text>,
	}
	
	impl GenericIdentification32 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			if let Some(ref shrt_nm_value) = self.shrt_nm { if let Err(e) = shrt_nm_value.validate() { return Err(e); } }
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
	
	
	// GenericPersonIdentification2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericPersonIdentification2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max256Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericPersonIdentification2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GroupHeader116 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GroupHeader116 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
		pub msg_id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
		pub cre_dt_tm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgRcpt", skip_serializing_if = "Option::is_none") )]
		pub msg_rcpt: Option<PartyIdentification272>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none") )]
		pub msg_pgntn: Option<Pagination1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBizQry", skip_serializing_if = "Option::is_none") )]
		pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Max500Text>,
	}
	
	impl GroupHeader116 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.msg_id.validate() { return Err(e); }
			if let Some(ref msg_rcpt_value) = self.msg_rcpt { if let Err(e) = msg_rcpt_value.validate() { return Err(e); } }
			if let Some(ref msg_pgntn_value) = self.msg_pgntn { if let Err(e) = msg_pgntn_value.validate() { return Err(e); } }
			if let Some(ref orgnl_biz_qry_value) = self.orgnl_biz_qry { if let Err(e) = orgnl_biz_qry_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// IBAN2007Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ISINOct2015Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISINOct2015Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub isin_oct2015_identifier: String,
	}
	
	impl ISINOct2015Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(&self.isin_oct2015_identifier) {
				return Err(ValidationError::new(1005, "isin_oct2015_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ISO2ALanguageCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISO2ALanguageCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso2_a_language_code: String,
	}
	
	impl ISO2ALanguageCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[a-z]{2,2}").unwrap();
			if !pattern.is_match(&self.iso2_a_language_code) {
				return Err(ValidationError::new(1005, "iso2_a_language_code does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ISODate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ISOYear ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISOYear {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_year: String,
	}
	
	impl ISOYear {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ISOYearMonth ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISOYearMonth {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_year_month: String,
	}
	
	impl ISOYearMonth {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// IdentificationSource3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IdentificationSource3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl IdentificationSource3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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
			if let Some(ref fr_amt_value) = self.fr_amt { if let Err(e) = fr_amt_value.validate() { return Err(e); } }
			if let Some(ref to_amt_value) = self.to_amt { if let Err(e) = to_amt_value.validate() { return Err(e); } }
			if let Some(ref fr_to_amt_value) = self.fr_to_amt { if let Err(e) = fr_to_amt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ImpliedCurrencyAndAmount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ImpliedCurrencyAndAmount {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub implied_currency_and_amount: f64,
	}
	
	impl ImpliedCurrencyAndAmount {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.implied_currency_and_amount < 0.000000 {
				return Err(ValidationError::new(1003, "implied_currency_and_amount is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// InterestRecord2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InterestRecord2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd") )]
		pub cdt_dbt_ind: CreditDebitCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<InterestType1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
		pub rate: Option<Rate4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDt", skip_serializing_if = "Option::is_none") )]
		pub fr_to_dt: Option<DateTimePeriod1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
		pub tax: Option<TaxCharges2>,
	}
	
	impl InterestRecord2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Err(e) = self.cdt_dbt_ind.validate() { return Err(e); }
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref rate_value) = self.rate { if let Err(e) = rate_value.validate() { return Err(e); } }
			if let Some(ref fr_to_dt_value) = self.fr_to_dt { if let Err(e) = fr_to_dt_value.validate() { return Err(e); } }
			if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
			if let Some(ref tax_value) = self.tax { if let Err(e) = tax_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InterestType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InterestType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<InterestType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl InterestType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InterestType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum InterestType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "INDY") )]
		CodeINDY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OVRN") )]
		CodeOVRN,
	}
	
	impl InterestType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// LEIIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// LocalInstrument2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct LocalInstrument2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalLocalInstrument1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl LocalInstrument2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Max1025Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max1025Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max1025_text: String,
	}
	
	impl Max1025Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max1025_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max1025_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max1025_text.chars().count() > 1025 {
				return Err(ValidationError::new(1002, "max1025_text exceeds the maximum length of 1025".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max105Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max128Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max140Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max15NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max15NumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max15_numeric_text: String,
	}
	
	impl Max15NumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(&self.max15_numeric_text) {
				return Err(ValidationError::new(1005, "max15_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max15PlusSignedNumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max15PlusSignedNumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max15_plus_signed_numeric_text: String,
	}
	
	impl Max15PlusSignedNumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[\\+]{0,1}[0-9]{1,15}").unwrap();
			if !pattern.is_match(&self.max15_plus_signed_numeric_text) {
				return Err(ValidationError::new(1005, "max15_plus_signed_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max16Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max256Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max350Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max350Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max350_text: String,
	}
	
	impl Max350Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max350_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max350_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max350_text.chars().count() > 350 {
				return Err(ValidationError::new(1002, "max350_text exceeds the maximum length of 350".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max35Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max3NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max3NumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max3_numeric_text: String,
	}
	
	impl Max3NumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{1,3}").unwrap();
			if !pattern.is_match(&self.max3_numeric_text) {
				return Err(ValidationError::new(1005, "max3_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max4Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max500Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max500Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max500_text: String,
	}
	
	impl Max500Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max500_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max500_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max500_text.chars().count() > 500 {
				return Err(ValidationError::new(1002, "max500_text exceeds the maximum length of 500".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max5NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max70Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// MessageIdentification2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MessageIdentification2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none") )]
		pub msg_nm_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId", skip_serializing_if = "Option::is_none") )]
		pub msg_id: Option<Max35Text>,
	}
	
	impl MessageIdentification2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref msg_nm_id_value) = self.msg_nm_id { if let Err(e) = msg_nm_id_value.validate() { return Err(e); } }
			if let Some(ref msg_id_value) = self.msg_id { if let Err(e) = msg_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Min2Max3NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Min2Max3NumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub min2_max3_numeric_text: String,
	}
	
	impl Min2Max3NumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{2,3}").unwrap();
			if !pattern.is_match(&self.min2_max3_numeric_text) {
				return Err(ValidationError::new(1005, "min2_max3_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Min3Max4NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Min3Max4NumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub min3_max4_numeric_text: String,
	}
	
	impl Min3Max4NumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{3,4}").unwrap();
			if !pattern.is_match(&self.min3_max4_numeric_text) {
				return Err(ValidationError::new(1005, "min3_max4_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Min8Max28NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Min8Max28NumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub min8_max28_numeric_text: String,
	}
	
	impl Min8Max28NumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{8,28}").unwrap();
			if !pattern.is_match(&self.min8_max28_numeric_text) {
				return Err(ValidationError::new(1005, "min8_max28_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// NameAndAddress18 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct NameAndAddress18 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: Max140Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Adr") )]
		pub adr: PostalAddress27,
	}
	
	impl NameAndAddress18 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.nm.validate() { return Err(e); }
			if let Err(e) = self.adr.validate() { return Err(e); }
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
	
	
	// NonNegativeDecimalNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct NonNegativeDecimalNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub non_negative_decimal_number: f64,
	}
	
	impl NonNegativeDecimalNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.non_negative_decimal_number < 0.000000 {
				return Err(ValidationError::new(1003, "non_negative_decimal_number is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Number ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// NumberAndSumOfTransactions1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct NumberAndSumOfTransactions1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfNtries", skip_serializing_if = "Option::is_none") )]
		pub nb_of_ntries: Option<Max15NumericText>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sum", skip_serializing_if = "Option::is_none") )]
		pub sum: Option<f64>,
	}
	
	impl NumberAndSumOfTransactions1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nb_of_ntries_value) = self.nb_of_ntries { if let Err(e) = nb_of_ntries_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// NumberAndSumOfTransactions4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct NumberAndSumOfTransactions4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfNtries", skip_serializing_if = "Option::is_none") )]
		pub nb_of_ntries: Option<Max15NumericText>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sum", skip_serializing_if = "Option::is_none") )]
		pub sum: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNetNtry", skip_serializing_if = "Option::is_none") )]
		pub ttl_net_ntry: Option<AmountAndDirection35>,
	}
	
	impl NumberAndSumOfTransactions4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nb_of_ntries_value) = self.nb_of_ntries { if let Err(e) = nb_of_ntries_value.validate() { return Err(e); } }
			if let Some(ref ttl_net_ntry_value) = self.ttl_net_ntry { if let Err(e) = ttl_net_ntry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OnLineCapability1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum OnLineCapability1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "OFLN") )]
		CodeOFLN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ONLN") )]
		CodeONLN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SMON") )]
		CodeSMON,
	}
	
	impl OnLineCapability1Code {
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// OriginalAndCurrentQuantities1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OriginalAndCurrentQuantities1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FaceAmt") )]
		pub face_amt: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AmtsdVal") )]
		pub amtsd_val: f64,
	}
	
	impl OriginalAndCurrentQuantities1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// OriginalBusinessQuery1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OriginalBusinessQuery1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
		pub msg_id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none") )]
		pub msg_nm_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
		pub cre_dt_tm: Option<String>,
	}
	
	impl OriginalBusinessQuery1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.msg_id.validate() { return Err(e); }
			if let Some(ref msg_nm_id_value) = self.msg_nm_id { if let Err(e) = msg_nm_id_value.validate() { return Err(e); } }
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
	
	
	// OtherIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OtherIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sfx", skip_serializing_if = "Option::is_none") )]
		pub sfx: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: IdentificationSource3Choice,
	}
	
	impl OtherIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref sfx_value) = self.sfx { if let Err(e) = sfx_value.validate() { return Err(e); } }
			if let Err(e) = self.tp.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// POIComponentType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum POIComponentType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "SOFT") )]
		CodeSOFT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EMVK") )]
		CodeEMVK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EMVO") )]
		CodeEMVO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MRIT") )]
		CodeMRIT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CHIT") )]
		CodeCHIT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SECM") )]
		CodeSECM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PEDV") )]
		CodePEDV,
	}
	
	impl POIComponentType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
			if let Some(ref pty_value) = self.pty { if let Err(e) = pty_value.validate() { return Err(e); } }
			if let Some(ref agt_value) = self.agt { if let Err(e) = agt_value.validate() { return Err(e); } }
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
			if let Some(ref org_id_value) = self.org_id { if let Err(e) = org_id_value.validate() { return Err(e); } }
			if let Some(ref prvt_id_value) = self.prvt_id { if let Err(e) = prvt_id_value.validate() { return Err(e); } }
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
		pub nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress27>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Party52Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none") )]
		pub ctry_of_res: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none") )]
		pub ctct_dtls: Option<Contact13>,
	}
	
	impl PartyIdentification272 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref ctry_of_res_value) = self.ctry_of_res { if let Err(e) = ctry_of_res_value.validate() { return Err(e); } }
			if let Some(ref ctct_dtls_value) = self.ctct_dtls { if let Err(e) = ctct_dtls_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyType3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PartyType3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "OPOI") )]
		CodeOPOI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MERC") )]
		CodeMERC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACCP") )]
		CodeACCP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ITAG") )]
		CodeITAG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACQR") )]
		CodeACQR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CISS") )]
		CodeCISS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DLIS") )]
		CodeDLIS,
	}
	
	impl PartyType3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PartyType4Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PartyType4Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "MERC") )]
		CodeMERC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACCP") )]
		CodeACCP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ITAG") )]
		CodeITAG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACQR") )]
		CodeACQR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CISS") )]
		CodeCISS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TAXH") )]
		CodeTAXH,
	}
	
	impl PartyType4Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PaymentCard4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentCard4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PlainCardData", skip_serializing_if = "Option::is_none") )]
		pub plain_card_data: Option<PlainCardData1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CardCtryCd", skip_serializing_if = "Option::is_none") )]
		pub card_ctry_cd: Option<Exact3NumericText>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CardBrnd", skip_serializing_if = "Option::is_none") )]
		pub card_brnd: Option<GenericIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlCardData", skip_serializing_if = "Option::is_none") )]
		pub addtl_card_data: Option<Max70Text>,
	}
	
	impl PaymentCard4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref plain_card_data_value) = self.plain_card_data { if let Err(e) = plain_card_data_value.validate() { return Err(e); } }
			if let Some(ref card_ctry_cd_value) = self.card_ctry_cd { if let Err(e) = card_ctry_cd_value.validate() { return Err(e); } }
			if let Some(ref card_brnd_value) = self.card_brnd { if let Err(e) = card_brnd_value.validate() { return Err(e); } }
			if let Some(ref addtl_card_data_value) = self.addtl_card_data { if let Err(e) = addtl_card_data_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaymentContext3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentContext3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CardPres", skip_serializing_if = "Option::is_none") )]
		pub card_pres: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CrdhldrPres", skip_serializing_if = "Option::is_none") )]
		pub crdhldr_pres: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OnLineCntxt", skip_serializing_if = "Option::is_none") )]
		pub on_line_cntxt: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AttndncCntxt", skip_serializing_if = "Option::is_none") )]
		pub attndnc_cntxt: Option<AttendanceContext1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxEnvt", skip_serializing_if = "Option::is_none") )]
		pub tx_envt: Option<TransactionEnvironment1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxChanl", skip_serializing_if = "Option::is_none") )]
		pub tx_chanl: Option<TransactionChannel1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AttndntMsgCpbl", skip_serializing_if = "Option::is_none") )]
		pub attndnt_msg_cpbl: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AttndntLang", skip_serializing_if = "Option::is_none") )]
		pub attndnt_lang: Option<ISO2ALanguageCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CardDataNtryMd") )]
		pub card_data_ntry_md: CardDataReading1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FllbckInd", skip_serializing_if = "Option::is_none") )]
		pub fllbck_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AuthntcnMtd", skip_serializing_if = "Option::is_none") )]
		pub authntcn_mtd: Option<CardholderAuthentication2>,
	}
	
	impl PaymentContext3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref attndnc_cntxt_value) = self.attndnc_cntxt { if let Err(e) = attndnc_cntxt_value.validate() { return Err(e); } }
			if let Some(ref tx_envt_value) = self.tx_envt { if let Err(e) = tx_envt_value.validate() { return Err(e); } }
			if let Some(ref tx_chanl_value) = self.tx_chanl { if let Err(e) = tx_chanl_value.validate() { return Err(e); } }
			if let Some(ref attndnt_lang_value) = self.attndnt_lang { if let Err(e) = attndnt_lang_value.validate() { return Err(e); } }
			if let Err(e) = self.card_data_ntry_md.validate() { return Err(e); }
			if let Some(ref authntcn_mtd_value) = self.authntcn_mtd { if let Err(e) = authntcn_mtd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaymentReturnReason8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentReturnReason8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBkTxCd", skip_serializing_if = "Option::is_none") )]
		pub orgnl_bk_tx_cd: Option<BankTransactionCodeStructure4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
		pub orgtr: Option<PartyIdentification272>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<ReturnReason5Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Vec<Max105Text>>,
	}
	
	impl PaymentReturnReason8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref orgnl_bk_tx_cd_value) = self.orgnl_bk_tx_cd { if let Err(e) = orgnl_bk_tx_cd_value.validate() { return Err(e); } }
			if let Some(ref orgtr_value) = self.orgtr { if let Err(e) = orgtr_value.validate() { return Err(e); } }
			if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
			if let Some(ref instr_prty_value) = self.instr_prty { if let Err(e) = instr_prty_value.validate() { return Err(e); } }
			if let Some(ref clr_chanl_value) = self.clr_chanl { if let Err(e) = clr_chanl_value.validate() { return Err(e); } }
			if let Some(ref svc_lvl_vec) = self.svc_lvl { for item in svc_lvl_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref lcl_instrm_value) = self.lcl_instrm { if let Err(e) = lcl_instrm_value.validate() { return Err(e); } }
			if let Some(ref seq_tp_value) = self.seq_tp { if let Err(e) = seq_tp_value.validate() { return Err(e); } }
			if let Some(ref ctgy_purp_value) = self.ctgy_purp { if let Err(e) = ctgy_purp_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PercentageRate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref dt_and_plc_of_birth_value) = self.dt_and_plc_of_birth { if let Err(e) = dt_and_plc_of_birth_value.validate() { return Err(e); } }
			if let Some(ref othr_vec) = self.othr { for item in othr_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
		pub cd: Option<ExternalPersonIdentification1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl PersonIdentificationSchemeName1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PhoneNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// PlainCardData1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PlainCardData1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PAN") )]
		pub pan: Min8Max28NumericText,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CardSeqNb", skip_serializing_if = "Option::is_none") )]
		pub card_seq_nb: Option<Min2Max3NumericText>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FctvDt", skip_serializing_if = "Option::is_none") )]
		pub fctv_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt") )]
		pub xpry_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvcCd", skip_serializing_if = "Option::is_none") )]
		pub svc_cd: Option<Exact3NumericText>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TrckData", skip_serializing_if = "Option::is_none") )]
		pub trck_data: Option<Vec<TrackData1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CardSctyCd", skip_serializing_if = "Option::is_none") )]
		pub card_scty_cd: Option<CardSecurityInformation1>,
	}
	
	impl PlainCardData1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pan.validate() { return Err(e); }
			if let Some(ref card_seq_nb_value) = self.card_seq_nb { if let Err(e) = card_seq_nb_value.validate() { return Err(e); } }
			if let Some(ref svc_cd_value) = self.svc_cd { if let Err(e) = svc_cd_value.validate() { return Err(e); } }
			if let Some(ref trck_data_vec) = self.trck_data { for item in trck_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref card_scty_cd_value) = self.card_scty_cd { if let Err(e) = card_scty_cd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PointOfInteraction1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PointOfInteraction1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: GenericIdentification32,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SysNm", skip_serializing_if = "Option::is_none") )]
		pub sys_nm: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GrpId", skip_serializing_if = "Option::is_none") )]
		pub grp_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cpblties", skip_serializing_if = "Option::is_none") )]
		pub cpblties: Option<PointOfInteractionCapabilities1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cmpnt", skip_serializing_if = "Option::is_none") )]
		pub cmpnt: Option<Vec<PointOfInteractionComponent1>>,
	}
	
	impl PointOfInteraction1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref sys_nm_value) = self.sys_nm { if let Err(e) = sys_nm_value.validate() { return Err(e); } }
			if let Some(ref grp_id_value) = self.grp_id { if let Err(e) = grp_id_value.validate() { return Err(e); } }
			if let Some(ref cpblties_value) = self.cpblties { if let Err(e) = cpblties_value.validate() { return Err(e); } }
			if let Some(ref cmpnt_vec) = self.cmpnt { for item in cmpnt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// PointOfInteractionCapabilities1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PointOfInteractionCapabilities1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CardRdngCpblties", skip_serializing_if = "Option::is_none") )]
		pub card_rdng_cpblties: Option<Vec<CardDataReading1Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CrdhldrVrfctnCpblties", skip_serializing_if = "Option::is_none") )]
		pub crdhldr_vrfctn_cpblties: Option<Vec<CardholderVerificationCapability1Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OnLineCpblties", skip_serializing_if = "Option::is_none") )]
		pub on_line_cpblties: Option<OnLineCapability1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DispCpblties", skip_serializing_if = "Option::is_none") )]
		pub disp_cpblties: Option<Vec<DisplayCapabilities1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtLineWidth", skip_serializing_if = "Option::is_none") )]
		pub prt_line_width: Option<Max3NumericText>,
	}
	
	impl PointOfInteractionCapabilities1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref card_rdng_cpblties_vec) = self.card_rdng_cpblties { for item in card_rdng_cpblties_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref crdhldr_vrfctn_cpblties_vec) = self.crdhldr_vrfctn_cpblties { for item in crdhldr_vrfctn_cpblties_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref on_line_cpblties_value) = self.on_line_cpblties { if let Err(e) = on_line_cpblties_value.validate() { return Err(e); } }
			if let Some(ref disp_cpblties_vec) = self.disp_cpblties { for item in disp_cpblties_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref prt_line_width_value) = self.prt_line_width { if let Err(e) = prt_line_width_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PointOfInteractionComponent1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PointOfInteractionComponent1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "POICmpntTp") )]
		pub poi_cmpnt_tp: POIComponentType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ManfctrId", skip_serializing_if = "Option::is_none") )]
		pub manfctr_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mdl", skip_serializing_if = "Option::is_none") )]
		pub mdl: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VrsnNb", skip_serializing_if = "Option::is_none") )]
		pub vrsn_nb: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SrlNb", skip_serializing_if = "Option::is_none") )]
		pub srl_nb: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ApprvlNb", skip_serializing_if = "Option::is_none") )]
		pub apprvl_nb: Option<Vec<Max70Text>>,
	}
	
	impl PointOfInteractionComponent1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.poi_cmpnt_tp.validate() { return Err(e); }
			if let Some(ref manfctr_id_value) = self.manfctr_id { if let Err(e) = manfctr_id_value.validate() { return Err(e); } }
			if let Some(ref mdl_value) = self.mdl { if let Err(e) = mdl_value.validate() { return Err(e); } }
			if let Some(ref vrsn_nb_value) = self.vrsn_nb { if let Err(e) = vrsn_nb_value.validate() { return Err(e); } }
			if let Some(ref srl_nb_value) = self.srl_nb { if let Err(e) = srl_nb_value.validate() { return Err(e); } }
			if let Some(ref apprvl_nb_vec) = self.apprvl_nb { for item in apprvl_nb_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// PositiveNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct PositiveNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub positive_number: f64,
	}
	
	impl PositiveNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.positive_number < 1.000000 {
				return Err(ValidationError::new(1003, "positive_number is less than the minimum value of 1.000000".to_string()));
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
	
	
	// Price7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Price7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: YieldedOrValueType1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
		pub val: PriceRateOrAmount3Choice,
	}
	
	impl Price7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Err(e) = self.val.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// PriceRateOrAmount3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PriceRateOrAmount3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
		pub rate: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
	}
	
	impl PriceRateOrAmount3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PriceValueType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PriceValueType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DISC") )]
		CodeDISC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PREM") )]
		CodePREM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PARV") )]
		CodePARV,
	}
	
	impl PriceValueType1Code {
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
	
	
	// Product2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Product2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdctCd") )]
		pub pdct_cd: Max70Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none") )]
		pub unit_of_measr: Option<UnitOfMeasure1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdctQty", skip_serializing_if = "Option::is_none") )]
		pub pdct_qty: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitPric", skip_serializing_if = "Option::is_none") )]
		pub unit_pric: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdctAmt", skip_serializing_if = "Option::is_none") )]
		pub pdct_amt: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxTp", skip_serializing_if = "Option::is_none") )]
		pub tax_tp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlPdctInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_pdct_inf: Option<Max35Text>,
	}
	
	impl Product2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pdct_cd.validate() { return Err(e); }
			if let Some(ref unit_of_measr_value) = self.unit_of_measr { if let Err(e) = unit_of_measr_value.validate() { return Err(e); } }
			if let Some(ref tax_tp_value) = self.tax_tp { if let Err(e) = tax_tp_value.validate() { return Err(e); } }
			if let Some(ref addtl_pdct_inf_value) = self.addtl_pdct_inf { if let Err(e) = addtl_pdct_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ProprietaryAgent5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ProprietaryAgent5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Agt") )]
		pub agt: BranchAndFinancialInstitutionIdentification8,
	}
	
	impl ProprietaryAgent5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Err(e) = self.agt.validate() { return Err(e); }
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
	
	
	// ProprietaryDate3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ProprietaryDate3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt") )]
		pub dt: DateAndDateTime2Choice,
	}
	
	impl ProprietaryDate3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Err(e) = self.dt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ProprietaryParty6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ProprietaryParty6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pty") )]
		pub pty: Party50Choice,
	}
	
	impl ProprietaryParty6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Err(e) = self.pty.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ProprietaryPrice2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ProprietaryPrice2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pric") )]
		pub pric: ActiveOrHistoricCurrencyAndAmount,
	}
	
	impl ProprietaryPrice2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Err(e) = self.pric.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ProprietaryQuantity1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ProprietaryQuantity1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Qty") )]
		pub qty: Max35Text,
	}
	
	impl ProprietaryQuantity1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Err(e) = self.qty.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ProprietaryReference1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ProprietaryReference1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
		pub ref_attr: Max35Text,
	}
	
	impl ProprietaryReference1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Err(e) = self.ref_attr.validate() { return Err(e); }
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Purpose2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Purpose2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalPurpose1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl Purpose2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Rate4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Rate4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: RateType4Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VldtyRg", skip_serializing_if = "Option::is_none") )]
		pub vldty_rg: Option<ActiveOrHistoricCurrencyAndAmountRange2>,
	}
	
	impl Rate4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Some(ref vldty_rg_value) = self.vldty_rg { if let Err(e) = vldty_rg_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RateType4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RateType4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pctg", skip_serializing_if = "Option::is_none") )]
		pub pctg: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<Max35Text>,
	}
	
	impl RateType4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ReferredDocumentInformation8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReferredDocumentInformation8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<DocumentType1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nb", skip_serializing_if = "Option::is_none") )]
		pub nb: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdDt", skip_serializing_if = "Option::is_none") )]
		pub rltd_dt: Option<DateAndType1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LineDtls", skip_serializing_if = "Option::is_none") )]
		pub line_dtls: Option<Vec<DocumentLineInformation2>>,
	}
	
	impl ReferredDocumentInformation8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref nb_value) = self.nb { if let Err(e) = nb_value.validate() { return Err(e); } }
			if let Some(ref rltd_dt_value) = self.rltd_dt { if let Err(e) = rltd_dt_value.validate() { return Err(e); } }
			if let Some(ref line_dtls_vec) = self.line_dtls { for item in line_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// RemittanceAmount4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RemittanceAmount4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RmtAmtAndTp", skip_serializing_if = "Option::is_none") )]
		pub rmt_amt_and_tp: Option<Vec<DocumentAmount1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none") )]
		pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
	}
	
	impl RemittanceAmount4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rmt_amt_and_tp_vec) = self.rmt_amt_and_tp { for item in rmt_amt_and_tp_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref adjstmnt_amt_and_rsn_vec) = self.adjstmnt_amt_and_rsn { for item in adjstmnt_amt_and_rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// RemittanceInformation22 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RemittanceInformation22 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ustrd", skip_serializing_if = "Option::is_none") )]
		pub ustrd: Option<Vec<Max140Text>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Strd", skip_serializing_if = "Option::is_none") )]
		pub strd: Option<Vec<StructuredRemittanceInformation18>>,
	}
	
	impl RemittanceInformation22 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ustrd_vec) = self.ustrd { for item in ustrd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref strd_vec) = self.strd { for item in strd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// RemittanceLocation8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RemittanceLocation8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RmtId", skip_serializing_if = "Option::is_none") )]
		pub rmt_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RmtLctnDtls", skip_serializing_if = "Option::is_none") )]
		pub rmt_lctn_dtls: Option<Vec<RemittanceLocationData2>>,
	}
	
	impl RemittanceLocation8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rmt_id_value) = self.rmt_id { if let Err(e) = rmt_id_value.validate() { return Err(e); } }
			if let Some(ref rmt_lctn_dtls_vec) = self.rmt_lctn_dtls { for item in rmt_lctn_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// RemittanceLocationData2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RemittanceLocationData2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mtd") )]
		pub mtd: RemittanceLocationMethod2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ElctrncAdr", skip_serializing_if = "Option::is_none") )]
		pub elctrnc_adr: Option<Max2048Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<NameAndAddress18>,
	}
	
	impl RemittanceLocationData2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.mtd.validate() { return Err(e); }
			if let Some(ref elctrnc_adr_value) = self.elctrnc_adr { if let Err(e) = elctrnc_adr_value.validate() { return Err(e); } }
			if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RemittanceLocationMethod2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum RemittanceLocationMethod2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "FAXI") )]
		CodeFAXI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EDIC") )]
		CodeEDIC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "URID") )]
		CodeURID,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EMAL") )]
		CodeEMAL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "POST") )]
		CodePOST,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SMSM") )]
		CodeSMSM,
	}
	
	impl RemittanceLocationMethod2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ReportEntry14 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReportEntry14 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtryRef", skip_serializing_if = "Option::is_none") )]
		pub ntry_ref: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd") )]
		pub cdt_dbt_ind: CreditDebitCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RvslInd", skip_serializing_if = "Option::is_none") )]
		pub rvsl_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
		pub sts: EntryStatus1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BookgDt", skip_serializing_if = "Option::is_none") )]
		pub bookg_dt: Option<DateAndDateTime2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ValDt", skip_serializing_if = "Option::is_none") )]
		pub val_dt: Option<DateAndDateTime2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none") )]
		pub acct_svcr_ref: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Avlbty", skip_serializing_if = "Option::is_none") )]
		pub avlbty: Option<Vec<CashAvailability1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BkTxCd") )]
		pub bk_tx_cd: BankTransactionCodeStructure4,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ComssnWvrInd", skip_serializing_if = "Option::is_none") )]
		pub comssn_wvr_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInfInd", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf_ind: Option<MessageIdentification2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AmtDtls", skip_serializing_if = "Option::is_none") )]
		pub amt_dtls: Option<AmountAndCurrencyExchange4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Chrgs", skip_serializing_if = "Option::is_none") )]
		pub chrgs: Option<Charges15>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TechInptChanl", skip_serializing_if = "Option::is_none") )]
		pub tech_inpt_chanl: Option<TechnicalInputChannel1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Intrst", skip_serializing_if = "Option::is_none") )]
		pub intrst: Option<TransactionInterest4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CardTx", skip_serializing_if = "Option::is_none") )]
		pub card_tx: Option<CardEntry5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtryDtls", skip_serializing_if = "Option::is_none") )]
		pub ntry_dtls: Option<Vec<EntryDetails13>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlNtryInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_ntry_inf: Option<Max500Text>,
	}
	
	impl ReportEntry14 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ntry_ref_value) = self.ntry_ref { if let Err(e) = ntry_ref_value.validate() { return Err(e); } }
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Err(e) = self.cdt_dbt_ind.validate() { return Err(e); }
			if let Err(e) = self.sts.validate() { return Err(e); }
			if let Some(ref bookg_dt_value) = self.bookg_dt { if let Err(e) = bookg_dt_value.validate() { return Err(e); } }
			if let Some(ref val_dt_value) = self.val_dt { if let Err(e) = val_dt_value.validate() { return Err(e); } }
			if let Some(ref acct_svcr_ref_value) = self.acct_svcr_ref { if let Err(e) = acct_svcr_ref_value.validate() { return Err(e); } }
			if let Some(ref avlbty_vec) = self.avlbty { for item in avlbty_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Err(e) = self.bk_tx_cd.validate() { return Err(e); }
			if let Some(ref addtl_inf_ind_value) = self.addtl_inf_ind { if let Err(e) = addtl_inf_ind_value.validate() { return Err(e); } }
			if let Some(ref amt_dtls_value) = self.amt_dtls { if let Err(e) = amt_dtls_value.validate() { return Err(e); } }
			if let Some(ref chrgs_value) = self.chrgs { if let Err(e) = chrgs_value.validate() { return Err(e); } }
			if let Some(ref tech_inpt_chanl_value) = self.tech_inpt_chanl { if let Err(e) = tech_inpt_chanl_value.validate() { return Err(e); } }
			if let Some(ref intrst_value) = self.intrst { if let Err(e) = intrst_value.validate() { return Err(e); } }
			if let Some(ref card_tx_value) = self.card_tx { if let Err(e) = card_tx_value.validate() { return Err(e); } }
			if let Some(ref ntry_dtls_vec) = self.ntry_dtls { for item in ntry_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref addtl_ntry_inf_value) = self.addtl_ntry_inf { if let Err(e) = addtl_ntry_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ReportingSource1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReportingSource1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalReportingSource1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl ReportingSource1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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
		pub cd: Option<ExternalReturnReason1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl ReturnReason5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<GenericIdentification30>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max70Text>,
	}
	
	impl SecuritiesAccount19 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SecurityIdentification19 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecurityIdentification19 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
		pub isin: Option<ISINOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
		pub othr_id: Option<Vec<OtherIdentification1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max140Text>,
	}
	
	impl SecurityIdentification19 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
			if let Some(ref othr_id_vec) = self.othr_id { for item in othr_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SequenceRange1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SequenceRange1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrSeq") )]
		pub fr_seq: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToSeq") )]
		pub to_seq: Max35Text,
	}
	
	impl SequenceRange1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.fr_seq.validate() { return Err(e); }
			if let Err(e) = self.to_seq.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SequenceRange1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SequenceRange1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrSeq", skip_serializing_if = "Option::is_none") )]
		pub fr_seq: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToSeq", skip_serializing_if = "Option::is_none") )]
		pub to_seq: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrToSeq", skip_serializing_if = "Option::is_none") )]
		pub fr_to_seq: Option<Vec<SequenceRange1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EQSeq", skip_serializing_if = "Option::is_none") )]
		pub eq_seq: Option<Vec<Max35Text>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NEQSeq", skip_serializing_if = "Option::is_none") )]
		pub neq_seq: Option<Vec<Max35Text>>,
	}
	
	impl SequenceRange1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref fr_seq_value) = self.fr_seq { if let Err(e) = fr_seq_value.validate() { return Err(e); } }
			if let Some(ref to_seq_value) = self.to_seq { if let Err(e) = to_seq_value.validate() { return Err(e); } }
			if let Some(ref fr_to_seq_vec) = self.fr_to_seq { for item in fr_to_seq_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref eq_seq_vec) = self.eq_seq { for item in eq_seq_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref neq_seq_vec) = self.neq_seq { for item in neq_seq_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
		pub cd: Option<ExternalServiceLevel1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl ServiceLevel8Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// StructuredRemittanceInformation18 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct StructuredRemittanceInformation18 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RfrdDocInf", skip_serializing_if = "Option::is_none") )]
		pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation8>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RfrdDocAmt", skip_serializing_if = "Option::is_none") )]
		pub rfrd_doc_amt: Option<RemittanceAmount4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrRefInf", skip_serializing_if = "Option::is_none") )]
		pub cdtr_ref_inf: Option<CreditorReferenceInformation3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Invcr", skip_serializing_if = "Option::is_none") )]
		pub invcr: Option<PartyIdentification272>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Invcee", skip_serializing_if = "Option::is_none") )]
		pub invcee: Option<PartyIdentification272>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRmt", skip_serializing_if = "Option::is_none") )]
		pub tax_rmt: Option<TaxData1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GrnshmtRmt", skip_serializing_if = "Option::is_none") )]
		pub grnshmt_rmt: Option<Garnishment4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRmtInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_rmt_inf: Option<Vec<Max140Text>>,
	}
	
	impl StructuredRemittanceInformation18 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rfrd_doc_inf_vec) = self.rfrd_doc_inf { for item in rfrd_doc_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref rfrd_doc_amt_value) = self.rfrd_doc_amt { if let Err(e) = rfrd_doc_amt_value.validate() { return Err(e); } }
			if let Some(ref cdtr_ref_inf_value) = self.cdtr_ref_inf { if let Err(e) = cdtr_ref_inf_value.validate() { return Err(e); } }
			if let Some(ref invcr_value) = self.invcr { if let Err(e) = invcr_value.validate() { return Err(e); } }
			if let Some(ref invcee_value) = self.invcee { if let Err(e) = invcee_value.validate() { return Err(e); } }
			if let Some(ref tax_rmt_value) = self.tax_rmt { if let Err(e) = tax_rmt_value.validate() { return Err(e); } }
			if let Some(ref grnshmt_rmt_value) = self.grnshmt_rmt { if let Err(e) = grnshmt_rmt_value.validate() { return Err(e); } }
			if let Some(ref addtl_rmt_inf_vec) = self.addtl_rmt_inf { for item in addtl_rmt_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
		pub plc_and_nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Envlp") )]
		pub envlp: SupplementaryDataEnvelope1,
	}
	
	impl SupplementaryData1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref plc_and_nm_value) = self.plc_and_nm { if let Err(e) = plc_and_nm_value.validate() { return Err(e); } }
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
	
	
	// TaxAmount3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxAmount3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
		pub rate: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxblBaseAmt", skip_serializing_if = "Option::is_none") )]
		pub taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none") )]
		pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dtls", skip_serializing_if = "Option::is_none") )]
		pub dtls: Option<Vec<TaxRecordDetails3>>,
	}
	
	impl TaxAmount3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref taxbl_base_amt_value) = self.taxbl_base_amt { if let Err(e) = taxbl_base_amt_value.validate() { return Err(e); } }
			if let Some(ref ttl_amt_value) = self.ttl_amt { if let Err(e) = ttl_amt_value.validate() { return Err(e); } }
			if let Some(ref dtls_vec) = self.dtls { for item in dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
		pub titl: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max140Text>,
	}
	
	impl TaxAuthorisation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref titl_value) = self.titl { if let Err(e) = titl_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TaxCharges2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxCharges2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
		pub rate: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	}
	
	impl TaxCharges2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TaxData1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxData1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
		pub cdtr: Option<TaxParty1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
		pub dbtr: Option<TaxParty2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
		pub ultmt_dbtr: Option<TaxParty2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdmstnZone", skip_serializing_if = "Option::is_none") )]
		pub admstn_zone: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefNb", skip_serializing_if = "Option::is_none") )]
		pub ref_nb: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mtd", skip_serializing_if = "Option::is_none") )]
		pub mtd: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlTaxblBaseAmt", skip_serializing_if = "Option::is_none") )]
		pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlTaxAmt", skip_serializing_if = "Option::is_none") )]
		pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
		pub dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SeqNb", skip_serializing_if = "Option::is_none") )]
		pub seq_nb: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rcrd", skip_serializing_if = "Option::is_none") )]
		pub rcrd: Option<Vec<TaxRecord3>>,
	}
	
	impl TaxData1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cdtr_value) = self.cdtr { if let Err(e) = cdtr_value.validate() { return Err(e); } }
			if let Some(ref dbtr_value) = self.dbtr { if let Err(e) = dbtr_value.validate() { return Err(e); } }
			if let Some(ref ultmt_dbtr_value) = self.ultmt_dbtr { if let Err(e) = ultmt_dbtr_value.validate() { return Err(e); } }
			if let Some(ref admstn_zone_value) = self.admstn_zone { if let Err(e) = admstn_zone_value.validate() { return Err(e); } }
			if let Some(ref ref_nb_value) = self.ref_nb { if let Err(e) = ref_nb_value.validate() { return Err(e); } }
			if let Some(ref mtd_value) = self.mtd { if let Err(e) = mtd_value.validate() { return Err(e); } }
			if let Some(ref ttl_taxbl_base_amt_value) = self.ttl_taxbl_base_amt { if let Err(e) = ttl_taxbl_base_amt_value.validate() { return Err(e); } }
			if let Some(ref ttl_tax_amt_value) = self.ttl_tax_amt { if let Err(e) = ttl_tax_amt_value.validate() { return Err(e); } }
			if let Some(ref rcrd_vec) = self.rcrd { for item in rcrd_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
		pub tax_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RegnId", skip_serializing_if = "Option::is_none") )]
		pub regn_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxTp", skip_serializing_if = "Option::is_none") )]
		pub tax_tp: Option<Max35Text>,
	}
	
	impl TaxParty1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tax_id_value) = self.tax_id { if let Err(e) = tax_id_value.validate() { return Err(e); } }
			if let Some(ref regn_id_value) = self.regn_id { if let Err(e) = regn_id_value.validate() { return Err(e); } }
			if let Some(ref tax_tp_value) = self.tax_tp { if let Err(e) = tax_tp_value.validate() { return Err(e); } }
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
		pub tax_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RegnId", skip_serializing_if = "Option::is_none") )]
		pub regn_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxTp", skip_serializing_if = "Option::is_none") )]
		pub tax_tp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Authstn", skip_serializing_if = "Option::is_none") )]
		pub authstn: Option<TaxAuthorisation1>,
	}
	
	impl TaxParty2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tax_id_value) = self.tax_id { if let Err(e) = tax_id_value.validate() { return Err(e); } }
			if let Some(ref regn_id_value) = self.regn_id { if let Err(e) = regn_id_value.validate() { return Err(e); } }
			if let Some(ref tax_tp_value) = self.tax_tp { if let Err(e) = tax_tp_value.validate() { return Err(e); } }
			if let Some(ref authstn_value) = self.authstn { if let Err(e) = authstn_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TaxPeriod3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxPeriod3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Yr", skip_serializing_if = "Option::is_none") )]
		pub yr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<TaxRecordPeriod1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDt", skip_serializing_if = "Option::is_none") )]
		pub fr_to_dt: Option<DatePeriod2>,
	}
	
	impl TaxPeriod3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref fr_to_dt_value) = self.fr_to_dt { if let Err(e) = fr_to_dt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TaxRecord3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxRecord3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctgy", skip_serializing_if = "Option::is_none") )]
		pub ctgy: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtgyDtls", skip_serializing_if = "Option::is_none") )]
		pub ctgy_dtls: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrSts", skip_serializing_if = "Option::is_none") )]
		pub dbtr_sts: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CertId", skip_serializing_if = "Option::is_none") )]
		pub cert_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none") )]
		pub frms_cd: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
		pub prd: Option<TaxPeriod3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none") )]
		pub tax_amt: Option<TaxAmount3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Max140Text>,
	}
	
	impl TaxRecord3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref ctgy_value) = self.ctgy { if let Err(e) = ctgy_value.validate() { return Err(e); } }
			if let Some(ref ctgy_dtls_value) = self.ctgy_dtls { if let Err(e) = ctgy_dtls_value.validate() { return Err(e); } }
			if let Some(ref dbtr_sts_value) = self.dbtr_sts { if let Err(e) = dbtr_sts_value.validate() { return Err(e); } }
			if let Some(ref cert_id_value) = self.cert_id { if let Err(e) = cert_id_value.validate() { return Err(e); } }
			if let Some(ref frms_cd_value) = self.frms_cd { if let Err(e) = frms_cd_value.validate() { return Err(e); } }
			if let Some(ref prd_value) = self.prd { if let Err(e) = prd_value.validate() { return Err(e); } }
			if let Some(ref tax_amt_value) = self.tax_amt { if let Err(e) = tax_amt_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TaxRecordDetails3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxRecordDetails3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
		pub prd: Option<TaxPeriod3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
	}
	
	impl TaxRecordDetails3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref prd_value) = self.prd { if let Err(e) = prd_value.validate() { return Err(e); } }
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
	
	
	// TechnicalInputChannel1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TechnicalInputChannel1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalTechnicalInputChannel1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl TechnicalInputChannel1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TotalTransactions6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TotalTransactions6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNtries", skip_serializing_if = "Option::is_none") )]
		pub ttl_ntries: Option<NumberAndSumOfTransactions4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlCdtNtries", skip_serializing_if = "Option::is_none") )]
		pub ttl_cdt_ntries: Option<NumberAndSumOfTransactions1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlDbtNtries", skip_serializing_if = "Option::is_none") )]
		pub ttl_dbt_ntries: Option<NumberAndSumOfTransactions1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNtriesPerBkTxCd", skip_serializing_if = "Option::is_none") )]
		pub ttl_ntries_per_bk_tx_cd: Option<Vec<TotalsPerBankTransactionCode5>>,
	}
	
	impl TotalTransactions6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ttl_ntries_value) = self.ttl_ntries { if let Err(e) = ttl_ntries_value.validate() { return Err(e); } }
			if let Some(ref ttl_cdt_ntries_value) = self.ttl_cdt_ntries { if let Err(e) = ttl_cdt_ntries_value.validate() { return Err(e); } }
			if let Some(ref ttl_dbt_ntries_value) = self.ttl_dbt_ntries { if let Err(e) = ttl_dbt_ntries_value.validate() { return Err(e); } }
			if let Some(ref ttl_ntries_per_bk_tx_cd_vec) = self.ttl_ntries_per_bk_tx_cd { for item in ttl_ntries_per_bk_tx_cd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TotalsPerBankTransactionCode5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TotalsPerBankTransactionCode5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfNtries", skip_serializing_if = "Option::is_none") )]
		pub nb_of_ntries: Option<Max15NumericText>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sum", skip_serializing_if = "Option::is_none") )]
		pub sum: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNetNtry", skip_serializing_if = "Option::is_none") )]
		pub ttl_net_ntry: Option<AmountAndDirection35>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtNtries", skip_serializing_if = "Option::is_none") )]
		pub cdt_ntries: Option<NumberAndSumOfTransactions1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtNtries", skip_serializing_if = "Option::is_none") )]
		pub dbt_ntries: Option<NumberAndSumOfTransactions1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FcstInd", skip_serializing_if = "Option::is_none") )]
		pub fcst_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BkTxCd") )]
		pub bk_tx_cd: BankTransactionCodeStructure4,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Avlbty", skip_serializing_if = "Option::is_none") )]
		pub avlbty: Option<Vec<CashAvailability1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
		pub dt: Option<DateAndDateTime2Choice>,
	}
	
	impl TotalsPerBankTransactionCode5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nb_of_ntries_value) = self.nb_of_ntries { if let Err(e) = nb_of_ntries_value.validate() { return Err(e); } }
			if let Some(ref ttl_net_ntry_value) = self.ttl_net_ntry { if let Err(e) = ttl_net_ntry_value.validate() { return Err(e); } }
			if let Some(ref cdt_ntries_value) = self.cdt_ntries { if let Err(e) = cdt_ntries_value.validate() { return Err(e); } }
			if let Some(ref dbt_ntries_value) = self.dbt_ntries { if let Err(e) = dbt_ntries_value.validate() { return Err(e); } }
			if let Err(e) = self.bk_tx_cd.validate() { return Err(e); }
			if let Some(ref avlbty_vec) = self.avlbty { for item in avlbty_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref dt_value) = self.dt { if let Err(e) = dt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TrackData1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TrackData1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TrckNb", skip_serializing_if = "Option::is_none") )]
		pub trck_nb: Option<Exact1NumericText>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TrckVal") )]
		pub trck_val: Max140Text,
	}
	
	impl TrackData1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref trck_nb_value) = self.trck_nb { if let Err(e) = trck_nb_value.validate() { return Err(e); } }
			if let Err(e) = self.trck_val.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// TransactionAgents6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TransactionAgents6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none") )]
		pub instg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none") )]
		pub instd_agt: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
		pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
		pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none") )]
		pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none") )]
		pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none") )]
		pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RcvgAgt", skip_serializing_if = "Option::is_none") )]
		pub rcvg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DlvrgAgt", skip_serializing_if = "Option::is_none") )]
		pub dlvrg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IssgAgt", skip_serializing_if = "Option::is_none") )]
		pub issg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmPlc", skip_serializing_if = "Option::is_none") )]
		pub sttlm_plc: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Vec<ProprietaryAgent5>>,
	}
	
	impl TransactionAgents6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref instg_agt_value) = self.instg_agt { if let Err(e) = instg_agt_value.validate() { return Err(e); } }
			if let Some(ref instd_agt_value) = self.instd_agt { if let Err(e) = instd_agt_value.validate() { return Err(e); } }
			if let Some(ref dbtr_agt_value) = self.dbtr_agt { if let Err(e) = dbtr_agt_value.validate() { return Err(e); } }
			if let Some(ref cdtr_agt_value) = self.cdtr_agt { if let Err(e) = cdtr_agt_value.validate() { return Err(e); } }
			if let Some(ref intrmy_agt1_value) = self.intrmy_agt1 { if let Err(e) = intrmy_agt1_value.validate() { return Err(e); } }
			if let Some(ref intrmy_agt2_value) = self.intrmy_agt2 { if let Err(e) = intrmy_agt2_value.validate() { return Err(e); } }
			if let Some(ref intrmy_agt3_value) = self.intrmy_agt3 { if let Err(e) = intrmy_agt3_value.validate() { return Err(e); } }
			if let Some(ref rcvg_agt_value) = self.rcvg_agt { if let Err(e) = rcvg_agt_value.validate() { return Err(e); } }
			if let Some(ref dlvrg_agt_value) = self.dlvrg_agt { if let Err(e) = dlvrg_agt_value.validate() { return Err(e); } }
			if let Some(ref issg_agt_value) = self.issg_agt { if let Err(e) = issg_agt_value.validate() { return Err(e); } }
			if let Some(ref sttlm_plc_value) = self.sttlm_plc { if let Err(e) = sttlm_plc_value.validate() { return Err(e); } }
			if let Some(ref prtry_vec) = self.prtry { for item in prtry_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TransactionChannel1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum TransactionChannel1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "MAIL") )]
		CodeMAIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TLPH") )]
		CodeTLPH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ECOM") )]
		CodeECOM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TVPY") )]
		CodeTVPY,
	}
	
	impl TransactionChannel1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TransactionDates3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TransactionDates3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AccptncDtTm", skip_serializing_if = "Option::is_none") )]
		pub accptnc_dt_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradActvtyCtrctlSttlmDt", skip_serializing_if = "Option::is_none") )]
		pub trad_actvty_ctrctl_sttlm_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradDt", skip_serializing_if = "Option::is_none") )]
		pub trad_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
		pub intr_bk_sttlm_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
		pub start_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt", skip_serializing_if = "Option::is_none") )]
		pub end_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxDtTm", skip_serializing_if = "Option::is_none") )]
		pub tx_dt_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Vec<ProprietaryDate3>>,
	}
	
	impl TransactionDates3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref prtry_vec) = self.prtry { for item in prtry_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TransactionEnvironment1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum TransactionEnvironment1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "MERC") )]
		CodeMERC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRIV") )]
		CodePRIV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PUBL") )]
		CodePUBL,
	}
	
	impl TransactionEnvironment1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TransactionIdentifier1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TransactionIdentifier1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxDtTm") )]
		pub tx_dt_tm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxRef") )]
		pub tx_ref: Max35Text,
	}
	
	impl TransactionIdentifier1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tx_ref.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// TransactionInterest4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TransactionInterest4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlIntrstAndTaxAmt", skip_serializing_if = "Option::is_none") )]
		pub ttl_intrst_and_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rcrd", skip_serializing_if = "Option::is_none") )]
		pub rcrd: Option<Vec<InterestRecord2>>,
	}
	
	impl TransactionInterest4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ttl_intrst_and_tax_amt_value) = self.ttl_intrst_and_tax_amt { if let Err(e) = ttl_intrst_and_tax_amt_value.validate() { return Err(e); } }
			if let Some(ref rcrd_vec) = self.rcrd { for item in rcrd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TransactionParties12 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TransactionParties12 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty", skip_serializing_if = "Option::is_none") )]
		pub initg_pty: Option<Party50Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
		pub dbtr: Option<Party50Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
		pub dbtr_acct: Option<CashAccount40>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
		pub ultmt_dbtr: Option<Party50Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
		pub cdtr: Option<Party50Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
		pub cdtr_acct: Option<CashAccount40>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
		pub ultmt_cdtr: Option<Party50Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradgPty", skip_serializing_if = "Option::is_none") )]
		pub tradg_pty: Option<Party50Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Vec<ProprietaryParty6>>,
	}
	
	impl TransactionParties12 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref initg_pty_value) = self.initg_pty { if let Err(e) = initg_pty_value.validate() { return Err(e); } }
			if let Some(ref dbtr_value) = self.dbtr { if let Err(e) = dbtr_value.validate() { return Err(e); } }
			if let Some(ref dbtr_acct_value) = self.dbtr_acct { if let Err(e) = dbtr_acct_value.validate() { return Err(e); } }
			if let Some(ref ultmt_dbtr_value) = self.ultmt_dbtr { if let Err(e) = ultmt_dbtr_value.validate() { return Err(e); } }
			if let Some(ref cdtr_value) = self.cdtr { if let Err(e) = cdtr_value.validate() { return Err(e); } }
			if let Some(ref cdtr_acct_value) = self.cdtr_acct { if let Err(e) = cdtr_acct_value.validate() { return Err(e); } }
			if let Some(ref ultmt_cdtr_value) = self.ultmt_cdtr { if let Err(e) = ultmt_cdtr_value.validate() { return Err(e); } }
			if let Some(ref tradg_pty_value) = self.tradg_pty { if let Err(e) = tradg_pty_value.validate() { return Err(e); } }
			if let Some(ref prtry_vec) = self.prtry { for item in prtry_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TransactionPrice4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TransactionPrice4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealPric", skip_serializing_if = "Option::is_none") )]
		pub deal_pric: Option<Price7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Vec<ProprietaryPrice2>>,
	}
	
	impl TransactionPrice4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref deal_pric_value) = self.deal_pric { if let Err(e) = deal_pric_value.validate() { return Err(e); } }
			if let Some(ref prtry_vec) = self.prtry { for item in prtry_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TransactionQuantities3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TransactionQuantities3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Qty", skip_serializing_if = "Option::is_none") )]
		pub qty: Option<FinancialInstrumentQuantity1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlAndCurFaceAmt", skip_serializing_if = "Option::is_none") )]
		pub orgnl_and_cur_face_amt: Option<OriginalAndCurrentQuantities1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<ProprietaryQuantity1>,
	}
	
	impl TransactionQuantities3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref qty_value) = self.qty { if let Err(e) = qty_value.validate() { return Err(e); } }
			if let Some(ref orgnl_and_cur_face_amt_value) = self.orgnl_and_cur_face_amt { if let Err(e) = orgnl_and_cur_face_amt_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TransactionReferences6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TransactionReferences6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId", skip_serializing_if = "Option::is_none") )]
		pub msg_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none") )]
		pub acct_svcr_ref: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInfId", skip_serializing_if = "Option::is_none") )]
		pub pmt_inf_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstrId", skip_serializing_if = "Option::is_none") )]
		pub instr_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none") )]
		pub end_to_end_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
		pub uetr: Option<UUIDv4Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
		pub tx_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MndtId", skip_serializing_if = "Option::is_none") )]
		pub mndt_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ChqNb", skip_serializing_if = "Option::is_none") )]
		pub chq_nb: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none") )]
		pub clr_sys_ref: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOwnrTxId", skip_serializing_if = "Option::is_none") )]
		pub acct_ownr_tx_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none") )]
		pub acct_svcr_tx_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none") )]
		pub mkt_infrstrctr_tx_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgId", skip_serializing_if = "Option::is_none") )]
		pub prcg_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Vec<ProprietaryReference1>>,
	}
	
	impl TransactionReferences6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref msg_id_value) = self.msg_id { if let Err(e) = msg_id_value.validate() { return Err(e); } }
			if let Some(ref acct_svcr_ref_value) = self.acct_svcr_ref { if let Err(e) = acct_svcr_ref_value.validate() { return Err(e); } }
			if let Some(ref pmt_inf_id_value) = self.pmt_inf_id { if let Err(e) = pmt_inf_id_value.validate() { return Err(e); } }
			if let Some(ref instr_id_value) = self.instr_id { if let Err(e) = instr_id_value.validate() { return Err(e); } }
			if let Some(ref end_to_end_id_value) = self.end_to_end_id { if let Err(e) = end_to_end_id_value.validate() { return Err(e); } }
			if let Some(ref uetr_value) = self.uetr { if let Err(e) = uetr_value.validate() { return Err(e); } }
			if let Some(ref tx_id_value) = self.tx_id { if let Err(e) = tx_id_value.validate() { return Err(e); } }
			if let Some(ref mndt_id_value) = self.mndt_id { if let Err(e) = mndt_id_value.validate() { return Err(e); } }
			if let Some(ref chq_nb_value) = self.chq_nb { if let Err(e) = chq_nb_value.validate() { return Err(e); } }
			if let Some(ref clr_sys_ref_value) = self.clr_sys_ref { if let Err(e) = clr_sys_ref_value.validate() { return Err(e); } }
			if let Some(ref acct_ownr_tx_id_value) = self.acct_ownr_tx_id { if let Err(e) = acct_ownr_tx_id_value.validate() { return Err(e); } }
			if let Some(ref acct_svcr_tx_id_value) = self.acct_svcr_tx_id { if let Err(e) = acct_svcr_tx_id_value.validate() { return Err(e); } }
			if let Some(ref mkt_infrstrctr_tx_id_value) = self.mkt_infrstrctr_tx_id { if let Err(e) = mkt_infrstrctr_tx_id_value.validate() { return Err(e); } }
			if let Some(ref prcg_id_value) = self.prcg_id { if let Err(e) = prcg_id_value.validate() { return Err(e); } }
			if let Some(ref prtry_vec) = self.prtry { for item in prtry_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TrueFalseIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// UUIDv4Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct UUIDv4Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub uui_dv4_identifier: String,
	}
	
	impl UUIDv4Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(&self.uui_dv4_identifier) {
				return Err(ValidationError::new(1005, "uui_dv4_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// UnitOfMeasure1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum UnitOfMeasure1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PIEC") )]
		CodePIEC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TONS") )]
		CodeTONS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FOOT") )]
		CodeFOOT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GBGA") )]
		CodeGBGA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "USGA") )]
		CodeUSGA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GRAM") )]
		CodeGRAM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INCH") )]
		CodeINCH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KILO") )]
		CodeKILO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PUND") )]
		CodePUND,
		#[cfg_attr( feature = "derive_serde", serde(rename = "METR") )]
		CodeMETR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CMET") )]
		CodeCMET,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MMET") )]
		CodeMMET,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LITR") )]
		CodeLITR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CELI") )]
		CodeCELI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MILI") )]
		CodeMILI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GBOU") )]
		CodeGBOU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "USOU") )]
		CodeUSOU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GBQA") )]
		CodeGBQA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "USQA") )]
		CodeUSQA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GBPI") )]
		CodeGBPI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "USPI") )]
		CodeUSPI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MILE") )]
		CodeMILE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KMET") )]
		CodeKMET,
		#[cfg_attr( feature = "derive_serde", serde(rename = "YARD") )]
		CodeYARD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SQKI") )]
		CodeSQKI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HECT") )]
		CodeHECT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ARES") )]
		CodeARES,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SMET") )]
		CodeSMET,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SCMT") )]
		CodeSCMT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SMIL") )]
		CodeSMIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SQMI") )]
		CodeSQMI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SQYA") )]
		CodeSQYA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SQFO") )]
		CodeSQFO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SQIN") )]
		CodeSQIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACRE") )]
		CodeACRE,
	}
	
	impl UnitOfMeasure1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// UserInterface2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum UserInterface2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "MDSP") )]
		CodeMDSP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CDSP") )]
		CodeCDSP,
	}
	
	impl UserInterface2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// YesNoIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// YieldedOrValueType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct YieldedOrValueType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Yldd", skip_serializing_if = "Option::is_none") )]
		pub yldd: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ValTp", skip_serializing_if = "Option::is_none") )]
		pub val_tp: Option<PriceValueType1Code>,
	}
	
	impl YieldedOrValueType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val_tp_value) = self.val_tp { if let Err(e) = val_tp_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
}