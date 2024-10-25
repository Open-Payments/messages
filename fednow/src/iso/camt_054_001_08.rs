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
	pub rsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
	pub tax: Option<TaxCharges2>,
}

impl AccountInterest4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.rate { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.fr_to_dt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.rsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rsn exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tax { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// AccountNotification17 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AccountNotification17 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtfctnPgntn", skip_serializing_if = "Option::is_none") )]
	pub ntfctn_pgntn: Option<Pagination1>,
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
	pub acct: CashAccount39,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdAcct", skip_serializing_if = "Option::is_none") )]
	pub rltd_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Intrst", skip_serializing_if = "Option::is_none") )]
	pub intrst: Option<Vec<AccountInterest4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxsSummry", skip_serializing_if = "Option::is_none") )]
	pub txs_summry: Option<TotalTransactions6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ntry", skip_serializing_if = "Option::is_none") )]
	pub ntry: Option<Vec<ReportEntry10>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlNtfctnInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_ntfctn_inf: Option<String>,
}

impl AccountNotification17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.ntfctn_pgntn { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.rptg_seq { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.fr_to_dt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.cpy_dplct_ind { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.rptg_src { if let Err(e) = val.validate() { return Err(e); } }
		if let Err(e) = self.acct.validate() { return Err(e); }
		if let Some(ref val) = self.rltd_acct { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.intrst { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.txs_summry { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.ntry { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.addtl_ntfctn_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_ntfctn_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 500 {
				return Err(ValidationError::new(1002, "addtl_ntfctn_inf exceeds the maximum length of 500".to_string()));
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


// AmountAndCurrencyExchange3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmountAndCurrencyExchange3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none") )]
	pub instd_amt: Option<AmountAndCurrencyExchangeDetails3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxAmt", skip_serializing_if = "Option::is_none") )]
	pub tx_amt: Option<AmountAndCurrencyExchangeDetails3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CntrValAmt", skip_serializing_if = "Option::is_none") )]
	pub cntr_val_amt: Option<AmountAndCurrencyExchangeDetails3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnncdPstngAmt", skip_serializing_if = "Option::is_none") )]
	pub anncd_pstng_amt: Option<AmountAndCurrencyExchangeDetails3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryAmt", skip_serializing_if = "Option::is_none") )]
	pub prtry_amt: Option<Vec<AmountAndCurrencyExchangeDetails4>>,
}

impl AmountAndCurrencyExchange3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instd_amt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tx_amt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.cntr_val_amt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.anncd_pstng_amt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.prtry_amt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// AmountAndCurrencyExchangeDetails3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmountAndCurrencyExchangeDetails3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CcyXchg", skip_serializing_if = "Option::is_none") )]
	pub ccy_xchg: Option<CurrencyExchange5>,
}

impl AmountAndCurrencyExchangeDetails3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Some(ref val) = self.ccy_xchg { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// AmountAndCurrencyExchangeDetails4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmountAndCurrencyExchangeDetails4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CcyXchg", skip_serializing_if = "Option::is_none") )]
	pub ccy_xchg: Option<CurrencyExchange5>,
}

impl AmountAndCurrencyExchangeDetails4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
		}
		if self.tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
		}
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Some(ref val) = self.ccy_xchg { if let Err(e) = val.validate() { return Err(e); } }
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
		if self.amt < 0.000000 {
			return Err(ValidationError::new(1003, "amt is less than the minimum value of 0.000000".to_string()));
		}
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
		if self.bdry_amt < 0.000000 {
			return Err(ValidationError::new(1003, "bdry_amt is less than the minimum value of 0.000000".to_string()));
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


// BankToCustomerDebitCreditNotificationV08 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BankToCustomerDebitCreditNotificationV08 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrpHdr") )]
	pub grp_hdr: GroupHeader81,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ntfctn") )]
	pub ntfctn: Vec<AccountNotification17>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl BankToCustomerDebitCreditNotificationV08 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.grp_hdr.validate() { return Err(e); }
		for item in &self.ntfctn { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// BatchInformation2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BatchInformation2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId", skip_serializing_if = "Option::is_none") )]
	pub msg_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInfId", skip_serializing_if = "Option::is_none") )]
	pub pmt_inf_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxs", skip_serializing_if = "Option::is_none") )]
	pub nb_of_txs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none") )]
	pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
}

impl BatchInformation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.msg_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.pmt_inf_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pmt_inf_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "pmt_inf_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.nb_of_txs {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "nb_of_txs does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ttl_amt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.cdt_dbt_ind { if let Err(e) = val.validate() { return Err(e); } }
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
	pub tx_ctgy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SaleRcncltnId", skip_serializing_if = "Option::is_none") )]
	pub sale_rcncltn_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SeqNbRg", skip_serializing_if = "Option::is_none") )]
	pub seq_nb_rg: Option<CardSequenceNumberRange1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxDtRg", skip_serializing_if = "Option::is_none") )]
	pub tx_dt_rg: Option<DateOrDateTimePeriod1Choice>,
}

impl CardAggregated2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.addtl_svc { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tx_ctgy {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_ctgy is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "tx_ctgy exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.sale_rcncltn_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sale_rcncltn_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sale_rcncltn_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.seq_nb_rg { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tx_dt_rg { if let Err(e) = val.validate() { return Err(e); } }
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


// CardEntry4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CardEntry4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Card", skip_serializing_if = "Option::is_none") )]
	pub card: Option<PaymentCard4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "POI", skip_serializing_if = "Option::is_none") )]
	pub poi: Option<PointOfInteraction1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AggtdNtry", skip_serializing_if = "Option::is_none") )]
	pub aggtd_ntry: Option<CardAggregated2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrePdAcct", skip_serializing_if = "Option::is_none") )]
	pub pre_pd_acct: Option<CashAccount38>,
}

impl CardEntry4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.card { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.poi { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.aggtd_ntry { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.pre_pd_acct { if let Err(e) = val.validate() { return Err(e); } }
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
	pub icc_rltd_data: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCntxt", skip_serializing_if = "Option::is_none") )]
	pub pmt_cntxt: Option<PaymentContext3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSvc", skip_serializing_if = "Option::is_none") )]
	pub addtl_svc: Option<CardPaymentServiceType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxCtgy", skip_serializing_if = "Option::is_none") )]
	pub tx_ctgy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SaleRcncltnId", skip_serializing_if = "Option::is_none") )]
	pub sale_rcncltn_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SaleRefNb", skip_serializing_if = "Option::is_none") )]
	pub sale_ref_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RePresntmntRsn", skip_serializing_if = "Option::is_none") )]
	pub re_presntmnt_rsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SeqNb", skip_serializing_if = "Option::is_none") )]
	pub seq_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
	pub tx_id: Option<TransactionIdentifier1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pdct", skip_serializing_if = "Option::is_none") )]
	pub pdct: Option<Product2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldtnDt", skip_serializing_if = "Option::is_none") )]
	pub vldtn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldtnSeqNb", skip_serializing_if = "Option::is_none") )]
	pub vldtn_seq_nb: Option<String>,
}

impl CardIndividualTransaction2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.icc_rltd_data {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "icc_rltd_data is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 1025 {
				return Err(ValidationError::new(1002, "icc_rltd_data exceeds the maximum length of 1025".to_string()));
			}
		}
		if let Some(ref val) = self.pmt_cntxt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.addtl_svc { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tx_ctgy {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_ctgy is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "tx_ctgy exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.sale_rcncltn_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sale_rcncltn_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sale_rcncltn_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.sale_ref_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sale_ref_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sale_ref_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.re_presntmnt_rsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "re_presntmnt_rsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "re_presntmnt_rsn exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.seq_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "seq_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "seq_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tx_id { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.pdct { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.vldtn_seq_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "vldtn_seq_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "vldtn_seq_nb exceeds the maximum length of 35".to_string()));
			}
		}
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
	pub csc_val: Option<String>,
}

impl CardSecurityInformation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.csc_mgmt.validate() { return Err(e); }
		if let Some(ref val) = self.csc_val {
			let pattern = Regex::new("[0-9]{3,4}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "csc_val does not match the required pattern".to_string()));
			}
		}
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
	pub frst_tx: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LastTx", skip_serializing_if = "Option::is_none") )]
	pub last_tx: Option<String>,
}

