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


// ActiveCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and13_decimal_amount_simple_type: f64,
}


// ActiveCurrencyAnd13DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmount {
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


// ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and13_decimal_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAnd13DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_code: String,
}


// AdditionalReference3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditionalReference3 {
	#[serde(rename = "Ref")]
	pub ref_attr: Max35Text,
	#[serde(rename = "RefIssr", skip_serializing_if = "Option::is_none")]
	pub ref_issr: Option<PartyIdentification2Choice>,
	#[serde(rename = "MsgNm", skip_serializing_if = "Option::is_none")]
	pub msg_nm: Option<Max35Text>,
}


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AddressType2Code {
	#[default]
	#[serde(rename = "ADDR")]
	CodeADDR,
	#[serde(rename = "PBOX")]
	CodePBOX,
	#[serde(rename = "HOME")]
	CodeHOME,
	#[serde(rename = "BIZZ")]
	CodeBIZZ,
	#[serde(rename = "MLTO")]
	CodeMLTO,
	#[serde(rename = "DLVY")]
	CodeDLVY,

}


// AlternateSecurityIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AlternateSecurityIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "DmstIdSrc", skip_serializing_if = "Option::is_none")]
	pub dmst_id_src: Option<CountryCode>,
	#[serde(rename = "PrtryIdSrc", skip_serializing_if = "Option::is_none")]
	pub prtry_id_src: Option<Max35Text>,
}


// AnyBICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICIdentifier {
	#[serde(rename = "$value")]
	pub any_bic_identifier: String,
}


// BelgianIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BelgianIdentifier {
	#[serde(rename = "$value")]
	pub belgian_identifier: String,
}


// BloombergIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BloombergIdentifier {
	#[serde(rename = "$value")]
	pub bloomberg_identifier: String,
}


// CUSIPIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CUSIPIdentifier {
	#[serde(rename = "$value")]
	pub cusip_identifier: String,
}


// CalculationBasis2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CalculationBasis2Code {
	#[default]
	#[serde(rename = "AVER")]
	CodeAVER,
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "YEAR")]
	CodeYEAR,

}


// Charge15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Charge15 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ChargeType9Code>,
	#[serde(rename = "XtndedTp", skip_serializing_if = "Option::is_none")]
	pub xtnded_tp: Option<Extended350Code>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "ClctnBsis", skip_serializing_if = "Option::is_none")]
	pub clctn_bsis: Option<CalculationBasis2Code>,
	#[serde(rename = "XtndedClctnBsis", skip_serializing_if = "Option::is_none")]
	pub xtnded_clctn_bsis: Option<Extended350Code>,
}


// ChargeType9Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ChargeType9Code {
	#[default]
	#[serde(rename = "MANF")]
	CodeMANF,
	#[serde(rename = "BEND")]
	CodeBEND,
	#[serde(rename = "FEND")]
	CodeFEND,
	#[serde(rename = "ADVI")]
	CodeADVI,
	#[serde(rename = "CUST")]
	CodeCUST,
	#[serde(rename = "PUBL")]
	CodePUBL,
	#[serde(rename = "ACCT")]
	CodeACCT,
	#[serde(rename = "EQUL")]
	CodeEQUL,
	#[serde(rename = "PENA")]
	CodePENA,

}


// ConsolidatedTapeAssociationIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConsolidatedTapeAssociationIdentifier {
	#[serde(rename = "$value")]
	pub consolidated_tape_association_identifier: String,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// DateAndDateTime1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime1Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}


// DateAndDateTimeChoice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTimeChoice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}


// DateOrDateTimePeriodChoice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateOrDateTimePeriodChoice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<DatePeriodDetails>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<DateTimePeriodDetails>,
}


// DatePeriodDetails ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriodDetails {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// DateTimePeriodDetails ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriodDetails {
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


// DistributionPolicy1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DistributionPolicy1Code {
	#[default]
	#[serde(rename = "DIST")]
	CodeDIST,
	#[serde(rename = "ACCU")]
	CodeACCU,

}


// DutchIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DutchIdentifier {
	#[serde(rename = "$value")]
	pub dutch_identifier: String,
}


// EUCapitalGain2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EUCapitalGain2Code {
	#[default]
	#[serde(rename = "EUSI")]
	CodeEUSI,
	#[serde(rename = "EUSO")]
	CodeEUSO,
	#[serde(rename = "UKWN")]
	CodeUKWN,

}


// EUDividendStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EUDividendStatus1Code {
	#[default]
	#[serde(rename = "DIVI")]
	CodeDIVI,
	#[serde(rename = "DIVO")]
	CodeDIVO,
	#[serde(rename = "UKWN")]
	CodeUKWN,

}


// EuroclearClearstreamIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EuroclearClearstreamIdentifier {
	#[serde(rename = "$value")]
	pub euroclear_clearstream_identifier: String,
}


// EventFrequency1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EventFrequency1Code {
	#[default]
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "SEMI")]
	CodeSEMI,
	#[serde(rename = "QUTR")]
	CodeQUTR,
	#[serde(rename = "TOMN")]
	CodeTOMN,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "TWMN")]
	CodeTWMN,
	#[serde(rename = "TOWK")]
	CodeTOWK,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "ADHO")]
	CodeADHO,
	#[serde(rename = "INDA")]
	CodeINDA,
	#[serde(rename = "OVNG")]
	CodeOVNG,
	#[serde(rename = "ONDE")]
	CodeONDE,

}


// Extended350Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Extended350Code {
	#[serde(rename = "$value")]
	pub extended350_code: String,
}


// Extension1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Extension1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Max350Text,
	#[serde(rename = "Txt")]
	pub txt: Max350Text,
}


// FinancialInstrument8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument8 {
	#[serde(rename = "Id")]
	pub id: Vec<SecurityIdentification3Choice>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
	#[serde(rename = "SplmtryId", skip_serializing_if = "Option::is_none")]
	pub splmtry_id: Option<Max35Text>,
	#[serde(rename = "DnmtnCcy", skip_serializing_if = "Option::is_none")]
	pub dnmtn_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "ClssTp", skip_serializing_if = "Option::is_none")]
	pub clss_tp: Option<Max35Text>,
	#[serde(rename = "SctiesForm", skip_serializing_if = "Option::is_none")]
	pub scties_form: Option<FormOfSecurity1Code>,
	#[serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none")]
	pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
	#[serde(rename = "DualFndInd")]
	pub dual_fnd_ind: bool,
}


// FinancialInstrumentQuantity1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentQuantity1 {
	#[serde(rename = "Unit")]
	pub unit: f64,
}


// FormOfSecurity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FormOfSecurity1Code {
	#[default]
	#[serde(rename = "BEAR")]
	CodeBEAR,
	#[serde(rename = "REGD")]
	CodeREGD,

}


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}


// ISINIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINIdentifier {
	#[serde(rename = "$value")]
	pub isin_identifier: String,
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


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
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


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max5NumericText {
	#[serde(rename = "$value")]
	pub max5_numeric_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}


// MessageIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}


// Pagination ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination {
	#[serde(rename = "PgNb")]
	pub pg_nb: Max5NumericText,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}


// PartyIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification2Choice {
	#[serde(rename = "BICOrBEI", skip_serializing_if = "Option::is_none")]
	pub bic_or_bei: Option<AnyBICIdentifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification1>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
}


// PerformanceFactors1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PerformanceFactors1 {
	#[serde(rename = "CorpActnFctr", skip_serializing_if = "Option::is_none")]
	pub corp_actn_fctr: Option<f64>,
	#[serde(rename = "CmltvCorpActnFctr", skip_serializing_if = "Option::is_none")]
	pub cmltv_corp_actn_fctr: Option<f64>,
	#[serde(rename = "AcmltnPrd", skip_serializing_if = "Option::is_none")]
	pub acmltn_prd: Option<DatePeriodDetails>,
	#[serde(rename = "NrmlPrfrmnc", skip_serializing_if = "Option::is_none")]
	pub nrml_prfrmnc: Option<f64>,
}


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "$value")]
	pub plus_or_minus_indicator: bool,
}


// PostalAddress1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType2Code>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<Max70Text>>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max70Text>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<Max16Text>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<Max16Text>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<Max35Text>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<Max35Text>,
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
}


// PriceMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PriceMethod1Code {
	#[default]
	#[serde(rename = "FORW")]
	CodeFORW,
	#[serde(rename = "HIST")]
	CodeHIST,

}


