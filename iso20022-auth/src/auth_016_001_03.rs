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


// ActiveCurrencyAnd13DecimalAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ActiveCurrencyAnd13DecimalAmount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl ActiveCurrencyAnd13DecimalAmount {
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


// AmountAndDirection53 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmountAndDirection53 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sgn", skip_serializing_if = "Option::is_none") )]
	pub sgn: Option<bool>,
}

impl AmountAndDirection53 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		Ok(())
	}
}


// AmountAndDirection61 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmountAndDirection61 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sgn", skip_serializing_if = "Option::is_none") )]
	pub sgn: Option<bool>,
}

impl AmountAndDirection61 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		Ok(())
	}
}


// AssetClassAttributes1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AssetClassAttributes1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Intrst") )]
	pub intrst: DerivativeInterest2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FX") )]
	pub fx: DerivativeForeignExchange2,
}

impl AssetClassAttributes1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.intrst.validate()?;
		self.fx.validate()?;
		Ok(())
	}
}


// AssetClassAttributes1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AssetClassAttributes1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Intrst", skip_serializing_if = "Option::is_none") )]
	pub intrst: Option<DerivativeInterest2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FX", skip_serializing_if = "Option::is_none") )]
	pub fx: Option<DerivativeForeignExchange2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Both", skip_serializing_if = "Option::is_none") )]
	pub both: Option<AssetClassAttributes1>,
}

impl AssetClassAttributes1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.intrst { val.validate()? }
		if let Some(ref val) = self.fx { val.validate()? }
		if let Some(ref val) = self.both { val.validate()? }
		Ok(())
	}
}


// BasketDescription3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BasketDescription3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Indx", skip_serializing_if = "Option::is_none") )]
	pub indx: Option<Vec<FinancialInstrument58>>,
}

impl BasketDescription3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.isin {
			for item in vec {
				let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
				}
			}
		}
		if let Some(ref vec) = self.indx { for item in vec { item.validate()? } }
		Ok(())
	}
}


// BenchmarkCurveName2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum BenchmarkCurveName2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "WIBO") )]
	CodeWIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TREA") )]
	CodeTREA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TIBO") )]
	CodeTIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TLBO") )]
	CodeTLBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SWAP") )]
	CodeSWAP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STBO") )]
	CodeSTBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRBO") )]
	CodePRBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PFAN") )]
	CodePFAN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NIBO") )]
	CodeNIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MAAA") )]
	CodeMAAA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MOSP") )]
	CodeMOSP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIBO") )]
	CodeLIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIBI") )]
	CodeLIBI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "JIBA") )]
	CodeJIBA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISDA") )]
	CodeISDA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GCFR") )]
	CodeGCFR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FUSW") )]
	CodeFUSW,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUCH") )]
	CodeEUCH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUUS") )]
	CodeEUUS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EURI") )]
	CodeEURI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EONS") )]
	CodeEONS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EONA") )]
	CodeEONA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CIBO") )]
	CodeCIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CDOR") )]
	CodeCDOR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BUBO") )]
	CodeBUBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BBSW") )]
	CodeBBSW,
}

impl BenchmarkCurveName2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// BenchmarkCurveName5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BenchmarkCurveName5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Indx", skip_serializing_if = "Option::is_none") )]
	pub indx: Option<BenchmarkCurveName2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
}

impl BenchmarkCurveName5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.indx { val.validate()? }
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 25 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 25".to_string()));
			}
		}
		Ok(())
	}
}


// CancelledStatusReason15Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum CancelledStatusReason15Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CANI") )]
	CodeCANI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSUB") )]
	CodeCSUB,
}

impl CancelledStatusReason15Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DebtInstrument4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DebtInstrument4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt") )]
	pub mtrty_dt: String,
}

impl DebtInstrument4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DerivativeForeignExchange2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DerivativeForeignExchange2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrNtnlCcy") )]
	pub othr_ntnl_ccy: String,
}

