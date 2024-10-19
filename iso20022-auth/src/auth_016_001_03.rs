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

use serde::{Deserialize, Serialize};
use regex::Regex;
use crate::validationerror::*;
// ActiveCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and13_decimal_amount_simple_type: f64,
}

impl ActiveCurrencyAnd13DecimalAmountSimpleType {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.active_currency_and13_decimal_amount_simple_type < 0.000000 {
			return Err(ValidationError::new(1003, "active_currency_and13_decimal_amount_simple_type is less than the minimum value of 0.000000".to_string()));
		}
		Ok(())
	}
}


// ActiveCurrencyAnd13DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveCurrencyAnd13DecimalAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
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


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAndAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "$value")]
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


// AmountAndDirection53 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection53 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}

impl AmountAndDirection53 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		Ok(())
	}
}


// AmountAndDirection61 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection61 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}

impl AmountAndDirection61 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		Ok(())
	}
}


// AssetClassAttributes1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassAttributes1 {
	#[serde(rename = "Intrst")]
	pub intrst: DerivativeInterest2,
	#[serde(rename = "FX")]
	pub fx: DerivativeForeignExchange2,
}

impl AssetClassAttributes1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.intrst.validate() { return Err(e); }
		if let Err(e) = self.fx.validate() { return Err(e); }
		Ok(())
	}
}


// AssetClassAttributes1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassAttributes1Choice {
	#[serde(rename = "Intrst", skip_serializing_if = "Option::is_none")]
	pub intrst: Option<DerivativeInterest2>,
	#[serde(rename = "FX", skip_serializing_if = "Option::is_none")]
	pub fx: Option<DerivativeForeignExchange2>,
	#[serde(rename = "Both", skip_serializing_if = "Option::is_none")]
	pub both: Option<AssetClassAttributes1>,
}

impl AssetClassAttributes1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref intrst_value) = self.intrst { if let Err(e) = intrst_value.validate() { return Err(e); } }
		if let Some(ref fx_value) = self.fx { if let Err(e) = fx_value.validate() { return Err(e); } }
		if let Some(ref both_value) = self.both { if let Err(e) = both_value.validate() { return Err(e); } }
		Ok(())
	}
}


// BasketDescription3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BasketDescription3 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<Vec<ISINOct2015Identifier>>,
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<Vec<FinancialInstrument58>>,
}

impl BasketDescription3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref isin_vec) = self.isin { for item in isin_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref indx_vec) = self.indx { for item in indx_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// BenchmarkCurveName2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BenchmarkCurveName2Code {
	#[default]
	#[serde(rename = "WIBO")]
	CodeWIBO,
	#[serde(rename = "TREA")]
	CodeTREA,
	#[serde(rename = "TIBO")]
	CodeTIBO,
	#[serde(rename = "TLBO")]
	CodeTLBO,
	#[serde(rename = "SWAP")]
	CodeSWAP,
	#[serde(rename = "STBO")]
	CodeSTBO,
	#[serde(rename = "PRBO")]
	CodePRBO,
	#[serde(rename = "PFAN")]
	CodePFAN,
	#[serde(rename = "NIBO")]
	CodeNIBO,
	#[serde(rename = "MAAA")]
	CodeMAAA,
	#[serde(rename = "MOSP")]
	CodeMOSP,
	#[serde(rename = "LIBO")]
	CodeLIBO,
	#[serde(rename = "LIBI")]
	CodeLIBI,
	#[serde(rename = "JIBA")]
	CodeJIBA,
	#[serde(rename = "ISDA")]
	CodeISDA,
	#[serde(rename = "GCFR")]
	CodeGCFR,
	#[serde(rename = "FUSW")]
	CodeFUSW,
	#[serde(rename = "EUCH")]
	CodeEUCH,
	#[serde(rename = "EUUS")]
	CodeEUUS,
	#[serde(rename = "EURI")]
	CodeEURI,
	#[serde(rename = "EONS")]
	CodeEONS,
	#[serde(rename = "EONA")]
	CodeEONA,
	#[serde(rename = "CIBO")]
	CodeCIBO,
	#[serde(rename = "CDOR")]
	CodeCDOR,
	#[serde(rename = "BUBO")]
	CodeBUBO,
	#[serde(rename = "BBSW")]
	CodeBBSW,
}

impl BenchmarkCurveName2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// BenchmarkCurveName5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName5Choice {
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<BenchmarkCurveName2Code>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max25Text>,
}

impl BenchmarkCurveName5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref indx_value) = self.indx { if let Err(e) = indx_value.validate() { return Err(e); } }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "$value")]
	pub cfi_oct2015_identifier: String,
}

