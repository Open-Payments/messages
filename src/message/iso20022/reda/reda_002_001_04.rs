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


// ActiveCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAnd13DecimalAmount_SimpleType")]
	pub active_currency_and13_decimal_amount_simple_type: f64,
}


// ActiveCurrencyAnd13DecimalAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[validate(pattern = "[A-Z]{3,3}")]
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAnd13DecimalAmount_SimpleType")]
	pub active_or_historic_currency_and13_decimal_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAnd13DecimalAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


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


// AdditionalReference3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AdditionalReference3 {
	#[serde(rename = "Ref")]
	pub ref_attr: String,
	#[validate]
	#[serde(rename = "RefIssr")]
	pub ref_issr: Option<PartyIdentification2Choice>,
	#[serde(rename = "MsgNm")]
	pub msg_nm: Option<String>,
}


// AddressType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[validate(enumerate = ["ADDR", "PBOX", "HOME", "BIZZ", "MLTO", "DLVY"])]
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AlternateSecurityIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AlternateSecurityIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "DmstIdSrc")]
	pub dmst_id_src: Option<String>,
	#[serde(rename = "PrtryIdSrc")]
	pub prtry_id_src: Option<String>,
}


// AnyBICIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AnyBICIdentifier {
	#[validate(pattern = "[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}")]
	#[serde(rename = "AnyBICIdentifier")]
	pub any_bic_identifier: String,
}


// BelgianIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BelgianIdentifier {
	#[serde(rename = "BelgianIdentifier")]
	pub belgian_identifier: String,
}


// BloombergIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BloombergIdentifier {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "BloombergIdentifier")]
	pub bloomberg_identifier: String,
}


// CUSIPIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CUSIPIdentifier {
	#[serde(rename = "CUSIPIdentifier")]
	pub cusip_identifier: String,
}


// CalculationBasis2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CalculationBasis2Code {
	#[validate(enumerate = ["AVER", "DAIL", "MNTH", "YEAR"])]
	#[serde(rename = "CalculationBasis2Code")]
	pub calculation_basis2_code: String,
}


// Charge15 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Charge15 {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "XtndedTp")]
	pub xtnded_tp: Option<String>,
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
	#[serde(rename = "ClctnBsis")]
	pub clctn_bsis: Option<String>,
	#[serde(rename = "XtndedClctnBsis")]
	pub xtnded_clctn_bsis: Option<String>,
}


// ChargeType9Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ChargeType9Code {
	#[validate(enumerate = ["MANF", "BEND", "FEND", "ADVI", "CUST", "PUBL", "ACCT", "EQUL", "PENA"])]
	#[serde(rename = "ChargeType9Code")]
	pub charge_type9_code: String,
}


// ConsolidatedTapeAssociationIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ConsolidatedTapeAssociationIdentifier {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "ConsolidatedTapeAssociationIdentifier")]
	pub consolidated_tape_association_identifier: String,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CountryCode {
	#[validate(pattern = "[A-Z]{2,2}")]
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DateAndDateTime1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateAndDateTime1Choice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<String>,
}


// DateAndDateTimeChoice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateAndDateTimeChoice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<String>,
}


// DateOrDateTimePeriodChoice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateOrDateTimePeriodChoice {
	#[validate]
	#[serde(rename = "Dt")]
	pub dt: Option<DatePeriodDetails>,
	#[validate]
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<DateTimePeriodDetails>,
}


// DatePeriodDetails ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DatePeriodDetails {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// DateTimePeriodDetails ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateTimePeriodDetails {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}


// DecimalNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// DistributionPolicy1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DistributionPolicy1Code {
	#[validate(enumerate = ["DIST", "ACCU"])]
	#[serde(rename = "DistributionPolicy1Code")]
	pub distribution_policy1_code: String,
}


// DutchIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DutchIdentifier {
	#[serde(rename = "DutchIdentifier")]
	pub dutch_identifier: String,
}


// EUCapitalGain2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EUCapitalGain2Code {
	#[validate(enumerate = ["EUSI", "EUSO", "UKWN"])]
	#[serde(rename = "EUCapitalGain2Code")]
	pub eu_capital_gain2_code: String,
}


// EUDividendStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EUDividendStatus1Code {
	#[validate(enumerate = ["DIVI", "DIVO", "UKWN"])]
	#[serde(rename = "EUDividendStatus1Code")]
	pub eu_dividend_status1_code: String,
}


// EuroclearClearstreamIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EuroclearClearstreamIdentifier {
	#[validate(min_length = 1)]
	#[validate(max_length = 12)]
	#[serde(rename = "EuroclearClearstreamIdentifier")]
	pub euroclear_clearstream_identifier: String,
}


// EventFrequency1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EventFrequency1Code {
	#[validate(enumerate = ["YEAR", "SEMI", "QUTR", "TOMN", "MNTH", "TWMN", "TOWK", "WEEK", "DAIL", "ADHO", "INDA", "OVNG", "ONDE"])]
	#[serde(rename = "EventFrequency1Code")]
	pub event_frequency1_code: String,
}


// Extended350Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Extended350Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 350)]
	#[serde(rename = "Extended350Code")]
	pub extended350_code: String,
}


// Extension1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Extension1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: String,
	#[serde(rename = "Txt")]
	pub txt: String,
}


// FinancialInstrument8 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrument8 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: Vec<SecurityIdentification3Choice>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "SplmtryId")]
	pub splmtry_id: Option<String>,
	#[serde(rename = "DnmtnCcy")]
	pub dnmtn_ccy: Option<String>,
	#[serde(rename = "ClssTp")]
	pub clss_tp: Option<String>,
	#[serde(rename = "SctiesForm")]
	pub scties_form: Option<String>,
	#[serde(rename = "DstrbtnPlcy")]
	pub dstrbtn_plcy: Option<String>,
	#[serde(rename = "DualFndInd")]
	pub dual_fnd_ind: bool,
}


// FinancialInstrumentQuantity1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrumentQuantity1 {
	#[serde(rename = "Unit")]
	pub unit: f64,
}


// FormOfSecurity1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FormOfSecurity1Code {
	#[validate(enumerate = ["BEAR", "REGD"])]
	#[serde(rename = "FormOfSecurity1Code")]
	pub form_of_security1_code: String,
}


// GenericIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// ISINIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISINIdentifier {
	#[validate(pattern = "[A-Z0-9]{12,12}")]
	#[serde(rename = "ISINIdentifier")]
	pub isin_identifier: String,
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


// Max16Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max16Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 16)]
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
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


// Max5NumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max5NumericText {
	#[validate(pattern = "[0-9]{1,5}")]
	#[serde(rename = "Max5NumericText")]
	pub max5_numeric_text: String,
}


// Max70Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max70Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 70)]
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// MessageIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MessageIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
}


// NameAndAddress5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[validate]
	#[serde(rename = "Adr")]
	pub adr: Option<PostalAddress1>,
}


// Number ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// Pagination ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Pagination {
	#[serde(rename = "PgNb")]
	pub pg_nb: String,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}


// PartyIdentification2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification2Choice {
	#[serde(rename = "BICOrBEI")]
	pub bic_or_bei: Option<String>,
	#[validate]
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<GenericIdentification1>,
	#[validate]
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PercentageRate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PerformanceFactors1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PerformanceFactors1 {
	#[serde(rename = "CorpActnFctr")]
	pub corp_actn_fctr: Option<f64>,
	#[serde(rename = "CmltvCorpActnFctr")]
	pub cmltv_corp_actn_fctr: Option<f64>,
	#[validate]
	#[serde(rename = "AcmltnPrd")]
	pub acmltn_prd: Option<DatePeriodDetails>,
	#[serde(rename = "NrmlPrfrmnc")]
	pub nrml_prfrmnc: Option<f64>,
}


// PlusOrMinusIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "PlusOrMinusIndicator")]
	pub plus_or_minus_indicator: bool,
}


// PostalAddress1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: Option<String>,
	#[serde(rename = "AdrLine")]
	pub adr_line: Option<Vec<String>>,
	#[serde(rename = "StrtNm")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "PstCd")]
	pub pst_cd: Option<String>,
	#[serde(rename = "TwnNm")]
	pub twn_nm: Option<String>,
	#[serde(rename = "CtrySubDvsn")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "Ctry")]
	pub ctry: String,
}


// PriceMethod1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PriceMethod1Code {
	#[validate(enumerate = ["FORW", "HIST"])]
	#[serde(rename = "PriceMethod1Code")]
	pub price_method1_code: String,
}


// PriceReport3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PriceReport3 {
	#[validate]
	#[serde(rename = "PricValtnDtls")]
	pub pric_valtn_dtls: Vec<PriceValuation4>,
}


