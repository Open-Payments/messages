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


// ActiveCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and13_decimal_amount_simple_type: f64,
}

impl ActiveCurrencyAnd13DecimalAmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.active_currency_and13_decimal_amount_simple_type < 0.000000 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_currency_code) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.active_or_historic_currency_and13_decimal_amount_simple_type < 0.000000 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if self.active_or_historic_currency_and_amount_simple_type < 0.000000 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_or_historic_currency_code) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if !self.ref_attr.validate() { return false }
		if let Some(ref ref_issr_value) = self.ref_issr { if !ref_issr_value.validate() { return false; } }
		if let Some(ref msg_nm_value) = self.msg_nm { if !msg_nm_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref dmst_id_src_value) = self.dmst_id_src { if !dmst_id_src_value.validate() { return false; } }
		if let Some(ref prtry_id_src_value) = self.prtry_id_src { if !prtry_id_src_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.any_bic_identifier) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if self.bloomberg_identifier.chars().count() < 1 {
			return false
		}
		if self.bloomberg_identifier.chars().count() > 35 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref xtnded_tp_value) = self.xtnded_tp { if !xtnded_tp_value.validate() { return false; } }
		if let Some(ref amt_value) = self.amt { if !amt_value.validate() { return false; } }
		if let Some(ref clctn_bsis_value) = self.clctn_bsis { if !clctn_bsis_value.validate() { return false; } }
		if let Some(ref xtnded_clctn_bsis_value) = self.xtnded_clctn_bsis { if !xtnded_clctn_bsis_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if self.consolidated_tape_association_identifier.chars().count() < 1 {
			return false
		}
		if self.consolidated_tape_association_identifier.chars().count() > 35 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.country_code) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref dt_value) = self.dt { if !dt_value.validate() { return false; } }
		if let Some(ref dt_tm_value) = self.dt_tm { if !dt_tm_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if self.euroclear_clearstream_identifier.chars().count() < 1 {
			return false
		}
		if self.euroclear_clearstream_identifier.chars().count() > 12 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if self.extended350_code.chars().count() < 1 {
			return false
		}
		if self.extended350_code.chars().count() > 350 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if !self.plc_and_nm.validate() { return false }
		if !self.txt.validate() { return false }
		return true
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
	pub fn validate(&self) -> bool {
		for item in &self.id { if !item.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref splmtry_id_value) = self.splmtry_id { if !splmtry_id_value.validate() { return false; } }
		if let Some(ref dnmtn_ccy_value) = self.dnmtn_ccy { if !dnmtn_ccy_value.validate() { return false; } }
		if let Some(ref clss_tp_value) = self.clss_tp { if !clss_tp_value.validate() { return false; } }
		if let Some(ref scties_form_value) = self.scties_form { if !scties_form_value.validate() { return false; } }
		if let Some(ref dstrbtn_plcy_value) = self.dstrbtn_plcy { if !dstrbtn_plcy_value.validate() { return false; } }
		return true
	}
}


// FinancialInstrumentQuantity1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentQuantity1 {
	#[serde(rename = "Unit")]
	pub unit: f64,
}

