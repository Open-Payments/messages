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


// AuctionData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AuctionData2 {
	#[serde(rename = "TradgPhs", skip_serializing_if = "Option::is_none")]
	pub tradg_phs: Option<Max50Text>,
	#[serde(rename = "IndctvAuctnPric", skip_serializing_if = "Option::is_none")]
	pub indctv_auctn_pric: Option<SecuritiesTransactionPrice21Choice>,
	#[serde(rename = "IndctvAuctnVol", skip_serializing_if = "Option::is_none")]
	pub indctv_auctn_vol: Option<FinancialInstrumentQuantity25Choice>,
}

impl AuctionData2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tradg_phs_value) = self.tradg_phs { if let Err(e) = tradg_phs_value.validate() { return Err(e); } }
		if let Some(ref indctv_auctn_pric_value) = self.indctv_auctn_pric { if let Err(e) = indctv_auctn_pric_value.validate() { return Err(e); } }
		if let Some(ref indctv_auctn_vol_value) = self.indctv_auctn_vol { if let Err(e) = indctv_auctn_vol_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CancelOrderReport1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CancelOrderReport1 {
	#[serde(rename = "RptId")]
	pub rpt_id: Max140Text,
}

impl CancelOrderReport1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.rpt_id.validate() { return Err(e); }
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


// DateTimePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}

impl DateTimePeriod1 {
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


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
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


// ExecutingParty2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExecutingParty2Choice {
	#[serde(rename = "Prsn", skip_serializing_if = "Option::is_none")]
	pub prsn: Option<GenericPersonIdentification1>,
	#[serde(rename = "Algo", skip_serializing_if = "Option::is_none")]
	pub algo: Option<Max50Text>,
	#[serde(rename = "Clnt", skip_serializing_if = "Option::is_none")]
	pub clnt: Option<NoReasonCode>,
}

impl ExecutingParty2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref prsn_value) = self.prsn { if let Err(e) = prsn_value.validate() { return Err(e); } }
		if let Some(ref algo_value) = self.algo { if let Err(e) = algo_value.validate() { return Err(e); } }
		if let Some(ref clnt_value) = self.clnt { if let Err(e) = clnt_value.validate() { return Err(e); } }
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


// FinancialInstrument99Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument99Choice {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ISINOct2015Identifier>,
	#[serde(rename = "StrtgyInstrms", skip_serializing_if = "Option::is_none")]
	pub strtgy_instrms: Option<Vec<ISINOct2015Identifier>>,
}

impl FinancialInstrument99Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		if let Some(ref strtgy_instrms_vec) = self.strtgy_instrms { for item in strtgy_instrms_vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// GenericIdentification30 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: Exact4AlphaNumericText,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
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


// MinimumExecutable1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MinimumExecutable1 {
	#[serde(rename = "Sz", skip_serializing_if = "Option::is_none")]
	pub sz: Option<FinancialInstrumentQuantity25Choice>,
	#[serde(rename = "FrstExctnOnly", skip_serializing_if = "Option::is_none")]
	pub frst_exctn_only: Option<bool>,
}

impl MinimumExecutable1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref sz_value) = self.sz { if let Err(e) = sz_value.validate() { return Err(e); } }
		Ok(())
	}
}


// NewOrderReport2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewOrderReport2 {
	#[serde(rename = "RptId")]
	pub rpt_id: Max140Text,
	#[serde(rename = "Ordr")]
	pub ordr: Vec<OrderData3>,
}

impl NewOrderReport2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.rpt_id.validate() { return Err(e); }
		for item in &self.ordr { if let Err(e) = item.validate() { return Err(e); } }
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


// OrderBookReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderBookReportV01 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: SecuritiesMarketReportHeader3,
	#[serde(rename = "OrdrRpt")]
	pub ordr_rpt: Vec<OrderReport2Choice>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl OrderBookReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.rpt_hdr.validate() { return Err(e); }
		for item in &self.ordr_rpt { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// OrderClassification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderClassification2 {
	#[serde(rename = "OrdrTp", skip_serializing_if = "Option::is_none")]
	pub ordr_tp: Option<Max50Text>,
	#[serde(rename = "OrdrTpClssfctn", skip_serializing_if = "Option::is_none")]
	pub ordr_tp_clssfctn: Option<OrderType3Code>,
}