impl CardSequenceNumberRange1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.frst_tx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "frst_tx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "frst_tx exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.last_tx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "last_tx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "last_tx exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CardTransaction17 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CardTransaction17 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Card", skip_serializing_if = "Option::is_none") )]
	pub card: Option<PaymentCard4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "POI", skip_serializing_if = "Option::is_none") )]
	pub poi: Option<PointOfInteraction1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tx", skip_serializing_if = "Option::is_none") )]
	pub tx: Option<CardTransaction3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrePdAcct", skip_serializing_if = "Option::is_none") )]
	pub pre_pd_acct: Option<CashAccount38>,
}

impl CardTransaction17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.card { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.poi { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tx { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.pre_pd_acct { if let Err(e) = val.validate() { return Err(e); } }
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
		if let Some(ref val) = self.aggtd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.indv { if let Err(e) = val.validate() { return Err(e); } }
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


// CashAccount39 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CashAccount39 {
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
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ownr", skip_serializing_if = "Option::is_none") )]
	pub ownr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Svcr", skip_serializing_if = "Option::is_none") )]
	pub svcr: Option<BranchAndFinancialInstitutionIdentification6>,
}

impl CashAccount39 {
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
		if let Some(ref val) = self.ownr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.svcr { if let Err(e) = val.validate() { return Err(e); } }
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
	pub nb_of_days: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ActlDt", skip_serializing_if = "Option::is_none") )]
	pub actl_dt: Option<String>,
}

impl CashAvailabilityDate1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nb_of_days {
			let pattern = Regex::new("[\\+]{0,1}[0-9]{1,15}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "nb_of_days does not match the required pattern".to_string()));
			}
		}
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
	pub nb_of_notes: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveCurrencyAndAmount,
}

impl CashDeposit1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.note_dnmtn.validate() { return Err(e); }
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.nb_of_notes) {
			return Err(ValidationError::new(1005, "nb_of_notes does not match the required pattern".to_string()));
		}
		if let Err(e) = self.amt.validate() { return Err(e); }
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


