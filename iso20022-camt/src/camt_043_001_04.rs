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


// AccountIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountIdentification1 {
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: SimpleIdentificationInformation,
}


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


// AdditionalParameters1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AdditionalParameters1 {
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
	#[serde(rename = "Ccy")]
	pub ccy: Option<String>,
	#[serde(rename = "GeoArea")]
	pub geo_area: Option<String>,
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


// AmountOrRate3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AmountOrRate3Choice {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
}


// AnyBICIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AnyBICIdentifier {
	#[validate(pattern = "[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}")]
	#[serde(rename = "AnyBICIdentifier")]
	pub any_bic_identifier: String,
}


// BaseOneRate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "BaseOneRate")]
	pub base_one_rate: f64,
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


// BreakdownByCountry2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BreakdownByCountry2 {
	#[serde(rename = "Ctry")]
	pub ctry: String,
	#[validate]
	#[serde(rename = "CshInFcst")]
	pub csh_in_fcst: Option<Vec<CashInForecast5>>,
	#[validate]
	#[serde(rename = "CshOutFcst")]
	pub csh_out_fcst: Option<Vec<CashOutForecast5>>,
	#[validate]
	#[serde(rename = "NetCshFcst")]
	pub net_csh_fcst: Option<Vec<NetCashForecast4>>,
}


// BreakdownByCurrency2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BreakdownByCurrency2 {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[validate]
	#[serde(rename = "CshOutFcst")]
	pub csh_out_fcst: Option<Vec<CashOutForecast5>>,
	#[validate]
	#[serde(rename = "CshInFcst")]
	pub csh_in_fcst: Option<Vec<CashInForecast5>>,
	#[validate]
	#[serde(rename = "NetCshFcst")]
	pub net_csh_fcst: Option<Vec<NetCashForecast4>>,
}


// BreakdownByParty3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BreakdownByParty3 {
	#[validate]
	#[serde(rename = "Pty")]
	pub pty: InvestmentAccount42,
	#[validate]
	#[serde(rename = "AddtlParams")]
	pub addtl_params: Option<AdditionalParameters1>,
	#[validate]
	#[serde(rename = "CshInFcst")]
	pub csh_in_fcst: Option<Vec<CashInForecast5>>,
	#[validate]
	#[serde(rename = "CshOutFcst")]
	pub csh_out_fcst: Option<Vec<CashOutForecast5>>,
	#[validate]
	#[serde(rename = "NetCshFcst")]
	pub net_csh_fcst: Option<Vec<NetCashForecast4>>,
}


// BreakdownByUserDefinedParameter3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BreakdownByUserDefinedParameter3 {
	#[validate]
	#[serde(rename = "Pty")]
	pub pty: Option<InvestmentAccount42>,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
	#[serde(rename = "Ccy")]
	pub ccy: Option<String>,
	#[validate]
	#[serde(rename = "UsrDfnd")]
	pub usr_dfnd: Option<DataFormat2Choice>,
	#[validate]
	#[serde(rename = "CshInFcst")]
	pub csh_in_fcst: Option<Vec<CashInForecast5>>,
	#[validate]
	#[serde(rename = "CshOutFcst")]
	pub csh_out_fcst: Option<Vec<CashOutForecast5>>,
	#[validate]
	#[serde(rename = "NetCshFcst")]
	pub net_csh_fcst: Option<Vec<NetCashForecast4>>,
}


// CUSIPIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CUSIPIdentifier {
	#[serde(rename = "CUSIPIdentifier")]
	pub cusip_identifier: String,
}


// CashInForecast5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CashInForecast5 {
	#[serde(rename = "CshSttlmDt")]
	pub csh_sttlm_dt: String,
	#[validate]
	#[serde(rename = "SubTtlAmt")]
	pub sub_ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "SubTtlUnitsNb")]
	pub sub_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "XcptnlCshFlowInd")]
	pub xcptnl_csh_flow_ind: Option<bool>,
	#[validate]
	#[serde(rename = "CshInBrkdwnDtls")]
	pub csh_in_brkdwn_dtls: Option<Vec<FundCashInBreakdown3>>,
	#[validate]
	#[serde(rename = "AddtlBal")]
	pub addtl_bal: Option<FundBalance1>,
}


