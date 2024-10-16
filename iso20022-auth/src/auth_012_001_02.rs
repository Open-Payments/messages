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


// BrokeredDeal1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BrokeredDeal1Code {
	#[default]
	#[serde(rename = "BILA")]
	CodeBILA,
	#[serde(rename = "BROK")]
	CodeBROK,
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "$value")]
	pub cfi_oct2015_identifier: String,
}


// Collateral18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Collateral18 {
	#[serde(rename = "Valtn")]
	pub valtn: SecuredCollateral2Choice,
	#[serde(rename = "Hrcut", skip_serializing_if = "Option::is_none")]
	pub hrcut: Option<f64>,
	#[serde(rename = "SpclCollInd", skip_serializing_if = "Option::is_none")]
	pub spcl_coll_ind: Option<SpecialCollateral2Code>,
}


// CollateralPool1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CollateralPool1Code {
	#[default]
	#[serde(rename = "NOPL")]
	CodeNOPL,
	#[serde(rename = "POOL")]
	CodePOOL,
}


// CollateralValuation6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralValuation6 {
	#[serde(rename = "NmnlAmt", skip_serializing_if = "Option::is_none")]
	pub nmnl_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "ISIN")]
	pub isin: ISINOct2015Identifier,
}


// CollateralValuation7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralValuation7 {
	#[serde(rename = "PoolSts")]
	pub pool_sts: CollateralPool1Code,
	#[serde(rename = "Tp")]
	pub tp: CFIOct2015Identifier,
	#[serde(rename = "Sctr")]
	pub sctr: String,
	#[serde(rename = "NmnlAmt", skip_serializing_if = "Option::is_none")]
	pub nmnl_amt: Option<ActiveCurrencyAndAmount>,
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


// FloatingRateNote2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatingRateNote2 {
	#[serde(rename = "RefRateIndx")]
	pub ref_rate_indx: ISINOct2015Identifier,
	#[serde(rename = "BsisPtSprd")]
	pub bsis_pt_sprd: f64,
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


// InterestRateType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InterestRateType1Code {
	#[default]
	#[serde(rename = "FIXE")]
	CodeFIXE,
	#[serde(rename = "VARI")]
	CodeVARI,
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


// MoneyMarketReportHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MoneyMarketReportHeader1 {
	#[serde(rename = "RptgAgt")]
	pub rptg_agt: LEIIdentifier,
	#[serde(rename = "RefPrd")]
	pub ref_prd: DateTimePeriod1,
}


// MoneyMarketSecuredMarketStatisticalReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MoneyMarketSecuredMarketStatisticalReportV02 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: MoneyMarketReportHeader1,
	#[serde(rename = "ScrdMktRpt")]
	pub scrd_mkt_rpt: SecuredMarketReport4Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// MoneyMarketTransactionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum MoneyMarketTransactionType1Code {
	#[default]
	#[serde(rename = "BORR")]
	CodeBORR,
	#[serde(rename = "LEND")]
	CodeLEND,
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


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
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


// SecuredCollateral2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuredCollateral2Choice {
	#[serde(rename = "SnglColl", skip_serializing_if = "Option::is_none")]
	pub sngl_coll: Option<CollateralValuation6>,
	#[serde(rename = "MltplColl", skip_serializing_if = "Option::is_none")]
	pub mltpl_coll: Option<Vec<CollateralValuation6>>,
	#[serde(rename = "PoolColl", skip_serializing_if = "Option::is_none")]
	pub pool_coll: Option<CollateralValuation6>,
	#[serde(rename = "OthrColl", skip_serializing_if = "Option::is_none")]
	pub othr_coll: Option<Vec<CollateralValuation7>>,
}


// SecuredMarketReport4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuredMarketReport4Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity3Code>,
	#[serde(rename = "Tx", skip_serializing_if = "Option::is_none")]
	pub tx: Option<Vec<SecuredMarketTransaction4>>,
}


// SecuredMarketTransaction4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuredMarketTransaction4 {
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
	#[serde(rename = "TrptyAgtId", skip_serializing_if = "Option::is_none")]
	pub trpty_agt_id: Option<LEIIdentifier>,
	#[serde(rename = "TradDt")]
	pub trad_dt: DateAndDateTimeChoice,
	#[serde(rename = "SttlmDt")]
	pub sttlm_dt: String,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: String,
	#[serde(rename = "TxTp")]
	pub tx_tp: MoneyMarketTransactionType1Code,
	#[serde(rename = "TxNmnlAmt")]
	pub tx_nmnl_amt: ActiveCurrencyAndAmount,
	#[serde(rename = "RateTp")]
	pub rate_tp: InterestRateType1Code,
	#[serde(rename = "DealRate", skip_serializing_if = "Option::is_none")]
	pub deal_rate: Option<f64>,
	#[serde(rename = "FltgRateRpAgrmt", skip_serializing_if = "Option::is_none")]
	pub fltg_rate_rp_agrmt: Option<FloatingRateNote2>,
	#[serde(rename = "BrkrdDeal", skip_serializing_if = "Option::is_none")]
	pub brkrd_deal: Option<BrokeredDeal1Code>,
	#[serde(rename = "Coll")]
	pub coll: Collateral18,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SpecialCollateral2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SpecialCollateral2Code {
	#[default]
	#[serde(rename = "GENE")]
	CodeGENE,
	#[serde(rename = "SPEC")]
	CodeSPEC,
	#[serde(rename = "MRRP")]
	CodeMRRP,
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
