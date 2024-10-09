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


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAndAmount_SimpleType")]
	pub active_currency_and_amount_simple_type: f64,
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// EquityInstrumentReportingClassification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EquityInstrumentReportingClassification1Code {
	#[serde(rename = "EquityInstrumentReportingClassification1Code")]
	pub equity_instrument_reporting_classification1_code: String,
}


// FinancialInstrumentReportingEquityTradingActivityResultV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentReportingEquityTradingActivityResultV02 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: SecuritiesMarketReportHeader1,
	#[serde(rename = "EqtyTrnsprncyData")]
	pub eqty_trnsprncy_data: Vec<TransparencyDataReport17>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "ISINOct2015Identifier")]
	pub isin_oct2015_identifier: String,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[serde(rename = "MICIdentifier")]
	pub mic_identifier: String,
}


// MarketDetail2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketDetail2 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "AvrgDalyNbOfTxs")]
	pub avrg_daly_nb_of_txs: Option<f64>,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max50Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max50Text {
	#[serde(rename = "Max50Text")]
	pub max50_text: String,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// Period2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// Period4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period4Choice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "FrDt")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt")]
	pub to_dt: Option<String>,
	#[serde(rename = "FrDtToDt")]
	pub fr_dt_to_dt: Option<Period2>,
}


// SecuritiesMarketReportHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesMarketReportHeader1 {
	#[serde(rename = "RptgNtty")]
	pub rptg_ntty: TradingVenueIdentification1Choice,
	#[serde(rename = "RptgPrd")]
	pub rptg_prd: Period4Choice,
	#[serde(rename = "SubmissnDtTm")]
	pub submissn_dt_tm: Option<String>,
}


// StatisticsTransparency3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatisticsTransparency3 {
	#[serde(rename = "AvrgDalyTrnvr")]
	pub avrg_daly_trnvr: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "AvrgTxVal")]
	pub avrg_tx_val: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "LrgInScale")]
	pub lrg_in_scale: Option<f64>,
	#[serde(rename = "StdMktSz")]
	pub std_mkt_sz: Option<f64>,
	#[serde(rename = "AvrgDalyNbOfTxs")]
	pub avrg_daly_nb_of_txs: Option<f64>,
	#[serde(rename = "TtlNbOfTxsExctd")]
	pub ttl_nb_of_txs_exctd: Option<f64>,
	#[serde(rename = "TtlVolOfTxsExctd")]
	pub ttl_vol_of_txs_exctd: Option<f64>,
	#[serde(rename = "TtlNbOfTradgDays")]
	pub ttl_nb_of_tradg_days: Option<f64>,
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// TradingVenue2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenue2Code {
	#[serde(rename = "TradingVenue2Code")]
	pub trading_venue2_code: String,
}


// TradingVenueIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueIdentification1Choice {
	#[serde(rename = "MktIdCd")]
	pub mkt_id_cd: Option<String>,
	#[serde(rename = "NtlCmptntAuthrty")]
	pub ntl_cmptnt_authrty: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<TradingVenueIdentification2>,
}


// TradingVenueIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueIdentification2 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Tp")]
	pub tp: String,
}


// TransparencyDataReport17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransparencyDataReport17 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "FinInstrmClssfctn")]
	pub fin_instrm_clssfctn: Option<String>,
	#[serde(rename = "FullNm")]
	pub full_nm: Option<String>,
	#[serde(rename = "TradgVn")]
	pub tradg_vn: Option<String>,
	#[serde(rename = "RptgPrd")]
	pub rptg_prd: Option<Period4Choice>,
	#[serde(rename = "Lqdty")]
	pub lqdty: Option<bool>,
	#[serde(rename = "Mthdlgy")]
	pub mthdlgy: Option<String>,
	#[serde(rename = "Sttstcs")]
	pub sttstcs: Option<StatisticsTransparency3>,
	#[serde(rename = "RlvntMkt")]
	pub rlvnt_mkt: Option<MarketDetail2>,
}


// TransparencyMethodology2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransparencyMethodology2Code {
	#[serde(rename = "TransparencyMethodology2Code")]
	pub transparency_methodology2_code: String,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}
