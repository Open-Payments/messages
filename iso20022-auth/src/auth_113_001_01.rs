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


// AmountAndDirection53 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection53 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}

impl AmountAndDirection53 {
	pub fn validate(&self) -> bool {
		if !self.amt.validate() { return false }
		return true
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
	pub fn validate(&self) -> bool {
		if !self.amt.validate() { return false }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref tradg_phs_value) = self.tradg_phs { if !tradg_phs_value.validate() { return false; } }
		if let Some(ref indctv_auctn_pric_value) = self.indctv_auctn_pric { if !indctv_auctn_pric_value.validate() { return false; } }
		if let Some(ref indctv_auctn_vol_value) = self.indctv_auctn_vol { if !indctv_auctn_vol_value.validate() { return false; } }
		return true
	}
}


// CancelOrderReport1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CancelOrderReport1 {
	#[serde(rename = "RptId")]
	pub rpt_id: Max140Text,
}

impl CancelOrderReport1 {
	pub fn validate(&self) -> bool {
		if !self.rpt_id.validate() { return false }
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


// DateTimePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}

impl DateTimePeriod1 {
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


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}

impl Exact4AlphaNumericText {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
		if !pattern.is_match(&self.exact4_alpha_numeric_text) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref prsn_value) = self.prsn { if !prsn_value.validate() { return false; } }
		if let Some(ref algo_value) = self.algo { if !algo_value.validate() { return false; } }
		if let Some(ref clnt_value) = self.clnt { if !clnt_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if self.external_person_identification1_code.chars().count() < 1 {
			return false
		}
		if self.external_person_identification1_code.chars().count() > 4 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref strtgy_instrms_vec) = self.strtgy_instrms { for item in strtgy_instrms_vec { if !item.validate() { return false; } } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref nmnl_val_value) = self.nmnl_val { if !nmnl_val_value.validate() { return false; } }
		if let Some(ref mntry_val_value) = self.mntry_val { if !mntry_val_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.issr.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
		if !pattern.is_match(&self.isin_oct2015_identifier) {
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


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}

impl LEIIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.lei_identifier) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
		if !pattern.is_match(&self.mic_identifier) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max140_text.chars().count() < 1 {
			return false
		}
		if self.max140_text.chars().count() > 140 {
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


// Max50Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max50Text {
	#[serde(rename = "$value")]
	pub max50_text: String,
}

impl Max50Text {
	pub fn validate(&self) -> bool {
		if self.max50_text.chars().count() < 1 {
			return false
		}
		if self.max50_text.chars().count() > 50 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max52_text.chars().count() < 1 {
			return false
		}
		if self.max52_text.chars().count() > 52 {
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


// MinimumExecutable1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MinimumExecutable1 {
	#[serde(rename = "Sz", skip_serializing_if = "Option::is_none")]
	pub sz: Option<FinancialInstrumentQuantity25Choice>,
	#[serde(rename = "FrstExctnOnly", skip_serializing_if = "Option::is_none")]
	pub frst_exctn_only: Option<bool>,
}

impl MinimumExecutable1 {
	pub fn validate(&self) -> bool {
		if let Some(ref sz_value) = self.sz { if !sz_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if !self.rpt_id.validate() { return false }
		for item in &self.ordr { if !item.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
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
	pub fn validate(&self) -> bool {
		if !self.rpt_hdr.validate() { return false }
		for item in &self.ordr_rpt { if !item.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref ordr_tp_value) = self.ordr_tp { if !ordr_tp_value.validate() { return false; } }
		if let Some(ref ordr_tp_clssfctn_value) = self.ordr_tp_clssfctn { if !ordr_tp_clssfctn_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if !self.ordr_id_data.validate() { return false }
		if let Some(ref auctn_data_value) = self.auctn_data { if !auctn_data_value.validate() { return false; } }
		if let Some(ref ordr_data_value) = self.ordr_data { if !ordr_data_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref submitg_ntty_value) = self.submitg_ntty { if !submitg_ntty_value.validate() { return false; } }
		if let Some(ref clnt_id_value) = self.clnt_id { if !clnt_id_value.validate() { return false; } }
		if let Some(ref invstmt_dcsn_prsn_value) = self.invstmt_dcsn_prsn { if !invstmt_dcsn_prsn_value.validate() { return false; } }
		if let Some(ref exctg_prsn_value) = self.exctg_prsn { if !exctg_prsn_value.validate() { return false; } }
		if let Some(ref non_exctg_brkr_value) = self.non_exctg_brkr { if !non_exctg_brkr_value.validate() { return false; } }
		if let Some(ref tradg_cpcty_value) = self.tradg_cpcty { if !tradg_cpcty_value.validate() { return false; } }
		if let Some(ref ordr_clssfctn_value) = self.ordr_clssfctn { if !ordr_clssfctn_value.validate() { return false; } }
		if let Some(ref ordr_prics_value) = self.ordr_prics { if !ordr_prics_value.validate() { return false; } }
		if let Some(ref instr_data_value) = self.instr_data { if !instr_data_value.validate() { return false; } }
		if let Some(ref tx_data_value) = self.tx_data { if !tx_data_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if !self.ordr_book_id.validate() { return false }
		if let Some(ref prty_value) = self.prty { if !prty_value.validate() { return false; } }
		if !self.trad_vn.validate() { return false }
		if !self.fin_instrm.validate() { return false }
		if let Some(ref ordr_id_value) = self.ordr_id { if !ordr_id_value.validate() { return false; } }
		if let Some(ref vldty_prd_value) = self.vldty_prd { if !vldty_prd_value.validate() { return false; } }
		if let Some(ref ordr_rstrctn_vec) = self.ordr_rstrctn { for item in ordr_rstrctn_vec { if !item.validate() { return false; } } }
		if let Some(ref evt_tp_value) = self.evt_tp { if !evt_tp_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref buy_sell_ind_value) = self.buy_sell_ind { if !buy_sell_ind_value.validate() { return false; } }
		if let Some(ref ordr_vldty_sts_value) = self.ordr_vldty_sts { if !ordr_vldty_sts_value.validate() { return false; } }
		if let Some(ref ordr_sts_vec) = self.ordr_sts { for item in ordr_sts_vec { if !item.validate() { return false; } } }
		if let Some(ref initl_qty_value) = self.initl_qty { if !initl_qty_value.validate() { return false; } }
		if let Some(ref rmng_qty_value) = self.rmng_qty { if !rmng_qty_value.validate() { return false; } }
		if let Some(ref dispd_qty_value) = self.dispd_qty { if !dispd_qty_value.validate() { return false; } }
		if let Some(ref min_accptbl_qty_value) = self.min_accptbl_qty { if !min_accptbl_qty_value.validate() { return false; } }
		if let Some(ref min_exctbl_value) = self.min_exctbl { if !min_exctbl_value.validate() { return false; } }
		if let Some(ref rtg_strtgy_value) = self.rtg_strtgy { if !rtg_strtgy_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref lmt_pric_value) = self.lmt_pric { if !lmt_pric_value.validate() { return false; } }
		if let Some(ref stop_pric_value) = self.stop_pric { if !stop_pric_value.validate() { return false; } }
		if let Some(ref addtl_lmt_pric_value) = self.addtl_lmt_pric { if !addtl_lmt_pric_value.validate() { return false; } }
		if let Some(ref pggd_pric_value) = self.pggd_pric { if !pggd_pric_value.validate() { return false; } }
		if let Some(ref ccy_scnd_leg_value) = self.ccy_scnd_leg { if !ccy_scnd_leg_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref new_value) = self.new { if !new_value.validate() { return false; } }
		if let Some(ref cxl_value) = self.cxl { if !cxl_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref ordr_rstrctn_cd_value) = self.ordr_rstrctn_cd { if !ordr_rstrctn_cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if !self.pg_nb.validate() { return false }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
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
	pub fn validate(&self) -> bool {
		if let Some(ref fr_to_dt_value) = self.fr_to_dt { if !fr_to_dt_value.validate() { return false; } }
		if let Some(ref fr_to_dt_tm_value) = self.fr_to_dt_tm { if !fr_to_dt_tm_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		if let Some(ref prsn_value) = self.prsn { if !prsn_value.validate() { return false; } }
		if let Some(ref xcptn_id_value) = self.xcptn_id { if !xcptn_id_value.validate() { return false; } }
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


// PositiveNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PositiveNumber {
	#[serde(rename = "$value")]
	pub positive_number: f64,
}

impl PositiveNumber {
	pub fn validate(&self) -> bool {
		if self.positive_number < 1.000000 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if !self.rptg_ntty.validate() { return false }
		if !self.rptg_prd.validate() { return false }
		if let Some(ref isin_vec) = self.isin { for item in isin_vec { if !item.validate() { return false; } } }
		if let Some(ref msg_pgntn_value) = self.msg_pgntn { if !msg_pgntn_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if !self.pdg.validate() { return false }
		if let Some(ref ccy_value) = self.ccy { if !ccy_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref mntry_val_value) = self.mntry_val { if !mntry_val_value.validate() { return false; } }
		if let Some(ref nmnl_val_value) = self.nmnl_val { if !nmnl_val_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref mntry_val_value) = self.mntry_val { if !mntry_val_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref pric_value) = self.pric { if !pric_value.validate() { return false; } }
		if let Some(ref no_pric_value) = self.no_pric { if !no_pric_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref plc_and_nm_value) = self.plc_and_nm { if !plc_and_nm_value.validate() { return false; } }
		if !self.envlp.validate() { return false }
		return true
	}
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref mkt_id_cd_value) = self.mkt_id_cd { if !mkt_id_cd_value.validate() { return false; } }
		if let Some(ref ntl_cmptnt_authrty_value) = self.ntl_cmptnt_authrty { if !ntl_cmptnt_authrty_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.tp.validate() { return false }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref tx_pric_value) = self.tx_pric { if !tx_pric_value.validate() { return false; } }
		if let Some(ref tradd_qty_value) = self.tradd_qty { if !tradd_qty_value.validate() { return false; } }
		if let Some(ref pssv_or_aggrssv_ind_value) = self.pssv_or_aggrssv_ind { if !pssv_or_aggrssv_ind_value.validate() { return false; } }
		if let Some(ref strtgy_lkd_ordr_id_value) = self.strtgy_lkd_ordr_id { if !strtgy_lkd_ordr_id_value.validate() { return false; } }
		if let Some(ref tx_id_value) = self.tx_id { if !tx_id_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref vldty_prd_cd_value) = self.vldty_prd_cd { if !vldty_prd_cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
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