// ChargeType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ChargeType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification3>,
}

impl ChargeType3Choice {
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


// Charges6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Charges6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlChrgsAndTaxAmt", skip_serializing_if = "Option::is_none") )]
	pub ttl_chrgs_and_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rcrd", skip_serializing_if = "Option::is_none") )]
	pub rcrd: Option<Vec<ChargesRecord3>>,
}

impl Charges6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ttl_chrgs_and_tax_amt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.rcrd { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// ChargesRecord3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ChargesRecord3 {
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
	pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
	pub tax: Option<TaxCharges2>,
}

impl ChargesRecord3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Some(ref val) = self.cdt_dbt_ind { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.br { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.agt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tax { if let Err(e) = val.validate() { return Err(e); } }
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
	pub evt_tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtId") )]
	pub evt_id: String,
}

impl CorporateAction9 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.evt_tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "evt_tp is shorter than the minimum length of 1".to_string()));
		}
		if self.evt_tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "evt_tp exceeds the maximum length of 35".to_string()));
		}
		if self.evt_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "evt_id is shorter than the minimum length of 1".to_string()));
		}
		if self.evt_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "evt_id exceeds the maximum length of 35".to_string()));
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


// CurrencyExchange5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CurrencyExchange5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SrcCcy") )]
	pub src_ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrgtCcy", skip_serializing_if = "Option::is_none") )]
	pub trgt_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none") )]
	pub unit_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRate") )]
	pub xchg_rate: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctId", skip_serializing_if = "Option::is_none") )]
	pub ctrct_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QtnDt", skip_serializing_if = "Option::is_none") )]
	pub qtn_dt: Option<String>,
}

impl CurrencyExchange5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.src_ccy) {
			return Err(ValidationError::new(1005, "src_ccy does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.trgt_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "trgt_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.unit_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "unit_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctrct_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctrct_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctrct_id exceeds the maximum length of 35".to_string()));
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
		if let Some(ref val) = self.dt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.dt_tm { if let Err(e) = val.validate() { return Err(e); } }
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
	pub nb_of_lines: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LineWidth") )]
	pub line_width: String,
}

impl DisplayCapabilities1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.disp_tp.validate() { return Err(e); }
		let pattern = Regex::new("[0-9]{1,3}").unwrap();
		if !pattern.is_match(&self.nb_of_lines) {
			return Err(ValidationError::new(1005, "nb_of_lines does not match the required pattern".to_string()));
		}
		let pattern = Regex::new("[0-9]{1,3}").unwrap();
		if !pattern.is_match(&self.line_width) {
			return Err(ValidationError::new(1005, "line_width does not match the required pattern".to_string()));
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


// EntryDetails9 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct EntryDetails9 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Btch", skip_serializing_if = "Option::is_none") )]
	pub btch: Option<BatchInformation2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxDtls", skip_serializing_if = "Option::is_none") )]
	pub tx_dtls: Option<Vec<EntryTransaction10>>,
}