// PriceReport3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceReport3 {
	#[serde(rename = "PricValtnDtls")]
	pub pric_valtn_dtls: Vec<PriceValuation4>,
}


// PriceReportCancellationV04 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceReportCancellationV04 {
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "PoolRef", skip_serializing_if = "Option::is_none")]
	pub pool_ref: Option<AdditionalReference3>,
	#[serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none")]
	pub prvs_ref: Option<AdditionalReference3>,
	#[serde(rename = "MsgPgntn")]
	pub msg_pgntn: Pagination,
	#[serde(rename = "PricRptId")]
	pub pric_rpt_id: Max35Text,
	#[serde(rename = "CxlId")]
	pub cxl_id: Max35Text,
	#[serde(rename = "CxlRsn", skip_serializing_if = "Option::is_none")]
	pub cxl_rsn: Option<Max350Text>,
	#[serde(rename = "XpctdPricCrrctnDt", skip_serializing_if = "Option::is_none")]
	pub xpctd_pric_crrctn_dt: Option<DateAndDateTime1Choice>,
	#[serde(rename = "CmpltPricCxl")]
	pub cmplt_pric_cxl: bool,
	#[serde(rename = "CancPricValtnDtls", skip_serializing_if = "Option::is_none")]
	pub canc_pric_valtn_dtls: Option<Vec<PriceReport3>>,
	#[serde(rename = "Xtnsn", skip_serializing_if = "Option::is_none")]
	pub xtnsn: Option<Vec<Extension1>>,
}


// PriceType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceType2 {
	#[serde(rename = "Strd")]
	pub strd: TypeOfPrice6Code,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}


// PriceValuation4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceValuation4 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "ValtnDtTm", skip_serializing_if = "Option::is_none")]
	pub valtn_dt_tm: Option<DateAndDateTimeChoice>,
	#[serde(rename = "NAVDtTm")]
	pub nav_dt_tm: DateAndDateTimeChoice,
	#[serde(rename = "FinInstrmDtls")]
	pub fin_instrm_dtls: FinancialInstrument8,
	#[serde(rename = "FndMgmtCpny", skip_serializing_if = "Option::is_none")]
	pub fnd_mgmt_cpny: Option<PartyIdentification2Choice>,
	#[serde(rename = "TtlNAV", skip_serializing_if = "Option::is_none")]
	pub ttl_nav: Option<Vec<ActiveOrHistoricCurrencyAndAmount>>,
	#[serde(rename = "TtlUnitsNb", skip_serializing_if = "Option::is_none")]
	pub ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "NxtValtnDtTm", skip_serializing_if = "Option::is_none")]
	pub nxt_valtn_dt_tm: Option<DateAndDateTimeChoice>,
	#[serde(rename = "PrvsValtnDtTm", skip_serializing_if = "Option::is_none")]
	pub prvs_valtn_dt_tm: Option<DateAndDateTimeChoice>,
	#[serde(rename = "ValtnTp")]
	pub valtn_tp: ValuationTiming1Code,
	#[serde(rename = "ValtnFrqcy", skip_serializing_if = "Option::is_none")]
	pub valtn_frqcy: Option<EventFrequency1Code>,
	#[serde(rename = "OffclValtnInd")]
	pub offcl_valtn_ind: bool,
	#[serde(rename = "SspdInd")]
	pub sspd_ind: bool,
	#[serde(rename = "PricDtls", skip_serializing_if = "Option::is_none")]
	pub pric_dtls: Option<Vec<UnitPrice15>>,
	#[serde(rename = "ValtnSttstcs", skip_serializing_if = "Option::is_none")]
	pub valtn_sttstcs: Option<Vec<ValuationStatistics3>>,
	#[serde(rename = "PrfrmncDtls", skip_serializing_if = "Option::is_none")]
	pub prfrmnc_dtls: Option<PerformanceFactors1>,
}


// PriceValue1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceValue1 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
}


// PriceValue5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceValue5 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAnd13DecimalAmount,
}


// PriceValueChange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceValueChange1 {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
	#[serde(rename = "AmtSgn", skip_serializing_if = "Option::is_none")]
	pub amt_sgn: Option<bool>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
}


// QUICKIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QUICKIdentifier {
	#[serde(rename = "$value")]
	pub quick_identifier: String,
}


// RICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RICIdentifier {
	#[serde(rename = "$value")]
	pub ric_identifier: String,
}


// SEDOLIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SEDOLIdentifier {
	#[serde(rename = "$value")]
	pub sedol_identifier: String,
}


// SecurityIdentification3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification3Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINIdentifier>,
	#[serde(rename = "SEDOL", skip_serializing_if = "Option::is_none")]
	pub sedol: Option<String>,
	#[serde(rename = "CUSIP", skip_serializing_if = "Option::is_none")]
	pub cusip: Option<String>,
	#[serde(rename = "RIC", skip_serializing_if = "Option::is_none")]
	pub ric: Option<RICIdentifier>,
	#[serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none")]
	pub tckr_symb: Option<TickerIdentifier>,
	#[serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none")]
	pub blmbrg: Option<BloombergIdentifier>,
	#[serde(rename = "CTA", skip_serializing_if = "Option::is_none")]
	pub cta: Option<ConsolidatedTapeAssociationIdentifier>,
	#[serde(rename = "QUICK", skip_serializing_if = "Option::is_none")]
	pub quick: Option<String>,
	#[serde(rename = "Wrtppr", skip_serializing_if = "Option::is_none")]
	pub wrtppr: Option<String>,
	#[serde(rename = "Dtch", skip_serializing_if = "Option::is_none")]
	pub dtch: Option<String>,
	#[serde(rename = "Vlrn", skip_serializing_if = "Option::is_none")]
	pub vlrn: Option<String>,
	#[serde(rename = "SCVM", skip_serializing_if = "Option::is_none")]
	pub scvm: Option<String>,
	#[serde(rename = "Belgn", skip_serializing_if = "Option::is_none")]
	pub belgn: Option<String>,
	#[serde(rename = "Cmon", skip_serializing_if = "Option::is_none")]
	pub cmon: Option<EuroclearClearstreamIdentifier>,
	#[serde(rename = "OthrPrtryId", skip_serializing_if = "Option::is_none")]
	pub othr_prtry_id: Option<AlternateSecurityIdentification1>,
}


// SicovamIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SicovamIdentifier {
	#[serde(rename = "$value")]
	pub sicovam_identifier: String,
}


// StatisticsByPredefinedTimePeriods2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatisticsByPredefinedTimePeriods2 {
	#[serde(rename = "HghstPricVal12Mnths", skip_serializing_if = "Option::is_none")]
	pub hghst_pric_val12_mnths: Option<PriceValue5>,
	#[serde(rename = "LwstPricVal12Mnths", skip_serializing_if = "Option::is_none")]
	pub lwst_pric_val12_mnths: Option<PriceValue5>,
	#[serde(rename = "OneYrPricChng", skip_serializing_if = "Option::is_none")]
	pub one_yr_pric_chng: Option<PriceValueChange1>,
	#[serde(rename = "ThreeYrPricChng", skip_serializing_if = "Option::is_none")]
	pub three_yr_pric_chng: Option<PriceValueChange1>,
	#[serde(rename = "FiveYrPricChng", skip_serializing_if = "Option::is_none")]
	pub five_yr_pric_chng: Option<PriceValueChange1>,
}


// StatisticsByUserDefinedTimePeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatisticsByUserDefinedTimePeriod2 {
	#[serde(rename = "Prd")]
	pub prd: DateOrDateTimePeriodChoice,
	#[serde(rename = "HghstPricVal", skip_serializing_if = "Option::is_none")]
	pub hghst_pric_val: Option<PriceValue5>,
	#[serde(rename = "LwstPricVal", skip_serializing_if = "Option::is_none")]
	pub lwst_pric_val: Option<PriceValue5>,
	#[serde(rename = "PricChng", skip_serializing_if = "Option::is_none")]
	pub pric_chng: Option<PriceValueChange1>,
	#[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
	pub yld: Option<f64>,
}


// Tax17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Tax17 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<TaxType12Code>,
	#[serde(rename = "XtndedTp", skip_serializing_if = "Option::is_none")]
	pub xtnded_tp: Option<Extended350Code>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<Vec<ActiveOrHistoricCurrencyAnd13DecimalAmount>>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
	#[serde(rename = "TaxClctnDtls", skip_serializing_if = "Option::is_none")]
	pub tax_clctn_dtls: Option<TaxCalculationInformation4>,
}