impl OrderClassification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ordr_tp_value) = self.ordr_tp { if let Err(e) = ordr_tp_value.validate() { return Err(e); } }
		if let Some(ref ordr_tp_clssfctn_value) = self.ordr_tp_clssfctn { if let Err(e) = ordr_tp_clssfctn_value.validate() { return Err(e); } }
		Ok(())
	}
}


// OrderData3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderData3 {
	#[serde(rename = "OrdrIdData")]
	pub ordr_id_data: OrderIdentification2,
	#[serde(rename = "AuctnData", skip_serializing_if = "Option::is_none")]
	pub auctn_data: Option<AuctionData2>,
	#[serde(rename = "OrdrData", skip_serializing_if = "Option::is_none")]
	pub ordr_data: Option<OrderData4>,
}

impl OrderData3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ordr_id_data.validate() { return Err(e); }
		if let Some(ref auctn_data_value) = self.auctn_data { if let Err(e) = auctn_data_value.validate() { return Err(e); } }
		if let Some(ref ordr_data_value) = self.ordr_data { if let Err(e) = ordr_data_value.validate() { return Err(e); } }
		Ok(())
	}
}


// OrderData4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderData4 {
	#[serde(rename = "SubmitgNtty", skip_serializing_if = "Option::is_none")]
	pub submitg_ntty: Option<LEIIdentifier>,
	#[serde(rename = "DrctElctrncAccs", skip_serializing_if = "Option::is_none")]
	pub drct_elctrnc_accs: Option<bool>,
	#[serde(rename = "ClntId", skip_serializing_if = "Option::is_none")]
	pub clnt_id: Option<PersonOrOrganisation4Choice>,
	#[serde(rename = "InvstmtDcsnPrsn", skip_serializing_if = "Option::is_none")]
	pub invstmt_dcsn_prsn: Option<ExecutingParty2Choice>,
	#[serde(rename = "ExctgPrsn", skip_serializing_if = "Option::is_none")]
	pub exctg_prsn: Option<ExecutingParty2Choice>,
	#[serde(rename = "NonExctgBrkr", skip_serializing_if = "Option::is_none")]
	pub non_exctg_brkr: Option<LEIIdentifier>,
	#[serde(rename = "TradgCpcty", skip_serializing_if = "Option::is_none")]
	pub tradg_cpcty: Option<RegulatoryTradingCapacity1Code>,
	#[serde(rename = "LqdtyPrvsnActvty", skip_serializing_if = "Option::is_none")]
	pub lqdty_prvsn_actvty: Option<bool>,
	#[serde(rename = "OrdrClssfctn", skip_serializing_if = "Option::is_none")]
	pub ordr_clssfctn: Option<OrderClassification2>,
	#[serde(rename = "OrdrPrics", skip_serializing_if = "Option::is_none")]
	pub ordr_prics: Option<OrderPriceData2>,
	#[serde(rename = "InstrData", skip_serializing_if = "Option::is_none")]
	pub instr_data: Option<OrderInstructionData2>,
	#[serde(rename = "TxData", skip_serializing_if = "Option::is_none")]
	pub tx_data: Option<TransactionData3>,
}