// CashOutForecast5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CashOutForecast5 {
	#[serde(rename = "CshSttlmDt")]
	pub csh_sttlm_dt: String,
	#[validate]
	#[serde(rename = "SubTtlAmt")]
	pub sub_ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "SubTtlUnitsNb")]
	pub sub_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "XcptnlCshFlowInd")]
	pub xcptnl_csh_flow_ind: Option<bool>,
	#[validate]
	#[serde(rename = "CshOutBrkdwnDtls")]
	pub csh_out_brkdwn_dtls: Option<Vec<FundCashOutBreakdown3>>,
	#[validate]
	#[serde(rename = "AddtlBal")]
	pub addtl_bal: Option<FundBalance1>,
}


// Charge26 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Charge26 {
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: ChargeType4Choice,
	#[validate]
	#[serde(rename = "ChrgApld")]
	pub chrg_apld: AmountOrRate3Choice,
}


// ChargeType12Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ChargeType12Code {
	#[validate(enumerate = ["BEND", "DISC", "FEND", "POST", "REGF", "SHIP", "SPCN", "TRAN"])]
	#[serde(rename = "ChargeType12Code")]
	pub charge_type12_code: String,
}


// ChargeType4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ChargeType4Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// Commission21 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Commission21 {
	#[validate]
	#[serde(rename = "ComssnTp")]
	pub comssn_tp: CommissionType5Choice,
	#[validate]
	#[serde(rename = "ComssnApld")]
	pub comssn_apld: AmountOrRate3Choice,
}


// CommissionType5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CommissionType5Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// CommissionType6Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CommissionType6Code {
	#[validate(enumerate = ["FEND", "BEND", "CDPL"])]
	#[serde(rename = "CommissionType6Code")]
	pub commission_type6_code: String,
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


// CurrencyDesignation1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CurrencyDesignation1 {
	#[serde(rename = "CcyDsgnt")]
	pub ccy_dsgnt: Option<String>,
	#[serde(rename = "Lctn")]
	pub lctn: Option<String>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// CurrencyDesignation1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CurrencyDesignation1Code {
	#[validate(enumerate = ["ONSH", "OFFS"])]
	#[serde(rename = "CurrencyDesignation1Code")]
	pub currency_designation1_code: String,
}


// DataFormat2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DataFormat2Choice {
	#[validate]
	#[serde(rename = "Strd")]
	pub strd: Option<GenericIdentification1>,
	#[serde(rename = "Ustrd")]
	pub ustrd: Option<String>,
}


// DateAndDateTimeChoice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateAndDateTimeChoice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<String>,
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


// EuroclearClearstreamIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EuroclearClearstreamIdentifier {
	#[validate(min_length = 1)]
	#[validate(max_length = 12)]
	#[serde(rename = "EuroclearClearstreamIdentifier")]
	pub euroclear_clearstream_identifier: String,
}


// Exact4AlphaNumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[validate(pattern = "[a-zA-Z0-9]{4}")]
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// Extension1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Extension1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: String,
	#[serde(rename = "Txt")]
	pub txt: String,
}


// FinancialInstrument9 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrument9 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: SecurityIdentification3Choice,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "SplmtryId")]
	pub splmtry_id: Option<String>,
	#[serde(rename = "ReqdNAVCcy")]
	pub reqd_nav_ccy: Option<String>,
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


// FlowDirectionType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FlowDirectionType1Code {
	#[validate(enumerate = ["INCG", "OUTG"])]
	#[serde(rename = "FlowDirectionType1Code")]
	pub flow_direction_type1_code: String,
}