// PriceReportCancellationV04 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PriceReportCancellationV04 {
	#[validate]
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[validate]
	#[serde(rename = "PoolRef")]
	pub pool_ref: Option<AdditionalReference3>,
	#[validate]
	#[serde(rename = "PrvsRef")]
	pub prvs_ref: Option<AdditionalReference3>,
	#[validate]
	#[serde(rename = "MsgPgntn")]
	pub msg_pgntn: Pagination,
	#[serde(rename = "PricRptId")]
	pub pric_rpt_id: String,
	#[serde(rename = "CxlId")]
	pub cxl_id: String,
	#[serde(rename = "CxlRsn")]
	pub cxl_rsn: Option<String>,
	#[validate]
	#[serde(rename = "XpctdPricCrrctnDt")]
	pub xpctd_pric_crrctn_dt: Option<DateAndDateTime1Choice>,
	#[serde(rename = "CmpltPricCxl")]
	pub cmplt_pric_cxl: bool,
	#[validate]
	#[serde(rename = "CancPricValtnDtls")]
	pub canc_pric_valtn_dtls: Option<Vec<PriceReport3>>,
	#[validate]
	#[serde(rename = "Xtnsn")]
	pub xtnsn: Option<Vec<Extension1>>,
}


// PriceType2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PriceType2 {
	#[serde(rename = "Strd")]
	pub strd: String,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// PriceValuation4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PriceValuation4 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[validate]
	#[serde(rename = "ValtnDtTm")]
	pub valtn_dt_tm: Option<DateAndDateTimeChoice>,
	#[validate]
	#[serde(rename = "NAVDtTm")]
	pub nav_dt_tm: DateAndDateTimeChoice,
	#[validate]
	#[serde(rename = "FinInstrmDtls")]
	pub fin_instrm_dtls: FinancialInstrument8,
	#[validate]
	#[serde(rename = "FndMgmtCpny")]
	pub fnd_mgmt_cpny: Option<PartyIdentification2Choice>,
	#[validate]
	#[serde(rename = "TtlNAV")]
	pub ttl_nav: Option<Vec<ActiveOrHistoricCurrencyAndAmount>>,
	#[validate]
	#[serde(rename = "TtlUnitsNb")]
	pub ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[validate]
	#[serde(rename = "NxtValtnDtTm")]
	pub nxt_valtn_dt_tm: Option<DateAndDateTimeChoice>,
	#[validate]
	#[serde(rename = "PrvsValtnDtTm")]
	pub prvs_valtn_dt_tm: Option<DateAndDateTimeChoice>,
	#[serde(rename = "ValtnTp")]
	pub valtn_tp: String,
	#[serde(rename = "ValtnFrqcy")]
	pub valtn_frqcy: Option<String>,
	#[serde(rename = "OffclValtnInd")]
	pub offcl_valtn_ind: bool,
	#[serde(rename = "SspdInd")]
	pub sspd_ind: bool,
	#[validate]
	#[serde(rename = "PricDtls")]
	pub pric_dtls: Option<Vec<UnitPrice15>>,
	#[validate]
	#[serde(rename = "ValtnSttstcs")]
	pub valtn_sttstcs: Option<Vec<ValuationStatistics3>>,
	#[validate]
	#[serde(rename = "PrfrmncDtls")]
	pub prfrmnc_dtls: Option<PerformanceFactors1>,
}


// PriceValue1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PriceValue1 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
}


// PriceValue5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PriceValue5 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAnd13DecimalAmount,
}


// PriceValueChange1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PriceValueChange1 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
	#[serde(rename = "AmtSgn")]
	pub amt_sgn: Option<bool>,
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
}


// QUICKIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct QUICKIdentifier {
	#[serde(rename = "QUICKIdentifier")]
	pub quick_identifier: String,
}


// RICIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RICIdentifier {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "RICIdentifier")]
	pub ric_identifier: String,
}


// SEDOLIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SEDOLIdentifier {
	#[serde(rename = "SEDOLIdentifier")]
	pub sedol_identifier: String,
}