impl OrderData4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref submitg_ntty_value) = self.submitg_ntty { if let Err(e) = submitg_ntty_value.validate() { return Err(e); } }
		if let Some(ref clnt_id_value) = self.clnt_id { if let Err(e) = clnt_id_value.validate() { return Err(e); } }
		if let Some(ref invstmt_dcsn_prsn_value) = self.invstmt_dcsn_prsn { if let Err(e) = invstmt_dcsn_prsn_value.validate() { return Err(e); } }
		if let Some(ref exctg_prsn_value) = self.exctg_prsn { if let Err(e) = exctg_prsn_value.validate() { return Err(e); } }
		if let Some(ref non_exctg_brkr_value) = self.non_exctg_brkr { if let Err(e) = non_exctg_brkr_value.validate() { return Err(e); } }
		if let Some(ref tradg_cpcty_value) = self.tradg_cpcty { if let Err(e) = tradg_cpcty_value.validate() { return Err(e); } }
		if let Some(ref ordr_clssfctn_value) = self.ordr_clssfctn { if let Err(e) = ordr_clssfctn_value.validate() { return Err(e); } }
		if let Some(ref ordr_prics_value) = self.ordr_prics { if let Err(e) = ordr_prics_value.validate() { return Err(e); } }
		if let Some(ref instr_data_value) = self.instr_data { if let Err(e) = instr_data_value.validate() { return Err(e); } }
		if let Some(ref tx_data_value) = self.tx_data { if let Err(e) = tx_data_value.validate() { return Err(e); } }
		Ok(())
	}
}


// OrderEventType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderEventType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<OrderEventType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl OrderEventType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// OrderEventType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OrderEventType1Code {
	#[default]
	#[serde(rename = "CAME")]
	CodeCAME,
	#[serde(rename = "CAMO")]
	CodeCAMO,
	#[serde(rename = "CHME")]
	CodeCHME,
	#[serde(rename = "CHMO")]
	CodeCHMO,
	#[serde(rename = "EXPI")]
	CodeEXPI,
	#[serde(rename = "FILL")]
	CodeFILL,
	#[serde(rename = "NEWO")]
	CodeNEWO,
	#[serde(rename = "PARF")]
	CodePARF,
	#[serde(rename = "REMA")]
	CodeREMA,
	#[serde(rename = "REMO")]
	CodeREMO,
	#[serde(rename = "REMH")]
	CodeREMH,
	#[serde(rename = "REME")]
	CodeREME,
	#[serde(rename = "TRIG")]
	CodeTRIG,
	#[serde(rename = "RFQS")]
	CodeRFQS,
	#[serde(rename = "RFQR")]
	CodeRFQR,
}

impl OrderEventType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OrderIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderIdentification2 {
	#[serde(rename = "OrdrBookId")]
	pub ordr_book_id: Max35Text,
	#[serde(rename = "SeqNb")]
	pub seq_nb: f64,
	#[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
	pub prty: Option<OrderPriority1>,
	#[serde(rename = "TmStmp")]
	pub tm_stmp: String,
	#[serde(rename = "TradVn")]
	pub trad_vn: MICIdentifier,
	#[serde(rename = "FinInstrm")]
	pub fin_instrm: FinancialInstrument99Choice,
	#[serde(rename = "OrdrId", skip_serializing_if = "Option::is_none")]
	pub ordr_id: Option<Max50Text>,
	#[serde(rename = "DtOfRct", skip_serializing_if = "Option::is_none")]
	pub dt_of_rct: Option<String>,
	#[serde(rename = "VldtyPrd", skip_serializing_if = "Option::is_none")]
	pub vldty_prd: Option<ValidityPeriod1Choice>,
	#[serde(rename = "OrdrRstrctn", skip_serializing_if = "Option::is_none")]
	pub ordr_rstrctn: Option<Vec<OrderRestriction1Choice>>,
	#[serde(rename = "VldtyDtTm", skip_serializing_if = "Option::is_none")]
	pub vldty_dt_tm: Option<String>,
	#[serde(rename = "EvtTp", skip_serializing_if = "Option::is_none")]
	pub evt_tp: Option<OrderEventType1Choice>,
}

