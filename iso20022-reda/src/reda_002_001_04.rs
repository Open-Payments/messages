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


// ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAnd13DecimalAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
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


// AdditionalReference3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditionalReference3 {
	#[serde(rename = "Ref")]
	pub ref_attr: Max35Text,
	#[serde(rename = "RefIssr", skip_serializing_if = "Option::is_none")]
	pub ref_issr: Option<PartyIdentification2Choice>,
	#[serde(rename = "MsgNm", skip_serializing_if = "Option::is_none")]
	pub msg_nm: Option<Max35Text>,
}

impl AdditionalReference3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ref_attr.validate() { return Err(e); }
		if let Some(ref ref_issr_value) = self.ref_issr { if let Err(e) = ref_issr_value.validate() { return Err(e); } }
		if let Some(ref msg_nm_value) = self.msg_nm { if let Err(e) = msg_nm_value.validate() { return Err(e); } }
		Ok(())
	}
}


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AddressType2Code {
	#[default]
	#[serde(rename = "ADDR")]
	CodeADDR,
	#[serde(rename = "PBOX")]
	CodePBOX,
	#[serde(rename = "HOME")]
	CodeHOME,
	#[serde(rename = "BIZZ")]
	CodeBIZZ,
	#[serde(rename = "MLTO")]
	CodeMLTO,
	#[serde(rename = "DLVY")]
	CodeDLVY,
}

impl AddressType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AlternateSecurityIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AlternateSecurityIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "DmstIdSrc", skip_serializing_if = "Option::is_none")]
	pub dmst_id_src: Option<CountryCode>,
	#[serde(rename = "PrtryIdSrc", skip_serializing_if = "Option::is_none")]
	pub prtry_id_src: Option<Max35Text>,
}

impl AlternateSecurityIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref dmst_id_src_value) = self.dmst_id_src { if let Err(e) = dmst_id_src_value.validate() { return Err(e); } }
		if let Some(ref prtry_id_src_value) = self.prtry_id_src { if let Err(e) = prtry_id_src_value.validate() { return Err(e); } }
		Ok(())
	}
}


// AnyBICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyBICIdentifier {
	#[serde(rename = "$value")]
	pub any_bic_identifier: String,
}

impl AnyBICIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.any_bic_identifier) {
			return Err(ValidationError::new(1005, "any_bic_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// BelgianIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BelgianIdentifier {
	#[serde(rename = "$value")]
	pub belgian_identifier: String,
}

impl BelgianIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// BloombergIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BloombergIdentifier {
	#[serde(rename = "$value")]
	pub bloomberg_identifier: String,
}

impl BloombergIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.bloomberg_identifier.chars().count() < 1 {
			return Err(ValidationError::new(1001, "bloomberg_identifier is shorter than the minimum length of 1".to_string()));
		}
		if self.bloomberg_identifier.chars().count() > 35 {
			return Err(ValidationError::new(1002, "bloomberg_identifier exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// CUSIPIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CUSIPIdentifier {
	#[serde(rename = "$value")]
	pub cusip_identifier: String,
}

impl CUSIPIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CalculationBasis2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CalculationBasis2Code {
	#[default]
	#[serde(rename = "AVER")]
	CodeAVER,
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "YEAR")]
	CodeYEAR,
}

impl CalculationBasis2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Charge15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Charge15 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ChargeType9Code>,
	#[serde(rename = "XtndedTp", skip_serializing_if = "Option::is_none")]
	pub xtnded_tp: Option<Extended350Code>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "ClctnBsis", skip_serializing_if = "Option::is_none")]
	pub clctn_bsis: Option<CalculationBasis2Code>,
	#[serde(rename = "XtndedClctnBsis", skip_serializing_if = "Option::is_none")]
	pub xtnded_clctn_bsis: Option<Extended350Code>,
}

impl Charge15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref xtnded_tp_value) = self.xtnded_tp { if let Err(e) = xtnded_tp_value.validate() { return Err(e); } }
		if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
		if let Some(ref clctn_bsis_value) = self.clctn_bsis { if let Err(e) = clctn_bsis_value.validate() { return Err(e); } }
		if let Some(ref xtnded_clctn_bsis_value) = self.xtnded_clctn_bsis { if let Err(e) = xtnded_clctn_bsis_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ChargeType9Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ChargeType9Code {
	#[default]
	#[serde(rename = "MANF")]
	CodeMANF,
	#[serde(rename = "BEND")]
	CodeBEND,
	#[serde(rename = "FEND")]
	CodeFEND,
	#[serde(rename = "ADVI")]
	CodeADVI,
	#[serde(rename = "CUST")]
	CodeCUST,
	#[serde(rename = "PUBL")]
	CodePUBL,
	#[serde(rename = "ACCT")]
	CodeACCT,
	#[serde(rename = "EQUL")]
	CodeEQUL,
	#[serde(rename = "PENA")]
	CodePENA,
}

impl ChargeType9Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ConsolidatedTapeAssociationIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ConsolidatedTapeAssociationIdentifier {
	#[serde(rename = "$value")]
	pub consolidated_tape_association_identifier: String,
}

impl ConsolidatedTapeAssociationIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.consolidated_tape_association_identifier.chars().count() < 1 {
			return Err(ValidationError::new(1001, "consolidated_tape_association_identifier is shorter than the minimum length of 1".to_string()));
		}
		if self.consolidated_tape_association_identifier.chars().count() > 35 {
			return Err(ValidationError::new(1002, "consolidated_tape_association_identifier exceeds the maximum length of 35".to_string()));
		}
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


// DateAndDateTime1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime1Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}

impl DateAndDateTime1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateAndDateTimeChoice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTimeChoice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}