impl FinancialInstrumentQuantity1 {
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{12,12}").unwrap();
		if !pattern.is_match(&self.isin_identifier) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if self.max16_text.chars().count() < 1 {
			return false
		}
		if self.max16_text.chars().count() > 16 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max350_text.chars().count() < 1 {
			return false
		}
		if self.max350_text.chars().count() > 350 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max35_text.chars().count() < 1 {
			return false
		}
		if self.max35_text.chars().count() > 35 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[0-9]{1,5}").unwrap();
		if !pattern.is_match(&self.max5_numeric_text) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max70_text.chars().count() < 1 {
			return false
		}
		if self.max70_text.chars().count() > 70 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		return true
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
	pub fn validate(&self) -> bool {
		if !self.nm.validate() { return false }
		if let Some(ref adr_value) = self.adr { if !adr_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if !self.pg_nb.validate() { return false }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref bic_or_bei_value) = self.bic_or_bei { if !bic_or_bei_value.validate() { return false; } }
		if let Some(ref prtry_id_value) = self.prtry_id { if !prtry_id_value.validate() { return false; } }
		if let Some(ref nm_and_adr_value) = self.nm_and_adr { if !nm_and_adr_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref acmltn_prd_value) = self.acmltn_prd { if !acmltn_prd_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref adr_tp_value) = self.adr_tp { if !adr_tp_value.validate() { return false; } }
		if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if !item.validate() { return false; } } }
		if let Some(ref strt_nm_value) = self.strt_nm { if !strt_nm_value.validate() { return false; } }
		if let Some(ref bldg_nb_value) = self.bldg_nb { if !bldg_nb_value.validate() { return false; } }
		if let Some(ref pst_cd_value) = self.pst_cd { if !pst_cd_value.validate() { return false; } }
		if let Some(ref twn_nm_value) = self.twn_nm { if !twn_nm_value.validate() { return false; } }
		if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if !ctry_sub_dvsn_value.validate() { return false; } }
		if !self.ctry.validate() { return false }
		return true
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
	pub fn validate(&self) -> bool {
		return true
	}
}


// PriceReportFunction1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PriceReportFunction1Code {
	#[default]
	#[serde(rename = "REPL")]
	CodeREPL,
	#[serde(rename = "NEWP")]
	CodeNEWP,
	#[serde(rename = "PART")]
	CodePART,
}

impl PriceReportFunction1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PriceReportV04 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceReportV04 {
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "PoolRef", skip_serializing_if = "Option::is_none")]
	pub pool_ref: Option<AdditionalReference3>,
	#[serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none")]
	pub prvs_ref: Option<Vec<AdditionalReference3>>,
	#[serde(rename = "RltdRef", skip_serializing_if = "Option::is_none")]
	pub rltd_ref: Option<AdditionalReference3>,
	#[serde(rename = "MsgPgntn")]
	pub msg_pgntn: Pagination,
	#[serde(rename = "PricRptId")]
	pub pric_rpt_id: Max35Text,
	#[serde(rename = "Fctn")]
	pub fctn: PriceReportFunction1Code,
	#[serde(rename = "CxlId", skip_serializing_if = "Option::is_none")]
	pub cxl_id: Option<Max35Text>,
	#[serde(rename = "PricValtnDtls")]
	pub pric_valtn_dtls: Vec<PriceValuation4>,
	#[serde(rename = "Xtnsn", skip_serializing_if = "Option::is_none")]
	pub xtnsn: Option<Vec<Extension1>>,
}

