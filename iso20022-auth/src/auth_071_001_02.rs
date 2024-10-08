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


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAndAmount_SimpleType")]
	pub active_or_historic_currency_and_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAndAmount ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// AmountAndDirection53 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection53 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "Sgn")]
	pub sgn: Option<bool>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// CashReuseData1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashReuseData1 {
	#[serde(rename = "RinvstdCsh")]
	pub rinvstd_csh: Vec<ReinvestedCashTypeAndAmount1>,
	#[serde(rename = "CshRinvstmtRate")]
	pub csh_rinvstmt_rate: f64,
}


// CollateralType19 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralType19 {
	#[serde(rename = "Scty")]
	pub scty: Option<Vec<SecurityReuseData1>>,
	#[serde(rename = "Csh")]
	pub csh: Option<Vec<CashReuseData1>>,
}


// CounterpartyData87 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyData87 {
	#[serde(rename = "RptSubmitgNtty")]
	pub rpt_submitg_ntty: OrganisationIdentification15Choice,
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: OrganisationIdentification15Choice,
	#[serde(rename = "NttyRspnsblForRpt")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
}


// FundingSource3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundingSource3 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "MktVal")]
	pub mkt_val: AmountAndDirection53,
}


// FundingSourceType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundingSourceType1Code {
	#[serde(rename = "FundingSourceType1Code")]
	pub funding_source_type1_code: String,
}


// GenericIdentification175 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification175 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
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


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max500Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max500Text {
	#[serde(rename = "Max500Text")]
	pub max500_text: String,
}


// Max72Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max72Text {
	#[serde(rename = "Max72Text")]
	pub max72_text: String,
}


// OrganisationIdentification15Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification15Choice {
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<OrganisationIdentification38>,
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
}


// OrganisationIdentification38 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification38 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Dmcl")]
	pub dmcl: Option<String>,
}


// PercentageRate ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PlusOrMinusIndicator ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "PlusOrMinusIndicator")]
	pub plus_or_minus_indicator: bool,
}


// ReinvestedCashTypeAndAmount1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReinvestedCashTypeAndAmount1 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "RinvstdCshAmt")]
	pub rinvstd_csh_amt: ActiveOrHistoricCurrencyAndAmount,
}


// ReinvestmentType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReinvestmentType1Code {
	#[serde(rename = "ReinvestmentType1Code")]
	pub reinvestment_type1_code: String,
}


// ReportPeriodActivity1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportPeriodActivity1Code {
	#[serde(rename = "ReportPeriodActivity1Code")]
	pub report_period_activity1_code: String,
}


// ReuseDataReport6Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReuseDataReport6Choice {
	#[serde(rename = "New")]
	pub new: Option<ReuseDataReportNew6>,
	#[serde(rename = "Err")]
	pub err: Option<ReuseDataReportError5>,
	#[serde(rename = "Crrctn")]
	pub crrctn: Option<ReuseDataReportCorrection14>,
	#[serde(rename = "CollReuseUpd")]
	pub coll_reuse_upd: Option<ReuseDataReportCorrection14>,
}


// ReuseDataReportCorrection14 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReuseDataReportCorrection14 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[serde(rename = "RptgDtTm")]
	pub rptg_dt_tm: String,
	#[serde(rename = "CtrPty")]
	pub ctr_pty: CounterpartyData87,
	#[serde(rename = "CollCmpnt")]
	pub coll_cmpnt: Option<Vec<CollateralType19>>,
	#[serde(rename = "EvtDay")]
	pub evt_day: String,
	#[serde(rename = "FndgSrc")]
	pub fndg_src: Option<Vec<FundingSource3>>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// ReuseDataReportError5 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReuseDataReportError5 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[serde(rename = "RptgDtTm")]
	pub rptg_dt_tm: String,
	#[serde(rename = "CtrPty")]
	pub ctr_pty: CounterpartyData87,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// ReuseDataReportNew6 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReuseDataReportNew6 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[serde(rename = "RptgDtTm")]
	pub rptg_dt_tm: String,
	#[serde(rename = "CtrPty")]
	pub ctr_pty: CounterpartyData87,
	#[serde(rename = "CollCmpnt")]
	pub coll_cmpnt: Option<Vec<CollateralType19>>,
	#[serde(rename = "EvtDay")]
	pub evt_day: String,
	#[serde(rename = "FndgSrc")]
	pub fndg_src: Option<Vec<FundingSource3>>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// ReuseValue1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReuseValue1Choice {
	#[serde(rename = "Actl")]
	pub actl: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Estmtd")]
	pub estmtd: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// SecuritiesFinancingReportingTransactionReusedCollateralDataReportV02 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesFinancingReportingTransactionReusedCollateralDataReportV02 {
	#[serde(rename = "TradData")]
	pub trad_data: TradeData36Choice,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SecurityReuseData1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityReuseData1 {
	#[serde(rename = "ISIN")]
	pub isin: String,
	#[serde(rename = "ReuseVal")]
	pub reuse_val: ReuseValue1Choice,
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


// TradeData36Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeData36Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[serde(rename = "Rpt")]
	pub rpt: Option<Vec<ReuseDataReport6Choice>>,
}