impl EntryDetails9 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.btch { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.tx_dtls { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl EntryStatus1Choice {
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


// EntryTransaction10 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct EntryTransaction10 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Refs", skip_serializing_if = "Option::is_none") )]
	pub refs: Option<TransactionReferences6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtDtls", skip_serializing_if = "Option::is_none") )]
	pub amt_dtls: Option<AmountAndCurrencyExchange3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Avlbty", skip_serializing_if = "Option::is_none") )]
	pub avlbty: Option<Vec<CashAvailability1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BkTxCd", skip_serializing_if = "Option::is_none") )]
	pub bk_tx_cd: Option<BankTransactionCodeStructure4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Chrgs", skip_serializing_if = "Option::is_none") )]
	pub chrgs: Option<Charges6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Intrst", skip_serializing_if = "Option::is_none") )]
	pub intrst: Option<TransactionInterest4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdPties", skip_serializing_if = "Option::is_none") )]
	pub rltd_pties: Option<TransactionParties6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdAgts", skip_serializing_if = "Option::is_none") )]
	pub rltd_agts: Option<TransactionAgents5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none") )]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<Purpose2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdRmtInf", skip_serializing_if = "Option::is_none") )]
	pub rltd_rmt_inf: Option<Vec<RemittanceLocation7>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
	pub rmt_inf: Option<RemittanceInformation16>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdDts", skip_serializing_if = "Option::is_none") )]
	pub rltd_dts: Option<TransactionDates3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdPric", skip_serializing_if = "Option::is_none") )]
	pub rltd_pric: Option<TransactionPrice4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdQties", skip_serializing_if = "Option::is_none") )]
	pub rltd_qties: Option<Vec<TransactionQuantities3Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_id: Option<SecurityIdentification19>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
	pub tax: Option<TaxInformation8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrInf", skip_serializing_if = "Option::is_none") )]
	pub rtr_inf: Option<PaymentReturnReason5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CorpActn", skip_serializing_if = "Option::is_none") )]
	pub corp_actn: Option<CorporateAction9>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none") )]
	pub sfkpg_acct: Option<SecuritiesAccount19>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshDpst", skip_serializing_if = "Option::is_none") )]
	pub csh_dpst: Option<Vec<CashDeposit1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CardTx", skip_serializing_if = "Option::is_none") )]
	pub card_tx: Option<CardTransaction17>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlTxInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_tx_inf: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl EntryTransaction10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.refs { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.amt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.cdt_dbt_ind { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.amt_dtls { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.avlbty { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.bk_tx_cd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.chrgs { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.intrst { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.rltd_pties { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.rltd_agts { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.lcl_instrm { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.purp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.rltd_rmt_inf { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.rmt_inf { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.rltd_dts { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.rltd_pric { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.rltd_qties { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.fin_instrm_id { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tax { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.rtr_inf { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.corp_actn { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.sfkpg_acct { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.csh_dpst { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.card_tx { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.addtl_tx_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_tx_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 500 {
				return Err(ValidationError::new(1002, "addtl_tx_inf exceeds the maximum length of 500".to_string()));
			}
		}
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// GenericIdentification3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericIdentification3 {
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


// GenericIdentification32 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification32 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<PartyType3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<PartyType4Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
	pub shrt_nm: Option<String>,
}

impl GenericIdentification32 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.issr { if let Err(e) = val.validate() { return Err(e); } }
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


// GroupHeader81 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GroupHeader81 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgRcpt", skip_serializing_if = "Option::is_none") )]
	pub msg_rcpt: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none") )]
	pub msg_pgntn: Option<Pagination1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBizQry", skip_serializing_if = "Option::is_none") )]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl GroupHeader81 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.msg_rcpt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.msg_pgntn { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.orgnl_biz_qry { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 500 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 500".to_string()));
			}
		}
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
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl IdentificationSource3Choice {
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
	pub rsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
	pub tax: Option<TaxCharges2>,
}

impl InterestRecord2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Err(e) = self.cdt_dbt_ind.validate() { return Err(e); }
		if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.rate { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.fr_to_dt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.rsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rsn exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tax { if let Err(e) = val.validate() { return Err(e); } }
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
	pub prtry: Option<String>,
}

impl InterestType1Choice {
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


// MessageIdentification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MessageIdentification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none") )]
	pub msg_nm_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId", skip_serializing_if = "Option::is_none") )]
	pub msg_id: Option<String>,
}

impl MessageIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.msg_nm_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_nm_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_nm_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.msg_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// NameAndAddress16 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NameAndAddress16 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adr") )]
	pub adr: PostalAddress24,
}

impl NameAndAddress16 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 140 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
		}
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


// NumberAndSumOfTransactions1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NumberAndSumOfTransactions1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfNtries", skip_serializing_if = "Option::is_none") )]
	pub nb_of_ntries: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sum", skip_serializing_if = "Option::is_none") )]
	pub sum: Option<f64>,
}

impl NumberAndSumOfTransactions1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nb_of_ntries {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "nb_of_ntries does not match the required pattern".to_string()));
			}
		}
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
	pub nb_of_ntries: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sum", skip_serializing_if = "Option::is_none") )]
	pub sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNetNtry", skip_serializing_if = "Option::is_none") )]
	pub ttl_net_ntry: Option<AmountAndDirection35>,
}

impl NumberAndSumOfTransactions4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nb_of_ntries {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "nb_of_ntries does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ttl_net_ntry { if let Err(e) = val.validate() { return Err(e); } }
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
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none") )]
	pub msg_nm_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
}

impl OriginalBusinessQuery1 {
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


// OtherIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OtherIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sfx", skip_serializing_if = "Option::is_none") )]
	pub sfx: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: IdentificationSource3Choice,
}

impl OtherIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.sfx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sfx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "sfx exceeds the maximum length of 16".to_string()));
			}
		}
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
	pub card_ctry_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CardBrnd", skip_serializing_if = "Option::is_none") )]
	pub card_brnd: Option<GenericIdentification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlCardData", skip_serializing_if = "Option::is_none") )]
	pub addtl_card_data: Option<String>,
}

impl PaymentCard4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.plain_card_data { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.card_ctry_cd {
			let pattern = Regex::new("[0-9]{3}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "card_ctry_cd does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.card_brnd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.addtl_card_data {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_card_data is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "addtl_card_data exceeds the maximum length of 70".to_string()));
			}
		}
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
	pub attndnt_lang: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CardDataNtryMd") )]
	pub card_data_ntry_md: CardDataReading1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FllbckInd", skip_serializing_if = "Option::is_none") )]
	pub fllbck_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AuthntcnMtd", skip_serializing_if = "Option::is_none") )]
	pub authntcn_mtd: Option<CardholderAuthentication2>,
}

impl PaymentContext3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.attndnc_cntxt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tx_envt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tx_chanl { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.attndnt_lang {
			let pattern = Regex::new("[a-z]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "attndnt_lang does not match the required pattern".to_string()));
			}
		}
		if let Err(e) = self.card_data_ntry_md.validate() { return Err(e); }
		if let Some(ref val) = self.authntcn_mtd { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// PaymentReturnReason5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentReturnReason5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBkTxCd", skip_serializing_if = "Option::is_none") )]
	pub orgnl_bk_tx_cd: Option<BankTransactionCodeStructure4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
	pub orgtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<ReturnReason5Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<String>>,
}

impl PaymentReturnReason5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_bk_tx_cd { if let Err(e) = val.validate() { return Err(e); } }
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


// PlainCardData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PlainCardData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PAN") )]
	pub pan: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CardSeqNb", skip_serializing_if = "Option::is_none") )]
	pub card_seq_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FctvDt", skip_serializing_if = "Option::is_none") )]
	pub fctv_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt") )]
	pub xpry_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvcCd", skip_serializing_if = "Option::is_none") )]
	pub svc_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrckData", skip_serializing_if = "Option::is_none") )]
	pub trck_data: Option<Vec<TrackData1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CardSctyCd", skip_serializing_if = "Option::is_none") )]
	pub card_scty_cd: Option<CardSecurityInformation1>,
}

impl PlainCardData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{8,28}").unwrap();
		if !pattern.is_match(&self.pan) {
			return Err(ValidationError::new(1005, "pan does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.card_seq_nb {
			let pattern = Regex::new("[0-9]{2,3}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "card_seq_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.svc_cd {
			let pattern = Regex::new("[0-9]{3}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "svc_cd does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.trck_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.card_scty_cd { if let Err(e) = val.validate() { return Err(e); } }
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
	pub sys_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrpId", skip_serializing_if = "Option::is_none") )]
	pub grp_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cpblties", skip_serializing_if = "Option::is_none") )]
	pub cpblties: Option<PointOfInteractionCapabilities1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cmpnt", skip_serializing_if = "Option::is_none") )]
	pub cmpnt: Option<Vec<PointOfInteractionComponent1>>,
}

impl PointOfInteraction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref val) = self.sys_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sys_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "sys_nm exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.grp_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "grp_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "grp_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.cpblties { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.cmpnt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	pub prt_line_width: Option<String>,
}

impl PointOfInteractionCapabilities1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.card_rdng_cpblties { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref vec) = self.crdhldr_vrfctn_cpblties { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.on_line_cpblties { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.disp_cpblties { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.prt_line_width {
			let pattern = Regex::new("[0-9]{1,3}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "prt_line_width does not match the required pattern".to_string()));
			}
		}
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
	pub manfctr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mdl", skip_serializing_if = "Option::is_none") )]
	pub mdl: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VrsnNb", skip_serializing_if = "Option::is_none") )]
	pub vrsn_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SrlNb", skip_serializing_if = "Option::is_none") )]
	pub srl_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ApprvlNb", skip_serializing_if = "Option::is_none") )]
	pub apprvl_nb: Option<Vec<String>>,
}

impl PointOfInteractionComponent1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.poi_cmpnt_tp.validate() { return Err(e); }
		if let Some(ref val) = self.manfctr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "manfctr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "manfctr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.mdl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mdl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mdl exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.vrsn_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "vrsn_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "vrsn_nb exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.srl_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "srl_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "srl_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.apprvl_nb {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "apprvl_nb is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 70 {
					return Err(ValidationError::new(1002, "apprvl_nb exceeds the maximum length of 70".to_string()));
				}
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
		if let Some(ref val) = self.amt { if let Err(e) = val.validate() { return Err(e); } }
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


// Product2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Product2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdctCd") )]
	pub pdct_cd: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none") )]
	pub unit_of_measr: Option<UnitOfMeasure1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdctQty", skip_serializing_if = "Option::is_none") )]
	pub pdct_qty: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitPric", skip_serializing_if = "Option::is_none") )]
	pub unit_pric: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdctAmt", skip_serializing_if = "Option::is_none") )]
	pub pdct_amt: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxTp", skip_serializing_if = "Option::is_none") )]
	pub tax_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlPdctInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_pdct_inf: Option<String>,
}

impl Product2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.pdct_cd.chars().count() < 1 {
			return Err(ValidationError::new(1001, "pdct_cd is shorter than the minimum length of 1".to_string()));
		}
		if self.pdct_cd.chars().count() > 70 {
			return Err(ValidationError::new(1002, "pdct_cd exceeds the maximum length of 70".to_string()));
		}
		if let Some(ref val) = self.unit_of_measr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tax_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.addtl_pdct_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_pdct_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "addtl_pdct_inf exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ProprietaryAgent4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProprietaryAgent4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Agt") )]
	pub agt: BranchAndFinancialInstitutionIdentification6,
}

impl ProprietaryAgent4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
		}
		if self.tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
		}
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


