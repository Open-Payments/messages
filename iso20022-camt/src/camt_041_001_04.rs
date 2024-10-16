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
#[serde(transparent)]
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
#[serde(transparent)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
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
#[serde(transparent)]
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
#[serde(transparent)]
pub struct AnyBICIdentifier {
	#[serde(rename = "$value")]
	pub any_bic_identifier: String,
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
}


// BelgianIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BelgianIdentifier {
	#[serde(rename = "$value")]
	pub belgian_identifier: String,
}


// BloombergIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BloombergIdentifier {
	#[serde(rename = "$value")]
	pub bloomberg_identifier: String,
}


// CUSIPIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CUSIPIdentifier {
	#[serde(rename = "$value")]
	pub cusip_identifier: String,
}


// CashInForecast6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashInForecast6 {
	#[serde(rename = "CshSttlmDt")]
	pub csh_sttlm_dt: String,
	#[serde(rename = "SubTtlAmt", skip_serializing_if = "Option::is_none")]
	pub sub_ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "SubTtlUnitsNb", skip_serializing_if = "Option::is_none")]
	pub sub_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "XcptnlCshFlowInd", skip_serializing_if = "Option::is_none")]
	pub xcptnl_csh_flow_ind: Option<bool>,
	#[serde(rename = "AddtlBal", skip_serializing_if = "Option::is_none")]
	pub addtl_bal: Option<FundBalance1>,
}


// CashInOutForecast7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashInOutForecast7 {
	#[serde(rename = "CshSttlmDt", skip_serializing_if = "Option::is_none")]
	pub csh_sttlm_dt: Option<String>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}


// CashOutForecast6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashOutForecast6 {
	#[serde(rename = "CshSttlmDt")]
	pub csh_sttlm_dt: String,
	#[serde(rename = "SubTtlAmt", skip_serializing_if = "Option::is_none")]
	pub sub_ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "SubTtlUnitsNb", skip_serializing_if = "Option::is_none")]
	pub sub_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "XcptnlCshFlowInd", skip_serializing_if = "Option::is_none")]
	pub xcptnl_csh_flow_ind: Option<bool>,
	#[serde(rename = "AddtlBal", skip_serializing_if = "Option::is_none")]
	pub addtl_bal: Option<FundBalance1>,
}


// ConsolidatedTapeAssociationIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ConsolidatedTapeAssociationIdentifier {
	#[serde(rename = "$value")]
	pub consolidated_tape_association_identifier: String,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// CurrencyDesignation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyDesignation1 {
	#[serde(rename = "CcyDsgnt", skip_serializing_if = "Option::is_none")]
	pub ccy_dsgnt: Option<CurrencyDesignation1Code>,
	#[serde(rename = "Lctn", skip_serializing_if = "Option::is_none")]
	pub lctn: Option<CountryCode>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}


// CurrencyDesignation1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CurrencyDesignation1Code {
	#[default]
	#[serde(rename = "ONSH")]
	CodeONSH,
	#[serde(rename = "OFFS")]
	CodeOFFS,
}


// DateAndDateTimeChoice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTimeChoice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
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
#[serde(transparent)]
pub struct DutchIdentifier {
	#[serde(rename = "$value")]
	pub dutch_identifier: String,
}


// EuroclearClearstreamIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EuroclearClearstreamIdentifier {
	#[serde(rename = "$value")]
	pub euroclear_clearstream_identifier: String,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}


// Extension1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Extension1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Max350Text,
	#[serde(rename = "Txt")]
	pub txt: Max350Text,
}


// FinancialInstrument9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument9 {
	#[serde(rename = "Id")]
	pub id: SecurityIdentification3Choice,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
	#[serde(rename = "SplmtryId", skip_serializing_if = "Option::is_none")]
	pub splmtry_id: Option<Max35Text>,
	#[serde(rename = "ReqdNAVCcy", skip_serializing_if = "Option::is_none")]
	pub reqd_nav_ccy: Option<ActiveOrHistoricCurrencyCode>,
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


// FlowDirectionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FlowDirectionType1Code {
	#[default]
	#[serde(rename = "INCG")]
	CodeINCG,
	#[serde(rename = "OUTG")]
	CodeOUTG,
}


// ForeignExchangeTerms19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ForeignExchangeTerms19 {
	#[serde(rename = "UnitCcy")]
	pub unit_ccy: ActiveCurrencyCode,
	#[serde(rename = "QtdCcy")]
	pub qtd_ccy: ActiveCurrencyCode,
	#[serde(rename = "XchgRate")]
	pub xchg_rate: f64,
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


