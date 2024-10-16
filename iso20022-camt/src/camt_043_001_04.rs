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


// AccountIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentification1 {
	#[serde(rename = "Prtry")]
	pub prtry: SimpleIdentificationInformation,
}


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


// AdditionalParameters1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditionalParameters1 {
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "GeoArea", skip_serializing_if = "Option::is_none")]
	pub geo_area: Option<Max35Text>,
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


// AmountOrRate3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountOrRate3Choice {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
}


// AnyBICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICIdentifier {
	#[serde(rename = "$value")]
	pub any_bic_identifier: String,
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
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


// BreakdownByCountry2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BreakdownByCountry2 {
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
	#[serde(rename = "CshInFcst", skip_serializing_if = "Option::is_none")]
	pub csh_in_fcst: Option<Vec<CashInForecast5>>,
	#[serde(rename = "CshOutFcst", skip_serializing_if = "Option::is_none")]
	pub csh_out_fcst: Option<Vec<CashOutForecast5>>,
	#[serde(rename = "NetCshFcst", skip_serializing_if = "Option::is_none")]
	pub net_csh_fcst: Option<Vec<NetCashForecast4>>,
}


// BreakdownByCurrency2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BreakdownByCurrency2 {
	#[serde(rename = "Ccy")]
	pub ccy: ActiveOrHistoricCurrencyCode,
	#[serde(rename = "CshOutFcst", skip_serializing_if = "Option::is_none")]
	pub csh_out_fcst: Option<Vec<CashOutForecast5>>,
	#[serde(rename = "CshInFcst", skip_serializing_if = "Option::is_none")]
	pub csh_in_fcst: Option<Vec<CashInForecast5>>,
	#[serde(rename = "NetCshFcst", skip_serializing_if = "Option::is_none")]
	pub net_csh_fcst: Option<Vec<NetCashForecast4>>,
}


// BreakdownByParty3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BreakdownByParty3 {
	#[serde(rename = "Pty")]
	pub pty: InvestmentAccount42,
	#[serde(rename = "AddtlParams", skip_serializing_if = "Option::is_none")]
	pub addtl_params: Option<AdditionalParameters1>,
	#[serde(rename = "CshInFcst", skip_serializing_if = "Option::is_none")]
	pub csh_in_fcst: Option<Vec<CashInForecast5>>,
	#[serde(rename = "CshOutFcst", skip_serializing_if = "Option::is_none")]
	pub csh_out_fcst: Option<Vec<CashOutForecast5>>,
	#[serde(rename = "NetCshFcst", skip_serializing_if = "Option::is_none")]
	pub net_csh_fcst: Option<Vec<NetCashForecast4>>,
}


// BreakdownByUserDefinedParameter3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BreakdownByUserDefinedParameter3 {
	#[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
	pub pty: Option<InvestmentAccount42>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "UsrDfnd", skip_serializing_if = "Option::is_none")]
	pub usr_dfnd: Option<DataFormat2Choice>,
	#[serde(rename = "CshInFcst", skip_serializing_if = "Option::is_none")]
	pub csh_in_fcst: Option<Vec<CashInForecast5>>,
	#[serde(rename = "CshOutFcst", skip_serializing_if = "Option::is_none")]
	pub csh_out_fcst: Option<Vec<CashOutForecast5>>,
	#[serde(rename = "NetCshFcst", skip_serializing_if = "Option::is_none")]
	pub net_csh_fcst: Option<Vec<NetCashForecast4>>,
}


// CUSIPIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CUSIPIdentifier {
	#[serde(rename = "$value")]
	pub cusip_identifier: String,
}


// CashInForecast5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashInForecast5 {
	#[serde(rename = "CshSttlmDt")]
	pub csh_sttlm_dt: String,
	#[serde(rename = "SubTtlAmt", skip_serializing_if = "Option::is_none")]
	pub sub_ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "SubTtlUnitsNb", skip_serializing_if = "Option::is_none")]
	pub sub_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "XcptnlCshFlowInd", skip_serializing_if = "Option::is_none")]
	pub xcptnl_csh_flow_ind: Option<bool>,
	#[serde(rename = "CshInBrkdwnDtls", skip_serializing_if = "Option::is_none")]
	pub csh_in_brkdwn_dtls: Option<Vec<FundCashInBreakdown3>>,
	#[serde(rename = "AddtlBal", skip_serializing_if = "Option::is_none")]
	pub addtl_bal: Option<FundBalance1>,
}