impl DerivativeForeignExchange2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.othr_ntnl_ccy) {
			return Err(ValidationError::new(1005, "othr_ntnl_ccy does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// DerivativeInstrument6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DerivativeInstrument6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt", skip_serializing_if = "Option::is_none") )]
	pub xpry_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricMltplr") )]
	pub pric_mltplr: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygInstrm") )]
	pub undrlyg_instrm: UnderlyingIdentification2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OptnTp", skip_serializing_if = "Option::is_none") )]
	pub optn_tp: Option<OptionType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StrkPric", skip_serializing_if = "Option::is_none") )]
	pub strk_pric: Option<SecuritiesTransactionPrice4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OptnExrcStyle", skip_serializing_if = "Option::is_none") )]
	pub optn_exrc_style: Option<OptionStyle7Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryTp") )]
	pub dlvry_tp: PhysicalTransferType4Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AsstClssSpcfcAttrbts", skip_serializing_if = "Option::is_none") )]
	pub asst_clss_spcfc_attrbts: Option<AssetClassAttributes1Choice>,
}

impl DerivativeInstrument6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.pric_mltplr < 0.000000 {
			return Err(ValidationError::new(1003, "pric_mltplr is less than the minimum value of 0.000000".to_string()));
		}
		self.undrlyg_instrm.validate()?;
		if let Some(ref val) = self.optn_tp { val.validate()? }
		if let Some(ref val) = self.strk_pric { val.validate()? }
		if let Some(ref val) = self.optn_exrc_style { val.validate()? }
		self.dlvry_tp.validate()?;
		if let Some(ref val) = self.asst_clss_spcfc_attrbts { val.validate()? }
		Ok(())
	}
}


// DerivativeInterest2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DerivativeInterest2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrNtnlCcy") )]
	pub othr_ntnl_ccy: String,
}

impl DerivativeInterest2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.othr_ntnl_ccy) {
			return Err(ValidationError::new(1005, "othr_ntnl_ccy does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// DigitalTokenAmount2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DigitalTokenAmount2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Idr") )]
	pub idr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
	pub unit: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
}

impl DigitalTokenAmount2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[1-9B-DF-HJ-NP-XZ][0-9B-DF-HJ-NP-XZ]{8,8}").unwrap();
		if !pattern.is_match(&self.idr) {
			return Err(ValidationError::new(1005, "idr does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.desc {
			if val.chars().count() > 30 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 30".to_string()));
			}
		}
		Ok(())
	}
}


// ExecutingParty1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ExecutingParty1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prsn", skip_serializing_if = "Option::is_none") )]
	pub prsn: Option<PersonIdentification12>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Algo", skip_serializing_if = "Option::is_none") )]
	pub algo: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Clnt", skip_serializing_if = "Option::is_none") )]
	pub clnt: Option<NoReasonCode>,
}

impl ExecutingParty1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.prsn { val.validate()? }
		if let Some(ref val) = self.algo {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "algo is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 50 {
				return Err(ValidationError::new(1002, "algo exceeds the maximum length of 50".to_string()));
			}
		}
		if let Some(ref val) = self.clnt { val.validate()? }
		Ok(())
	}
}


// FinancialInstrument58 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrument58 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: FloatingInterestRate8,
}

impl FinancialInstrument58 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.isin {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		self.nm.validate()?;
		Ok(())
	}
}


// FinancialInstrumentAttributes5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrumentAttributes5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AltrnId", skip_serializing_if = "Option::is_none") )]
	pub altrn_id: Option<SecurityIdentification19>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<SecurityInstrumentDescription22>,
}

impl FinancialInstrumentAttributes5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.altrn_id { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
		Ok(())
	}
}


// FinancialInstrumentIdentification6Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrumentIdentification6Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Indx", skip_serializing_if = "Option::is_none") )]
	pub indx: Option<FinancialInstrument58>,
}

impl FinancialInstrumentIdentification6Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.isin {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.indx { val.validate()? }
		Ok(())
	}
}