impl CFIOct2015Identifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{6,6}").unwrap();
		if !pattern.is_match(&self.cfi_oct2015_identifier) {
			return Err(ValidationError::new(1005, "cfi_oct2015_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// CancelledStatusReason15Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CancelledStatusReason15Code {
	#[default]
	#[serde(rename = "CANI")]
	CodeCANI,
	#[serde(rename = "CSUB")]
	CodeCSUB,
}

impl CancelledStatusReason15Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountryCode {
	#[serde(rename = "$value")]
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


// DTI2021Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DTI2021Identifier {
	#[serde(rename = "$value")]
	pub dti2021_identifier: String,
}

impl DTI2021Identifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[1-9B-DF-HJ-NP-XZ][0-9B-DF-HJ-NP-XZ]{8,8}").unwrap();
		if !pattern.is_match(&self.dti2021_identifier) {
			return Err(ValidationError::new(1005, "dti2021_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// DebtInstrument4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DebtInstrument4 {
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: String,
}

impl DebtInstrument4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}

impl DecimalNumber {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DerivativeForeignExchange2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeForeignExchange2 {
	#[serde(rename = "OthrNtnlCcy")]
	pub othr_ntnl_ccy: ActiveOrHistoricCurrencyCode,
}

impl DerivativeForeignExchange2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.othr_ntnl_ccy.validate() { return Err(e); }
		Ok(())
	}
}


// DerivativeInstrument6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeInstrument6 {
	#[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
	pub xpry_dt: Option<String>,
	#[serde(rename = "PricMltplr")]
	pub pric_mltplr: f64,
	#[serde(rename = "UndrlygInstrm")]
	pub undrlyg_instrm: UnderlyingIdentification2Choice,
	#[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
	pub optn_tp: Option<OptionType2Code>,
	#[serde(rename = "StrkPric", skip_serializing_if = "Option::is_none")]
	pub strk_pric: Option<SecuritiesTransactionPrice4Choice>,
	#[serde(rename = "OptnExrcStyle", skip_serializing_if = "Option::is_none")]
	pub optn_exrc_style: Option<OptionStyle7Code>,
	#[serde(rename = "DlvryTp")]
	pub dlvry_tp: PhysicalTransferType4Code,
	#[serde(rename = "AsstClssSpcfcAttrbts", skip_serializing_if = "Option::is_none")]
	pub asst_clss_spcfc_attrbts: Option<AssetClassAttributes1Choice>,
}

impl DerivativeInstrument6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.undrlyg_instrm.validate() { return Err(e); }
		if let Some(ref optn_tp_value) = self.optn_tp { if let Err(e) = optn_tp_value.validate() { return Err(e); } }
		if let Some(ref strk_pric_value) = self.strk_pric { if let Err(e) = strk_pric_value.validate() { return Err(e); } }
		if let Some(ref optn_exrc_style_value) = self.optn_exrc_style { if let Err(e) = optn_exrc_style_value.validate() { return Err(e); } }
		if let Err(e) = self.dlvry_tp.validate() { return Err(e); }
		if let Some(ref asst_clss_spcfc_attrbts_value) = self.asst_clss_spcfc_attrbts { if let Err(e) = asst_clss_spcfc_attrbts_value.validate() { return Err(e); } }
		Ok(())
	}
}


// DerivativeInterest2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeInterest2 {
	#[serde(rename = "OthrNtnlCcy")]
	pub othr_ntnl_ccy: ActiveOrHistoricCurrencyCode,
}

impl DerivativeInterest2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.othr_ntnl_ccy.validate() { return Err(e); }
		Ok(())
	}
}


// DigitalTokenAmount2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DigitalTokenAmount2 {
	#[serde(rename = "Idr")]
	pub idr: DTI2021Identifier,
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max30Text>,
}

impl DigitalTokenAmount2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.idr.validate() { return Err(e); }
		if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ExecutingParty1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExecutingParty1Choice {
	#[serde(rename = "Prsn", skip_serializing_if = "Option::is_none")]
	pub prsn: Option<PersonIdentification12>,
	#[serde(rename = "Algo", skip_serializing_if = "Option::is_none")]
	pub algo: Option<Max50Text>,
	#[serde(rename = "Clnt", skip_serializing_if = "Option::is_none")]
	pub clnt: Option<NoReasonCode>,
}

impl ExecutingParty1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref prsn_value) = self.prsn { if let Err(e) = prsn_value.validate() { return Err(e); } }
		if let Some(ref algo_value) = self.algo { if let Err(e) = algo_value.validate() { return Err(e); } }
		if let Some(ref clnt_value) = self.clnt { if let Err(e) = clnt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ExternalAuthorityExchangeReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalAuthorityExchangeReason1Code {
	#[serde(rename = "$value")]
	pub external_authority_exchange_reason1_code: String,
}

impl ExternalAuthorityExchangeReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_authority_exchange_reason1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_authority_exchange_reason1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_authority_exchange_reason1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_authority_exchange_reason1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalFinancialInstrumentIdentificationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalFinancialInstrumentIdentificationType1Code {
	#[serde(rename = "$value")]
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


// ExternalPersonIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalPersonIdentification1Code {
	#[serde(rename = "$value")]
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


// FinancialInstrument58 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument58 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "Nm")]
	pub nm: FloatingInterestRate8,
}

impl FinancialInstrument58 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
		if let Err(e) = self.nm.validate() { return Err(e); }
		Ok(())
	}
}