// Fund2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Fund2 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
	#[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
	pub lgl_ntty_idr: Option<LEIIdentifier>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<OtherIdentification4>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "TradDtTm", skip_serializing_if = "Option::is_none")]
	pub trad_dt_tm: Option<DateAndDateTimeChoice>,
	#[serde(rename = "PrvsTradDtTm", skip_serializing_if = "Option::is_none")]
	pub prvs_trad_dt_tm: Option<DateAndDateTimeChoice>,
	#[serde(rename = "TtlNAV", skip_serializing_if = "Option::is_none")]
	pub ttl_nav: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "PrvsTtlNAV", skip_serializing_if = "Option::is_none")]
	pub prvs_ttl_nav: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlUnitsNb", skip_serializing_if = "Option::is_none")]
	pub ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "PrvsTtlUnitsNb", skip_serializing_if = "Option::is_none")]
	pub prvs_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "PctgOfFndTtlNAV", skip_serializing_if = "Option::is_none")]
	pub pctg_of_fnd_ttl_nav: Option<f64>,
	#[serde(rename = "CshInFcstDtls", skip_serializing_if = "Option::is_none")]
	pub csh_in_fcst_dtls: Option<Vec<CashInOutForecast7>>,
	#[serde(rename = "CshOutFcstDtls", skip_serializing_if = "Option::is_none")]
	pub csh_out_fcst_dtls: Option<Vec<CashInOutForecast7>>,
	#[serde(rename = "NetCshFcstDtls", skip_serializing_if = "Option::is_none")]
	pub net_csh_fcst_dtls: Option<Vec<NetCashForecast5>>,
}


// FundBalance1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundBalance1 {
	#[serde(rename = "TtlUnitsFrUnitOrdrs", skip_serializing_if = "Option::is_none")]
	pub ttl_units_fr_unit_ordrs: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "TtlUnitsFrCshOrdrs", skip_serializing_if = "Option::is_none")]
	pub ttl_units_fr_csh_ordrs: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "TtlCshFrUnitOrdrs", skip_serializing_if = "Option::is_none")]
	pub ttl_csh_fr_unit_ordrs: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlCshFrCshOrdrs", skip_serializing_if = "Option::is_none")]
	pub ttl_csh_fr_csh_ordrs: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// FundCashForecast7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundCashForecast7 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "TradDtTm")]
	pub trad_dt_tm: DateAndDateTimeChoice,
	#[serde(rename = "PrvsTradDtTm", skip_serializing_if = "Option::is_none")]
	pub prvs_trad_dt_tm: Option<DateAndDateTimeChoice>,
	#[serde(rename = "FinInstrmDtls")]
	pub fin_instrm_dtls: FinancialInstrument9,
	#[serde(rename = "TtlNAV", skip_serializing_if = "Option::is_none")]
	pub ttl_nav: Option<Vec<ActiveOrHistoricCurrencyAndAmount>>,
	#[serde(rename = "PrvsTtlNAV", skip_serializing_if = "Option::is_none")]
	pub prvs_ttl_nav: Option<Vec<ActiveOrHistoricCurrencyAndAmount>>,
	#[serde(rename = "TtlUnitsNb", skip_serializing_if = "Option::is_none")]
	pub ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "PrvsTtlUnitsNb", skip_serializing_if = "Option::is_none")]
	pub prvs_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "TtlNAVChngRate", skip_serializing_if = "Option::is_none")]
	pub ttl_nav_chng_rate: Option<f64>,
	#[serde(rename = "InvstmtCcy", skip_serializing_if = "Option::is_none")]
	pub invstmt_ccy: Option<Vec<ActiveOrHistoricCurrencyCode>>,
	#[serde(rename = "CcySts", skip_serializing_if = "Option::is_none")]
	pub ccy_sts: Option<CurrencyDesignation1>,
	#[serde(rename = "XcptnlNetCshFlowInd")]
	pub xcptnl_net_csh_flow_ind: bool,
	#[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
	pub pric: Option<UnitPrice19>,
	#[serde(rename = "FXRate", skip_serializing_if = "Option::is_none")]
	pub fx_rate: Option<ForeignExchangeTerms19>,
	#[serde(rename = "PctgOfShrClssTtlNAV", skip_serializing_if = "Option::is_none")]
	pub pctg_of_shr_clss_ttl_nav: Option<f64>,
	#[serde(rename = "CshInFcstDtls", skip_serializing_if = "Option::is_none")]
	pub csh_in_fcst_dtls: Option<Vec<CashInForecast6>>,
	#[serde(rename = "CshOutFcstDtls", skip_serializing_if = "Option::is_none")]
	pub csh_out_fcst_dtls: Option<Vec<CashOutForecast6>>,
	#[serde(rename = "NetCshFcstDtls", skip_serializing_if = "Option::is_none")]
	pub net_csh_fcst_dtls: Option<Vec<NetCashForecast4>>,
}