// FinancialInstrumentIdentification7Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrumentIdentification7Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sngl", skip_serializing_if = "Option::is_none") )]
	pub sngl: Option<FinancialInstrumentIdentification6Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Bskt", skip_serializing_if = "Option::is_none") )]
	pub bskt: Option<BasketDescription3>,
}

impl FinancialInstrumentIdentification7Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sngl { val.validate()? }
		if let Some(ref val) = self.bskt { val.validate()? }
		Ok(())
	}
}


// FinancialInstrumentQuantity25Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrumentQuantity25Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
	pub unit: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none") )]
	pub nmnl_val: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MntryVal", skip_serializing_if = "Option::is_none") )]
	pub mntry_val: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl FinancialInstrumentQuantity25Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nmnl_val { val.validate()? }
		if let Some(ref val) = self.mntry_val { val.validate()? }
		Ok(())
	}
}


// FinancialInstrumentReportingTransactionReportV03 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrumentReportingTransactionReportV03 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tx") )]
	pub tx: Vec<ReportingTransactionType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl FinancialInstrumentReportingTransactionReportV03 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.tx { item.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// FloatingInterestRate8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FloatingInterestRate8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefRate") )]
	pub ref_rate: BenchmarkCurveName5Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Term", skip_serializing_if = "Option::is_none") )]
	pub term: Option<InterestRateContractTerm2>,
}

impl FloatingInterestRate8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.ref_rate.validate()?;
		if let Some(ref val) = self.term { val.validate()? }
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
		if let Some(ref val) = self.schme_nm { val.validate()? }
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


// InterestRateContractTerm2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InterestRateContractTerm2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit") )]
	pub unit: RateBasis1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: f64,
}

impl InterestRateContractTerm2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.unit.validate()?;
		Ok(())
	}
}


// InternalPartyRole1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum InternalPartyRole1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "INTC") )]
	CodeINTC,
}

impl InternalPartyRole1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestmentParty1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InvestmentParty1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prsn", skip_serializing_if = "Option::is_none") )]
	pub prsn: Option<PersonIdentification12>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Algo", skip_serializing_if = "Option::is_none") )]
	pub algo: Option<String>,
}

impl InvestmentParty1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.prsn { val.validate()? }
		if let Some(ref val) = self.algo {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "algo is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 50 {
				return Err(ValidationError::new(1002, "algo exceeds the maximum length of 50".to_string()));
			}
		}
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


// OptionStyle7Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OptionStyle7Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AMER") )]
	CodeAMER,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ASIA") )]
	CodeASIA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BERM") )]
	CodeBERM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EURO") )]
	CodeEURO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
}

impl OptionStyle7Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OptionType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OptionType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CALL") )]
	CodeCALL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUTO") )]
	CodePUTO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
}

impl OptionType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
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
		self.tp.validate()?;
		Ok(())
	}
}


// PartyIdentification76 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification76 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: PersonOrOrganisation1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfBrnch", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_brnch: Option<String>,
}

impl PartyIdentification76 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.ctry_of_brnch {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_brnch does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PartyIdentification79 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification79 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOwnr") )]
	pub acct_ownr: Vec<PartyIdentification76>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DcsnMakr", skip_serializing_if = "Option::is_none") )]
	pub dcsn_makr: Option<Vec<PersonOrOrganisation2Choice>>,
}

impl PartyIdentification79 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.acct_ownr { item.validate()? }
		if let Some(ref vec) = self.dcsn_makr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PersonIdentification10 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PersonIdentification10 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrstNm") )]
	pub frst_nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BirthDt") )]
	pub birth_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr") )]
	pub othr: GenericPersonIdentification1,
}

impl PersonIdentification10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.frst_nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "frst_nm is shorter than the minimum length of 1".to_string()));
		}
		if self.frst_nm.chars().count() > 140 {
			return Err(ValidationError::new(1002, "frst_nm exceeds the maximum length of 140".to_string()));
		}
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 140 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
		}
		self.othr.validate()?;
		Ok(())
	}
}


// PersonIdentification12 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PersonIdentification12 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfBrnch") )]
	pub ctry_of_brnch: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr") )]
	pub othr: GenericPersonIdentification1,
}