// FinancialInstrumentAttributes5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentAttributes5Choice {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ISINOct2015Identifier>,
	#[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
	pub altrn_id: Option<SecurityIdentification19>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<SecurityInstrumentDescription22>,
}

impl FinancialInstrumentAttributes5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		if let Some(ref altrn_id_value) = self.altrn_id { if let Err(e) = altrn_id_value.validate() { return Err(e); } }
		if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// FinancialInstrumentIdentification6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentIdentification6Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<FinancialInstrument58>,
}

impl FinancialInstrumentIdentification6Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
		if let Some(ref indx_value) = self.indx { if let Err(e) = indx_value.validate() { return Err(e); } }
		Ok(())
	}
}


// FinancialInstrumentIdentification7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentIdentification7Choice {
	#[serde(rename = "Sngl", skip_serializing_if = "Option::is_none")]
	pub sngl: Option<FinancialInstrumentIdentification6Choice>,
	#[serde(rename = "Bskt", skip_serializing_if = "Option::is_none")]
	pub bskt: Option<BasketDescription3>,
}

impl FinancialInstrumentIdentification7Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref sngl_value) = self.sngl { if let Err(e) = sngl_value.validate() { return Err(e); } }
		if let Some(ref bskt_value) = self.bskt { if let Err(e) = bskt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// FinancialInstrumentQuantity25Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentQuantity25Choice {
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
	#[serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none")]
	pub nmnl_val: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl FinancialInstrumentQuantity25Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref nmnl_val_value) = self.nmnl_val { if let Err(e) = nmnl_val_value.validate() { return Err(e); } }
		if let Some(ref mntry_val_value) = self.mntry_val { if let Err(e) = mntry_val_value.validate() { return Err(e); } }
		Ok(())
	}
}


// FinancialInstrumentReportingTransactionReportV03 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentReportingTransactionReportV03 {
	#[serde(rename = "Tx")]
	pub tx: Vec<ReportingTransactionType3Choice>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl FinancialInstrumentReportingTransactionReportV03 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.tx { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// FloatingInterestRate8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatingInterestRate8 {
	#[serde(rename = "RefRate")]
	pub ref_rate: BenchmarkCurveName5Choice,
	#[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
	pub term: Option<InterestRateContractTerm2>,
}

impl FloatingInterestRate8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ref_rate.validate() { return Err(e); }
		if let Some(ref term_value) = self.term { if let Err(e) = term_value.validate() { return Err(e); } }
		Ok(())
	}
}


// GenericPersonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericPersonIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
		if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "$value")]
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


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}

impl ISODate {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}

impl ISODateTime {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// IdentificationSource3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl IdentificationSource3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ImpliedCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "$value")]
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


// InterestRateContractTerm2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateContractTerm2 {
	#[serde(rename = "Unit")]
	pub unit: RateBasis1Code,
	#[serde(rename = "Val")]
	pub val: f64,
}

impl InterestRateContractTerm2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.unit.validate() { return Err(e); }
		Ok(())
	}
}


// InternalPartyRole1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InternalPartyRole1Code {
	#[default]
	#[serde(rename = "INTC")]
	CodeINTC,
}

impl InternalPartyRole1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestmentParty1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentParty1Choice {
	#[serde(rename = "Prsn", skip_serializing_if = "Option::is_none")]
	pub prsn: Option<PersonIdentification12>,
	#[serde(rename = "Algo", skip_serializing_if = "Option::is_none")]
	pub algo: Option<Max50Text>,
}

impl InvestmentParty1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref prsn_value) = self.prsn { if let Err(e) = prsn_value.validate() { return Err(e); } }
		if let Some(ref algo_value) = self.algo { if let Err(e) = algo_value.validate() { return Err(e); } }
		Ok(())
	}
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
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


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MICIdentifier {
	#[serde(rename = "$value")]
	pub mic_identifier: String,
}

impl MICIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
		if !pattern.is_match(&self.mic_identifier) {
			return Err(ValidationError::new(1005, "mic_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max140Text {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max16Text {
	#[serde(rename = "$value")]
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


// Max25Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max25Text {
	#[serde(rename = "$value")]
	pub max25_text: String,
}

impl Max25Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max25_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max25_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max25_text.chars().count() > 25 {
			return Err(ValidationError::new(1002, "max25_text exceeds the maximum length of 25".to_string()));
		}
		Ok(())
	}
}


// Max30DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max30DecimalNumber {
	#[serde(rename = "$value")]
	pub max30_decimal_number: f64,
}

