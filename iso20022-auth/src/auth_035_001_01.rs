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


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_code: String,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// FinancialInstrumentReportingTradingVolumeCapDataReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentReportingTradingVolumeCapDataReportV01 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: SecuritiesMarketReportHeader1,
	#[serde(rename = "VolCapData")]
	pub vol_cap_data: Vec<VolumeCapReport1>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "$value")]
	pub isin_oct2015_identifier: String,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}


// ImpliedCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "$value")]
	pub implied_currency_and_amount: f64,
}


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[serde(rename = "$value")]
	pub mic_identifier: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}


// Max50Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max50Text {
	#[serde(rename = "$value")]
	pub max50_text: String,
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
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
	pub to_dt: Option<String>,
	#[serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt_to_dt: Option<Period2>,
}


// SecuritiesMarketReportHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesMarketReportHeader1 {
	#[serde(rename = "RptgNtty")]
	pub rptg_ntty: TradingVenueIdentification1Choice,
	#[serde(rename = "RptgPrd")]
	pub rptg_prd: Period4Choice,
	#[serde(rename = "SubmissnDtTm", skip_serializing_if = "Option::is_none")]
	pub submissn_dt_tm: Option<String>,
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
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


// TradingVenueIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueIdentification2 {
	#[serde(rename = "Id")]
	pub id: Max50Text,
	#[serde(rename = "Tp")]
	pub tp: TradingVenue2Code,
}


// VolumeCapReport1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct VolumeCapReport1 {
	#[serde(rename = "RptgPrd", skip_serializing_if = "Option::is_none")]
	pub rptg_prd: Option<Period4Choice>,
	#[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
	pub tradg_vn: Option<MICIdentifier>,
	#[serde(rename = "InstrmRpt")]
	pub instrm_rpt: Vec<VolumeCapReport2>,
}


// VolumeCapReport2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct VolumeCapReport2 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max35Text>,
	#[serde(rename = "Id")]
	pub id: ISINOct2015Identifier,
	#[serde(rename = "Ccy")]
	pub ccy: ActiveOrHistoricCurrencyCode,
	#[serde(rename = "TtlTradgVol")]
	pub ttl_tradg_vol: f64,
	#[serde(rename = "TtlRefPricTradgVol")]
	pub ttl_ref_pric_tradg_vol: f64,
	#[serde(rename = "TtlNgtdTxsTradgVol")]
	pub ttl_ngtd_txs_tradg_vol: f64,
}