impl PersonIdentification12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry_of_brnch) {
			return Err(ValidationError::new(1005, "ctry_of_brnch does not match the required pattern".to_string()));
		}
		self.othr.validate()?;
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


// PersonOrOrganisation1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PersonOrOrganisation1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIC", skip_serializing_if = "Option::is_none") )]
	pub mic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prsn", skip_serializing_if = "Option::is_none") )]
	pub prsn: Option<PersonIdentification10>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Intl", skip_serializing_if = "Option::is_none") )]
	pub intl: Option<InternalPartyRole1Code>,
}

impl PersonOrOrganisation1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.mic {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "mic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prsn { val.validate()? }
		if let Some(ref val) = self.intl { val.validate()? }
		Ok(())
	}
}


// PersonOrOrganisation2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PersonOrOrganisation2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prsn", skip_serializing_if = "Option::is_none") )]
	pub prsn: Option<PersonIdentification10>,
}

impl PersonOrOrganisation2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prsn { val.validate()? }
		Ok(())
	}
}


// PhysicalTransferType4Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PhysicalTransferType4Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PHYS") )]
	CodePHYS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OPTL") )]
	CodeOPTL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CASH") )]
	CodeCASH,
}

impl PhysicalTransferType4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PriceStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PriceStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PNDG") )]
	CodePNDG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOAP") )]
	CodeNOAP,
}

impl PriceStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RateBasis1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum RateBasis1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DAYS") )]
	CodeDAYS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
	CodeMNTH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
	CodeWEEK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
	CodeYEAR,
}

impl RateBasis1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RecordTechnicalData2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RecordTechnicalData2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RctDtTm") )]
	pub rct_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlRsn") )]
	pub cxl_rsn: CancelledStatusReason15Code,
}

impl RecordTechnicalData2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cxl_rsn.validate()?;
		Ok(())
	}
}


// RecordTechnicalData5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RecordTechnicalData5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RctDtTm") )]
	pub rct_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRsn") )]
	pub xchg_rsn: Vec<String>,
}

impl RecordTechnicalData5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.xchg_rsn {
			if item.chars().count() < 1 {
				return Err(ValidationError::new(1001, "xchg_rsn is shorter than the minimum length of 1".to_string()));
			}
			if item.chars().count() > 4 {
				return Err(ValidationError::new(1002, "xchg_rsn exceeds the maximum length of 4".to_string()));
			}
		}
		Ok(())
	}
}


// RegulatoryTradingCapacity1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum RegulatoryTradingCapacity1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MTCH") )]
	CodeMTCH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DEAL") )]
	CodeDEAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AOTC") )]
	CodeAOTC,
}

impl RegulatoryTradingCapacity1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ReportingTransactionType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReportingTransactionType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "New", skip_serializing_if = "Option::is_none") )]
	pub new: Option<SecuritiesTransactionReport7>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cxl", skip_serializing_if = "Option::is_none") )]
	pub cxl: Option<SecuritiesTransactionReport2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl ReportingTransactionType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.new { val.validate()? }
		if let Some(ref val) = self.cxl { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ReportingWaiverType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ReportingWaiverType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "OILQ") )]
	CodeOILQ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NLIQ") )]
	CodeNLIQ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRIC") )]
	CodePRIC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ILQD") )]
	CodeILQD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RFPT") )]
	CodeRFPT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SIZE") )]
	CodeSIZE,
}

impl ReportingWaiverType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ReportingWaiverType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ReportingWaiverType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BENC") )]
	CodeBENC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACTX") )]
	CodeACTX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ILQD") )]
	CodeILQD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SIZE") )]
	CodeSIZE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CANC") )]
	CodeCANC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AMND") )]
	CodeAMND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SDIV") )]
	CodeSDIV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RPRI") )]
	CodeRPRI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DUPL") )]
	CodeDUPL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LRGS") )]
	CodeLRGS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TNCP") )]
	CodeTNCP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TPAC") )]
	CodeTPAC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XFPH") )]
	CodeXFPH,
}