impl Max30DecimalNumber {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Max30Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max30Text {
	#[serde(rename = "$value")]
	pub max30_text: String,
}

impl Max30Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max30_text.chars().count() > 30 {
			return Err(ValidationError::new(1002, "max30_text exceeds the maximum length of 30".to_string()));
		}
		Ok(())
	}
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
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


// Max3Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max3Number {
	#[serde(rename = "$value")]
	pub max3_number: f64,
}

impl Max3Number {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Max50Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max50Text {
	#[serde(rename = "$value")]
	pub max50_text: String,
}

impl Max50Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max50_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max50_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max50_text.chars().count() > 50 {
			return Err(ValidationError::new(1002, "max50_text exceeds the maximum length of 50".to_string()));
		}
		Ok(())
	}
}


// Max52Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max52Text {
	#[serde(rename = "$value")]
	pub max52_text: String,
}

impl Max52Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max52_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max52_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max52_text.chars().count() > 52 {
			return Err(ValidationError::new(1002, "max52_text exceeds the maximum length of 52".to_string()));
		}
		Ok(())
	}
}


// NoReasonCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NoReasonCode {
	#[default]
	#[serde(rename = "NORE")]
	CodeNORE,
}

impl NoReasonCode {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// NonNegativeDecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NonNegativeDecimalNumber {
	#[serde(rename = "$value")]
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


// OptionStyle7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionStyle7Code {
	#[default]
	#[serde(rename = "AMER")]
	CodeAMER,
	#[serde(rename = "ASIA")]
	CodeASIA,
	#[serde(rename = "BERM")]
	CodeBERM,
	#[serde(rename = "EURO")]
	CodeEURO,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl OptionStyle7Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OptionType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionType2Code {
	#[default]
	#[serde(rename = "CALL")]
	CodeCALL,
	#[serde(rename = "PUTO")]
	CodePUTO,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl OptionType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OtherIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Sfx", skip_serializing_if = "Option::is_none")]
	pub sfx: Option<Max16Text>,
	#[serde(rename = "Tp")]
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


// PartyIdentification76 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification76 {
	#[serde(rename = "Id")]
	pub id: PersonOrOrganisation1Choice,
	#[serde(rename = "CtryOfBrnch", skip_serializing_if = "Option::is_none")]
	pub ctry_of_brnch: Option<CountryCode>,
}

impl PartyIdentification76 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref ctry_of_brnch_value) = self.ctry_of_brnch { if let Err(e) = ctry_of_brnch_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyIdentification79 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification79 {
	#[serde(rename = "AcctOwnr")]
	pub acct_ownr: Vec<PartyIdentification76>,
	#[serde(rename = "DcsnMakr", skip_serializing_if = "Option::is_none")]
	pub dcsn_makr: Option<Vec<PersonOrOrganisation2Choice>>,
}

impl PartyIdentification79 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.acct_ownr { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref dcsn_makr_vec) = self.dcsn_makr { for item in dcsn_makr_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
}

impl PercentageRate {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PersonIdentification10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification10 {
	#[serde(rename = "FrstNm")]
	pub frst_nm: Max140Text,
	#[serde(rename = "Nm")]
	pub nm: Max140Text,
	#[serde(rename = "BirthDt")]
	pub birth_dt: String,
	#[serde(rename = "Othr")]
	pub othr: GenericPersonIdentification1,
}

impl PersonIdentification10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.frst_nm.validate() { return Err(e); }
		if let Err(e) = self.nm.validate() { return Err(e); }
		if let Err(e) = self.othr.validate() { return Err(e); }
		Ok(())
	}
}


// PersonIdentification12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification12 {
	#[serde(rename = "CtryOfBrnch")]
	pub ctry_of_brnch: CountryCode,
	#[serde(rename = "Othr")]
	pub othr: GenericPersonIdentification1,
}

impl PersonIdentification12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ctry_of_brnch.validate() { return Err(e); }
		if let Err(e) = self.othr.validate() { return Err(e); }
		Ok(())
	}
}


// PersonIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalPersonIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl PersonIdentificationSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PersonOrOrganisation1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonOrOrganisation1Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "MIC", skip_serializing_if = "Option::is_none")]
	pub mic: Option<MICIdentifier>,
	#[serde(rename = "Prsn", skip_serializing_if = "Option::is_none")]
	pub prsn: Option<PersonIdentification10>,
	#[serde(rename = "Intl", skip_serializing_if = "Option::is_none")]
	pub intl: Option<InternalPartyRole1Code>,
}

impl PersonOrOrganisation1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
		if let Some(ref mic_value) = self.mic { if let Err(e) = mic_value.validate() { return Err(e); } }
		if let Some(ref prsn_value) = self.prsn { if let Err(e) = prsn_value.validate() { return Err(e); } }
		if let Some(ref intl_value) = self.intl { if let Err(e) = intl_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PersonOrOrganisation2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonOrOrganisation2Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Prsn", skip_serializing_if = "Option::is_none")]
	pub prsn: Option<PersonIdentification10>,
}