impl DateAndDateTimeChoice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateOrDateTimePeriodChoice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateOrDateTimePeriodChoice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<DatePeriodDetails>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<DateTimePeriodDetails>,
}

impl DateOrDateTimePeriodChoice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref dt_value) = self.dt { if let Err(e) = dt_value.validate() { return Err(e); } }
		if let Some(ref dt_tm_value) = self.dt_tm { if let Err(e) = dt_tm_value.validate() { return Err(e); } }
		Ok(())
	}
}


// DatePeriodDetails ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriodDetails {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}

impl DatePeriodDetails {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateTimePeriodDetails ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriodDetails {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}

impl DateTimePeriodDetails {
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


// DistributionPolicy1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DistributionPolicy1Code {
	#[default]
	#[serde(rename = "DIST")]
	CodeDIST,
	#[serde(rename = "ACCU")]
	CodeACCU,
}

impl DistributionPolicy1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DutchIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DutchIdentifier {
	#[serde(rename = "$value")]
	pub dutch_identifier: String,
}

impl DutchIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// EUCapitalGain2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EUCapitalGain2Code {
	#[default]
	#[serde(rename = "EUSI")]
	CodeEUSI,
	#[serde(rename = "EUSO")]
	CodeEUSO,
	#[serde(rename = "UKWN")]
	CodeUKWN,
}

impl EUCapitalGain2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// EUDividendStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EUDividendStatus1Code {
	#[default]
	#[serde(rename = "DIVI")]
	CodeDIVI,
	#[serde(rename = "DIVO")]
	CodeDIVO,
	#[serde(rename = "UKWN")]
	CodeUKWN,
}

impl EUDividendStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// EuroclearClearstreamIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EuroclearClearstreamIdentifier {
	#[serde(rename = "$value")]
	pub euroclear_clearstream_identifier: String,
}

impl EuroclearClearstreamIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.euroclear_clearstream_identifier.chars().count() < 1 {
			return Err(ValidationError::new(1001, "euroclear_clearstream_identifier is shorter than the minimum length of 1".to_string()));
		}
		if self.euroclear_clearstream_identifier.chars().count() > 12 {
			return Err(ValidationError::new(1002, "euroclear_clearstream_identifier exceeds the maximum length of 12".to_string()));
		}
		Ok(())
	}
}


// EventFrequency1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EventFrequency1Code {
	#[default]
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "SEMI")]
	CodeSEMI,
	#[serde(rename = "QUTR")]
	CodeQUTR,
	#[serde(rename = "TOMN")]
	CodeTOMN,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "TWMN")]
	CodeTWMN,
	#[serde(rename = "TOWK")]
	CodeTOWK,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "ADHO")]
	CodeADHO,
	#[serde(rename = "INDA")]
	CodeINDA,
	#[serde(rename = "OVNG")]
	CodeOVNG,
	#[serde(rename = "ONDE")]
	CodeONDE,
}

impl EventFrequency1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Extended350Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Extended350Code {
	#[serde(rename = "$value")]
	pub extended350_code: String,
}

impl Extended350Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.extended350_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "extended350_code is shorter than the minimum length of 1".to_string()));
		}
		if self.extended350_code.chars().count() > 350 {
			return Err(ValidationError::new(1002, "extended350_code exceeds the maximum length of 350".to_string()));
		}
		Ok(())
	}
}


// Extension1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Extension1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Max350Text,
	#[serde(rename = "Txt")]
	pub txt: Max350Text,
}

impl Extension1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.plc_and_nm.validate() { return Err(e); }
		if let Err(e) = self.txt.validate() { return Err(e); }
		Ok(())
	}
}


// FinancialInstrument8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument8 {
	#[serde(rename = "Id")]
	pub id: Vec<SecurityIdentification3Choice>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
	#[serde(rename = "SplmtryId", skip_serializing_if = "Option::is_none")]
	pub splmtry_id: Option<Max35Text>,
	#[serde(rename = "DnmtnCcy", skip_serializing_if = "Option::is_none")]
	pub dnmtn_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "ClssTp", skip_serializing_if = "Option::is_none")]
	pub clss_tp: Option<Max35Text>,
	#[serde(rename = "SctiesForm", skip_serializing_if = "Option::is_none")]
	pub scties_form: Option<FormOfSecurity1Code>,
	#[serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none")]
	pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
	#[serde(rename = "DualFndInd")]
	pub dual_fnd_ind: bool,
}

impl FinancialInstrument8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.id { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		if let Some(ref splmtry_id_value) = self.splmtry_id { if let Err(e) = splmtry_id_value.validate() { return Err(e); } }
		if let Some(ref dnmtn_ccy_value) = self.dnmtn_ccy { if let Err(e) = dnmtn_ccy_value.validate() { return Err(e); } }
		if let Some(ref clss_tp_value) = self.clss_tp { if let Err(e) = clss_tp_value.validate() { return Err(e); } }
		if let Some(ref scties_form_value) = self.scties_form { if let Err(e) = scties_form_value.validate() { return Err(e); } }
		if let Some(ref dstrbtn_plcy_value) = self.dstrbtn_plcy { if let Err(e) = dstrbtn_plcy_value.validate() { return Err(e); } }
		Ok(())
	}
}