// ForeignExchangeTerms19 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ForeignExchangeTerms19 {
	#[serde(rename = "UnitCcy")]
	pub unit_ccy: String,
	#[serde(rename = "QtdCcy")]
	pub qtd_ccy: String,
	#[serde(rename = "XchgRate")]
	pub xchg_rate: f64,
}


// FormOfSecurity1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FormOfSecurity1Code {
	#[validate(enumerate = ["BEAR", "REGD"])]
	#[serde(rename = "FormOfSecurity1Code")]
	pub form_of_security1_code: String,
}


// Fund4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Fund4 {
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "LglNttyIdr")]
	pub lgl_ntty_idr: Option<String>,
	#[validate]
	#[serde(rename = "Id")]
	pub id: Option<OtherIdentification4>,
	#[serde(rename = "Ccy")]
	pub ccy: Option<String>,
	#[validate]
	#[serde(rename = "TtlNAV")]
	pub ttl_nav: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "PrvsTtlNAV")]
	pub prvs_ttl_nav: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "TtlUnitsNb")]
	pub ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[validate]
	#[serde(rename = "PrvsTtlUnitsNb")]
	pub prvs_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "PctgOfFndTtlNAV")]
	pub pctg_of_fnd_ttl_nav: Option<f64>,
}


// FundBalance1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FundBalance1 {
	#[validate]
	#[serde(rename = "TtlUnitsFrUnitOrdrs")]
	pub ttl_units_fr_unit_ordrs: Option<FinancialInstrumentQuantity1>,
	#[validate]
	#[serde(rename = "TtlUnitsFrCshOrdrs")]
	pub ttl_units_fr_csh_ordrs: Option<FinancialInstrumentQuantity1>,
	#[validate]
	#[serde(rename = "TtlCshFrUnitOrdrs")]
	pub ttl_csh_fr_unit_ordrs: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "TtlCshFrCshOrdrs")]
	pub ttl_csh_fr_csh_ordrs: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// FundCashForecast6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FundCashForecast6 {
	#[serde(rename = "Id")]
	pub id: String,
	#[validate]
	#[serde(rename = "TradDtTm")]
	pub trad_dt_tm: DateAndDateTimeChoice,
	#[validate]
	#[serde(rename = "PrvsTradDtTm")]
	pub prvs_trad_dt_tm: Option<DateAndDateTimeChoice>,
	#[validate]
	#[serde(rename = "FinInstrmDtls")]
	pub fin_instrm_dtls: FinancialInstrument9,
	#[validate]
	#[serde(rename = "TtlNAV")]
	pub ttl_nav: Option<Vec<ActiveOrHistoricCurrencyAndAmount>>,
	#[validate]
	#[serde(rename = "PrvsTtlNAV")]
	pub prvs_ttl_nav: Option<Vec<ActiveOrHistoricCurrencyAndAmount>>,
	#[validate]
	#[serde(rename = "TtlUnitsNb")]
	pub ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[validate]
	#[serde(rename = "PrvsTtlUnitsNb")]
	pub prvs_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "TtlNAVChngRate")]
	pub ttl_nav_chng_rate: Option<f64>,
	#[serde(rename = "InvstmtCcy")]
	pub invstmt_ccy: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "CcySts")]
	pub ccy_sts: Option<CurrencyDesignation1>,
	#[serde(rename = "XcptnlNetCshFlowInd")]
	pub xcptnl_net_csh_flow_ind: bool,
	#[validate]
	#[serde(rename = "Pric")]
	pub pric: Option<UnitPrice19>,
	#[validate]
	#[serde(rename = "FXRate")]
	pub fx_rate: Option<ForeignExchangeTerms19>,
	#[serde(rename = "PctgOfShrClssTtlNAV")]
	pub pctg_of_shr_clss_ttl_nav: Option<f64>,
	#[validate]
	#[serde(rename = "BrkdwnByPty")]
	pub brkdwn_by_pty: Option<Vec<BreakdownByParty3>>,
	#[validate]
	#[serde(rename = "BrkdwnByCtry")]
	pub brkdwn_by_ctry: Option<Vec<BreakdownByCountry2>>,
	#[validate]
	#[serde(rename = "BrkdwnByCcy")]
	pub brkdwn_by_ccy: Option<Vec<BreakdownByCurrency2>>,
	#[validate]
	#[serde(rename = "BrkdwnByUsrDfndParam")]
	pub brkdwn_by_usr_dfnd_param: Option<Vec<BreakdownByUserDefinedParameter3>>,
	#[validate]
	#[serde(rename = "NetCshFcstDtls")]
	pub net_csh_fcst_dtls: Option<Vec<NetCashForecast4>>,
}