impl PersonOrOrganisation2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
		if let Some(ref prsn_value) = self.prsn { if let Err(e) = prsn_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PhysicalTransferType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PhysicalTransferType4Code {
	#[default]
	#[serde(rename = "PHYS")]
	CodePHYS,
	#[serde(rename = "OPTL")]
	CodeOPTL,
	#[serde(rename = "CASH")]
	CodeCASH,
}

impl PhysicalTransferType4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "$value")]
	pub plus_or_minus_indicator: bool,
}

impl PlusOrMinusIndicator {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PriceStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PriceStatus1Code {
	#[default]
	#[serde(rename = "PNDG")]
	CodePNDG,
	#[serde(rename = "NOAP")]
	CodeNOAP,
}

impl PriceStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RateBasis1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RateBasis1Code {
	#[default]
	#[serde(rename = "DAYS")]
	CodeDAYS,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "YEAR")]
	CodeYEAR,
}

impl RateBasis1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RecordTechnicalData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RecordTechnicalData2 {
	#[serde(rename = "RctDtTm")]
	pub rct_dt_tm: String,
	#[serde(rename = "CxlRsn")]
	pub cxl_rsn: CancelledStatusReason15Code,
}

impl RecordTechnicalData2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.cxl_rsn.validate() { return Err(e); }
		Ok(())
	}
}


// RecordTechnicalData5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RecordTechnicalData5 {
	#[serde(rename = "RctDtTm")]
	pub rct_dt_tm: String,
	#[serde(rename = "XchgRsn")]
	pub xchg_rsn: Vec<ExternalAuthorityExchangeReason1Code>,
}

impl RecordTechnicalData5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.xchg_rsn { if let Err(e) = item.validate() { return Err(e); } }
		Ok(())
	}
}


// RegulatoryTradingCapacity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RegulatoryTradingCapacity1Code {
	#[default]
	#[serde(rename = "MTCH")]
	CodeMTCH,
	#[serde(rename = "DEAL")]
	CodeDEAL,
	#[serde(rename = "AOTC")]
	CodeAOTC,
}

impl RegulatoryTradingCapacity1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ReportingTransactionType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportingTransactionType3Choice {
	#[serde(rename = "New", skip_serializing_if = "Option::is_none")]
	pub new: Option<SecuritiesTransactionReport7>,
	#[serde(rename = "Cxl", skip_serializing_if = "Option::is_none")]
	pub cxl: Option<SecuritiesTransactionReport2>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl ReportingTransactionType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref new_value) = self.new { if let Err(e) = new_value.validate() { return Err(e); } }
		if let Some(ref cxl_value) = self.cxl { if let Err(e) = cxl_value.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// ReportingWaiverType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportingWaiverType1Code {
	#[default]
	#[serde(rename = "OILQ")]
	CodeOILQ,
	#[serde(rename = "NLIQ")]
	CodeNLIQ,
	#[serde(rename = "PRIC")]
	CodePRIC,
	#[serde(rename = "ILQD")]
	CodeILQD,
	#[serde(rename = "RFPT")]
	CodeRFPT,
	#[serde(rename = "SIZE")]
	CodeSIZE,
}

impl ReportingWaiverType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ReportingWaiverType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportingWaiverType3Code {
	#[default]
	#[serde(rename = "BENC")]
	CodeBENC,
	#[serde(rename = "ACTX")]
	CodeACTX,
	#[serde(rename = "ILQD")]
	CodeILQD,
	#[serde(rename = "SIZE")]
	CodeSIZE,
	#[serde(rename = "CANC")]
	CodeCANC,
	#[serde(rename = "AMND")]
	CodeAMND,
	#[serde(rename = "SDIV")]
	CodeSDIV,
	#[serde(rename = "RPRI")]
	CodeRPRI,
	#[serde(rename = "DUPL")]
	CodeDUPL,
	#[serde(rename = "LRGS")]
	CodeLRGS,
	#[serde(rename = "TNCP")]
	CodeTNCP,
	#[serde(rename = "TPAC")]
	CodeTPAC,
	#[serde(rename = "XFPH")]
	CodeXFPH,
}