// FinancialInstrumentQuantity1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentQuantity1 {
	#[serde(rename = "Unit")]
	pub unit: f64,
}

impl FinancialInstrumentQuantity1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FormOfSecurity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FormOfSecurity1Code {
	#[default]
	#[serde(rename = "BEAR")]
	CodeBEAR,
	#[serde(rename = "REGD")]
	CodeREGD,
}

impl FormOfSecurity1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
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


// ISINIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISINIdentifier {
	#[serde(rename = "$value")]
	pub isin_identifier: String,
}

impl ISINIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z0-9]{12,12}").unwrap();
		if !pattern.is_match(&self.isin_identifier) {
			return Err(ValidationError::new(1005, "isin_identifier does not match the required pattern".to_string()));
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


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max5NumericText {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max70Text {
	#[serde(rename = "$value")]
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


// MessageIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
}

impl MessageIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		Ok(())
	}
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}

impl NameAndAddress5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.nm.validate() { return Err(e); }
		if let Some(ref adr_value) = self.adr { if let Err(e) = adr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}

impl Number {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Pagination ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination {
	#[serde(rename = "PgNb")]
	pub pg_nb: Max5NumericText,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}

impl Pagination {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pg_nb.validate() { return Err(e); }
		Ok(())
	}
}


// PartyIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification2Choice {
	#[serde(rename = "BICOrBEI", skip_serializing_if = "Option::is_none")]
	pub bic_or_bei: Option<AnyBICIdentifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification1>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
}

impl PartyIdentification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref bic_or_bei_value) = self.bic_or_bei { if let Err(e) = bic_or_bei_value.validate() { return Err(e); } }
		if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
		if let Some(ref nm_and_adr_value) = self.nm_and_adr { if let Err(e) = nm_and_adr_value.validate() { return Err(e); } }
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


// PerformanceFactors1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PerformanceFactors1 {
	#[serde(rename = "CorpActnFctr", skip_serializing_if = "Option::is_none")]
	pub corp_actn_fctr: Option<f64>,
	#[serde(rename = "CmltvCorpActnFctr", skip_serializing_if = "Option::is_none")]
	pub cmltv_corp_actn_fctr: Option<f64>,
	#[serde(rename = "AcmltnPrd", skip_serializing_if = "Option::is_none")]
	pub acmltn_prd: Option<DatePeriodDetails>,
	#[serde(rename = "NrmlPrfrmnc", skip_serializing_if = "Option::is_none")]
	pub nrml_prfrmnc: Option<f64>,
}

impl PerformanceFactors1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref acmltn_prd_value) = self.acmltn_prd { if let Err(e) = acmltn_prd_value.validate() { return Err(e); } }
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


// PostalAddress1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType2Code>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<Max70Text>>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max70Text>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<Max16Text>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<Max16Text>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<Max35Text>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<Max35Text>,
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
}

impl PostalAddress1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref adr_tp_value) = self.adr_tp { if let Err(e) = adr_tp_value.validate() { return Err(e); } }
		if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref strt_nm_value) = self.strt_nm { if let Err(e) = strt_nm_value.validate() { return Err(e); } }
		if let Some(ref bldg_nb_value) = self.bldg_nb { if let Err(e) = bldg_nb_value.validate() { return Err(e); } }
		if let Some(ref pst_cd_value) = self.pst_cd { if let Err(e) = pst_cd_value.validate() { return Err(e); } }
		if let Some(ref twn_nm_value) = self.twn_nm { if let Err(e) = twn_nm_value.validate() { return Err(e); } }
		if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if let Err(e) = ctry_sub_dvsn_value.validate() { return Err(e); } }
		if let Err(e) = self.ctry.validate() { return Err(e); }
		Ok(())
	}
}


// PriceMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PriceMethod1Code {
	#[default]
	#[serde(rename = "FORW")]
	CodeFORW,
	#[serde(rename = "HIST")]
	CodeHIST,
}

impl PriceMethod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PriceReport3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceReport3 {
	#[serde(rename = "PricValtnDtls")]
	pub pric_valtn_dtls: Vec<PriceValuation4>,
}

impl PriceReport3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.pric_valtn_dtls { if let Err(e) = item.validate() { return Err(e); } }
		Ok(())
	}
}