// FundCashInBreakdown3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FundCashInBreakdown3 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "UnitsNb")]
	pub units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "NewAmtInd")]
	pub new_amt_ind: Option<bool>,
	#[validate]
	#[serde(rename = "InvstmtFndTxInTp")]
	pub invstmt_fnd_tx_in_tp: InvestmentFundTransactionInType1Choice,
	#[validate]
	#[serde(rename = "OrgnlOrdrQtyTp")]
	pub orgnl_ordr_qty_tp: QuantityType1Choice,
	#[validate]
	#[serde(rename = "ChrgDtls")]
	pub chrg_dtls: Option<Vec<Charge26>>,
	#[validate]
	#[serde(rename = "ComssnDtls")]
	pub comssn_dtls: Option<Vec<Commission21>>,
	#[serde(rename = "SttlmCcy")]
	pub sttlm_ccy: Option<String>,
}


// FundCashOutBreakdown3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FundCashOutBreakdown3 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "UnitsNb")]
	pub units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "NewAmtInd")]
	pub new_amt_ind: Option<bool>,
	#[validate]
	#[serde(rename = "InvstmtFndTxOutTp")]
	pub invstmt_fnd_tx_out_tp: InvestmentFundTransactionOutType1Choice,
	#[validate]
	#[serde(rename = "OrgnlOrdrQtyTp")]
	pub orgnl_ordr_qty_tp: QuantityType1Choice,
	#[validate]
	#[serde(rename = "ChrgDtls")]
	pub chrg_dtls: Option<Vec<Charge26>>,
	#[validate]
	#[serde(rename = "ComssnDtls")]
	pub comssn_dtls: Option<Vec<Commission21>>,
	#[serde(rename = "SttlmCcy")]
	pub sttlm_ccy: Option<String>,
}


// FundDetailedConfirmedCashForecastReportV04 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FundDetailedConfirmedCashForecastReportV04 {
	#[validate]
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[validate]
	#[serde(rename = "PoolRef")]
	pub pool_ref: Option<AdditionalReference3>,
	#[validate]
	#[serde(rename = "PrvsRef")]
	pub prvs_ref: Option<Vec<AdditionalReference3>>,
	#[validate]
	#[serde(rename = "RltdRef")]
	pub rltd_ref: Option<Vec<AdditionalReference3>>,
	#[validate]
	#[serde(rename = "MsgPgntn")]
	pub msg_pgntn: Pagination,
	#[validate]
	#[serde(rename = "FndOrSubFndDtls")]
	pub fnd_or_sub_fnd_dtls: Option<Fund4>,
	#[validate]
	#[serde(rename = "FndCshFcstDtls")]
	pub fnd_csh_fcst_dtls: Vec<FundCashForecast6>,
	#[validate]
	#[serde(rename = "CnsltdNetCshFcst")]
	pub cnsltd_net_csh_fcst: Option<NetCashForecast3>,
	#[validate]
	#[serde(rename = "Xtnsn")]
	pub xtnsn: Option<Vec<Extension1>>,
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


// GenericIdentification47 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification47 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
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


// IdentificationSource5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IdentificationSource5Choice {
	#[serde(rename = "DmstIdSrc")]
	pub dmst_id_src: Option<String>,
	#[serde(rename = "PrtryIdSrc")]
	pub prtry_id_src: Option<String>,
}