impl OrderIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ordr_book_id.validate() { return Err(e); }
		if let Some(ref prty_value) = self.prty { if let Err(e) = prty_value.validate() { return Err(e); } }
		if let Err(e) = self.trad_vn.validate() { return Err(e); }
		if let Err(e) = self.fin_instrm.validate() { return Err(e); }
		if let Some(ref ordr_id_value) = self.ordr_id { if let Err(e) = ordr_id_value.validate() { return Err(e); } }
		if let Some(ref vldty_prd_value) = self.vldty_prd { if let Err(e) = vldty_prd_value.validate() { return Err(e); } }
		if let Some(ref ordr_rstrctn_vec) = self.ordr_rstrctn { for item in ordr_rstrctn_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref evt_tp_value) = self.evt_tp { if let Err(e) = evt_tp_value.validate() { return Err(e); } }
		Ok(())
	}
}


// OrderInstructionData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderInstructionData2 {
	#[serde(rename = "BuySellInd", skip_serializing_if = "Option::is_none")]
	pub buy_sell_ind: Option<Side6Code>,
	#[serde(rename = "OrdrVldtySts", skip_serializing_if = "Option::is_none")]
	pub ordr_vldty_sts: Option<OrderStatus10Code>,
	#[serde(rename = "OrdrSts", skip_serializing_if = "Option::is_none")]
	pub ordr_sts: Option<Vec<OrderStatus11Code>>,
	#[serde(rename = "InitlQty", skip_serializing_if = "Option::is_none")]
	pub initl_qty: Option<FinancialInstrumentQuantity25Choice>,
	#[serde(rename = "RmngQty", skip_serializing_if = "Option::is_none")]
	pub rmng_qty: Option<FinancialInstrumentQuantity25Choice>,
	#[serde(rename = "DispdQty", skip_serializing_if = "Option::is_none")]
	pub dispd_qty: Option<FinancialInstrumentQuantity25Choice>,
	#[serde(rename = "MinAccptblQty", skip_serializing_if = "Option::is_none")]
	pub min_accptbl_qty: Option<FinancialInstrumentQuantity25Choice>,
	#[serde(rename = "MinExctbl", skip_serializing_if = "Option::is_none")]
	pub min_exctbl: Option<MinimumExecutable1>,
	#[serde(rename = "PssvOnlyInd", skip_serializing_if = "Option::is_none")]
	pub pssv_only_ind: Option<bool>,
	#[serde(rename = "SlfExctnPrvntn", skip_serializing_if = "Option::is_none")]
	pub slf_exctn_prvntn: Option<bool>,
	#[serde(rename = "RtgStrtgy", skip_serializing_if = "Option::is_none")]
	pub rtg_strtgy: Option<Max50Text>,
}

impl OrderInstructionData2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref buy_sell_ind_value) = self.buy_sell_ind { if let Err(e) = buy_sell_ind_value.validate() { return Err(e); } }
		if let Some(ref ordr_vldty_sts_value) = self.ordr_vldty_sts { if let Err(e) = ordr_vldty_sts_value.validate() { return Err(e); } }
		if let Some(ref ordr_sts_vec) = self.ordr_sts { for item in ordr_sts_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref initl_qty_value) = self.initl_qty { if let Err(e) = initl_qty_value.validate() { return Err(e); } }
		if let Some(ref rmng_qty_value) = self.rmng_qty { if let Err(e) = rmng_qty_value.validate() { return Err(e); } }
		if let Some(ref dispd_qty_value) = self.dispd_qty { if let Err(e) = dispd_qty_value.validate() { return Err(e); } }
		if let Some(ref min_accptbl_qty_value) = self.min_accptbl_qty { if let Err(e) = min_accptbl_qty_value.validate() { return Err(e); } }
		if let Some(ref min_exctbl_value) = self.min_exctbl { if let Err(e) = min_exctbl_value.validate() { return Err(e); } }
		if let Some(ref rtg_strtgy_value) = self.rtg_strtgy { if let Err(e) = rtg_strtgy_value.validate() { return Err(e); } }
		Ok(())
	}
}


