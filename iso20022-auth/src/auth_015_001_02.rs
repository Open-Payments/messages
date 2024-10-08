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


// CounterpartyIdentification3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyIdentification3Choice {
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "SctrAndLctn")]
	pub sctr_and_lctn: Option<SectorAndLocation1>,
	#[serde(rename = "NmAndLctn")]
	pub nm_and_lctn: Option<NameAndLocation1>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DateAndDateTimeChoice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTimeChoice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm")]
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


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "Max105Text")]
	pub max105_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// MoneyMarketOvernightIndexSwapsStatisticalReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MoneyMarketOvernightIndexSwapsStatisticalReportV02 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: MoneyMarketReportHeader1,
	#[serde(rename = "OvrnghtIndxSwpsRpt")]
	pub ovrnght_indx_swps_rpt: OvernightIndexSwap4Choice,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// MoneyMarketReportHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MoneyMarketReportHeader1 {
	#[serde(rename = "RptgAgt")]
	pub rptg_agt: String,
	#[serde(rename = "RefPrd")]
	pub ref_prd: DateTimePeriod1,
}


// NameAndLocation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndLocation1 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Lctn")]
	pub lctn: String,
}


// NovationStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NovationStatus1Code {
	#[serde(rename = "NovationStatus1Code")]
	pub novation_status1_code: String,
}


// OvernightIndexSwap4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OvernightIndexSwap4Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[serde(rename = "Tx")]
	pub tx: Option<Vec<OvernightIndexSwapTransaction4>>,
}


// OvernightIndexSwapTransaction4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OvernightIndexSwapTransaction4 {
	#[serde(rename = "RptdTxSts")]
	pub rptd_tx_sts: String,
	#[serde(rename = "NvtnSts")]
	pub nvtn_sts: Option<String>,
	#[serde(rename = "BrnchId")]
	pub brnch_id: Option<String>,
	#[serde(rename = "UnqTxIdr")]
	pub unq_tx_idr: Option<String>,
	#[serde(rename = "PrtryTxId")]
	pub prtry_tx_id: String,
	#[serde(rename = "RltdPrtryTxId")]
	pub rltd_prtry_tx_id: Option<String>,
	#[serde(rename = "CtrPtyPrtryTxId")]
	pub ctr_pty_prtry_tx_id: Option<String>,
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: CounterpartyIdentification3Choice,
	#[serde(rename = "TradDt")]
	pub trad_dt: DateAndDateTimeChoice,
	#[serde(rename = "StartDt")]
	pub start_dt: String,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: String,
	#[serde(rename = "FxdIntrstRate")]
	pub fxd_intrst_rate: f64,
	#[serde(rename = "TxTp")]
	pub tx_tp: String,
	#[serde(rename = "TxNmnlAmt")]
	pub tx_nmnl_amt: ActiveCurrencyAndAmount,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// OvernightIndexSwapType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OvernightIndexSwapType1Code {
	#[serde(rename = "OvernightIndexSwapType1Code")]
	pub overnight_index_swap_type1_code: String,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// ReportPeriodActivity3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportPeriodActivity3Code {
	#[serde(rename = "ReportPeriodActivity3Code")]
	pub report_period_activity3_code: String,
}


// SNA2008SectorIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SNA2008SectorIdentifier {
	#[serde(rename = "SNA2008SectorIdentifier")]
	pub sna2008_sector_identifier: String,
}


// SectorAndLocation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SectorAndLocation1 {
	#[serde(rename = "Sctr")]
	pub sctr: String,
	#[serde(rename = "Lctn")]
	pub lctn: String,
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


// TransactionOperationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionOperationType1Code {
	#[serde(rename = "TransactionOperationType1Code")]
	pub transaction_operation_type1_code: String,
}
