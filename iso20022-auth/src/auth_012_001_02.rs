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
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAndAmount_SimpleType")]
	pub active_currency_and_amount_simple_type: f64,
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// BrokeredDeal1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BrokeredDeal1Code {
	#[serde(rename = "BrokeredDeal1Code")]
	pub brokered_deal1_code: String,
}


// CFIOct2015Identifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "CFIOct2015Identifier")]
	pub cfi_oct2015_identifier: String,
}


// Collateral18 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Collateral18 {
	#[serde(rename = "Valtn")]
	pub valtn: SecuredCollateral2Choice,
	#[serde(rename = "Hrcut")]
	pub hrcut: Option<f64>,
	#[serde(rename = "SpclCollInd")]
	pub spcl_coll_ind: Option<String>,
}


// CollateralPool1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralPool1Code {
	#[serde(rename = "CollateralPool1Code")]
	pub collateral_pool1_code: String,
}


// CollateralValuation6 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralValuation6 {
	#[serde(rename = "NmnlAmt")]
	pub nmnl_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "ISIN")]
	pub isin: String,
}


// CollateralValuation7 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralValuation7 {
	#[serde(rename = "PoolSts")]
	pub pool_sts: String,
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "Sctr")]
	pub sctr: String,
	#[serde(rename = "NmnlAmt")]
	pub nmnl_amt: Option<ActiveCurrencyAndAmount>,
}


// CounterpartyIdentification3Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyIdentification3Choice {
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "SctrAndLctn")]
	pub sctr_and_lctn: Option<SectorAndLocation1>,
	#[serde(rename = "NmAndLctn")]
	pub nm_and_lctn: Option<NameAndLocation1>,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DateAndDateTimeChoice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTimeChoice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<String>,
}


// DateTimePeriod1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}


// FloatingRateNote2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatingRateNote2 {
	#[serde(rename = "RefRateIndx")]
	pub ref_rate_indx: String,
	#[serde(rename = "BsisPtSprd")]
	pub bsis_pt_sprd: f64,
}


// ISINOct2015Identifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "ISINOct2015Identifier")]
	pub isin_oct2015_identifier: String,
}


// ISODate ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// InterestRateType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateType1Code {
	#[serde(rename = "InterestRateType1Code")]
	pub interest_rate_type1_code: String,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// Max105Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "Max105Text")]
	pub max105_text: String,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max70Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// MoneyMarketReportHeader1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MoneyMarketReportHeader1 {
	#[serde(rename = "RptgAgt")]
	pub rptg_agt: String,
	#[serde(rename = "RefPrd")]
	pub ref_prd: DateTimePeriod1,
}


// MoneyMarketSecuredMarketStatisticalReportV02 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MoneyMarketSecuredMarketStatisticalReportV02 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: MoneyMarketReportHeader1,
	#[serde(rename = "ScrdMktRpt")]
	pub scrd_mkt_rpt: SecuredMarketReport4Choice,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// MoneyMarketTransactionType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MoneyMarketTransactionType1Code {
	#[serde(rename = "MoneyMarketTransactionType1Code")]
	pub money_market_transaction_type1_code: String,
}


// NameAndLocation1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndLocation1 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Lctn")]
	pub lctn: String,
}


// NovationStatus1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NovationStatus1Code {
	#[serde(rename = "NovationStatus1Code")]
	pub novation_status1_code: String,
}


// Number ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// PercentageRate ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// ReportPeriodActivity3Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportPeriodActivity3Code {
	#[serde(rename = "ReportPeriodActivity3Code")]
	pub report_period_activity3_code: String,
}


// SNA2008SectorIdentifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SNA2008SectorIdentifier {
	#[serde(rename = "SNA2008SectorIdentifier")]
	pub sna2008_sector_identifier: String,
}


// SectorAndLocation1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SectorAndLocation1 {
	#[serde(rename = "Sctr")]
	pub sctr: String,
	#[serde(rename = "Lctn")]
	pub lctn: String,
}


// SecuredCollateral2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuredCollateral2Choice {
	#[serde(rename = "SnglColl")]
	pub sngl_coll: Option<CollateralValuation6>,
	#[serde(rename = "MltplColl")]
	pub mltpl_coll: Option<Vec<CollateralValuation6>>,
	#[serde(rename = "PoolColl")]
	pub pool_coll: Option<CollateralValuation6>,
	#[serde(rename = "OthrColl")]
	pub othr_coll: Option<Vec<CollateralValuation7>>,
}


// SecuredMarketReport4Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuredMarketReport4Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[serde(rename = "Tx")]
	pub tx: Option<Vec<SecuredMarketTransaction4>>,
}


// SecuredMarketTransaction4 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuredMarketTransaction4 {
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
	#[serde(rename = "TrptyAgtId")]
	pub trpty_agt_id: Option<String>,
	#[serde(rename = "TradDt")]
	pub trad_dt: DateAndDateTimeChoice,
	#[serde(rename = "SttlmDt")]
	pub sttlm_dt: String,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: String,
	#[serde(rename = "TxTp")]
	pub tx_tp: String,
	#[serde(rename = "TxNmnlAmt")]
	pub tx_nmnl_amt: ActiveCurrencyAndAmount,
	#[serde(rename = "RateTp")]
	pub rate_tp: String,
	#[serde(rename = "DealRate")]
	pub deal_rate: Option<f64>,
	#[serde(rename = "FltgRateRpAgrmt")]
	pub fltg_rate_rp_agrmt: Option<FloatingRateNote2>,
	#[serde(rename = "BrkrdDeal")]
	pub brkrd_deal: Option<String>,
	#[serde(rename = "Coll")]
	pub coll: Collateral18,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SpecialCollateral2Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SpecialCollateral2Code {
	#[serde(rename = "SpecialCollateral2Code")]
	pub special_collateral2_code: String,
}


// SupplementaryData1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// TransactionOperationType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionOperationType1Code {
	#[serde(rename = "TransactionOperationType1Code")]
	pub transaction_operation_type1_code: String,
}