// SecurityIdentification3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityIdentification3Choice {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "SEDOL")]
	pub sedol: Option<String>,
	#[serde(rename = "CUSIP")]
	pub cusip: Option<String>,
	#[serde(rename = "RIC")]
	pub ric: Option<String>,
	#[serde(rename = "TckrSymb")]
	pub tckr_symb: Option<String>,
	#[serde(rename = "Blmbrg")]
	pub blmbrg: Option<String>,
	#[serde(rename = "CTA")]
	pub cta: Option<String>,
	#[serde(rename = "QUICK")]
	pub quick: Option<String>,
	#[serde(rename = "Wrtppr")]
	pub wrtppr: Option<String>,
	#[serde(rename = "Dtch")]
	pub dtch: Option<String>,
	#[serde(rename = "Vlrn")]
	pub vlrn: Option<String>,
	#[serde(rename = "SCVM")]
	pub scvm: Option<String>,
	#[serde(rename = "Belgn")]
	pub belgn: Option<String>,
	#[serde(rename = "Cmon")]
	pub cmon: Option<String>,
	#[validate]
	#[serde(rename = "OthrPrtryId")]
	pub othr_prtry_id: Option<AlternateSecurityIdentification1>,
}


// SicovamIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SicovamIdentifier {
	#[serde(rename = "SicovamIdentifier")]
	pub sicovam_identifier: String,
}


// StatisticsByPredefinedTimePeriods2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct StatisticsByPredefinedTimePeriods2 {
	#[validate]
	#[serde(rename = "HghstPricVal12Mnths")]
	pub hghst_pric_val12_mnths: Option<PriceValue5>,
	#[validate]
	#[serde(rename = "LwstPricVal12Mnths")]
	pub lwst_pric_val12_mnths: Option<PriceValue5>,
	#[validate]
	#[serde(rename = "OneYrPricChng")]
	pub one_yr_pric_chng: Option<PriceValueChange1>,
	#[validate]
	#[serde(rename = "ThreeYrPricChng")]
	pub three_yr_pric_chng: Option<PriceValueChange1>,
	#[validate]
	#[serde(rename = "FiveYrPricChng")]
	pub five_yr_pric_chng: Option<PriceValueChange1>,
}


// StatisticsByUserDefinedTimePeriod2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct StatisticsByUserDefinedTimePeriod2 {
	#[validate]
	#[serde(rename = "Prd")]
	pub prd: DateOrDateTimePeriodChoice,
	#[validate]
	#[serde(rename = "HghstPricVal")]
	pub hghst_pric_val: Option<PriceValue5>,
	#[validate]
	#[serde(rename = "LwstPricVal")]
	pub lwst_pric_val: Option<PriceValue5>,
	#[validate]
	#[serde(rename = "PricChng")]
	pub pric_chng: Option<PriceValueChange1>,
	#[serde(rename = "Yld")]
	pub yld: Option<f64>,
}


// Tax17 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Tax17 {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "XtndedTp")]
	pub xtnded_tp: Option<String>,
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: Option<Vec<ActiveOrHistoricCurrencyAnd13DecimalAmount>>,
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
	#[serde(rename = "Ctry")]
	pub ctry: String,
	#[validate]
	#[serde(rename = "TaxClctnDtls")]
	pub tax_clctn_dtls: Option<TaxCalculationInformation4>,
}


// TaxCalculationInformation4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxCalculationInformation4 {
	#[serde(rename = "EUCptlGn")]
	pub eu_cptl_gn: Option<String>,
	#[serde(rename = "XtndedEUCptlGn")]
	pub xtnded_eu_cptl_gn: Option<String>,
	#[serde(rename = "PctgOfDebtClm")]
	pub pctg_of_debt_clm: Option<f64>,
	#[serde(rename = "PctgGrdfthdDebt")]
	pub pctg_grdfthd_debt: Option<f64>,
	#[validate]
	#[serde(rename = "TaxblIncmPerDvdd")]
	pub taxbl_incm_per_dvdd: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
	#[serde(rename = "EUDvddSts")]
	pub eu_dvdd_sts: Option<String>,
	#[serde(rename = "XtndedEUDvddSts")]
	pub xtnded_eu_dvdd_sts: Option<String>,
}


// TaxType12Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxType12Code {
	#[validate(enumerate = ["INPO", "EUTR", "AKT1", "AKT2", "ZWIS", "MIET"])]
	#[serde(rename = "TaxType12Code")]
	pub tax_type12_code: String,
}


// TaxableIncomePerShareCalculated2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxableIncomePerShareCalculated2Code {
	#[validate(enumerate = ["TSIY", "TSIN", "UKWN"])]
	#[serde(rename = "TaxableIncomePerShareCalculated2Code")]
	pub taxable_income_per_share_calculated2_code: String,
}


// TickerIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TickerIdentifier {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "TickerIdentifier")]
	pub ticker_identifier: String,
}