// TaxCalculationInformation4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxCalculationInformation4 {
	#[serde(rename = "EUCptlGn", skip_serializing_if = "Option::is_none")]
	pub eu_cptl_gn: Option<EUCapitalGain2Code>,
	#[serde(rename = "XtndedEUCptlGn", skip_serializing_if = "Option::is_none")]
	pub xtnded_eu_cptl_gn: Option<Extended350Code>,
	#[serde(rename = "PctgOfDebtClm", skip_serializing_if = "Option::is_none")]
	pub pctg_of_debt_clm: Option<f64>,
	#[serde(rename = "PctgGrdfthdDebt", skip_serializing_if = "Option::is_none")]
	pub pctg_grdfthd_debt: Option<f64>,
	#[serde(rename = "TaxblIncmPerDvdd", skip_serializing_if = "Option::is_none")]
	pub taxbl_incm_per_dvdd: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
	#[serde(rename = "EUDvddSts", skip_serializing_if = "Option::is_none")]
	pub eu_dvdd_sts: Option<EUDividendStatus1Code>,
	#[serde(rename = "XtndedEUDvddSts", skip_serializing_if = "Option::is_none")]
	pub xtnded_eu_dvdd_sts: Option<Extended350Code>,
}


// TaxType12Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TaxType12Code {
	#[default]
	#[serde(rename = "INPO")]
	CodeINPO,
	#[serde(rename = "EUTR")]
	CodeEUTR,
	#[serde(rename = "AKT1")]
	CodeAKT1,
	#[serde(rename = "AKT2")]
	CodeAKT2,
	#[serde(rename = "ZWIS")]
	CodeZWIS,
	#[serde(rename = "MIET")]
	CodeMIET,

}


// TaxableIncomePerShareCalculated2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TaxableIncomePerShareCalculated2Code {
	#[default]
	#[serde(rename = "TSIY")]
	CodeTSIY,
	#[serde(rename = "TSIN")]
	CodeTSIN,
	#[serde(rename = "UKWN")]
	CodeUKWN,

}


// TickerIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TickerIdentifier {
	#[serde(rename = "$value")]
	pub ticker_identifier: String,
}


// TypeOfPrice6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypeOfPrice6Code {
	#[default]
	#[serde(rename = "BIDE")]
	CodeBIDE,
	#[serde(rename = "OFFR")]
	CodeOFFR,
	#[serde(rename = "NAVL")]
	CodeNAVL,
	#[serde(rename = "CREA")]
	CodeCREA,
	#[serde(rename = "CANC")]
	CodeCANC,
	#[serde(rename = "INTE")]
	CodeINTE,
	#[serde(rename = "SWNG")]
	CodeSWNG,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "MIDD")]
	CodeMIDD,
	#[serde(rename = "RINV")]
	CodeRINV,
	#[serde(rename = "SWIC")]
	CodeSWIC,
	#[serde(rename = "DDVR")]
	CodeDDVR,
	#[serde(rename = "ACTU")]
	CodeACTU,
	#[serde(rename = "NAUP")]
	CodeNAUP,

}


// TypeOfPrice9Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypeOfPrice9Code {
	#[default]
	#[serde(rename = "BIDE")]
	CodeBIDE,
	#[serde(rename = "OFFR")]
	CodeOFFR,
	#[serde(rename = "NAVL")]
	CodeNAVL,
	#[serde(rename = "CREA")]
	CodeCREA,
	#[serde(rename = "CANC")]
	CodeCANC,
	#[serde(rename = "INTE")]
	CodeINTE,
	#[serde(rename = "SWNG")]
	CodeSWNG,
	#[serde(rename = "MIDD")]
	CodeMIDD,
	#[serde(rename = "RINV")]
	CodeRINV,
	#[serde(rename = "SWIC")]
	CodeSWIC,
	#[serde(rename = "DDVR")]
	CodeDDVR,
	#[serde(rename = "ACTU")]
	CodeACTU,
	#[serde(rename = "NAUP")]
	CodeNAUP,
	#[serde(rename = "GUAR")]
	CodeGUAR,
	#[serde(rename = "ENAV")]
	CodeENAV,

}