// PriceReportCancellationV04 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceReportCancellationV04 {
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "PoolRef", skip_serializing_if = "Option::is_none")]
	pub pool_ref: Option<AdditionalReference3>,
	#[serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none")]
	pub prvs_ref: Option<AdditionalReference3>,
	#[serde(rename = "MsgPgntn")]
	pub msg_pgntn: Pagination,
	#[serde(rename = "PricRptId")]
	pub pric_rpt_id: Max35Text,
	#[serde(rename = "CxlId")]
	pub cxl_id: Max35Text,
	#[serde(rename = "CxlRsn", skip_serializing_if = "Option::is_none")]
	pub cxl_rsn: Option<Max350Text>,
	#[serde(rename = "XpctdPricCrrctnDt", skip_serializing_if = "Option::is_none")]
	pub xpctd_pric_crrctn_dt: Option<DateAndDateTime1Choice>,
	#[serde(rename = "CmpltPricCxl")]
	pub cmplt_pric_cxl: bool,
	#[serde(rename = "CancPricValtnDtls", skip_serializing_if = "Option::is_none")]
	pub canc_pric_valtn_dtls: Option<Vec<PriceReport3>>,
	#[serde(rename = "Xtnsn", skip_serializing_if = "Option::is_none")]
	pub xtnsn: Option<Vec<Extension1>>,
}

impl PriceReportCancellationV04 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_id.validate() { return Err(e); }
		if let Some(ref pool_ref_value) = self.pool_ref { if let Err(e) = pool_ref_value.validate() { return Err(e); } }
		if let Some(ref prvs_ref_value) = self.prvs_ref { if let Err(e) = prvs_ref_value.validate() { return Err(e); } }
		if let Err(e) = self.msg_pgntn.validate() { return Err(e); }
		if let Err(e) = self.pric_rpt_id.validate() { return Err(e); }
		if let Err(e) = self.cxl_id.validate() { return Err(e); }
		if let Some(ref cxl_rsn_value) = self.cxl_rsn { if let Err(e) = cxl_rsn_value.validate() { return Err(e); } }
		if let Some(ref xpctd_pric_crrctn_dt_value) = self.xpctd_pric_crrctn_dt { if let Err(e) = xpctd_pric_crrctn_dt_value.validate() { return Err(e); } }
		if let Some(ref canc_pric_valtn_dtls_vec) = self.canc_pric_valtn_dtls { for item in canc_pric_valtn_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref xtnsn_vec) = self.xtnsn { for item in xtnsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// PriceType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceType2 {
	#[serde(rename = "Strd")]
	pub strd: TypeOfPrice6Code,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}

impl PriceType2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.strd.validate() { return Err(e); }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PriceValuation4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceValuation4 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "ValtnDtTm", skip_serializing_if = "Option::is_none")]
	pub valtn_dt_tm: Option<DateAndDateTimeChoice>,
	#[serde(rename = "NAVDtTm")]
	pub nav_dt_tm: DateAndDateTimeChoice,
	#[serde(rename = "FinInstrmDtls")]
	pub fin_instrm_dtls: FinancialInstrument8,
	#[serde(rename = "FndMgmtCpny", skip_serializing_if = "Option::is_none")]
	pub fnd_mgmt_cpny: Option<PartyIdentification2Choice>,
	#[serde(rename = "TtlNAV", skip_serializing_if = "Option::is_none")]
	pub ttl_nav: Option<Vec<ActiveOrHistoricCurrencyAndAmount>>,
	#[serde(rename = "TtlUnitsNb", skip_serializing_if = "Option::is_none")]
	pub ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "NxtValtnDtTm", skip_serializing_if = "Option::is_none")]
	pub nxt_valtn_dt_tm: Option<DateAndDateTimeChoice>,
	#[serde(rename = "PrvsValtnDtTm", skip_serializing_if = "Option::is_none")]
	pub prvs_valtn_dt_tm: Option<DateAndDateTimeChoice>,
	#[serde(rename = "ValtnTp")]
	pub valtn_tp: ValuationTiming1Code,
	#[serde(rename = "ValtnFrqcy", skip_serializing_if = "Option::is_none")]
	pub valtn_frqcy: Option<EventFrequency1Code>,
	#[serde(rename = "OffclValtnInd")]
	pub offcl_valtn_ind: bool,
	#[serde(rename = "SspdInd")]
	pub sspd_ind: bool,
	#[serde(rename = "PricDtls", skip_serializing_if = "Option::is_none")]
	pub pric_dtls: Option<Vec<UnitPrice15>>,
	#[serde(rename = "ValtnSttstcs", skip_serializing_if = "Option::is_none")]
	pub valtn_sttstcs: Option<Vec<ValuationStatistics3>>,
	#[serde(rename = "PrfrmncDtls", skip_serializing_if = "Option::is_none")]
	pub prfrmnc_dtls: Option<PerformanceFactors1>,
}

impl PriceValuation4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		if let Some(ref valtn_dt_tm_value) = self.valtn_dt_tm { if let Err(e) = valtn_dt_tm_value.validate() { return Err(e); } }
		if let Err(e) = self.nav_dt_tm.validate() { return Err(e); }
		if let Err(e) = self.fin_instrm_dtls.validate() { return Err(e); }
		if let Some(ref fnd_mgmt_cpny_value) = self.fnd_mgmt_cpny { if let Err(e) = fnd_mgmt_cpny_value.validate() { return Err(e); } }
		if let Some(ref ttl_nav_vec) = self.ttl_nav { for item in ttl_nav_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref ttl_units_nb_value) = self.ttl_units_nb { if let Err(e) = ttl_units_nb_value.validate() { return Err(e); } }
		if let Some(ref nxt_valtn_dt_tm_value) = self.nxt_valtn_dt_tm { if let Err(e) = nxt_valtn_dt_tm_value.validate() { return Err(e); } }
		if let Some(ref prvs_valtn_dt_tm_value) = self.prvs_valtn_dt_tm { if let Err(e) = prvs_valtn_dt_tm_value.validate() { return Err(e); } }
		if let Err(e) = self.valtn_tp.validate() { return Err(e); }
		if let Some(ref valtn_frqcy_value) = self.valtn_frqcy { if let Err(e) = valtn_frqcy_value.validate() { return Err(e); } }
		if let Some(ref pric_dtls_vec) = self.pric_dtls { for item in pric_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref valtn_sttstcs_vec) = self.valtn_sttstcs { for item in valtn_sttstcs_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref prfrmnc_dtls_value) = self.prfrmnc_dtls { if let Err(e) = prfrmnc_dtls_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PriceValue1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceValue1 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
}