// TypeOfPrice6Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TypeOfPrice6Code {
	#[validate(enumerate = ["BIDE", "OFFR", "NAVL", "CREA", "CANC", "INTE", "SWNG", "OTHR", "MIDD", "RINV", "SWIC", "DDVR", "ACTU", "NAUP"])]
	#[serde(rename = "TypeOfPrice6Code")]
	pub type_of_price6_code: String,
}


// TypeOfPrice9Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TypeOfPrice9Code {
	#[validate(enumerate = ["BIDE", "OFFR", "NAVL", "CREA", "CANC", "INTE", "SWNG", "MIDD", "RINV", "SWIC", "DDVR", "ACTU", "NAUP", "GUAR", "ENAV"])]
	#[serde(rename = "TypeOfPrice9Code")]
	pub type_of_price9_code: String,
}


// UnitPrice15 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UnitPrice15 {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "XtndedTp")]
	pub xtnded_tp: Option<String>,
	#[serde(rename = "PricMtd")]
	pub pric_mtd: Option<String>,
	#[validate]
	#[serde(rename = "ValInInvstmtCcy")]
	pub val_in_invstmt_ccy: Vec<PriceValue1>,
	#[validate]
	#[serde(rename = "ValInAltrntvCcy")]
	pub val_in_altrntv_ccy: Option<Vec<PriceValue1>>,
	#[serde(rename = "ForExctnInd")]
	pub for_exctn_ind: bool,
	#[serde(rename = "CumDvddInd")]
	pub cum_dvdd_ind: bool,
	#[serde(rename = "ClctnBsis")]
	pub clctn_bsis: Option<f64>,
	#[serde(rename = "EstmtdPricInd")]
	pub estmtd_pric_ind: bool,
	#[serde(rename = "NbOfDaysAcrd")]
	pub nb_of_days_acrd: Option<f64>,
	#[validate]
	#[serde(rename = "TaxblIncmPerShr")]
	pub taxbl_incm_per_shr: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
	#[serde(rename = "TaxblIncmPerShrClctd")]
	pub taxbl_incm_per_shr_clctd: Option<String>,
	#[serde(rename = "XtndedTaxblIncmPerShrClctd")]
	pub xtnded_taxbl_incm_per_shr_clctd: Option<String>,
	#[validate]
	#[serde(rename = "TaxblIncmPerDvdd")]
	pub taxbl_incm_per_dvdd: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
	#[serde(rename = "EUDvddSts")]
	pub eu_dvdd_sts: Option<String>,
	#[serde(rename = "XtndedEUDvddSts")]
	pub xtnded_eu_dvdd_sts: Option<String>,
	#[validate]
	#[serde(rename = "ChrgDtls")]
	pub chrg_dtls: Option<Vec<Charge15>>,
	#[validate]
	#[serde(rename = "TaxLbltyDtls")]
	pub tax_lblty_dtls: Option<Vec<Tax17>>,
	#[validate]
	#[serde(rename = "TaxRfndDtls")]
	pub tax_rfnd_dtls: Option<Vec<Tax17>>,
}


// ValorenIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ValorenIdentifier {
	#[serde(rename = "ValorenIdentifier")]
	pub valoren_identifier: String,
}


// ValuationStatistics3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ValuationStatistics3 {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[validate]
	#[serde(rename = "PricTpChngBsis")]
	pub pric_tp_chng_bsis: PriceType2,
	#[validate]
	#[serde(rename = "PricChng")]
	pub pric_chng: PriceValueChange1,
	#[serde(rename = "Yld")]
	pub yld: Option<f64>,
	#[validate]
	#[serde(rename = "ByPrdfndTmPrds")]
	pub by_prdfnd_tm_prds: Option<StatisticsByPredefinedTimePeriods2>,
	#[validate]
	#[serde(rename = "ByUsrDfndTmPrd")]
	pub by_usr_dfnd_tm_prd: Option<Vec<StatisticsByUserDefinedTimePeriod2>>,
}


// ValuationTiming1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ValuationTiming1Code {
	#[validate(enumerate = ["EXCP", "USUA", "PATC"])]
	#[serde(rename = "ValuationTiming1Code")]
	pub valuation_timing1_code: String,
}


// WertpapierIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct WertpapierIdentifier {
	#[serde(rename = "WertpapierIdentifier")]
	pub wertpapier_identifier: String,
}


// YesNoIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}