// UnitPrice15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitPrice15 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<TypeOfPrice9Code>,
	#[serde(rename = "XtndedTp", skip_serializing_if = "Option::is_none")]
	pub xtnded_tp: Option<Extended350Code>,
	#[serde(rename = "PricMtd", skip_serializing_if = "Option::is_none")]
	pub pric_mtd: Option<PriceMethod1Code>,
	#[serde(rename = "ValInInvstmtCcy")]
	pub val_in_invstmt_ccy: Vec<PriceValue1>,
	#[serde(rename = "ValInAltrntvCcy", skip_serializing_if = "Option::is_none")]
	pub val_in_altrntv_ccy: Option<Vec<PriceValue1>>,
	#[serde(rename = "ForExctnInd")]
	pub for_exctn_ind: bool,
	#[serde(rename = "CumDvddInd")]
	pub cum_dvdd_ind: bool,
	#[serde(rename = "ClctnBsis", skip_serializing_if = "Option::is_none")]
	pub clctn_bsis: Option<f64>,
	#[serde(rename = "EstmtdPricInd")]
	pub estmtd_pric_ind: bool,
	#[serde(rename = "NbOfDaysAcrd", skip_serializing_if = "Option::is_none")]
	pub nb_of_days_acrd: Option<f64>,
	#[serde(rename = "TaxblIncmPerShr", skip_serializing_if = "Option::is_none")]
	pub taxbl_incm_per_shr: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
	#[serde(rename = "TaxblIncmPerShrClctd", skip_serializing_if = "Option::is_none")]
	pub taxbl_incm_per_shr_clctd: Option<TaxableIncomePerShareCalculated2Code>,
	#[serde(rename = "XtndedTaxblIncmPerShrClctd", skip_serializing_if = "Option::is_none")]
	pub xtnded_taxbl_incm_per_shr_clctd: Option<Extended350Code>,
	#[serde(rename = "TaxblIncmPerDvdd", skip_serializing_if = "Option::is_none")]
	pub taxbl_incm_per_dvdd: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
	#[serde(rename = "EUDvddSts", skip_serializing_if = "Option::is_none")]
	pub eu_dvdd_sts: Option<EUDividendStatus1Code>,
	#[serde(rename = "XtndedEUDvddSts", skip_serializing_if = "Option::is_none")]
	pub xtnded_eu_dvdd_sts: Option<Extended350Code>,
	#[serde(rename = "ChrgDtls", skip_serializing_if = "Option::is_none")]
	pub chrg_dtls: Option<Vec<Charge15>>,
	#[serde(rename = "TaxLbltyDtls", skip_serializing_if = "Option::is_none")]
	pub tax_lblty_dtls: Option<Vec<Tax17>>,
	#[serde(rename = "TaxRfndDtls", skip_serializing_if = "Option::is_none")]
	pub tax_rfnd_dtls: Option<Vec<Tax17>>,
}


// ValorenIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValorenIdentifier {
	#[serde(rename = "$value")]
	pub valoren_identifier: String,
}


// ValuationStatistics3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValuationStatistics3 {
	#[serde(rename = "Ccy")]
	pub ccy: ActiveOrHistoricCurrencyCode,
	#[serde(rename = "PricTpChngBsis")]
	pub pric_tp_chng_bsis: PriceType2,
	#[serde(rename = "PricChng")]
	pub pric_chng: PriceValueChange1,
	#[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
	pub yld: Option<f64>,
	#[serde(rename = "ByPrdfndTmPrds", skip_serializing_if = "Option::is_none")]
	pub by_prdfnd_tm_prds: Option<StatisticsByPredefinedTimePeriods2>,
	#[serde(rename = "ByUsrDfndTmPrd", skip_serializing_if = "Option::is_none")]
	pub by_usr_dfnd_tm_prd: Option<Vec<StatisticsByUserDefinedTimePeriod2>>,
}


// ValuationTiming1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ValuationTiming1Code {
	#[default]
	#[serde(rename = "EXCP")]
	CodeEXCP,
	#[serde(rename = "USUA")]
	CodeUSUA,
	#[serde(rename = "PATC")]
	CodePATC,

}


// WertpapierIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct WertpapierIdentifier {
	#[serde(rename = "$value")]
	pub wertpapier_identifier: String,
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}