impl ReportingWaiverType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecuritiesTransaction3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransaction3 {
	#[serde(rename = "TradDt")]
	pub trad_dt: String,
	#[serde(rename = "TradgCpcty")]
	pub tradg_cpcty: RegulatoryTradingCapacity1Code,
	#[serde(rename = "Qty")]
	pub qty: FinancialInstrumentQuantity25Choice,
	#[serde(rename = "DgtlTknQty", skip_serializing_if = "Option::is_none")]
	pub dgtl_tkn_qty: Option<Vec<DigitalTokenAmount2>>,
	#[serde(rename = "DerivNtnlChng", skip_serializing_if = "Option::is_none")]
	pub deriv_ntnl_chng: Option<VariationType1Code>,
	#[serde(rename = "Pric")]
	pub pric: SecuritiesTransactionPrice22Choice,
	#[serde(rename = "NetAmt", skip_serializing_if = "Option::is_none")]
	pub net_amt: Option<f64>,
	#[serde(rename = "TradVn")]
	pub trad_vn: MICIdentifier,
	#[serde(rename = "CtryOfBrnch", skip_serializing_if = "Option::is_none")]
	pub ctry_of_brnch: Option<CountryCode>,
	#[serde(rename = "UpFrntPmt", skip_serializing_if = "Option::is_none")]
	pub up_frnt_pmt: Option<AmountAndDirection53>,
	#[serde(rename = "TradPlcMtchgId", skip_serializing_if = "Option::is_none")]
	pub trad_plc_mtchg_id: Option<Max52Text>,
	#[serde(rename = "CmplxTradCmpntId", skip_serializing_if = "Option::is_none")]
	pub cmplx_trad_cmpnt_id: Option<Max35Text>,
}

impl SecuritiesTransaction3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tradg_cpcty.validate() { return Err(e); }
		if let Err(e) = self.qty.validate() { return Err(e); }
		if let Some(ref dgtl_tkn_qty_vec) = self.dgtl_tkn_qty { for item in dgtl_tkn_qty_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref deriv_ntnl_chng_value) = self.deriv_ntnl_chng { if let Err(e) = deriv_ntnl_chng_value.validate() { return Err(e); } }
		if let Err(e) = self.pric.validate() { return Err(e); }
		if let Err(e) = self.trad_vn.validate() { return Err(e); }
		if let Some(ref ctry_of_brnch_value) = self.ctry_of_brnch { if let Err(e) = ctry_of_brnch_value.validate() { return Err(e); } }
		if let Some(ref up_frnt_pmt_value) = self.up_frnt_pmt { if let Err(e) = up_frnt_pmt_value.validate() { return Err(e); } }
		if let Some(ref trad_plc_mtchg_id_value) = self.trad_plc_mtchg_id { if let Err(e) = trad_plc_mtchg_id_value.validate() { return Err(e); } }
		if let Some(ref cmplx_trad_cmpnt_id_value) = self.cmplx_trad_cmpnt_id { if let Err(e) = cmplx_trad_cmpnt_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecuritiesTransactionIndicator2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionIndicator2 {
	#[serde(rename = "WvrInd", skip_serializing_if = "Option::is_none")]
	pub wvr_ind: Option<Vec<ReportingWaiverType1Code>>,
	#[serde(rename = "ShrtSellgInd", skip_serializing_if = "Option::is_none")]
	pub shrt_sellg_ind: Option<Side5Code>,
	#[serde(rename = "OTCPstTradInd", skip_serializing_if = "Option::is_none")]
	pub otc_pst_trad_ind: Option<Vec<ReportingWaiverType3Code>>,
	#[serde(rename = "RskRdcgTx", skip_serializing_if = "Option::is_none")]
	pub rsk_rdcg_tx: Option<bool>,
	#[serde(rename = "SctiesFincgTxInd")]
	pub scties_fincg_tx_ind: bool,
}

impl SecuritiesTransactionIndicator2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref wvr_ind_vec) = self.wvr_ind { for item in wvr_ind_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref shrt_sellg_ind_value) = self.shrt_sellg_ind { if let Err(e) = shrt_sellg_ind_value.validate() { return Err(e); } }
		if let Some(ref otc_pst_trad_ind_vec) = self.otc_pst_trad_ind { for item in otc_pst_trad_ind_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SecuritiesTransactionPrice1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice1 {
	#[serde(rename = "Pdg")]
	pub pdg: PriceStatus1Code,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
}

impl SecuritiesTransactionPrice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pdg.validate() { return Err(e); }
		if let Some(ref ccy_value) = self.ccy { if let Err(e) = ccy_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecuritiesTransactionPrice22Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice22Choice {
	#[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
	pub pric: Option<SecuritiesTransactionPrice2Choice>,
	#[serde(rename = "DgtlTknPric", skip_serializing_if = "Option::is_none")]
	pub dgtl_tkn_pric: Option<SecuritiesTransactionPrice7>,
	#[serde(rename = "NoPric", skip_serializing_if = "Option::is_none")]
	pub no_pric: Option<SecuritiesTransactionPrice6>,
}

impl SecuritiesTransactionPrice22Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref pric_value) = self.pric { if let Err(e) = pric_value.validate() { return Err(e); } }
		if let Some(ref dgtl_tkn_pric_value) = self.dgtl_tkn_pric { if let Err(e) = dgtl_tkn_pric_value.validate() { return Err(e); } }
		if let Some(ref no_pric_value) = self.no_pric { if let Err(e) = no_pric_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecuritiesTransactionPrice2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice2Choice {
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<AmountAndDirection61>,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
	#[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
	pub yld: Option<f64>,
	#[serde(rename = "BsisPts", skip_serializing_if = "Option::is_none")]
	pub bsis_pts: Option<f64>,
}

impl SecuritiesTransactionPrice2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref mntry_val_value) = self.mntry_val { if let Err(e) = mntry_val_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecuritiesTransactionPrice4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice4Choice {
	#[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
	pub pric: Option<SecuritiesTransactionPrice2Choice>,
	#[serde(rename = "NoPric", skip_serializing_if = "Option::is_none")]
	pub no_pric: Option<SecuritiesTransactionPrice1>,
}

impl SecuritiesTransactionPrice4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref pric_value) = self.pric { if let Err(e) = pric_value.validate() { return Err(e); } }
		if let Some(ref no_pric_value) = self.no_pric { if let Err(e) = no_pric_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecuritiesTransactionPrice6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice6 {
	#[serde(rename = "Pdg")]
	pub pdg: PriceStatus1Code,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "DgtlTkn", skip_serializing_if = "Option::is_none")]
	pub dgtl_tkn: Option<Vec<DigitalTokenAmount2>>,
}