impl PriceReportV04 {
	pub fn validate(&self) -> bool {
		if !self.msg_id.validate() { return false }
		if let Some(ref pool_ref_value) = self.pool_ref { if !pool_ref_value.validate() { return false; } }
		if let Some(ref prvs_ref_vec) = self.prvs_ref { for item in prvs_ref_vec { if !item.validate() { return false; } } }
		if let Some(ref rltd_ref_value) = self.rltd_ref { if !rltd_ref_value.validate() { return false; } }
		if !self.msg_pgntn.validate() { return false }
		if !self.pric_rpt_id.validate() { return false }
		if !self.fctn.validate() { return false }
		if let Some(ref cxl_id_value) = self.cxl_id { if !cxl_id_value.validate() { return false; } }
		for item in &self.pric_valtn_dtls { if !item.validate() { return false; } }
		if let Some(ref xtnsn_vec) = self.xtnsn { for item in xtnsn_vec { if !item.validate() { return false; } } }
		return true
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
	pub fn validate(&self) -> bool {
		if !self.strd.validate() { return false }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if !addtl_inf_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref valtn_dt_tm_value) = self.valtn_dt_tm { if !valtn_dt_tm_value.validate() { return false; } }
		if !self.nav_dt_tm.validate() { return false }
		if !self.fin_instrm_dtls.validate() { return false }
		if let Some(ref fnd_mgmt_cpny_value) = self.fnd_mgmt_cpny { if !fnd_mgmt_cpny_value.validate() { return false; } }
		if let Some(ref ttl_nav_vec) = self.ttl_nav { for item in ttl_nav_vec { if !item.validate() { return false; } } }
		if let Some(ref ttl_units_nb_value) = self.ttl_units_nb { if !ttl_units_nb_value.validate() { return false; } }
		if let Some(ref nxt_valtn_dt_tm_value) = self.nxt_valtn_dt_tm { if !nxt_valtn_dt_tm_value.validate() { return false; } }
		if let Some(ref prvs_valtn_dt_tm_value) = self.prvs_valtn_dt_tm { if !prvs_valtn_dt_tm_value.validate() { return false; } }
		if !self.valtn_tp.validate() { return false }
		if let Some(ref valtn_frqcy_value) = self.valtn_frqcy { if !valtn_frqcy_value.validate() { return false; } }
		if let Some(ref pric_dtls_vec) = self.pric_dtls { for item in pric_dtls_vec { if !item.validate() { return false; } } }
		if let Some(ref valtn_sttstcs_vec) = self.valtn_sttstcs { for item in valtn_sttstcs_vec { if !item.validate() { return false; } } }
		if let Some(ref prfrmnc_dtls_value) = self.prfrmnc_dtls { if !prfrmnc_dtls_value.validate() { return false; } }
		return true
	}
}


// PriceValue1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceValue1 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
}

impl PriceValue1 {
	pub fn validate(&self) -> bool {
		if !self.amt.validate() { return false }
		return true
	}
}


// PriceValue5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceValue5 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAnd13DecimalAmount,
}