// CashOutForecast5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashOutForecast5 {
	#[serde(rename = "CshSttlmDt")]
	pub csh_sttlm_dt: String,
	#[serde(rename = "SubTtlAmt", skip_serializing_if = "Option::is_none")]
	pub sub_ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "SubTtlUnitsNb", skip_serializing_if = "Option::is_none")]
	pub sub_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "XcptnlCshFlowInd", skip_serializing_if = "Option::is_none")]
	pub xcptnl_csh_flow_ind: Option<bool>,
	#[serde(rename = "CshOutBrkdwnDtls", skip_serializing_if = "Option::is_none")]
	pub csh_out_brkdwn_dtls: Option<Vec<FundCashOutBreakdown3>>,
	#[serde(rename = "AddtlBal", skip_serializing_if = "Option::is_none")]
	pub addtl_bal: Option<FundBalance1>,
}


// Charge26 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Charge26 {
	#[serde(rename = "Tp")]
	pub tp: ChargeType4Choice,
	#[serde(rename = "ChrgApld")]
	pub chrg_apld: AmountOrRate3Choice,
}


// ChargeType12Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ChargeType12Code {
	#[default]
	#[serde(rename = "BEND")]
	CodeBEND,
	#[serde(rename = "DISC")]
	CodeDISC,
	#[serde(rename = "FEND")]
	CodeFEND,
	#[serde(rename = "POST")]
	CodePOST,
	#[serde(rename = "REGF")]
	CodeREGF,
	#[serde(rename = "SHIP")]
	CodeSHIP,
	#[serde(rename = "SPCN")]
	CodeSPCN,
	#[serde(rename = "TRAN")]
	CodeTRAN,
}


// ChargeType4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChargeType4Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ChargeType12Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// Commission21 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Commission21 {
	#[serde(rename = "ComssnTp")]
	pub comssn_tp: CommissionType5Choice,
	#[serde(rename = "ComssnApld")]
	pub comssn_apld: AmountOrRate3Choice,
}


// CommissionType5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommissionType5Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CommissionType6Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// CommissionType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CommissionType6Code {
	#[default]
	#[serde(rename = "FEND")]
	CodeFEND,
	#[serde(rename = "BEND")]
	CodeBEND,
	#[serde(rename = "CDPL")]
	CodeCDPL,
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


// DataFormat2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DataFormat2Choice {
	#[serde(rename = "Strd", skip_serializing_if = "Option::is_none")]
	pub strd: Option<GenericIdentification1>,
	#[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
	pub ustrd: Option<Max140Text>,
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


// EuroclearClearstreamIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EuroclearClearstreamIdentifier {
	#[serde(rename = "$value")]
	pub euroclear_clearstream_identifier: String,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
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


// Fund4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Fund4 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
	#[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
	pub lgl_ntty_idr: Option<LEIIdentifier>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<OtherIdentification4>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
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


// FundCashForecast6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundCashForecast6 {
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
	#[serde(rename = "BrkdwnByPty", skip_serializing_if = "Option::is_none")]
	pub brkdwn_by_pty: Option<Vec<BreakdownByParty3>>,
	#[serde(rename = "BrkdwnByCtry", skip_serializing_if = "Option::is_none")]
	pub brkdwn_by_ctry: Option<Vec<BreakdownByCountry2>>,
	#[serde(rename = "BrkdwnByCcy", skip_serializing_if = "Option::is_none")]
	pub brkdwn_by_ccy: Option<Vec<BreakdownByCurrency2>>,
	#[serde(rename = "BrkdwnByUsrDfndParam", skip_serializing_if = "Option::is_none")]
	pub brkdwn_by_usr_dfnd_param: Option<Vec<BreakdownByUserDefinedParameter3>>,
	#[serde(rename = "NetCshFcstDtls", skip_serializing_if = "Option::is_none")]
	pub net_csh_fcst_dtls: Option<Vec<NetCashForecast4>>,
}