// OrderPriceData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderPriceData2 {
	#[serde(rename = "LmtPric", skip_serializing_if = "Option::is_none")]
	pub lmt_pric: Option<SecuritiesTransactionPrice2Choice>,
	#[serde(rename = "StopPric", skip_serializing_if = "Option::is_none")]
	pub stop_pric: Option<SecuritiesTransactionPrice2Choice>,
	#[serde(rename = "AddtlLmtPric", skip_serializing_if = "Option::is_none")]
	pub addtl_lmt_pric: Option<SecuritiesTransactionPrice2Choice>,
	#[serde(rename = "PggdPric", skip_serializing_if = "Option::is_none")]
	pub pggd_pric: Option<SecuritiesTransactionPrice2Choice>,
	#[serde(rename = "CcyScndLeg", skip_serializing_if = "Option::is_none")]
	pub ccy_scnd_leg: Option<ActiveOrHistoricCurrencyCode>,
}

impl OrderPriceData2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref lmt_pric_value) = self.lmt_pric { if let Err(e) = lmt_pric_value.validate() { return Err(e); } }
		if let Some(ref stop_pric_value) = self.stop_pric { if let Err(e) = stop_pric_value.validate() { return Err(e); } }
		if let Some(ref addtl_lmt_pric_value) = self.addtl_lmt_pric { if let Err(e) = addtl_lmt_pric_value.validate() { return Err(e); } }
		if let Some(ref pggd_pric_value) = self.pggd_pric { if let Err(e) = pggd_pric_value.validate() { return Err(e); } }
		if let Some(ref ccy_scnd_leg_value) = self.ccy_scnd_leg { if let Err(e) = ccy_scnd_leg_value.validate() { return Err(e); } }
		Ok(())
	}
}


// OrderPriority1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderPriority1 {
	#[serde(rename = "TmStmp", skip_serializing_if = "Option::is_none")]
	pub tm_stmp: Option<String>,
	#[serde(rename = "Sz", skip_serializing_if = "Option::is_none")]
	pub sz: Option<f64>,
}

impl OrderPriority1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OrderReport2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderReport2Choice {
	#[serde(rename = "New", skip_serializing_if = "Option::is_none")]
	pub new: Option<NewOrderReport2>,
	#[serde(rename = "Cxl", skip_serializing_if = "Option::is_none")]
	pub cxl: Option<CancelOrderReport1>,
}

impl OrderReport2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref new_value) = self.new { if let Err(e) = new_value.validate() { return Err(e); } }
		if let Some(ref cxl_value) = self.cxl { if let Err(e) = cxl_value.validate() { return Err(e); } }
		Ok(())
	}
}


// OrderRestriction1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderRestriction1Choice {
	#[serde(rename = "OrdrRstrctnCd", skip_serializing_if = "Option::is_none")]
	pub ordr_rstrctn_cd: Option<OrderRestrictionType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl OrderRestriction1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ordr_rstrctn_cd_value) = self.ordr_rstrctn_cd { if let Err(e) = ordr_rstrctn_cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// OrderRestrictionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OrderRestrictionType1Code {
	#[default]
	#[serde(rename = "SESR")]
	CodeSESR,
	#[serde(rename = "VFAR")]
	CodeVFAR,
	#[serde(rename = "VFCR")]
	CodeVFCR,
}

impl OrderRestrictionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OrderStatus10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OrderStatus10Code {
	#[default]
	#[serde(rename = "ACTI")]
	CodeACTI,
	#[serde(rename = "INAC")]
	CodeINAC,
	#[serde(rename = "SUSP")]
	CodeSUSP,
}

impl OrderStatus10Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OrderStatus11Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OrderStatus11Code {
	#[default]
	#[serde(rename = "FIRM")]
	CodeFIRM,
	#[serde(rename = "IMPL")]
	CodeIMPL,
	#[serde(rename = "INDI")]
	CodeINDI,
	#[serde(rename = "ROUT")]
	CodeROUT,
}

impl OrderStatus11Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OrderType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OrderType3Code {
	#[default]
	#[serde(rename = "LMTO")]
	CodeLMTO,
	#[serde(rename = "STOP")]
	CodeSTOP,
}