impl SecuritiesTransactionPrice6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pdg.validate() { return Err(e); }
		if let Some(ref ccy_value) = self.ccy { if let Err(e) = ccy_value.validate() { return Err(e); } }
		if let Some(ref dgtl_tkn_vec) = self.dgtl_tkn { for item in dgtl_tkn_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SecuritiesTransactionPrice7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice7 {
	#[serde(rename = "MntryVal")]
	pub mntry_val: AmountAndDirection61,
	#[serde(rename = "DgtlTknQty")]
	pub dgtl_tkn_qty: DigitalTokenAmount2,
}

impl SecuritiesTransactionPrice7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.mntry_val.validate() { return Err(e); }
		if let Err(e) = self.dgtl_tkn_qty.validate() { return Err(e); }
		Ok(())
	}
}


// SecuritiesTransactionReport2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionReport2 {
	#[serde(rename = "TxId")]
	pub tx_id: Max52Text,
	#[serde(rename = "ExctgPty")]
	pub exctg_pty: LEIIdentifier,
	#[serde(rename = "SubmitgPty")]
	pub submitg_pty: LEIIdentifier,
	#[serde(rename = "TechAttrbts", skip_serializing_if = "Option::is_none")]
	pub tech_attrbts: Option<RecordTechnicalData2>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SecuritiesTransactionReport2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tx_id.validate() { return Err(e); }
		if let Err(e) = self.exctg_pty.validate() { return Err(e); }
		if let Err(e) = self.submitg_pty.validate() { return Err(e); }
		if let Some(ref tech_attrbts_value) = self.tech_attrbts { if let Err(e) = tech_attrbts_value.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SecuritiesTransactionReport7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionReport7 {
	#[serde(rename = "TxId")]
	pub tx_id: Max52Text,
	#[serde(rename = "ExctgPty")]
	pub exctg_pty: LEIIdentifier,
	#[serde(rename = "InvstmtPtyInd")]
	pub invstmt_pty_ind: bool,
	#[serde(rename = "SubmitgPty")]
	pub submitg_pty: LEIIdentifier,
	#[serde(rename = "Buyr")]
	pub buyr: PartyIdentification79,
	#[serde(rename = "Sellr")]
	pub sellr: PartyIdentification79,
	#[serde(rename = "OrdrTrnsmssn")]
	pub ordr_trnsmssn: SecuritiesTransactionTransmission2,
	#[serde(rename = "Tx")]
	pub tx: SecuritiesTransaction3,
	#[serde(rename = "FinInstrm")]
	pub fin_instrm: FinancialInstrumentAttributes5Choice,
	#[serde(rename = "InvstmtDcsnPrsn", skip_serializing_if = "Option::is_none")]
	pub invstmt_dcsn_prsn: Option<InvestmentParty1Choice>,
	#[serde(rename = "ExctgPrsn")]
	pub exctg_prsn: ExecutingParty1Choice,
	#[serde(rename = "AddtlAttrbts")]
	pub addtl_attrbts: SecuritiesTransactionIndicator2,
	#[serde(rename = "TechAttrbts", skip_serializing_if = "Option::is_none")]
	pub tech_attrbts: Option<RecordTechnicalData5>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SecuritiesTransactionReport7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tx_id.validate() { return Err(e); }
		if let Err(e) = self.exctg_pty.validate() { return Err(e); }
		if let Err(e) = self.submitg_pty.validate() { return Err(e); }
		if let Err(e) = self.buyr.validate() { return Err(e); }
		if let Err(e) = self.sellr.validate() { return Err(e); }
		if let Err(e) = self.ordr_trnsmssn.validate() { return Err(e); }
		if let Err(e) = self.tx.validate() { return Err(e); }
		if let Err(e) = self.fin_instrm.validate() { return Err(e); }
		if let Some(ref invstmt_dcsn_prsn_value) = self.invstmt_dcsn_prsn { if let Err(e) = invstmt_dcsn_prsn_value.validate() { return Err(e); } }
		if let Err(e) = self.exctg_prsn.validate() { return Err(e); }
		if let Err(e) = self.addtl_attrbts.validate() { return Err(e); }
		if let Some(ref tech_attrbts_value) = self.tech_attrbts { if let Err(e) = tech_attrbts_value.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SecuritiesTransactionTransmission2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionTransmission2 {
	#[serde(rename = "TrnsmssnInd")]
	pub trnsmssn_ind: bool,
	#[serde(rename = "TrnsmttgBuyr", skip_serializing_if = "Option::is_none")]
	pub trnsmttg_buyr: Option<LEIIdentifier>,
	#[serde(rename = "TrnsmttgSellr", skip_serializing_if = "Option::is_none")]
	pub trnsmttg_sellr: Option<LEIIdentifier>,
}

impl SecuritiesTransactionTransmission2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref trnsmttg_buyr_value) = self.trnsmttg_buyr { if let Err(e) = trnsmttg_buyr_value.validate() { return Err(e); } }
		if let Some(ref trnsmttg_sellr_value) = self.trnsmttg_sellr { if let Err(e) = trnsmttg_sellr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecurityIdentification19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification19 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
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


// SecurityInstrumentDescription22 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityInstrumentDescription22 {
	#[serde(rename = "FinInstrmGnlAttrbts")]
	pub fin_instrm_gnl_attrbts: SecurityInstrumentDescription23,
	#[serde(rename = "DebtInstrmAttrbts", skip_serializing_if = "Option::is_none")]
	pub debt_instrm_attrbts: Option<DebtInstrument4>,
	#[serde(rename = "DerivInstrmAttrbts")]
	pub deriv_instrm_attrbts: DerivativeInstrument6,
}

impl SecurityInstrumentDescription22 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.fin_instrm_gnl_attrbts.validate() { return Err(e); }
		if let Some(ref debt_instrm_attrbts_value) = self.debt_instrm_attrbts { if let Err(e) = debt_instrm_attrbts_value.validate() { return Err(e); } }
		if let Err(e) = self.deriv_instrm_attrbts.validate() { return Err(e); }
		Ok(())
	}
}