impl PriceValue1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		Ok(())
	}
}


// PriceValue5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceValue5 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAnd13DecimalAmount,
}

impl PriceValue5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		Ok(())
	}
}


// PriceValueChange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceValueChange1 {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
	#[serde(rename = "AmtSgn", skip_serializing_if = "Option::is_none")]
	pub amt_sgn: Option<bool>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
}

impl PriceValueChange1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// QUICKIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct QUICKIdentifier {
	#[serde(rename = "$value")]
	pub quick_identifier: String,
}

impl QUICKIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RICIdentifier {
	#[serde(rename = "$value")]
	pub ric_identifier: String,
}

impl RICIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.ric_identifier.chars().count() < 1 {
			return Err(ValidationError::new(1001, "ric_identifier is shorter than the minimum length of 1".to_string()));
		}
		if self.ric_identifier.chars().count() > 35 {
			return Err(ValidationError::new(1002, "ric_identifier exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// SEDOLIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SEDOLIdentifier {
	#[serde(rename = "$value")]
	pub sedol_identifier: String,
}

impl SEDOLIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecurityIdentification3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification3Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINIdentifier>,
	#[serde(rename = "SEDOL", skip_serializing_if = "Option::is_none")]
	pub sedol: Option<String>,
	#[serde(rename = "CUSIP", skip_serializing_if = "Option::is_none")]
	pub cusip: Option<String>,
	#[serde(rename = "RIC", skip_serializing_if = "Option::is_none")]
	pub ric: Option<RICIdentifier>,
	#[serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none")]
	pub tckr_symb: Option<TickerIdentifier>,
	#[serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none")]
	pub blmbrg: Option<BloombergIdentifier>,
	#[serde(rename = "CTA", skip_serializing_if = "Option::is_none")]
	pub cta: Option<ConsolidatedTapeAssociationIdentifier>,
	#[serde(rename = "QUICK", skip_serializing_if = "Option::is_none")]
	pub quick: Option<String>,
	#[serde(rename = "Wrtppr", skip_serializing_if = "Option::is_none")]
	pub wrtppr: Option<String>,
	#[serde(rename = "Dtch", skip_serializing_if = "Option::is_none")]
	pub dtch: Option<String>,
	#[serde(rename = "Vlrn", skip_serializing_if = "Option::is_none")]
	pub vlrn: Option<String>,
	#[serde(rename = "SCVM", skip_serializing_if = "Option::is_none")]
	pub scvm: Option<String>,
	#[serde(rename = "Belgn", skip_serializing_if = "Option::is_none")]
	pub belgn: Option<String>,
	#[serde(rename = "Cmon", skip_serializing_if = "Option::is_none")]
	pub cmon: Option<EuroclearClearstreamIdentifier>,
	#[serde(rename = "OthrPrtryId", skip_serializing_if = "Option::is_none")]
	pub othr_prtry_id: Option<AlternateSecurityIdentification1>,
}

impl SecurityIdentification3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
		if let Some(ref ric_value) = self.ric { if let Err(e) = ric_value.validate() { return Err(e); } }
		if let Some(ref tckr_symb_value) = self.tckr_symb { if let Err(e) = tckr_symb_value.validate() { return Err(e); } }
		if let Some(ref blmbrg_value) = self.blmbrg { if let Err(e) = blmbrg_value.validate() { return Err(e); } }
		if let Some(ref cta_value) = self.cta { if let Err(e) = cta_value.validate() { return Err(e); } }
		if let Some(ref cmon_value) = self.cmon { if let Err(e) = cmon_value.validate() { return Err(e); } }
		if let Some(ref othr_prtry_id_value) = self.othr_prtry_id { if let Err(e) = othr_prtry_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SicovamIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SicovamIdentifier {
	#[serde(rename = "$value")]
	pub sicovam_identifier: String,
}

impl SicovamIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// StatisticsByPredefinedTimePeriods2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatisticsByPredefinedTimePeriods2 {
	#[serde(rename = "HghstPricVal12Mnths", skip_serializing_if = "Option::is_none")]
	pub hghst_pric_val12_mnths: Option<PriceValue5>,
	#[serde(rename = "LwstPricVal12Mnths", skip_serializing_if = "Option::is_none")]
	pub lwst_pric_val12_mnths: Option<PriceValue5>,
	#[serde(rename = "OneYrPricChng", skip_serializing_if = "Option::is_none")]
	pub one_yr_pric_chng: Option<PriceValueChange1>,
	#[serde(rename = "ThreeYrPricChng", skip_serializing_if = "Option::is_none")]
	pub three_yr_pric_chng: Option<PriceValueChange1>,
	#[serde(rename = "FiveYrPricChng", skip_serializing_if = "Option::is_none")]
	pub five_yr_pric_chng: Option<PriceValueChange1>,
}

impl StatisticsByPredefinedTimePeriods2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref hghst_pric_val12_mnths_value) = self.hghst_pric_val12_mnths { if let Err(e) = hghst_pric_val12_mnths_value.validate() { return Err(e); } }
		if let Some(ref lwst_pric_val12_mnths_value) = self.lwst_pric_val12_mnths { if let Err(e) = lwst_pric_val12_mnths_value.validate() { return Err(e); } }
		if let Some(ref one_yr_pric_chng_value) = self.one_yr_pric_chng { if let Err(e) = one_yr_pric_chng_value.validate() { return Err(e); } }
		if let Some(ref three_yr_pric_chng_value) = self.three_yr_pric_chng { if let Err(e) = three_yr_pric_chng_value.validate() { return Err(e); } }
		if let Some(ref five_yr_pric_chng_value) = self.five_yr_pric_chng { if let Err(e) = five_yr_pric_chng_value.validate() { return Err(e); } }
		Ok(())
	}
}


// StatisticsByUserDefinedTimePeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatisticsByUserDefinedTimePeriod2 {
	#[serde(rename = "Prd")]
	pub prd: DateOrDateTimePeriodChoice,
	#[serde(rename = "HghstPricVal", skip_serializing_if = "Option::is_none")]
	pub hghst_pric_val: Option<PriceValue5>,
	#[serde(rename = "LwstPricVal", skip_serializing_if = "Option::is_none")]
	pub lwst_pric_val: Option<PriceValue5>,
	#[serde(rename = "PricChng", skip_serializing_if = "Option::is_none")]
	pub pric_chng: Option<PriceValueChange1>,
	#[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
	pub yld: Option<f64>,
}

impl StatisticsByUserDefinedTimePeriod2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.prd.validate() { return Err(e); }
		if let Some(ref hghst_pric_val_value) = self.hghst_pric_val { if let Err(e) = hghst_pric_val_value.validate() { return Err(e); } }
		if let Some(ref lwst_pric_val_value) = self.lwst_pric_val { if let Err(e) = lwst_pric_val_value.validate() { return Err(e); } }
		if let Some(ref pric_chng_value) = self.pric_chng { if let Err(e) = pric_chng_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Tax17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Tax17 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<TaxType12Code>,
	#[serde(rename = "XtndedTp", skip_serializing_if = "Option::is_none")]
	pub xtnded_tp: Option<Extended350Code>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<Vec<ActiveOrHistoricCurrencyAnd13DecimalAmount>>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
	#[serde(rename = "TaxClctnDtls", skip_serializing_if = "Option::is_none")]
	pub tax_clctn_dtls: Option<TaxCalculationInformation4>,
}