impl ReportingWaiverType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecuritiesTransaction3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransaction3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradDt") )]
	pub trad_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgCpcty") )]
	pub tradg_cpcty: RegulatoryTradingCapacity1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Qty") )]
	pub qty: FinancialInstrumentQuantity25Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DgtlTknQty", skip_serializing_if = "Option::is_none") )]
	pub dgtl_tkn_qty: Option<Vec<DigitalTokenAmount2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DerivNtnlChng", skip_serializing_if = "Option::is_none") )]
	pub deriv_ntnl_chng: Option<VariationType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pric") )]
	pub pric: SecuritiesTransactionPrice22Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetAmt", skip_serializing_if = "Option::is_none") )]
	pub net_amt: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradVn") )]
	pub trad_vn: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfBrnch", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_brnch: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UpFrntPmt", skip_serializing_if = "Option::is_none") )]
	pub up_frnt_pmt: Option<AmountAndDirection53>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradPlcMtchgId", skip_serializing_if = "Option::is_none") )]
	pub trad_plc_mtchg_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CmplxTradCmpntId", skip_serializing_if = "Option::is_none") )]
	pub cmplx_trad_cmpnt_id: Option<String>,
}

impl SecuritiesTransaction3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tradg_cpcty.validate()?;
		self.qty.validate()?;
		if let Some(ref vec) = self.dgtl_tkn_qty { for item in vec { item.validate()? } }
		if let Some(ref val) = self.deriv_ntnl_chng { val.validate()? }
		self.pric.validate()?;
		let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
		if !pattern.is_match(&self.trad_vn) {
			return Err(ValidationError::new(1005, "trad_vn does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.ctry_of_brnch {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_brnch does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.up_frnt_pmt { val.validate()? }
		if let Some(ref val) = self.trad_plc_mtchg_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "trad_plc_mtchg_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "trad_plc_mtchg_id exceeds the maximum length of 52".to_string()));
			}
		}
		if let Some(ref val) = self.cmplx_trad_cmpnt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cmplx_trad_cmpnt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cmplx_trad_cmpnt_id exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// SecuritiesTransactionIndicator2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionIndicator2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "WvrInd", skip_serializing_if = "Option::is_none") )]
	pub wvr_ind: Option<Vec<ReportingWaiverType1Code>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtSellgInd", skip_serializing_if = "Option::is_none") )]
	pub shrt_sellg_ind: Option<Side5Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTCPstTradInd", skip_serializing_if = "Option::is_none") )]
	pub otc_pst_trad_ind: Option<Vec<ReportingWaiverType3Code>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RskRdcgTx", skip_serializing_if = "Option::is_none") )]
	pub rsk_rdcg_tx: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesFincgTxInd") )]
	pub scties_fincg_tx_ind: bool,
}

impl SecuritiesTransactionIndicator2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.wvr_ind { for item in vec { item.validate()? } }
		if let Some(ref val) = self.shrt_sellg_ind { val.validate()? }
		if let Some(ref vec) = self.otc_pst_trad_ind { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecuritiesTransactionPrice1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionPrice1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pdg") )]
	pub pdg: PriceStatus1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
	pub ccy: Option<String>,
}

impl SecuritiesTransactionPrice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pdg.validate()?;
		if let Some(ref val) = self.ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// SecuritiesTransactionPrice22Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionPrice22Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pric", skip_serializing_if = "Option::is_none") )]
	pub pric: Option<SecuritiesTransactionPrice2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DgtlTknPric", skip_serializing_if = "Option::is_none") )]
	pub dgtl_tkn_pric: Option<SecuritiesTransactionPrice7>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoPric", skip_serializing_if = "Option::is_none") )]
	pub no_pric: Option<SecuritiesTransactionPrice6>,
}

impl SecuritiesTransactionPrice22Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pric { val.validate()? }
		if let Some(ref val) = self.dgtl_tkn_pric { val.validate()? }
		if let Some(ref val) = self.no_pric { val.validate()? }
		Ok(())
	}
}


