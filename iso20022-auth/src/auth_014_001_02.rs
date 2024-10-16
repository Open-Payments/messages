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
	#[serde(rename = "$value")]
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
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_code: String,
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
}


// CounterpartyIdentification3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyIdentification3Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "SctrAndLctn", skip_serializing_if = "Option::is_none")]
	pub sctr_and_lctn: Option<SectorAndLocation1>,
	#[serde(rename = "NmAndLctn", skip_serializing_if = "Option::is_none")]
	pub nm_and_lctn: Option<NameAndLocation1>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// DateAndDateTimeChoice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTimeChoice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}


// DateTimePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}


// ForeignExchange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ForeignExchange1 {
	#[serde(rename = "FrgnCcy")]
	pub frgn_ccy: ActiveOrHistoricCurrencyCode,
	#[serde(rename = "XchgSpotRate")]
	pub xchg_spot_rate: f64,
	#[serde(rename = "XchgFwdPt")]
	pub xchg_fwd_pt: f64,
}


// ForeignExchangeSwap3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ForeignExchangeSwap3Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity3Code>,
	#[serde(rename = "Tx", skip_serializing_if = "Option::is_none")]
	pub tx: Option<Vec<ForeignExchangeSwapTransaction3>>,
}


// ForeignExchangeSwapTransaction3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ForeignExchangeSwapTransaction3 {
	#[serde(rename = "RptdTxSts")]
	pub rptd_tx_sts: TransactionOperationType1Code,
	#[serde(rename = "NvtnSts", skip_serializing_if = "Option::is_none")]
	pub nvtn_sts: Option<NovationStatus1Code>,
	#[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
	pub brnch_id: Option<LEIIdentifier>,
	#[serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none")]
	pub unq_tx_idr: Option<Max105Text>,
	#[serde(rename = "PrtryTxId")]
	pub prtry_tx_id: Max105Text,
	#[serde(rename = "RltdPrtryTxId", skip_serializing_if = "Option::is_none")]
	pub rltd_prtry_tx_id: Option<Max105Text>,
	#[serde(rename = "CtrPtyPrtryTxId", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_prtry_tx_id: Option<Max105Text>,
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: CounterpartyIdentification3Choice,
	#[serde(rename = "TradDt")]
	pub trad_dt: DateAndDateTimeChoice,
	#[serde(rename = "SpotValDt")]
	pub spot_val_dt: String,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: String,
	#[serde(rename = "TxTp")]
	pub tx_tp: SecuritiesTransactionType15Code,
	#[serde(rename = "TxNmnlAmt")]
	pub tx_nmnl_amt: ActiveCurrencyAndAmount,
	#[serde(rename = "FX")]
	pub fx: ForeignExchange1,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
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


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "$value")]
	pub max105_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}


// MoneyMarketForeignExchangeSwapsStatisticalReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MoneyMarketForeignExchangeSwapsStatisticalReportV02 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: MoneyMarketReportHeader1,
	#[serde(rename = "FXSwpsRpt")]
	pub fx_swps_rpt: ForeignExchangeSwap3Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// MoneyMarketReportHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MoneyMarketReportHeader1 {
	#[serde(rename = "RptgAgt")]
	pub rptg_agt: LEIIdentifier,
	#[serde(rename = "RefPrd")]
	pub ref_prd: DateTimePeriod1,
}


// NameAndLocation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndLocation1 {
	#[serde(rename = "Nm")]
	pub nm: Max70Text,
	#[serde(rename = "Lctn")]
	pub lctn: CountryCode,
}


// NovationStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NovationStatus1Code {
	#[default]
	#[serde(rename = "NONO")]
	CodeNONO,
	#[serde(rename = "NOVA")]
	CodeNOVA,
}


// ReportPeriodActivity3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportPeriodActivity3Code {
	#[default]
	#[serde(rename = "NOTX")]
	CodeNOTX,
	#[serde(rename = "NORA")]
	CodeNORA,
}


// SNA2008SectorIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SNA2008SectorIdentifier {
	#[serde(rename = "$value")]
	pub sna2008_sector_identifier: String,
}


// SectorAndLocation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SectorAndLocation1 {
	#[serde(rename = "Sctr")]
	pub sctr: String,
	#[serde(rename = "Lctn")]
	pub lctn: CountryCode,
}


// SecuritiesTransactionType15Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SecuritiesTransactionType15Code {
	#[default]
	#[serde(rename = "BUYI")]
	CodeBUYI,
	#[serde(rename = "SELL")]
	CodeSELL,
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


// TransactionOperationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionOperationType1Code {
	#[default]
	#[serde(rename = "AMND")]
	CodeAMND,
	#[serde(rename = "CANC")]
	CodeCANC,
	#[serde(rename = "CORR")]
	CodeCORR,
	#[serde(rename = "NEWT")]
	CodeNEWT,
}