// FundCashInBreakdown3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundCashInBreakdown3 {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "UnitsNb", skip_serializing_if = "Option::is_none")]
	pub units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "NewAmtInd", skip_serializing_if = "Option::is_none")]
	pub new_amt_ind: Option<bool>,
	#[serde(rename = "InvstmtFndTxInTp")]
	pub invstmt_fnd_tx_in_tp: InvestmentFundTransactionInType1Choice,
	#[serde(rename = "OrgnlOrdrQtyTp")]
	pub orgnl_ordr_qty_tp: QuantityType1Choice,
	#[serde(rename = "ChrgDtls", skip_serializing_if = "Option::is_none")]
	pub chrg_dtls: Option<Vec<Charge26>>,
	#[serde(rename = "ComssnDtls", skip_serializing_if = "Option::is_none")]
	pub comssn_dtls: Option<Vec<Commission21>>,
	#[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy: Option<ActiveCurrencyCode>,
}


// FundCashOutBreakdown3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundCashOutBreakdown3 {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "UnitsNb", skip_serializing_if = "Option::is_none")]
	pub units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "NewAmtInd", skip_serializing_if = "Option::is_none")]
	pub new_amt_ind: Option<bool>,
	#[serde(rename = "InvstmtFndTxOutTp")]
	pub invstmt_fnd_tx_out_tp: InvestmentFundTransactionOutType1Choice,
	#[serde(rename = "OrgnlOrdrQtyTp")]
	pub orgnl_ordr_qty_tp: QuantityType1Choice,
	#[serde(rename = "ChrgDtls", skip_serializing_if = "Option::is_none")]
	pub chrg_dtls: Option<Vec<Charge26>>,
	#[serde(rename = "ComssnDtls", skip_serializing_if = "Option::is_none")]
	pub comssn_dtls: Option<Vec<Commission21>>,
	#[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy: Option<ActiveCurrencyCode>,
}


// FundDetailedConfirmedCashForecastReportV04 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundDetailedConfirmedCashForecastReportV04 {
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
	pub fnd_or_sub_fnd_dtls: Option<Fund4>,
	#[serde(rename = "FndCshFcstDtls")]
	pub fnd_csh_fcst_dtls: Vec<FundCashForecast6>,
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


// IdentificationSource5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource5Choice {
	#[serde(rename = "DmstIdSrc", skip_serializing_if = "Option::is_none")]
	pub dmst_id_src: Option<CountryCode>,
	#[serde(rename = "PrtryIdSrc", skip_serializing_if = "Option::is_none")]
	pub prtry_id_src: Option<Max35Text>,
}


// InvestmentAccount42 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentAccount42 {
	#[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
	pub acct_id: Option<AccountIdentification1>,
	#[serde(rename = "OwnrId", skip_serializing_if = "Option::is_none")]
	pub ownr_id: Option<PartyIdentification2Choice>,
	#[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
	pub acct_svcr: Option<PartyIdentification2Choice>,
}


// InvestmentFundTransactionInType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentFundTransactionInType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestmentFundTransactionInType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// InvestmentFundTransactionInType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestmentFundTransactionInType1Code {
	#[default]
	#[serde(rename = "SUBS")]
	CodeSUBS,
	#[serde(rename = "SWII")]
	CodeSWII,
	#[serde(rename = "INSP")]
	CodeINSP,
	#[serde(rename = "CROI")]
	CodeCROI,
	#[serde(rename = "RDIV")]
	CodeRDIV,
}


// InvestmentFundTransactionOutType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentFundTransactionOutType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestmentFundTransactionOutType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// InvestmentFundTransactionOutType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestmentFundTransactionOutType1Code {
	#[default]
	#[serde(rename = "REDM")]
	CodeREDM,
	#[serde(rename = "SWIO")]
	CodeSWIO,
	#[serde(rename = "INSP")]
	CodeINSP,
	#[serde(rename = "CROO")]
	CodeCROO,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
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


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "$value")]
	pub max4_alpha_numeric_text: String,
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


// OrderQuantityType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OrderQuantityType2Code {
	#[default]
	#[serde(rename = "UNIT")]
	CodeUNIT,
	#[serde(rename = "CASH")]
	CodeCASH,
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
pub struct QUICKIdentifier {
	#[serde(rename = "$value")]
	pub quick_identifier: String,
}


// QuantityType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QuantityType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<OrderQuantityType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
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


// SimpleIdentificationInformation ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SimpleIdentificationInformation {
	#[serde(rename = "Id")]
	pub id: Max35Text,
}


// TickerIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
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
pub struct ValorenIdentifier {
	#[serde(rename = "$value")]
	pub valoren_identifier: String,
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