// InvestmentAccount42 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentAccount42 {
	#[validate]
	#[serde(rename = "AcctId")]
	pub acct_id: Option<AccountIdentification1>,
	#[validate]
	#[serde(rename = "OwnrId")]
	pub ownr_id: Option<PartyIdentification2Choice>,
	#[validate]
	#[serde(rename = "AcctSvcr")]
	pub acct_svcr: Option<PartyIdentification2Choice>,
}


// InvestmentFundTransactionInType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentFundTransactionInType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// InvestmentFundTransactionInType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentFundTransactionInType1Code {
	#[validate(enumerate = ["SUBS", "SWII", "INSP", "CROI", "RDIV"])]
	#[serde(rename = "InvestmentFundTransactionInType1Code")]
	pub investment_fund_transaction_in_type1_code: String,
}


// InvestmentFundTransactionOutType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentFundTransactionOutType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// InvestmentFundTransactionOutType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentFundTransactionOutType1Code {
	#[validate(enumerate = ["REDM", "SWIO", "INSP", "CROO"])]
	#[serde(rename = "InvestmentFundTransactionOutType1Code")]
	pub investment_fund_transaction_out_type1_code: String,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[validate(pattern = "[A-Z0-9]{18,18}[0-9]{2,2}")]
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max140Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 140)]
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
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


// Max4AlphaNumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max4AlphaNumericText {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[validate(pattern = "[a-zA-Z0-9]{1,4}")]
	#[serde(rename = "Max4AlphaNumericText")]
	pub max4_alpha_numeric_text: String,
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


// NetCashForecast3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NetCashForecast3 {
	#[validate]
	#[serde(rename = "NetAmt")]
	pub net_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "NetUnitsNb")]
	pub net_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "FlowDrctn")]
	pub flow_drctn: String,
}


// NetCashForecast4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NetCashForecast4 {
	#[serde(rename = "CshSttlmDt")]
	pub csh_sttlm_dt: String,
	#[validate]
	#[serde(rename = "NetAmt")]
	pub net_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "NetUnitsNb")]
	pub net_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "FlowDrctn")]
	pub flow_drctn: String,
	#[validate]
	#[serde(rename = "AddtlBal")]
	pub addtl_bal: Option<FundBalance1>,
}


// OrderQuantityType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OrderQuantityType2Code {
	#[validate(enumerate = ["UNIT", "CASH"])]
	#[serde(rename = "OrderQuantityType2Code")]
	pub order_quantity_type2_code: String,
}


// OtherIdentification4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OtherIdentification4 {
	#[serde(rename = "Id")]
	pub id: String,
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: IdentificationSource5Choice,
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


// PriceValue1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PriceValue1 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
}


// QUICKIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct QUICKIdentifier {
	#[serde(rename = "QUICKIdentifier")]
	pub quick_identifier: String,
}


// QuantityType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct QuantityType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
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


// SimpleIdentificationInformation ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SimpleIdentificationInformation {
	#[serde(rename = "Id")]
	pub id: String,
}


// TickerIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TickerIdentifier {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "TickerIdentifier")]
	pub ticker_identifier: String,
}


// TypeOfPrice10Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TypeOfPrice10Code {
	#[validate(enumerate = ["BIDE", "OFFR", "NAVL", "CREA", "CANC", "INTE", "SWNG", "MIDD", "RINV", "SWIC", "DDVR", "ACTU"])]
	#[serde(rename = "TypeOfPrice10Code")]
	pub type_of_price10_code: String,
}


// UnitPrice19 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UnitPrice19 {
	#[validate]
	#[serde(rename = "PricTp")]
	pub pric_tp: UnitPriceType2Choice,
	#[validate]
	#[serde(rename = "Val")]
	pub val: PriceValue1,
}


// UnitPriceType2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UnitPriceType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// ValorenIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ValorenIdentifier {
	#[serde(rename = "ValorenIdentifier")]
	pub valoren_identifier: String,
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