// SecurityInstrumentDescription23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityInstrumentDescription23 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ISINOct2015Identifier>,
	#[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "FullNm")]
	pub full_nm: Max350Text,
	#[serde(rename = "ClssfctnTp")]
	pub clssfctn_tp: CFIOct2015Identifier,
	#[serde(rename = "NtnlCcy", skip_serializing_if = "Option::is_none")]
	pub ntnl_ccy: Option<ActiveOrHistoricCurrencyCode>,
}

impl SecurityInstrumentDescription23 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		if let Some(ref othr_id_vec) = self.othr_id { for item in othr_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Err(e) = self.full_nm.validate() { return Err(e); }
		if let Err(e) = self.clssfctn_tp.validate() { return Err(e); }
		if let Some(ref ntnl_ccy_value) = self.ntnl_ccy { if let Err(e) = ntnl_ccy_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Side5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Side5Code {
	#[default]
	#[serde(rename = "SESH")]
	CodeSESH,
	#[serde(rename = "SELL")]
	CodeSELL,
	#[serde(rename = "SSEX")]
	CodeSSEX,
	#[serde(rename = "UNDI")]
	CodeUNDI,
}

impl Side5Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SwapLegIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SwapLegIdentification2 {
	#[serde(rename = "SwpIn", skip_serializing_if = "Option::is_none")]
	pub swp_in: Option<FinancialInstrumentIdentification7Choice>,
	#[serde(rename = "SwpOut", skip_serializing_if = "Option::is_none")]
	pub swp_out: Option<FinancialInstrumentIdentification7Choice>,
}

impl SwapLegIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref swp_in_value) = self.swp_in { if let Err(e) = swp_in_value.validate() { return Err(e); } }
		if let Some(ref swp_out_value) = self.swp_out { if let Err(e) = swp_out_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}

impl TrueFalseIndicator {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UnderlyingIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingIdentification2Choice {
	#[serde(rename = "Swp", skip_serializing_if = "Option::is_none")]
	pub swp: Option<SwapLegIdentification2>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<FinancialInstrumentIdentification7Choice>,
}

impl UnderlyingIdentification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref swp_value) = self.swp { if let Err(e) = swp_value.validate() { return Err(e); } }
		if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// VariationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum VariationType1Code {
	#[default]
	#[serde(rename = "DECR")]
	CodeDECR,
	#[serde(rename = "INCR")]
	CodeINCR,
}

impl VariationType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