// FundConfirmedCashForecastReportV04 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundConfirmedCashForecastReportV04 {
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "PoolRef", skip_serializing_if = "Option::is_none")]
	pub pool_ref: Option<AdditionalReference3>,
	#[serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none")]
	pub prvs_ref: Option<Vec<AdditionalReference3>>,
	#[serde(rename = "RltdRef", skip_serializing_if = "Option::is_none")]
	pub rltd_ref: Option<Vec<AdditionalReference3>>,
	#[serde(rename = "MsgPgntn")]
	pub msg_pgntn: Pagination,
	#[serde(rename = "FndOrSubFndDtls", skip_serializing_if = "Option::is_none")]
	pub fnd_or_sub_fnd_dtls: Option<Vec<Fund2>>,
	#[serde(rename = "FndCshFcstDtls", skip_serializing_if = "Option::is_none")]
	pub fnd_csh_fcst_dtls: Option<Vec<FundCashForecast7>>,
	#[serde(rename = "CnsltdNetCshFcst", skip_serializing_if = "Option::is_none")]
	pub cnsltd_net_csh_fcst: Option<NetCashForecast3>,
	#[serde(rename = "Xtnsn", skip_serializing_if = "Option::is_none")]
	pub xtnsn: Option<Vec<Extension1>>,
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


// GenericIdentification47 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification47 {
	#[serde(rename = "Id")]
	pub id: Exact4AlphaNumericText,
	#[serde(rename = "Issr")]
	pub issr: Max4AlphaNumericText,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max4AlphaNumericText>,
}


// ISINIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISINIdentifier {
	#[serde(rename = "$value")]
	pub isin_identifier: String,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}


// IdentificationSource5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource5Choice {
	#[serde(rename = "DmstIdSrc", skip_serializing_if = "Option::is_none")]
	pub dmst_id_src: Option<CountryCode>,
	#[serde(rename = "PrtryIdSrc", skip_serializing_if = "Option::is_none")]
	pub prtry_id_src: Option<Max35Text>,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "$value")]
	pub max4_alpha_numeric_text: String,
}


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max5NumericText {
	#[serde(rename = "$value")]
	pub max5_numeric_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
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


// NetCashForecast3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NetCashForecast3 {
	#[serde(rename = "NetAmt", skip_serializing_if = "Option::is_none")]
	pub net_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "NetUnitsNb", skip_serializing_if = "Option::is_none")]
	pub net_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "FlowDrctn")]
	pub flow_drctn: FlowDirectionType1Code,
}


// NetCashForecast4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NetCashForecast4 {
	#[serde(rename = "CshSttlmDt")]
	pub csh_sttlm_dt: String,
	#[serde(rename = "NetAmt", skip_serializing_if = "Option::is_none")]
	pub net_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "NetUnitsNb", skip_serializing_if = "Option::is_none")]
	pub net_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "FlowDrctn")]
	pub flow_drctn: FlowDirectionType1Code,
	#[serde(rename = "AddtlBal", skip_serializing_if = "Option::is_none")]
	pub addtl_bal: Option<FundBalance1>,
}


// NetCashForecast5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NetCashForecast5 {
	#[serde(rename = "CshSttlmDt", skip_serializing_if = "Option::is_none")]
	pub csh_sttlm_dt: Option<String>,
	#[serde(rename = "NetAmt", skip_serializing_if = "Option::is_none")]
	pub net_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "NetUnitsNb", skip_serializing_if = "Option::is_none")]
	pub net_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "FlowDrctn")]
	pub flow_drctn: FlowDirectionType1Code,
}


// OtherIdentification4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification4 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Tp")]
	pub tp: IdentificationSource5Choice,
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
#[serde(transparent)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
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


// PriceValue1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceValue1 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
}


// QUICKIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct QUICKIdentifier {
	#[serde(rename = "$value")]
	pub quick_identifier: String,
}


// RICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RICIdentifier {
	#[serde(rename = "$value")]
	pub ric_identifier: String,
}


// SEDOLIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
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
#[serde(transparent)]
pub struct SicovamIdentifier {
	#[serde(rename = "$value")]
	pub sicovam_identifier: String,
}


// TickerIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TickerIdentifier {
	#[serde(rename = "$value")]
	pub ticker_identifier: String,
}


// TypeOfPrice10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypeOfPrice10Code {
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
}


// UnitPrice19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitPrice19 {
	#[serde(rename = "PricTp")]
	pub pric_tp: UnitPriceType2Choice,
	#[serde(rename = "Val")]
	pub val: PriceValue1,
}


// UnitPriceType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitPriceType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<TypeOfPrice10Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// ValorenIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ValorenIdentifier {
	#[serde(rename = "$value")]
	pub valoren_identifier: String,
}


// WertpapierIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WertpapierIdentifier {
	#[serde(rename = "$value")]
	pub wertpapier_identifier: String,
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}
