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
use serde_valid::Validate;


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAndAmount_SimpleType")]
	pub active_or_historic_currency_and_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAndAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[validate(pattern = "[A-Z]{3,3}")]
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// AmountAndDirection53 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AmountAndDirection53 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "Sgn")]
	pub sgn: Option<bool>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[validate(pattern = "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}")]
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// CashReuseData1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CashReuseData1 {
	#[validate]
	#[serde(rename = "RinvstdCsh")]
	pub rinvstd_csh: Vec<ReinvestedCashTypeAndAmount1>,
	#[serde(rename = "CshRinvstmtRate")]
	pub csh_rinvstmt_rate: f64,
}


// CollateralType19 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CollateralType19 {
	#[validate]
	#[serde(rename = "Scty")]
	pub scty: Option<Vec<SecurityReuseData1>>,
	#[validate]
	#[serde(rename = "Csh")]
	pub csh: Option<Vec<CashReuseData1>>,
}


// ContractModification3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ContractModification3 {
	#[serde(rename = "ActnTp")]
	pub actn_tp: String,
	#[serde(rename = "Lvl")]
	pub lvl: Option<String>,
}


// CounterpartyData87 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CounterpartyData87 {
	#[validate]
	#[serde(rename = "RptSubmitgNtty")]
	pub rpt_submitg_ntty: OrganisationIdentification15Choice,
	#[validate]
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: OrganisationIdentification15Choice,
	#[validate]
	#[serde(rename = "NttyRspnsblForRpt")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
}


// FundingSource3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FundingSource3 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[validate]
	#[serde(rename = "MktVal")]
	pub mkt_val: AmountAndDirection53,
}


// FundingSourceType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FundingSourceType1Code {
	#[validate(enumerate = ["SECL", "FREE", "OTHR", "BSHS", "CSHS", "REPO", "UBOR"])]
	#[serde(rename = "FundingSourceType1Code")]
	pub funding_source_type1_code: String,
}


// GenericIdentification175 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification175 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// ISINOct2015Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[validate(pattern = "[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}")]
	#[serde(rename = "ISINOct2015Identifier")]
	pub isin_oct2015_identifier: String,
}


// ISODate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[validate(pattern = "[A-Z0-9]{18,18}[0-9]{2,2}")]
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// Max105Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max105Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 105)]
	#[serde(rename = "Max105Text")]
	pub max105_text: String,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max140Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 140)]
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max350Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 350)]
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max35Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max500Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max500Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 500)]
	#[serde(rename = "Max500Text")]
	pub max500_text: String,
}


// Max72Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max72Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 72)]
	#[serde(rename = "Max72Text")]
	pub max72_text: String,
}


// ModificationLevel1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ModificationLevel1Code {
	#[validate(enumerate = ["PSTN", "TCTN"])]
	#[serde(rename = "ModificationLevel1Code")]
	pub modification_level1_code: String,
}


// OrganisationIdentification15Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OrganisationIdentification15Choice {
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<OrganisationIdentification38>,
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
}


// OrganisationIdentification38 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OrganisationIdentification38 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Dmcl")]
	pub dmcl: Option<String>,
}


// PercentageRate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PlusOrMinusIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "PlusOrMinusIndicator")]
	pub plus_or_minus_indicator: bool,
}


// ReconciliationFlag2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReconciliationFlag2 {
	#[serde(rename = "RptTp")]
	pub rpt_tp: Option<String>,
	#[serde(rename = "BothCtrPtiesRptg")]
	pub both_ctr_pties_rptg: Option<bool>,
	#[serde(rename = "PairdSts")]
	pub paird_sts: Option<bool>,
	#[serde(rename = "LnRcncltnSts")]
	pub ln_rcncltn_sts: Option<bool>,
	#[serde(rename = "CollRcncltnSts")]
	pub coll_rcncltn_sts: Option<bool>,
	#[serde(rename = "ModSts")]
	pub mod_sts: Option<bool>,
}


// ReinvestedCashTypeAndAmount1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReinvestedCashTypeAndAmount1 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[validate]
	#[serde(rename = "RinvstdCshAmt")]
	pub rinvstd_csh_amt: ActiveOrHistoricCurrencyAndAmount,
}


// ReinvestmentType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReinvestmentType1Code {
	#[validate(enumerate = ["OTHR", "OCMP", "MMFT", "REPM", "SDPU"])]
	#[serde(rename = "ReinvestmentType1Code")]
	pub reinvestment_type1_code: String,
}


// ReportPeriodActivity1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReportPeriodActivity1Code {
	#[validate(enumerate = ["NOTX"])]
	#[serde(rename = "ReportPeriodActivity1Code")]
	pub report_period_activity1_code: String,
}


// ReuseDataReportCorrection15 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReuseDataReportCorrection15 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[validate]
	#[serde(rename = "CtrPty")]
	pub ctr_pty: CounterpartyData87,
	#[validate]
	#[serde(rename = "CollCmpnt")]
	pub coll_cmpnt: Option<Vec<CollateralType19>>,
	#[serde(rename = "EvtDay")]
	pub evt_day: String,
	#[serde(rename = "RptgDtTm")]
	pub rptg_dt_tm: String,
	#[validate]
	#[serde(rename = "FndgSrc")]
	pub fndg_src: Option<Vec<FundingSource3>>,
	#[validate]
	#[serde(rename = "RcncltnFlg")]
	pub rcncltn_flg: Option<ReconciliationFlag2>,
	#[validate]
	#[serde(rename = "CtrctMod")]
	pub ctrct_mod: ContractModification3,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// ReuseValue1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReuseValue1Choice {
	#[validate]
	#[serde(rename = "Actl")]
	pub actl: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "Estmtd")]
	pub estmtd: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// SecuritiesFinancingReportingReusedCollateralDataTransactionStateReportV02 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesFinancingReportingReusedCollateralDataTransactionStateReportV02 {
	#[validate]
	#[serde(rename = "TradData")]
	pub trad_data: TradeData37Choice,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SecurityReuseData1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityReuseData1 {
	#[serde(rename = "ISIN")]
	pub isin: String,
	#[validate]
	#[serde(rename = "ReuseVal")]
	pub reuse_val: ReuseValue1Choice,
}


// SupplementaryData1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[validate]
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// TradeData37Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeData37Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[validate]
	#[serde(rename = "Stat")]
	pub stat: Option<Vec<ReuseDataReportCorrection15>>,
}


// TradeRepositoryReportingType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeRepositoryReportingType1Code {
	#[validate(enumerate = ["SWOS", "TWOS"])]
	#[serde(rename = "TradeRepositoryReportingType1Code")]
	pub trade_repository_reporting_type1_code: String,
}


// TransactionOperationType6Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TransactionOperationType6Code {
	#[validate(enumerate = ["REUU", "COLU", "CORR", "ETRM", "VALU", "POSC", "NEWT", "MODI", "MARU", "EROR"])]
	#[serde(rename = "TransactionOperationType6Code")]
	pub transaction_operation_type6_code: String,
}


// TrueFalseIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}