// SecuritiesTransactionPrice2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionPrice2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MntryVal", skip_serializing_if = "Option::is_none") )]
	pub mntry_val: Option<AmountAndDirection61>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pctg", skip_serializing_if = "Option::is_none") )]
	pub pctg: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Yld", skip_serializing_if = "Option::is_none") )]
	pub yld: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BsisPts", skip_serializing_if = "Option::is_none") )]
	pub bsis_pts: Option<f64>,
}

impl SecuritiesTransactionPrice2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mntry_val { val.validate()? }
		Ok(())
	}
}


// SecuritiesTransactionPrice4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionPrice4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pric", skip_serializing_if = "Option::is_none") )]
	pub pric: Option<SecuritiesTransactionPrice2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoPric", skip_serializing_if = "Option::is_none") )]
	pub no_pric: Option<SecuritiesTransactionPrice1>,
}

impl SecuritiesTransactionPrice4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pric { val.validate()? }
		if let Some(ref val) = self.no_pric { val.validate()? }
		Ok(())
	}
}


// SecuritiesTransactionPrice6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionPrice6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pdg") )]
	pub pdg: PriceStatus1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
	pub ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DgtlTkn", skip_serializing_if = "Option::is_none") )]
	pub dgtl_tkn: Option<Vec<DigitalTokenAmount2>>,
}

impl SecuritiesTransactionPrice6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pdg.validate()?;
		if let Some(ref val) = self.ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.dgtl_tkn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecuritiesTransactionPrice7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionPrice7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MntryVal") )]
	pub mntry_val: AmountAndDirection61,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DgtlTknQty") )]
	pub dgtl_tkn_qty: DigitalTokenAmount2,
}

impl SecuritiesTransactionPrice7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.mntry_val.validate()?;
		self.dgtl_tkn_qty.validate()?;
		Ok(())
	}
}


// SecuritiesTransactionReport2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionReport2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxId") )]
	pub tx_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExctgPty") )]
	pub exctg_pty: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubmitgPty") )]
	pub submitg_pty: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechAttrbts", skip_serializing_if = "Option::is_none") )]
	pub tech_attrbts: Option<RecordTechnicalData2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SecuritiesTransactionReport2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tx_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tx_id is shorter than the minimum length of 1".to_string()));
		}
		if self.tx_id.chars().count() > 52 {
			return Err(ValidationError::new(1002, "tx_id exceeds the maximum length of 52".to_string()));
		}
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.exctg_pty) {
			return Err(ValidationError::new(1005, "exctg_pty does not match the required pattern".to_string()));
		}
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.submitg_pty) {
			return Err(ValidationError::new(1005, "submitg_pty does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.tech_attrbts { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecuritiesTransactionReport7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionReport7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxId") )]
	pub tx_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExctgPty") )]
	pub exctg_pty: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtPtyInd") )]
	pub invstmt_pty_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubmitgPty") )]
	pub submitg_pty: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Buyr") )]
	pub buyr: PartyIdentification79,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sellr") )]
	pub sellr: PartyIdentification79,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrTrnsmssn") )]
	pub ordr_trnsmssn: SecuritiesTransactionTransmission2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tx") )]
	pub tx: SecuritiesTransaction3,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrm") )]
	pub fin_instrm: FinancialInstrumentAttributes5Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtDcsnPrsn", skip_serializing_if = "Option::is_none") )]
	pub invstmt_dcsn_prsn: Option<InvestmentParty1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExctgPrsn") )]
	pub exctg_prsn: ExecutingParty1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlAttrbts") )]
	pub addtl_attrbts: SecuritiesTransactionIndicator2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechAttrbts", skip_serializing_if = "Option::is_none") )]
	pub tech_attrbts: Option<RecordTechnicalData5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SecuritiesTransactionReport7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tx_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tx_id is shorter than the minimum length of 1".to_string()));
		}
		if self.tx_id.chars().count() > 52 {
			return Err(ValidationError::new(1002, "tx_id exceeds the maximum length of 52".to_string()));
		}
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.exctg_pty) {
			return Err(ValidationError::new(1005, "exctg_pty does not match the required pattern".to_string()));
		}
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.submitg_pty) {
			return Err(ValidationError::new(1005, "submitg_pty does not match the required pattern".to_string()));
		}
		self.buyr.validate()?;
		self.sellr.validate()?;
		self.ordr_trnsmssn.validate()?;
		self.tx.validate()?;
		self.fin_instrm.validate()?;
		if let Some(ref val) = self.invstmt_dcsn_prsn { val.validate()? }
		self.exctg_prsn.validate()?;
		self.addtl_attrbts.validate()?;
		if let Some(ref val) = self.tech_attrbts { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecuritiesTransactionTransmission2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionTransmission2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrnsmssnInd") )]
	pub trnsmssn_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrnsmttgBuyr", skip_serializing_if = "Option::is_none") )]
	pub trnsmttg_buyr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrnsmttgSellr", skip_serializing_if = "Option::is_none") )]
	pub trnsmttg_sellr: Option<String>,
}