// ProprietaryDate3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProprietaryDate3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt") )]
	pub dt: DateAndDateTime2Choice,
}

impl ProprietaryDate3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
		}
		if self.tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
		}
		if let Err(e) = self.dt.validate() { return Err(e); }
		Ok(())
	}
}


// ProprietaryParty5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProprietaryParty5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty") )]
	pub pty: Party40Choice,
}

impl ProprietaryParty5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
		}
		if self.tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
		}
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
	pub tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pric") )]
	pub pric: ActiveOrHistoricCurrencyAndAmount,
}

impl ProprietaryPrice2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
		}
		if self.tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
		}
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
	pub tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Qty") )]
	pub qty: String,
}

impl ProprietaryQuantity1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
		}
		if self.tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
		}
		if self.qty.chars().count() < 1 {
			return Err(ValidationError::new(1001, "qty is shorter than the minimum length of 1".to_string()));
		}
		if self.qty.chars().count() > 35 {
			return Err(ValidationError::new(1002, "qty exceeds the maximum length of 35".to_string()));
		}
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
	pub tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
	pub ref_attr: String,
}

impl ProprietaryReference1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
		}
		if self.tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
		}
		if self.ref_attr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "ref_attr is shorter than the minimum length of 1".to_string()));
		}
		if self.ref_attr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "ref_attr exceeds the maximum length of 35".to_string()));
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
		if let Some(ref val) = self.vldty_rg { if let Err(e) = val.validate() { return Err(e); } }
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
	pub othr: Option<String>,
}

