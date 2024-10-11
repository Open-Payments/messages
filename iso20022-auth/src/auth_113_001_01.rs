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
	#[serde(rename = "ActiveCurrencyAnd13DecimalAmount_SimpleType")]
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
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAndAmount_SimpleType")]
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
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// AmountAndDirection53 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection53 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}


// AmountAndDirection61 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection61 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}


// AuctionData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AuctionData2 {
	#[serde(rename = "TradgPhs", skip_serializing_if = "Option::is_none")]
	pub tradg_phs: Option<String>,
	#[serde(rename = "IndctvAuctnPric", skip_serializing_if = "Option::is_none")]
	pub indctv_auctn_pric: Option<SecuritiesTransactionPrice21Choice>,
	#[serde(rename = "IndctvAuctnVol", skip_serializing_if = "Option::is_none")]
	pub indctv_auctn_vol: Option<FinancialInstrumentQuantity25Choice>,
}


// CancelOrderReport1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CancelOrderReport1 {
	#[serde(rename = "RptId")]
	pub rpt_id: String,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DateTimePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// ExecutingParty2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExecutingParty2Choice {
	#[serde(rename = "Prsn", skip_serializing_if = "Option::is_none")]
	pub prsn: Option<GenericPersonIdentification1>,
	#[serde(rename = "Algo", skip_serializing_if = "Option::is_none")]
	pub algo: Option<String>,
	#[serde(rename = "Clnt", skip_serializing_if = "Option::is_none")]
	pub clnt: Option<String>,
}


// ExternalPersonIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPersonIdentification1Code {
	#[serde(rename = "ExternalPersonIdentification1Code")]
	pub external_person_identification1_code: String,
}


// FinancialInstrument99Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument99Choice {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(rename = "StrtgyInstrms", skip_serializing_if = "Option::is_none")]
	pub strtgy_instrms: Option<Vec<String>>,
}


// FinancialInstrumentQuantity25Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentQuantity25Choice {
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
	#[serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none")]
	pub nmnl_val: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// GenericIdentification30 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<String>,
}


// GenericPersonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "ISINOct2015Identifier")]
	pub isin_oct2015_identifier: String,
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


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[serde(rename = "MICIdentifier")]
	pub mic_identifier: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max50Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max50Text {
	#[serde(rename = "Max50Text")]
	pub max50_text: String,
}


// Max52Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max52Text {
	#[serde(rename = "Max52Text")]
	pub max52_text: String,
}


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max5NumericText {
	#[serde(rename = "Max5NumericText")]
	pub max5_numeric_text: String,
}


// MinimumExecutable1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MinimumExecutable1 {
	#[serde(rename = "Sz", skip_serializing_if = "Option::is_none")]
	pub sz: Option<FinancialInstrumentQuantity25Choice>,
	#[serde(rename = "FrstExctnOnly", skip_serializing_if = "Option::is_none")]
	pub frst_exctn_only: Option<bool>,
}


// NewOrderReport2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewOrderReport2 {
	#[serde(rename = "RptId")]
	pub rpt_id: String,
	#[serde(rename = "Ordr")]
	pub ordr: Vec<OrderData3>,
}


// NoReasonCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NoReasonCode {
	#[serde(rename = "NoReasonCode")]
	pub no_reason_code: String,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// OrderBookReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderBookReportV01 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: SecuritiesMarketReportHeader3,
	#[serde(rename = "OrdrRpt")]
	pub ordr_rpt: Vec<OrderReport2Choice>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// OrderClassification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderClassification2 {
	#[serde(rename = "OrdrTp", skip_serializing_if = "Option::is_none")]
	pub ordr_tp: Option<String>,
	#[serde(rename = "OrdrTpClssfctn", skip_serializing_if = "Option::is_none")]
	pub ordr_tp_clssfctn: Option<String>,
}


// OrderData3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderData3 {
	#[serde(rename = "OrdrIdData")]
	pub ordr_id_data: OrderIdentification2,
	#[serde(rename = "AuctnData", skip_serializing_if = "Option::is_none")]
	pub auctn_data: Option<AuctionData2>,
	#[serde(rename = "OrdrData", skip_serializing_if = "Option::is_none")]
	pub ordr_data: Option<OrderData4>,
}