impl OrderType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Pagination1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: Max5NumericText,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}

impl Pagination1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pg_nb.validate() { return Err(e); }
		Ok(())
	}
}


// PartyExceptionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PartyExceptionType1Code {
	#[default]
	#[serde(rename = "AGGR")]
	CodeAGGR,
	#[serde(rename = "PNAL")]
	CodePNAL,
}

impl PartyExceptionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PassiveOrAgressiveType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PassiveOrAgressiveType1Code {
	#[default]
	#[serde(rename = "AGRE")]
	CodeAGRE,
	#[serde(rename = "PASV")]
	CodePASV,
}

impl PassiveOrAgressiveType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
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


// Period11Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period11Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
	pub to_dt: Option<String>,
	#[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt: Option<Period2>,
	#[serde(rename = "FrToDtTm", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt_tm: Option<DateTimePeriod1>,
}

impl Period11Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref fr_to_dt_value) = self.fr_to_dt { if let Err(e) = fr_to_dt_value.validate() { return Err(e); } }
		if let Some(ref fr_to_dt_tm_value) = self.fr_to_dt_tm { if let Err(e) = fr_to_dt_tm_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Period2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}

impl Period2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
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


// PersonOrOrganisation4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonOrOrganisation4Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Prsn", skip_serializing_if = "Option::is_none")]
	pub prsn: Option<GenericPersonIdentification1>,
	#[serde(rename = "XcptnId", skip_serializing_if = "Option::is_none")]
	pub xcptn_id: Option<PartyExceptionType1Code>,
}

impl PersonOrOrganisation4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
		if let Some(ref prsn_value) = self.prsn { if let Err(e) = prsn_value.validate() { return Err(e); } }
		if let Some(ref xcptn_id_value) = self.xcptn_id { if let Err(e) = xcptn_id_value.validate() { return Err(e); } }
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


// PositiveNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PositiveNumber {
	#[serde(rename = "$value")]
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


// SecuritiesMarketReportHeader3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesMarketReportHeader3 {
	#[serde(rename = "RptgNtty")]
	pub rptg_ntty: TradingVenueIdentification1Choice,
	#[serde(rename = "RptgPrd")]
	pub rptg_prd: Period11Choice,
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<Vec<ISINOct2015Identifier>>,
	#[serde(rename = "SubmissnDtTm", skip_serializing_if = "Option::is_none")]
	pub submissn_dt_tm: Option<String>,
	#[serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none")]
	pub msg_pgntn: Option<Pagination1>,
	#[serde(rename = "NbRcrds", skip_serializing_if = "Option::is_none")]
	pub nb_rcrds: Option<f64>,
}

impl SecuritiesMarketReportHeader3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.rptg_ntty.validate() { return Err(e); }
		if let Err(e) = self.rptg_prd.validate() { return Err(e); }
		if let Some(ref isin_vec) = self.isin { for item in isin_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref msg_pgntn_value) = self.msg_pgntn { if let Err(e) = msg_pgntn_value.validate() { return Err(e); } }
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


// SecuritiesTransactionPrice21Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice21Choice {
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<AmountAndDirection53>,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
	#[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
	pub yld: Option<f64>,
	#[serde(rename = "BsisPts", skip_serializing_if = "Option::is_none")]
	pub bsis_pts: Option<f64>,
	#[serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none")]
	pub nmnl_val: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl SecuritiesTransactionPrice21Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref mntry_val_value) = self.mntry_val { if let Err(e) = mntry_val_value.validate() { return Err(e); } }
		if let Some(ref nmnl_val_value) = self.nmnl_val { if let Err(e) = nmnl_val_value.validate() { return Err(e); } }
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


// Side6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Side6Code {
	#[default]
	#[serde(rename = "BUYI")]
	CodeBUYI,
	#[serde(rename = "SELL")]
	CodeSELL,
}

impl Side6Code {
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


// TradingVenue2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradingVenue2Code {
	#[default]
	#[serde(rename = "APPA")]
	CodeAPPA,
	#[serde(rename = "CTPS")]
	CodeCTPS,
}