impl RateType4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.othr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "othr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "othr exceeds the maximum length of 35".to_string()));
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


// RemittanceLocation7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RemittanceLocation7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtId", skip_serializing_if = "Option::is_none") )]
	pub rmt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtLctnDtls", skip_serializing_if = "Option::is_none") )]
	pub rmt_lctn_dtls: Option<Vec<RemittanceLocationData1>>,
}

impl RemittanceLocation7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rmt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rmt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rmt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.rmt_lctn_dtls { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// RemittanceLocationData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RemittanceLocationData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mtd") )]
	pub mtd: RemittanceLocationMethod2Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ElctrncAdr", skip_serializing_if = "Option::is_none") )]
	pub elctrnc_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<NameAndAddress16>,
}

impl RemittanceLocationData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.mtd.validate() { return Err(e); }
		if let Some(ref val) = self.elctrnc_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "elctrnc_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "elctrnc_adr exceeds the maximum length of 2048".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { if let Err(e) = val.validate() { return Err(e); } }
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


// ReportEntry10 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReportEntry10 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtryRef", skip_serializing_if = "Option::is_none") )]
	pub ntry_ref: Option<String>,
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
	pub acct_svcr_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Avlbty", skip_serializing_if = "Option::is_none") )]
	pub avlbty: Option<Vec<CashAvailability1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BkTxCd") )]
	pub bk_tx_cd: BankTransactionCodeStructure4,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ComssnWvrInd", skip_serializing_if = "Option::is_none") )]
	pub comssn_wvr_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInfInd", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf_ind: Option<MessageIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtDtls", skip_serializing_if = "Option::is_none") )]
	pub amt_dtls: Option<AmountAndCurrencyExchange3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Chrgs", skip_serializing_if = "Option::is_none") )]
	pub chrgs: Option<Charges6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechInptChanl", skip_serializing_if = "Option::is_none") )]
	pub tech_inpt_chanl: Option<TechnicalInputChannel1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Intrst", skip_serializing_if = "Option::is_none") )]
	pub intrst: Option<TransactionInterest4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CardTx", skip_serializing_if = "Option::is_none") )]
	pub card_tx: Option<CardEntry4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtryDtls", skip_serializing_if = "Option::is_none") )]
	pub ntry_dtls: Option<Vec<EntryDetails9>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlNtryInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_ntry_inf: Option<String>,
}