// OrderData4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderData4 {
	#[serde(rename = "SubmitgNtty", skip_serializing_if = "Option::is_none")]
	pub submitg_ntty: Option<String>,
	#[serde(rename = "DrctElctrncAccs", skip_serializing_if = "Option::is_none")]
	pub drct_elctrnc_accs: Option<bool>,
	#[serde(rename = "ClntId", skip_serializing_if = "Option::is_none")]
	pub clnt_id: Option<PersonOrOrganisation4Choice>,
	#[serde(rename = "InvstmtDcsnPrsn", skip_serializing_if = "Option::is_none")]
	pub invstmt_dcsn_prsn: Option<ExecutingParty2Choice>,
	#[serde(rename = "ExctgPrsn", skip_serializing_if = "Option::is_none")]
	pub exctg_prsn: Option<ExecutingParty2Choice>,
	#[serde(rename = "NonExctgBrkr", skip_serializing_if = "Option::is_none")]
	pub non_exctg_brkr: Option<String>,
	#[serde(rename = "TradgCpcty", skip_serializing_if = "Option::is_none")]
	pub tradg_cpcty: Option<String>,
	#[serde(rename = "LqdtyPrvsnActvty", skip_serializing_if = "Option::is_none")]
	pub lqdty_prvsn_actvty: Option<bool>,
	#[serde(rename = "OrdrClssfctn", skip_serializing_if = "Option::is_none")]
	pub ordr_clssfctn: Option<OrderClassification2>,
	#[serde(rename = "OrdrPrics", skip_serializing_if = "Option::is_none")]
	pub ordr_prics: Option<OrderPriceData2>,
	#[serde(rename = "InstrData", skip_serializing_if = "Option::is_none")]
	pub instr_data: Option<OrderInstructionData2>,
	#[serde(rename = "TxData", skip_serializing_if = "Option::is_none")]
	pub tx_data: Option<TransactionData3>,
}


// OrderEventType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderEventType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// OrderEventType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderEventType1Code {
	#[serde(rename = "OrderEventType1Code")]
	pub order_event_type1_code: String,
}


// OrderIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderIdentification2 {
	#[serde(rename = "OrdrBookId")]
	pub ordr_book_id: String,
	#[serde(rename = "SeqNb")]
	pub seq_nb: f64,
	#[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
	pub prty: Option<OrderPriority1>,
	#[serde(rename = "TmStmp")]
	pub tm_stmp: String,
	#[serde(rename = "TradVn")]
	pub trad_vn: String,
	#[serde(rename = "FinInstrm")]
	pub fin_instrm: FinancialInstrument99Choice,
	#[serde(rename = "OrdrId", skip_serializing_if = "Option::is_none")]
	pub ordr_id: Option<String>,
	#[serde(rename = "DtOfRct", skip_serializing_if = "Option::is_none")]
	pub dt_of_rct: Option<String>,
	#[serde(rename = "VldtyPrd", skip_serializing_if = "Option::is_none")]
	pub vldty_prd: Option<ValidityPeriod1Choice>,
	#[serde(rename = "OrdrRstrctn", skip_serializing_if = "Option::is_none")]
	pub ordr_rstrctn: Option<Vec<OrderRestriction1Choice>>,
	#[serde(rename = "VldtyDtTm", skip_serializing_if = "Option::is_none")]
	pub vldty_dt_tm: Option<String>,
	#[serde(rename = "EvtTp", skip_serializing_if = "Option::is_none")]
	pub evt_tp: Option<OrderEventType1Choice>,
}