impl SecuritiesTransactionTransmission2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.trnsmttg_buyr {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "trnsmttg_buyr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.trnsmttg_sellr {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "trnsmttg_sellr does not match the required pattern".to_string()));
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
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.othr_id { for item in vec { item.validate()? } }
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


// SecurityInstrumentDescription22 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityInstrumentDescription22 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmGnlAttrbts") )]
	pub fin_instrm_gnl_attrbts: SecurityInstrumentDescription23,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DebtInstrmAttrbts", skip_serializing_if = "Option::is_none") )]
	pub debt_instrm_attrbts: Option<DebtInstrument4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DerivInstrmAttrbts") )]
	pub deriv_instrm_attrbts: DerivativeInstrument6,
}

impl SecurityInstrumentDescription22 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.fin_instrm_gnl_attrbts.validate()?;
		if let Some(ref val) = self.debt_instrm_attrbts { val.validate()? }
		self.deriv_instrm_attrbts.validate()?;
		Ok(())
	}
}


// SecurityInstrumentDescription23 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityInstrumentDescription23 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FullNm") )]
	pub full_nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnTp") )]
	pub clssfctn_tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlCcy", skip_serializing_if = "Option::is_none") )]
	pub ntnl_ccy: Option<String>,
}

impl SecurityInstrumentDescription23 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.othr_id { for item in vec { item.validate()? } }
		if self.full_nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "full_nm is shorter than the minimum length of 1".to_string()));
		}
		if self.full_nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "full_nm exceeds the maximum length of 350".to_string()));
		}
		let pattern = Regex::new("[A-Z]{6,6}").unwrap();
		if !pattern.is_match(&self.clssfctn_tp) {
			return Err(ValidationError::new(1005, "clssfctn_tp does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.ntnl_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ntnl_ccy does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// Side5Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum Side5Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SESH") )]
	CodeSESH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SELL") )]
	CodeSELL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SSEX") )]
	CodeSSEX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UNDI") )]
	CodeUNDI,
}

impl Side5Code {
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
		self.envlp.validate()?;
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


// SwapLegIdentification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SwapLegIdentification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SwpIn", skip_serializing_if = "Option::is_none") )]
	pub swp_in: Option<FinancialInstrumentIdentification7Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SwpOut", skip_serializing_if = "Option::is_none") )]
	pub swp_out: Option<FinancialInstrumentIdentification7Choice>,
}

impl SwapLegIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.swp_in { val.validate()? }
		if let Some(ref val) = self.swp_out { val.validate()? }
		Ok(())
	}
}


// UnderlyingIdentification2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnderlyingIdentification2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Swp", skip_serializing_if = "Option::is_none") )]
	pub swp: Option<SwapLegIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<FinancialInstrumentIdentification7Choice>,
}

impl UnderlyingIdentification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.swp { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
		Ok(())
	}
}


// VariationType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum VariationType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DECR") )]
	CodeDECR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INCR") )]
	CodeINCR,
}

impl VariationType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