impl Tax17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref xtnded_tp_value) = self.xtnded_tp { if let Err(e) = xtnded_tp_value.validate() { return Err(e); } }
		if let Some(ref amt_vec) = self.amt { for item in amt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Err(e) = self.ctry.validate() { return Err(e); }
		if let Some(ref tax_clctn_dtls_value) = self.tax_clctn_dtls { if let Err(e) = tax_clctn_dtls_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TaxCalculationInformation4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxCalculationInformation4 {
	#[serde(rename = "EUCptlGn", skip_serializing_if = "Option::is_none")]
	pub eu_cptl_gn: Option<EUCapitalGain2Code>,
	#[serde(rename = "XtndedEUCptlGn", skip_serializing_if = "Option::is_none")]
	pub xtnded_eu_cptl_gn: Option<Extended350Code>,
	#[serde(rename = "PctgOfDebtClm", skip_serializing_if = "Option::is_none")]
	pub pctg_of_debt_clm: Option<f64>,
	#[serde(rename = "PctgGrdfthdDebt", skip_serializing_if = "Option::is_none")]
	pub pctg_grdfthd_debt: Option<f64>,
	#[serde(rename = "TaxblIncmPerDvdd", skip_serializing_if = "Option::is_none")]
	pub taxbl_incm_per_dvdd: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
	#[serde(rename = "EUDvddSts", skip_serializing_if = "Option::is_none")]
	pub eu_dvdd_sts: Option<EUDividendStatus1Code>,
	#[serde(rename = "XtndedEUDvddSts", skip_serializing_if = "Option::is_none")]
	pub xtnded_eu_dvdd_sts: Option<Extended350Code>,
}

impl TaxCalculationInformation4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref eu_cptl_gn_value) = self.eu_cptl_gn { if let Err(e) = eu_cptl_gn_value.validate() { return Err(e); } }
		if let Some(ref xtnded_eu_cptl_gn_value) = self.xtnded_eu_cptl_gn { if let Err(e) = xtnded_eu_cptl_gn_value.validate() { return Err(e); } }
		if let Some(ref taxbl_incm_per_dvdd_value) = self.taxbl_incm_per_dvdd { if let Err(e) = taxbl_incm_per_dvdd_value.validate() { return Err(e); } }
		if let Some(ref eu_dvdd_sts_value) = self.eu_dvdd_sts { if let Err(e) = eu_dvdd_sts_value.validate() { return Err(e); } }
		if let Some(ref xtnded_eu_dvdd_sts_value) = self.xtnded_eu_dvdd_sts { if let Err(e) = xtnded_eu_dvdd_sts_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TaxType12Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TaxType12Code {
	#[default]
	#[serde(rename = "INPO")]
	CodeINPO,
	#[serde(rename = "EUTR")]
	CodeEUTR,
	#[serde(rename = "AKT1")]
	CodeAKT1,
	#[serde(rename = "AKT2")]
	CodeAKT2,
	#[serde(rename = "ZWIS")]
	CodeZWIS,
	#[serde(rename = "MIET")]
	CodeMIET,
}

impl TaxType12Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TaxableIncomePerShareCalculated2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TaxableIncomePerShareCalculated2Code {
	#[default]
	#[serde(rename = "TSIY")]
	CodeTSIY,
	#[serde(rename = "TSIN")]
	CodeTSIN,
	#[serde(rename = "UKWN")]
	CodeUKWN,
}

impl TaxableIncomePerShareCalculated2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TickerIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TickerIdentifier {
	#[serde(rename = "$value")]
	pub ticker_identifier: String,
}

impl TickerIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.ticker_identifier.chars().count() < 1 {
			return Err(ValidationError::new(1001, "ticker_identifier is shorter than the minimum length of 1".to_string()));
		}
		if self.ticker_identifier.chars().count() > 35 {
			return Err(ValidationError::new(1002, "ticker_identifier exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// TypeOfPrice6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypeOfPrice6Code {
	#[default]
	#[serde(rename = "BIDE")]
	CodeBIDE,
	#[serde(rename = "OFFR")]
	CodeOFFR,
	#[serde(rename = "NAVL")]
	CodeNAVL,
	#[serde(rename = "CREA")]
	CodeCREA,
	#[serde(rename = "CANC")]
	CodeCANC,
	#[serde(rename = "INTE")]
	CodeINTE,
	#[serde(rename = "SWNG")]
	CodeSWNG,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "MIDD")]
	CodeMIDD,
	#[serde(rename = "RINV")]
	CodeRINV,
	#[serde(rename = "SWIC")]
	CodeSWIC,
	#[serde(rename = "DDVR")]
	CodeDDVR,
	#[serde(rename = "ACTU")]
	CodeACTU,
	#[serde(rename = "NAUP")]
	CodeNAUP,
}

impl TypeOfPrice6Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TypeOfPrice9Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypeOfPrice9Code {
	#[default]
	#[serde(rename = "BIDE")]
	CodeBIDE,
	#[serde(rename = "OFFR")]
	CodeOFFR,
	#[serde(rename = "NAVL")]
	CodeNAVL,
	#[serde(rename = "CREA")]
	CodeCREA,
	#[serde(rename = "CANC")]
	CodeCANC,
	#[serde(rename = "INTE")]
	CodeINTE,
	#[serde(rename = "SWNG")]
	CodeSWNG,
	#[serde(rename = "MIDD")]
	CodeMIDD,
	#[serde(rename = "RINV")]
	CodeRINV,
	#[serde(rename = "SWIC")]
	CodeSWIC,
	#[serde(rename = "DDVR")]
	CodeDDVR,
	#[serde(rename = "ACTU")]
	CodeACTU,
	#[serde(rename = "NAUP")]
	CodeNAUP,
	#[serde(rename = "GUAR")]
	CodeGUAR,
	#[serde(rename = "ENAV")]
	CodeENAV,
}

impl TypeOfPrice9Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UnitPrice15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitPrice15 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<TypeOfPrice9Code>,
	#[serde(rename = "XtndedTp", skip_serializing_if = "Option::is_none")]
	pub xtnded_tp: Option<Extended350Code>,
	#[serde(rename = "PricMtd", skip_serializing_if = "Option::is_none")]
	pub pric_mtd: Option<PriceMethod1Code>,
	#[serde(rename = "ValInInvstmtCcy")]
	pub val_in_invstmt_ccy: Vec<PriceValue1>,
	#[serde(rename = "ValInAltrntvCcy", skip_serializing_if = "Option::is_none")]
	pub val_in_altrntv_ccy: Option<Vec<PriceValue1>>,
	#[serde(rename = "ForExctnInd")]
	pub for_exctn_ind: bool,
	#[serde(rename = "CumDvddInd")]
	pub cum_dvdd_ind: bool,
	#[serde(rename = "ClctnBsis", skip_serializing_if = "Option::is_none")]
	pub clctn_bsis: Option<f64>,
	#[serde(rename = "EstmtdPricInd")]
	pub estmtd_pric_ind: bool,
	#[serde(rename = "NbOfDaysAcrd", skip_serializing_if = "Option::is_none")]
	pub nb_of_days_acrd: Option<f64>,
	#[serde(rename = "TaxblIncmPerShr", skip_serializing_if = "Option::is_none")]
	pub taxbl_incm_per_shr: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
	#[serde(rename = "TaxblIncmPerShrClctd", skip_serializing_if = "Option::is_none")]
	pub taxbl_incm_per_shr_clctd: Option<TaxableIncomePerShareCalculated2Code>,
	#[serde(rename = "XtndedTaxblIncmPerShrClctd", skip_serializing_if = "Option::is_none")]
	pub xtnded_taxbl_incm_per_shr_clctd: Option<Extended350Code>,
	#[serde(rename = "TaxblIncmPerDvdd", skip_serializing_if = "Option::is_none")]
	pub taxbl_incm_per_dvdd: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
	#[serde(rename = "EUDvddSts", skip_serializing_if = "Option::is_none")]
	pub eu_dvdd_sts: Option<EUDividendStatus1Code>,
	#[serde(rename = "XtndedEUDvddSts", skip_serializing_if = "Option::is_none")]
	pub xtnded_eu_dvdd_sts: Option<Extended350Code>,
	#[serde(rename = "ChrgDtls", skip_serializing_if = "Option::is_none")]
	pub chrg_dtls: Option<Vec<Charge15>>,
	#[serde(rename = "TaxLbltyDtls", skip_serializing_if = "Option::is_none")]
	pub tax_lblty_dtls: Option<Vec<Tax17>>,
	#[serde(rename = "TaxRfndDtls", skip_serializing_if = "Option::is_none")]
	pub tax_rfnd_dtls: Option<Vec<Tax17>>,
}

impl UnitPrice15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref xtnded_tp_value) = self.xtnded_tp { if let Err(e) = xtnded_tp_value.validate() { return Err(e); } }
		if let Some(ref pric_mtd_value) = self.pric_mtd { if let Err(e) = pric_mtd_value.validate() { return Err(e); } }
		for item in &self.val_in_invstmt_ccy { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref val_in_altrntv_ccy_vec) = self.val_in_altrntv_ccy { for item in val_in_altrntv_ccy_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref taxbl_incm_per_shr_value) = self.taxbl_incm_per_shr { if let Err(e) = taxbl_incm_per_shr_value.validate() { return Err(e); } }
		if let Some(ref taxbl_incm_per_shr_clctd_value) = self.taxbl_incm_per_shr_clctd { if let Err(e) = taxbl_incm_per_shr_clctd_value.validate() { return Err(e); } }
		if let Some(ref xtnded_taxbl_incm_per_shr_clctd_value) = self.xtnded_taxbl_incm_per_shr_clctd { if let Err(e) = xtnded_taxbl_incm_per_shr_clctd_value.validate() { return Err(e); } }
		if let Some(ref taxbl_incm_per_dvdd_value) = self.taxbl_incm_per_dvdd { if let Err(e) = taxbl_incm_per_dvdd_value.validate() { return Err(e); } }
		if let Some(ref eu_dvdd_sts_value) = self.eu_dvdd_sts { if let Err(e) = eu_dvdd_sts_value.validate() { return Err(e); } }
		if let Some(ref xtnded_eu_dvdd_sts_value) = self.xtnded_eu_dvdd_sts { if let Err(e) = xtnded_eu_dvdd_sts_value.validate() { return Err(e); } }
		if let Some(ref chrg_dtls_vec) = self.chrg_dtls { for item in chrg_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref tax_lblty_dtls_vec) = self.tax_lblty_dtls { for item in tax_lblty_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref tax_rfnd_dtls_vec) = self.tax_rfnd_dtls { for item in tax_rfnd_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// ValorenIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ValorenIdentifier {
	#[serde(rename = "$value")]
	pub valoren_identifier: String,
}