impl PriceValue5 {
	pub fn validate(&self) -> bool {
		if !self.amt.validate() { return false }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref amt_value) = self.amt { if !amt_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if self.ric_identifier.chars().count() < 1 {
			return false
		}
		if self.ric_identifier.chars().count() > 35 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref isin_value) = self.isin { if !isin_value.validate() { return false; } }
		if let Some(ref ric_value) = self.ric { if !ric_value.validate() { return false; } }
		if let Some(ref tckr_symb_value) = self.tckr_symb { if !tckr_symb_value.validate() { return false; } }
		if let Some(ref blmbrg_value) = self.blmbrg { if !blmbrg_value.validate() { return false; } }
		if let Some(ref cta_value) = self.cta { if !cta_value.validate() { return false; } }
		if let Some(ref cmon_value) = self.cmon { if !cmon_value.validate() { return false; } }
		if let Some(ref othr_prtry_id_value) = self.othr_prtry_id { if !othr_prtry_id_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref hghst_pric_val12_mnths_value) = self.hghst_pric_val12_mnths { if !hghst_pric_val12_mnths_value.validate() { return false; } }
		if let Some(ref lwst_pric_val12_mnths_value) = self.lwst_pric_val12_mnths { if !lwst_pric_val12_mnths_value.validate() { return false; } }
		if let Some(ref one_yr_pric_chng_value) = self.one_yr_pric_chng { if !one_yr_pric_chng_value.validate() { return false; } }
		if let Some(ref three_yr_pric_chng_value) = self.three_yr_pric_chng { if !three_yr_pric_chng_value.validate() { return false; } }
		if let Some(ref five_yr_pric_chng_value) = self.five_yr_pric_chng { if !five_yr_pric_chng_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if !self.prd.validate() { return false }
		if let Some(ref hghst_pric_val_value) = self.hghst_pric_val { if !hghst_pric_val_value.validate() { return false; } }
		if let Some(ref lwst_pric_val_value) = self.lwst_pric_val { if !lwst_pric_val_value.validate() { return false; } }
		if let Some(ref pric_chng_value) = self.pric_chng { if !pric_chng_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref xtnded_tp_value) = self.xtnded_tp { if !xtnded_tp_value.validate() { return false; } }
		if let Some(ref amt_vec) = self.amt { for item in amt_vec { if !item.validate() { return false; } } }
		if !self.ctry.validate() { return false }
		if let Some(ref tax_clctn_dtls_value) = self.tax_clctn_dtls { if !tax_clctn_dtls_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref eu_cptl_gn_value) = self.eu_cptl_gn { if !eu_cptl_gn_value.validate() { return false; } }
		if let Some(ref xtnded_eu_cptl_gn_value) = self.xtnded_eu_cptl_gn { if !xtnded_eu_cptl_gn_value.validate() { return false; } }
		if let Some(ref taxbl_incm_per_dvdd_value) = self.taxbl_incm_per_dvdd { if !taxbl_incm_per_dvdd_value.validate() { return false; } }
		if let Some(ref eu_dvdd_sts_value) = self.eu_dvdd_sts { if !eu_dvdd_sts_value.validate() { return false; } }
		if let Some(ref xtnded_eu_dvdd_sts_value) = self.xtnded_eu_dvdd_sts { if !xtnded_eu_dvdd_sts_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if self.ticker_identifier.chars().count() < 1 {
			return false
		}
		if self.ticker_identifier.chars().count() > 35 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref xtnded_tp_value) = self.xtnded_tp { if !xtnded_tp_value.validate() { return false; } }
		if let Some(ref pric_mtd_value) = self.pric_mtd { if !pric_mtd_value.validate() { return false; } }
		for item in &self.val_in_invstmt_ccy { if !item.validate() { return false; } }
		if let Some(ref val_in_altrntv_ccy_vec) = self.val_in_altrntv_ccy { for item in val_in_altrntv_ccy_vec { if !item.validate() { return false; } } }
		if let Some(ref taxbl_incm_per_shr_value) = self.taxbl_incm_per_shr { if !taxbl_incm_per_shr_value.validate() { return false; } }
		if let Some(ref taxbl_incm_per_shr_clctd_value) = self.taxbl_incm_per_shr_clctd { if !taxbl_incm_per_shr_clctd_value.validate() { return false; } }
		if let Some(ref xtnded_taxbl_incm_per_shr_clctd_value) = self.xtnded_taxbl_incm_per_shr_clctd { if !xtnded_taxbl_incm_per_shr_clctd_value.validate() { return false; } }
		if let Some(ref taxbl_incm_per_dvdd_value) = self.taxbl_incm_per_dvdd { if !taxbl_incm_per_dvdd_value.validate() { return false; } }
		if let Some(ref eu_dvdd_sts_value) = self.eu_dvdd_sts { if !eu_dvdd_sts_value.validate() { return false; } }
		if let Some(ref xtnded_eu_dvdd_sts_value) = self.xtnded_eu_dvdd_sts { if !xtnded_eu_dvdd_sts_value.validate() { return false; } }
		if let Some(ref chrg_dtls_vec) = self.chrg_dtls { for item in chrg_dtls_vec { if !item.validate() { return false; } } }
		if let Some(ref tax_lblty_dtls_vec) = self.tax_lblty_dtls { for item in tax_lblty_dtls_vec { if !item.validate() { return false; } } }
		if let Some(ref tax_rfnd_dtls_vec) = self.tax_rfnd_dtls { for item in tax_rfnd_dtls_vec { if !item.validate() { return false; } } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if !self.ccy.validate() { return false }
		if !self.pric_tp_chng_bsis.validate() { return false }
		if !self.pric_chng.validate() { return false }
		if let Some(ref by_prdfnd_tm_prds_value) = self.by_prdfnd_tm_prds { if !by_prdfnd_tm_prds_value.validate() { return false; } }
		if let Some(ref by_usr_dfnd_tm_prd_vec) = self.by_usr_dfnd_tm_prd { for item in by_usr_dfnd_tm_prd_vec { if !item.validate() { return false; } } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
	}
}