impl ReportEntry10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ntry_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ntry_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ntry_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Err(e) = self.cdt_dbt_ind.validate() { return Err(e); }
		if let Err(e) = self.sts.validate() { return Err(e); }
		if let Some(ref val) = self.bookg_dt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.val_dt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.acct_svcr_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_svcr_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_svcr_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.avlbty { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Err(e) = self.bk_tx_cd.validate() { return Err(e); }
		if let Some(ref val) = self.addtl_inf_ind { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.amt_dtls { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.chrgs { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tech_inpt_chanl { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.intrst { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.card_tx { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.ntry_dtls { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.addtl_ntry_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_ntry_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 500 {
				return Err(ValidationError::new(1002, "addtl_ntry_inf exceeds the maximum length of 500".to_string()));
			}
		}
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
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ReportingSource1Choice {
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


// SecurityIdentification19 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityIdentification19 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
}

impl SecurityIdentification19 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.isin {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.othr_id { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// SequenceRange1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SequenceRange1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrSeq") )]
	pub fr_seq: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToSeq") )]
	pub to_seq: String,
}

impl SequenceRange1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.fr_seq.chars().count() < 1 {
			return Err(ValidationError::new(1001, "fr_seq is shorter than the minimum length of 1".to_string()));
		}
		if self.fr_seq.chars().count() > 35 {
			return Err(ValidationError::new(1002, "fr_seq exceeds the maximum length of 35".to_string()));
		}
		if self.to_seq.chars().count() < 1 {
			return Err(ValidationError::new(1001, "to_seq is shorter than the minimum length of 1".to_string()));
		}
		if self.to_seq.chars().count() > 35 {
			return Err(ValidationError::new(1002, "to_seq exceeds the maximum length of 35".to_string()));
		}
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
	pub fr_seq: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToSeq", skip_serializing_if = "Option::is_none") )]
	pub to_seq: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrToSeq", skip_serializing_if = "Option::is_none") )]
	pub fr_to_seq: Option<Vec<SequenceRange1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EQSeq", skip_serializing_if = "Option::is_none") )]
	pub eq_seq: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEQSeq", skip_serializing_if = "Option::is_none") )]
	pub neq_seq: Option<Vec<String>>,
}

impl SequenceRange1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fr_seq {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "fr_seq is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "fr_seq exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.to_seq {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "to_seq is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "to_seq exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.fr_to_seq { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref vec) = self.eq_seq {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "eq_seq is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "eq_seq exceeds the maximum length of 35".to_string()));
				}
			}
		}
		if let Some(ref vec) = self.neq_seq {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "neq_seq is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "neq_seq exceeds the maximum length of 35".to_string()));
				}
			}
		}
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


// TaxCharges2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TaxCharges2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl TaxCharges2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.amt { if let Err(e) = val.validate() { return Err(e); } }
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


// TechnicalInputChannel1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TechnicalInputChannel1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl TechnicalInputChannel1Choice {
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
		if let Some(ref val) = self.ttl_ntries { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ttl_cdt_ntries { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ttl_dbt_ntries { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.ttl_ntries_per_bk_tx_cd { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	pub nb_of_ntries: Option<String>,
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
		if let Some(ref val) = self.nb_of_ntries {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "nb_of_ntries does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ttl_net_ntry { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.cdt_ntries { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.dbt_ntries { if let Err(e) = val.validate() { return Err(e); } }
		if let Err(e) = self.bk_tx_cd.validate() { return Err(e); }
		if let Some(ref vec) = self.avlbty { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.dt { if let Err(e) = val.validate() { return Err(e); } }
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
	pub trck_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrckVal") )]
	pub trck_val: String,
}

impl TrackData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.trck_nb {
			let pattern = Regex::new("[0-9]").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "trck_nb does not match the required pattern".to_string()));
			}
		}
		if self.trck_val.chars().count() < 1 {
			return Err(ValidationError::new(1001, "trck_val is shorter than the minimum length of 1".to_string()));
		}
		if self.trck_val.chars().count() > 140 {
			return Err(ValidationError::new(1002, "trck_val exceeds the maximum length of 140".to_string()));
		}
		Ok(())
	}
}


// TransactionAgents5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransactionAgents5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none") )]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none") )]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RcvgAgt", skip_serializing_if = "Option::is_none") )]
	pub rcvg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DlvrgAgt", skip_serializing_if = "Option::is_none") )]
	pub dlvrg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssgAgt", skip_serializing_if = "Option::is_none") )]
	pub issg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmPlc", skip_serializing_if = "Option::is_none") )]
	pub sttlm_plc: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<Vec<ProprietaryAgent4>>,
}

impl TransactionAgents5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instg_agt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.instd_agt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.dbtr_agt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.cdtr_agt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.intrmy_agt1 { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.intrmy_agt2 { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.intrmy_agt3 { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.rcvg_agt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.dlvrg_agt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.issg_agt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.sttlm_plc { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.prtry { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
		if let Some(ref vec) = self.prtry { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	pub tx_ref: String,
}

impl TransactionIdentifier1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tx_ref.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tx_ref is shorter than the minimum length of 1".to_string()));
		}
		if self.tx_ref.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tx_ref exceeds the maximum length of 35".to_string()));
		}
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
		if let Some(ref val) = self.ttl_intrst_and_tax_amt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.rcrd { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TransactionParties6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransactionParties6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty", skip_serializing_if = "Option::is_none") )]
	pub initg_pty: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
	pub dbtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
	pub cdtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgPty", skip_serializing_if = "Option::is_none") )]
	pub tradg_pty: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<Vec<ProprietaryParty5>>,
}

impl TransactionParties6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.initg_pty { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.dbtr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.dbtr_acct { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ultmt_dbtr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.cdtr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.cdtr_acct { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ultmt_cdtr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tradg_pty { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.prtry { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
		if let Some(ref val) = self.deal_pric { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.prtry { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
		if let Some(ref val) = self.qty { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.orgnl_and_cur_face_amt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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
	pub msg_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInfId", skip_serializing_if = "Option::is_none") )]
	pub pmt_inf_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrId", skip_serializing_if = "Option::is_none") )]
	pub instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none") )]
	pub end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
	pub uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
	pub tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtId", skip_serializing_if = "Option::is_none") )]
	pub mndt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqNb", skip_serializing_if = "Option::is_none") )]
	pub chq_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none") )]
	pub clr_sys_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOwnrTxId", skip_serializing_if = "Option::is_none") )]
	pub acct_ownr_tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr_tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none") )]
	pub mkt_infrstrctr_tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgId", skip_serializing_if = "Option::is_none") )]
	pub prcg_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<Vec<ProprietaryReference1>>,
}

impl TransactionReferences6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.msg_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.acct_svcr_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_svcr_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_svcr_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.pmt_inf_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pmt_inf_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "pmt_inf_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "instr_id exceeds the maximum length of 35".to_string()));
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
		if let Some(ref val) = self.uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.mndt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mndt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mndt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.chq_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "chq_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "chq_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.clr_sys_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clr_sys_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clr_sys_ref exceeds the maximum length of 35".to_string()));
			}
		}
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
		if let Some(ref val) = self.prcg_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prcg_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prcg_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.prtry { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
		if let Some(ref val) = self.val_tp { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}