// OrderInstructionData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderInstructionData2 {
	#[serde(rename = "BuySellInd", skip_serializing_if = "Option::is_none")]
	pub buy_sell_ind: Option<String>,
	#[serde(rename = "OrdrVldtySts", skip_serializing_if = "Option::is_none")]
	pub ordr_vldty_sts: Option<String>,
	#[serde(rename = "OrdrSts", skip_serializing_if = "Option::is_none")]
	pub ordr_sts: Option<Vec<String>>,
	#[serde(rename = "InitlQty", skip_serializing_if = "Option::is_none")]
	pub initl_qty: Option<FinancialInstrumentQuantity25Choice>,
	#[serde(rename = "RmngQty", skip_serializing_if = "Option::is_none")]
	pub rmng_qty: Option<FinancialInstrumentQuantity25Choice>,
	#[serde(rename = "DispdQty", skip_serializing_if = "Option::is_none")]
	pub dispd_qty: Option<FinancialInstrumentQuantity25Choice>,
	#[serde(rename = "MinAccptblQty", skip_serializing_if = "Option::is_none")]
	pub min_accptbl_qty: Option<FinancialInstrumentQuantity25Choice>,
	#[serde(rename = "MinExctbl", skip_serializing_if = "Option::is_none")]
	pub min_exctbl: Option<MinimumExecutable1>,
	#[serde(rename = "PssvOnlyInd", skip_serializing_if = "Option::is_none")]
	pub pssv_only_ind: Option<bool>,
	#[serde(rename = "SlfExctnPrvntn", skip_serializing_if = "Option::is_none")]
	pub slf_exctn_prvntn: Option<bool>,
	#[serde(rename = "RtgStrtgy", skip_serializing_if = "Option::is_none")]
	pub rtg_strtgy: Option<String>,
}


// OrderPriceData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderPriceData2 {
	#[serde(rename = "LmtPric", skip_serializing_if = "Option::is_none")]
	pub lmt_pric: Option<SecuritiesTransactionPrice2Choice>,
	#[serde(rename = "StopPric", skip_serializing_if = "Option::is_none")]
	pub stop_pric: Option<SecuritiesTransactionPrice2Choice>,
	#[serde(rename = "AddtlLmtPric", skip_serializing_if = "Option::is_none")]
	pub addtl_lmt_pric: Option<SecuritiesTransactionPrice2Choice>,
	#[serde(rename = "PggdPric", skip_serializing_if = "Option::is_none")]
	pub pggd_pric: Option<SecuritiesTransactionPrice2Choice>,
	#[serde(rename = "CcyScndLeg", skip_serializing_if = "Option::is_none")]
	pub ccy_scnd_leg: Option<String>,
}


// OrderPriority1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderPriority1 {
	#[serde(rename = "TmStmp", skip_serializing_if = "Option::is_none")]
	pub tm_stmp: Option<String>,
	#[serde(rename = "Sz", skip_serializing_if = "Option::is_none")]
	pub sz: Option<f64>,
}


// OrderReport2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderReport2Choice {
	#[serde(rename = "New", skip_serializing_if = "Option::is_none")]
	pub new: Option<NewOrderReport2>,
	#[serde(rename = "Cxl", skip_serializing_if = "Option::is_none")]
	pub cxl: Option<CancelOrderReport1>,
}


// OrderRestriction1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderRestriction1Choice {
	#[serde(rename = "OrdrRstrctnCd", skip_serializing_if = "Option::is_none")]
	pub ordr_rstrctn_cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// OrderRestrictionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderRestrictionType1Code {
	#[serde(rename = "OrderRestrictionType1Code")]
	pub order_restriction_type1_code: String,
}


// OrderStatus10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderStatus10Code {
	#[serde(rename = "OrderStatus10Code")]
	pub order_status10_code: String,
}


// OrderStatus11Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderStatus11Code {
	#[serde(rename = "OrderStatus11Code")]
	pub order_status11_code: String,
}


// OrderType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderType3Code {
	#[serde(rename = "OrderType3Code")]
	pub order_type3_code: String,
}


// Pagination1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: String,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}


// PartyExceptionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyExceptionType1Code {
	#[serde(rename = "PartyExceptionType1Code")]
	pub party_exception_type1_code: String,
}


// PassiveOrAgressiveType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PassiveOrAgressiveType1Code {
	#[serde(rename = "PassiveOrAgressiveType1Code")]
	pub passive_or_agressive_type1_code: String,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// Period11Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period11Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
	pub to_dt: Option<String>,
	#[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt: Option<Period2>,
	#[serde(rename = "FrToDtTm", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt_tm: Option<DateTimePeriod1>,
}


// Period2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// PersonIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// PersonOrOrganisation4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonOrOrganisation4Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
	#[serde(rename = "Prsn", skip_serializing_if = "Option::is_none")]
	pub prsn: Option<GenericPersonIdentification1>,
	#[serde(rename = "XcptnId", skip_serializing_if = "Option::is_none")]
	pub xcptn_id: Option<String>,
}


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "PlusOrMinusIndicator")]
	pub plus_or_minus_indicator: bool,
}


// PositiveNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositiveNumber {
	#[serde(rename = "PositiveNumber")]
	pub positive_number: f64,
}


// PriceStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceStatus1Code {
	#[serde(rename = "PriceStatus1Code")]
	pub price_status1_code: String,
}


// RegulatoryTradingCapacity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegulatoryTradingCapacity1Code {
	#[serde(rename = "RegulatoryTradingCapacity1Code")]
	pub regulatory_trading_capacity1_code: String,
}


// SecuritiesMarketReportHeader3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesMarketReportHeader3 {
	#[serde(rename = "RptgNtty")]
	pub rptg_ntty: TradingVenueIdentification1Choice,
	#[serde(rename = "RptgPrd")]
	pub rptg_prd: Period11Choice,
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<Vec<String>>,
	#[serde(rename = "SubmissnDtTm", skip_serializing_if = "Option::is_none")]
	pub submissn_dt_tm: Option<String>,
	#[serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none")]
	pub msg_pgntn: Option<Pagination1>,
	#[serde(rename = "NbRcrds", skip_serializing_if = "Option::is_none")]
	pub nb_rcrds: Option<f64>,
}


// SecuritiesTransactionPrice1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice1 {
	#[serde(rename = "Pdg")]
	pub pdg: String,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<String>,
}


// SecuritiesTransactionPrice21Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice21Choice {
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<AmountAndDirection53>,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
	#[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
	pub yld: Option<f64>,
	#[serde(rename = "BsisPts", skip_serializing_if = "Option::is_none")]
	pub bsis_pts: Option<f64>,
	#[serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none")]
	pub nmnl_val: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// SecuritiesTransactionPrice2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice2Choice {
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<AmountAndDirection61>,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
	#[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
	pub yld: Option<f64>,
	#[serde(rename = "BsisPts", skip_serializing_if = "Option::is_none")]
	pub bsis_pts: Option<f64>,
}


// SecuritiesTransactionPrice4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice4Choice {
	#[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
	pub pric: Option<SecuritiesTransactionPrice2Choice>,
	#[serde(rename = "NoPric", skip_serializing_if = "Option::is_none")]
	pub no_pric: Option<SecuritiesTransactionPrice1>,
}


// Side6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Side6Code {
	#[serde(rename = "Side6Code")]
	pub side6_code: String,
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<String>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// TradingVenue2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenue2Code {
	#[serde(rename = "TradingVenue2Code")]
	pub trading_venue2_code: String,
}


// TradingVenueIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueIdentification1Choice {
	#[serde(rename = "MktIdCd", skip_serializing_if = "Option::is_none")]
	pub mkt_id_cd: Option<String>,
	#[serde(rename = "NtlCmptntAuthrty", skip_serializing_if = "Option::is_none")]
	pub ntl_cmptnt_authrty: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<TradingVenueIdentification2>,
}


// TradingVenueIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueIdentification2 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Tp")]
	pub tp: String,
}


// TransactionData3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionData3 {
	#[serde(rename = "TxPric", skip_serializing_if = "Option::is_none")]
	pub tx_pric: Option<SecuritiesTransactionPrice4Choice>,
	#[serde(rename = "TraddQty", skip_serializing_if = "Option::is_none")]
	pub tradd_qty: Option<FinancialInstrumentQuantity25Choice>,
	#[serde(rename = "PssvOrAggrssvInd", skip_serializing_if = "Option::is_none")]
	pub pssv_or_aggrssv_ind: Option<String>,
	#[serde(rename = "StrtgyLkdOrdrId", skip_serializing_if = "Option::is_none")]
	pub strtgy_lkd_ordr_id: Option<String>,
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<String>,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// ValidityPeriod1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValidityPeriod1Choice {
	#[serde(rename = "VldtyPrdCd", skip_serializing_if = "Option::is_none")]
	pub vldty_prd_cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// ValidityPeriodType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValidityPeriodType1Code {
	#[serde(rename = "ValidityPeriodType1Code")]
	pub validity_period_type1_code: String,
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}