impl TradingVenue2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TradingVenueIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueIdentification1Choice {
	#[serde(rename = "MktIdCd", skip_serializing_if = "Option::is_none")]
	pub mkt_id_cd: Option<MICIdentifier>,
	#[serde(rename = "NtlCmptntAuthrty", skip_serializing_if = "Option::is_none")]
	pub ntl_cmptnt_authrty: Option<CountryCode>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<TradingVenueIdentification2>,
}

impl TradingVenueIdentification1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref mkt_id_cd_value) = self.mkt_id_cd { if let Err(e) = mkt_id_cd_value.validate() { return Err(e); } }
		if let Some(ref ntl_cmptnt_authrty_value) = self.ntl_cmptnt_authrty { if let Err(e) = ntl_cmptnt_authrty_value.validate() { return Err(e); } }
		if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TradingVenueIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueIdentification2 {
	#[serde(rename = "Id")]
	pub id: Max50Text,
	#[serde(rename = "Tp")]
	pub tp: TradingVenue2Code,
}

impl TradingVenueIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Err(e) = self.tp.validate() { return Err(e); }
		Ok(())
	}
}


// TransactionData3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionData3 {
	#[serde(rename = "TxPric", skip_serializing_if = "Option::is_none")]
	pub tx_pric: Option<SecuritiesTransactionPrice4Choice>,
	#[serde(rename = "TraddQty", skip_serializing_if = "Option::is_none")]
	pub tradd_qty: Option<FinancialInstrumentQuantity25Choice>,
	#[serde(rename = "PssvOrAggrssvInd", skip_serializing_if = "Option::is_none")]
	pub pssv_or_aggrssv_ind: Option<PassiveOrAgressiveType1Code>,
	#[serde(rename = "StrtgyLkdOrdrId", skip_serializing_if = "Option::is_none")]
	pub strtgy_lkd_ordr_id: Option<Max50Text>,
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<Max52Text>,
}

impl TransactionData3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tx_pric_value) = self.tx_pric { if let Err(e) = tx_pric_value.validate() { return Err(e); } }
		if let Some(ref tradd_qty_value) = self.tradd_qty { if let Err(e) = tradd_qty_value.validate() { return Err(e); } }
		if let Some(ref pssv_or_aggrssv_ind_value) = self.pssv_or_aggrssv_ind { if let Err(e) = pssv_or_aggrssv_ind_value.validate() { return Err(e); } }
		if let Some(ref strtgy_lkd_ordr_id_value) = self.strtgy_lkd_ordr_id { if let Err(e) = strtgy_lkd_ordr_id_value.validate() { return Err(e); } }
		if let Some(ref tx_id_value) = self.tx_id { if let Err(e) = tx_id_value.validate() { return Err(e); } }
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


// ValidityPeriod1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValidityPeriod1Choice {
	#[serde(rename = "VldtyPrdCd", skip_serializing_if = "Option::is_none")]
	pub vldty_prd_cd: Option<ValidityPeriodType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl ValidityPeriod1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vldty_prd_cd_value) = self.vldty_prd_cd { if let Err(e) = vldty_prd_cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ValidityPeriodType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ValidityPeriodType1Code {
	#[default]
	#[serde(rename = "FOKV")]
	CodeFOKV,
	#[serde(rename = "GADV")]
	CodeGADV,
	#[serde(rename = "GASV")]
	CodeGASV,
	#[serde(rename = "GATV")]
	CodeGATV,
	#[serde(rename = "DAVY")]
	CodeDAVY,
	#[serde(rename = "GTCV")]
	CodeGTCV,
	#[serde(rename = "GTDV")]
	CodeGTDV,
	#[serde(rename = "GTSV")]
	CodeGTSV,
	#[serde(rename = "GTTV")]
	CodeGTTV,
	#[serde(rename = "IOCV")]
	CodeIOCV,
}

impl ValidityPeriodType1Code {
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