impl ValorenIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ValuationStatistics3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValuationStatistics3 {
	#[serde(rename = "Ccy")]
	pub ccy: ActiveOrHistoricCurrencyCode,
	#[serde(rename = "PricTpChngBsis")]
	pub pric_tp_chng_bsis: PriceType2,
	#[serde(rename = "PricChng")]
	pub pric_chng: PriceValueChange1,
	#[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
	pub yld: Option<f64>,
	#[serde(rename = "ByPrdfndTmPrds", skip_serializing_if = "Option::is_none")]
	pub by_prdfnd_tm_prds: Option<StatisticsByPredefinedTimePeriods2>,
	#[serde(rename = "ByUsrDfndTmPrd", skip_serializing_if = "Option::is_none")]
	pub by_usr_dfnd_tm_prd: Option<Vec<StatisticsByUserDefinedTimePeriod2>>,
}

impl ValuationStatistics3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ccy.validate() { return Err(e); }
		if let Err(e) = self.pric_tp_chng_bsis.validate() { return Err(e); }
		if let Err(e) = self.pric_chng.validate() { return Err(e); }
		if let Some(ref by_prdfnd_tm_prds_value) = self.by_prdfnd_tm_prds { if let Err(e) = by_prdfnd_tm_prds_value.validate() { return Err(e); } }
		if let Some(ref by_usr_dfnd_tm_prd_vec) = self.by_usr_dfnd_tm_prd { for item in by_usr_dfnd_tm_prd_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// ValuationTiming1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ValuationTiming1Code {
	#[default]
	#[serde(rename = "EXCP")]
	CodeEXCP,
	#[serde(rename = "USUA")]
	CodeUSUA,
	#[serde(rename = "PATC")]
	CodePATC,
}

impl ValuationTiming1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// WertpapierIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WertpapierIdentifier {
	#[serde(rename = "$value")]
	pub wertpapier_identifier: String,
}

impl WertpapierIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}

impl YesNoIndicator {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
